use super::model::{Level, LevelInfo, PlayerCombat, PlayerScore, PlayerStat, ProbeStat, RoomType};
use super::{Ge, VertexIDType};

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;
use structopt::StructOpt;

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::i16;
use std::io;
use std::io::Write;
use std::ops::{AddAssign, SubAssign};
use std::rc::Rc;
use std::time::Instant;
use std::u8;

// An iterator for DenseBitSet that returns the position of each enabled bit in the set
struct BitSetIter(BitSet);

impl From<BitSet> for BitSetIter {
    fn from(bitset: BitSet) -> Self {
        Self(bitset)
    }
}

impl Iterator for BitSetIter {
    type Item = u8;

    // Return position of next enabled bit in set
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.none() {
            None
        } else {
            let first_set = self.0.first_set();
            assert!(first_set < 64);
            self.0.set_bit(first_set as usize, false);
            Some(first_set as u8)
        }
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct PlayerProgress {
    visited: BitSet,

    #[cfg(feature = "closed-level")]
    memory: BitSet,
}

impl SubAssign<&Self> for PlayerProgress {
    fn sub_assign(&mut self, other: &Self) {
        self.visited ^= other.visited;

        #[cfg(feature = "closed-level")]
        {
            self.memory ^= other.memory;
        }
    }
}

impl SubAssign<&PlayerProgressDiff> for PlayerProgress {
    fn sub_assign(&mut self, diff: &PlayerProgressDiff) {
        #[cfg(not(feature = "closed-level"))]
        self.visited.set_bit(diff.location as usize, false);

        *self -= &diff.progress;
    }
}

impl Display for PlayerProgress {
    #[cfg(feature = "closed-level")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "visited: {}, memory: {}",
            self.visited.to_integer(),
            self.memory.to_integer()
        )
    }

    #[cfg(not(feature = "closed-level"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "visited: {}", self.visited.to_integer())
    }
}

#[derive(Clone)]
struct PlayerProgressDiff {
    progress: PlayerProgress,
    location: VertexIDType,
}

impl PlayerProgressDiff {
    fn new() -> Self {
        Self {
            progress: PlayerProgress::default(),
            location: u8::MAX,
        }
    }
}

impl Default for PlayerProgressDiff {
    fn default() -> Self {
        Self {
            progress: PlayerProgress::default(),
            location: u8::MAX,
        }
    }
}

#[derive(Clone, Default)]
pub struct Player {
    stat: PlayerStat,
    progress: PlayerProgress,
    diff: PlayerProgressDiff,
    neighbors: BitSet,

    #[cfg(feature = "closed-level")]
    disabled: BitSet,
}

impl Player {
    pub fn new(hp: i32, atk: i16, def: i16) -> Self {
        Self {
            stat: PlayerStat::with_stat(hp, atk, def),
            progress: PlayerProgress::default(),
            diff: PlayerProgressDiff::new(),
            neighbors: BitSet::new(),

            #[cfg(feature = "closed-level")]
            disabled: BitSet::new(),
        }
    }

    fn reverted_progress(&self) -> PlayerProgress {
        let mut progress = self.progress.clone();
        progress -= &self.diff;
        progress
    }

    fn enter(&mut self, level: &Level) {
        self.neighbors.set_bit(level.entrance as usize, true);
    }

    #[cfg(feature = "closed-level")]
    fn visit(&mut self, location: VertexIDType, level: &Level, probe: &ProbeStat) {
        let loc_idx = location as usize;
        *self += &probe.diff;
        let old_memory = self.progress.memory;
        let old_visited = self.progress.visited;
        self.disabled ^= level.neighbors[loc_idx];

        let enabled_boundary = level.boundary_mask & !self.disabled;
        if enabled_boundary.get_bit(loc_idx)
            || enabled_boundary.get_bit(self.diff.location as usize)
            || level
                .vertex_of_id(location)
                .room_type
                .contains(RoomType::CLEAR_NEIGHBORS)
        {
            self.progress.memory |= self.progress.visited;
            self.progress.memory &= !enabled_boundary;
            self.neighbors.reset();
            self.progress.visited.reset();
        }

        let mut explore = BitSet::new();
        explore.set_bit(loc_idx, true);
        while explore.any() {
            let v = explore.first_set() as usize;
            self.progress.visited.set_bit(v, true);
            self.neighbors |= level.neighbors[v] & !self.disabled;
            let memory_visited = self.progress.memory & self.neighbors;
            self.progress.memory &= !memory_visited;
            explore |= memory_visited;
            explore &= !self.progress.visited;
        }

        self.neighbors &= !self.progress.visited;
        self.neighbors &= !self.disabled;
        self.diff.location = location;
        self.diff.progress.memory = old_memory ^ self.progress.memory;
        self.diff.progress.visited = old_visited ^ self.progress.visited;
    }

    #[cfg(not(feature = "closed-level"))]
    fn visit(&mut self, location: VertexIDType, level: &Level, probe: &ProbeStat) {
        let loc_idx = location as usize;
        *self += &probe.diff;
        self.progress.visited.set_bit(loc_idx, true);
        self.neighbors |= level.neighbors[loc_idx];
        self.neighbors &= !level.toggle_neighbors[loc_idx];
        self.neighbors |= self.progress.visited;
        self.diff.location = location;
    }

    // TODO support other score functions
    fn score(&self) -> PlayerScore {
        let stat = &self.stat;
        let combat = stat.as_ref();
        let score = (stat.hp + 1) * 25
            + (combat.atk as i32 * 5
                + combat.def as i32 * 3
                + stat.yk as i32 * 10
                + stat.gk as i32 * 20
                + stat.bk as i32 * 30)
                * 1000;
        PlayerScore { score }
    }

    fn print_room_list(writer: &mut dyn Write, level: &Level, list: BitSet) -> io::Result<()> {
        let mut first = true;
        for id in BitSetIter::from(list) {
            if first {
                first = false;
            } else {
                write!(writer, ", ")?;
            }
            write!(writer, "{}", level.vertex_of_id(id).name)?;
        }
        writeln!(writer, "")
    }

    fn print(&self, writer: &mut dyn Write, level: &Level) -> io::Result<()> {
        write!(
            writer,
            "Score: {}\n{{{}}}\nNeighbours: ",
            self.score(),
            self
        )?;
        Self::print_room_list(writer, level, self.neighbors)?;
        #[cfg(feature = "closed-level")]
        {
            write!(writer, "Visited: ")?;
            Self::print_room_list(writer, level, self.progress.visited)?;
            write!(writer, "Memory: ")?;
            Self::print_room_list(writer, level, self.progress.memory)?;
        }
        Ok(())
    }
}

impl Ge<PlayerStat> for Player {
    fn ge(&self, stat: &PlayerStat) -> bool {
        self.stat.ge(stat)
    }
}

impl AddAssign<&PlayerStat> for Player {
    fn add_assign(&mut self, stat: &PlayerStat) {
        self.stat += stat;
    }
}

impl Display for Player {
    #[cfg(feature = "closed-level")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, disabled: {}, neighbors: {}",
            self.stat,
            self.progress,
            self.disabled.to_integer(),
            self.neighbors.to_integer()
        )
    }

    #[cfg(not(feature = "closed-level"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, neighbors: {}",
            self.stat,
            self.progress,
            self.neighbors.to_integer()
        )
    }
}

#[derive(Clone)]
struct PlayerTrace {
    level_config: i32,
    level: Rc<Level>,
    player: Player,
    trace: Vec<VertexIDType>,
}

impl PlayerTrace {
    fn new() -> Self {
        Self {
            level_config: 0,
            level: Rc::new(Level::new()),
            player: Player::new(500, 10, 10),
            trace: Vec::new(),
        }
    }

    fn visit(&mut self, location: VertexIDType) {
        self.player.visit(
            location,
            &self.level,
            &self
                .level
                .vertex_of_id(location)
                .probe(self.player.stat.as_ref()),
        );
        self.trace.push(location);
    }

    fn print_room_list(writer: &mut dyn Write, level: &Level, list: &Vec<u8>) -> io::Result<()> {
        let mut first = true;
        for id in list {
            if first {
                first = false;
            } else {
                write!(writer, ", ")?;
            }
            write!(writer, "{}", level.vertex_of_id(*id).name)?;
        }
        writeln!(writer, "")
    }

    fn write(&self, writer: &mut dyn Write) -> io::Result<()> {
        self.player.print(writer, &self.level)?;
        write!(writer, "Trace: ")?;
        Self::print_room_list(writer, &self.level, &self.trace)
    }

    fn print(&self, writer: &mut dyn Write, init_player: &Player) -> io::Result<()> {
        let mut player = PlayerTrace {
            level_config: self.level_config,
            level: Rc::clone(&self.level),
            player: init_player.clone(),
            trace: Vec::new(),
        };

        self.write(writer)?;
        writeln!(
            writer,
            "--------------------------------------------------------------------------------"
        )?;
        for id in &self.trace {
            player.visit(*id);
            player.write(writer)?;
            writeln!(
                writer,
                "--------------------------------------------------------------------------------"
            )?;
        }
        Ok(())
    }
}

// Track pareto frontier of traces by stat
struct OptimalStatSet {
    trace: Vec<PlayerTrace>,
}

impl OptimalStatSet {
    fn new() -> Self {
        Self { trace: Vec::new() }
    }

    fn addable(&self, stat: &PlayerStat) -> bool {
        self.trace.iter().all(|trace| !trace.player.stat.ge(stat))
    }

    fn add(&mut self, trace: PlayerTrace, force: bool) -> bool {
        let new_stat = &trace.player.stat;
        if force || self.addable(&new_stat) {
            self.trace.retain(|trace| !new_stat.ge(&trace.player.stat));
            self.trace.push(trace);
            true
        } else {
            false
        }
    }

    fn add_all(&mut self, other: &Self) {
        for trace in &other.trace {
            self.add(trace.clone(), false);
        }
    }
}

// Track trace with optimal score
struct OptimalScore {
    trace: PlayerTrace,
    score: PlayerScore,
}

impl OptimalScore {
    fn new() -> Self {
        Self {
            trace: PlayerTrace::new(),
            score: PlayerScore::new(),
        }
    }

    fn addable(&self, score: &PlayerScore) -> bool {
        !self.score.ge(score)
    }

    fn add_all(&mut self, other: &Self) -> bool {
        if self.score.ge(&other.score) {
            false
        } else {
            self.trace = other.trace.clone();
            self.score = other.score.clone();
            true
        }
    }

    fn add(&mut self, trace: PlayerTrace, force: bool) -> bool {
        let score = trace.player.score();
        if force || self.addable(&score) {
            self.trace = trace;
            self.score = score;
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.trace = PlayerTrace::new();
        self.score = PlayerScore::new();
    }
}

#[derive(StructOpt)]
pub struct SearchConfig {
    /// Estimate when rooms cannot be improved by increasing stats
    #[structopt(
        name = "use_max_combat",
        long,
        default_value = "true",
        parse(try_from_str)
    )]
    use_estimated_max_combat: bool,

    /// Output new highscores when reaching exit room
    #[structopt(long, default_value = "false", parse(try_from_str))]
    print_new_highscore: bool,

    /// Calculate scores for the pareto optimal set of stats and keys
    #[structopt(
        name = "calculate_by_stat",
        long,
        default_value = "false",
        parse(try_from_str)
    )]
    calculate_optimal_player_by_stat: bool,

    /// Output optimal scores for each level config
    #[structopt(
        name = "print_local_score",
        long,
        default_value = "false",
        parse(try_from_str)
    )]
    print_local_optimal_player_by_score: bool,

    /// Output pareto scores for each level config
    #[structopt(
        name = "print_local_stat",
        long,
        default_value = "false",
        parse(try_from_str)
    )]
    print_local_optimal_player_by_stat: bool,

    /// Output optimal score across all level configs
    #[structopt(
        name = "print_global_score",
        long,
        default_value = "true",
        parse(try_from_str)
    )]
    print_global_optimal_player_by_score: bool,

    /// Output pareto scores across all level configs
    #[structopt(
        name = "print_global_stat",
        long,
        default_value = "false",
        parse(try_from_str)
    )]
    print_global_optimal_player_by_stat: bool,
}

struct SearchProgress {
    total_search_count: usize,
    current_search_count: usize,
    timer_begin: Instant,
}

impl SearchProgress {
    fn new() -> Self {
        Self {
            total_search_count: 0,
            current_search_count: 0,
            timer_begin: Instant::now(),
        }
    }
}

struct ExtendedProbeStat {
    location: VertexIDType,
    probe: ProbeStat,
}

pub struct Search<'a> {
    search_config: SearchConfig,
    level_info: LevelInfo,
    init_player: Player,
    max_combat_probe_result: Vec<ProbeStat>,
    search_progress: SearchProgress,
    level_config: i32,
    level: Rc<Level>,
    local_optimal_player_by_score: OptimalScore,
    global_optimal_player_by_score: OptimalScore,
    local_optimal_player_by_stat: OptimalStatSet,
    global_optimal_player_by_stat: OptimalStatSet,
    probe_result: HashMap<PlayerCombat, Vec<ProbeStat>>,
    player_progress_rc: HashMap<PlayerProgress, i32>,
    optimal_player: HashMap<PlayerProgress, Player>, // TODO only store objective, diff?
    clones: VecDeque<PlayerProgress>,
    writer: &'a mut dyn Write,
    log_writer: &'a mut dyn Write,
}

impl<'a> Search<'a> {
    pub fn new(
        search_config: SearchConfig,
        level_info: LevelInfo,
        writer: &'a mut dyn Write,
        log_writer: &'a mut dyn Write,
    ) -> Self {
        let init_player = Player {
            stat: level_info.init_player(),
            ..Default::default()
        };
        Self {
            search_config,
            level_info,
            init_player,
            max_combat_probe_result: Vec::new(),
            search_progress: SearchProgress::new(),
            level_config: 0,
            level: Rc::new(Level::new()),
            local_optimal_player_by_score: OptimalScore::new(),
            global_optimal_player_by_score: OptimalScore::new(),
            local_optimal_player_by_stat: OptimalStatSet::new(),
            global_optimal_player_by_stat: OptimalStatSet::new(),
            probe_result: HashMap::new(),
            player_progress_rc: HashMap::new(),
            optimal_player: HashMap::new(),
            clones: VecDeque::new(),
            writer,
            log_writer,
        }
    }

    fn probe(&mut self, combat: &PlayerCombat) -> &Vec<ProbeStat> {
        if !self.probe_result.contains_key(combat) {
            let mut res = Vec::new();
            for i in 0..self.level.next_id {
                res.push(self.level.vertex_of_id(i).probe(combat));
            }
            self.probe_result.insert(combat.clone(), res);
        }
        self.probe_result.get(combat).unwrap()
    }

    fn add_exit_player(&mut self, player: &Player) -> io::Result<()> {
        if self.search_config.calculate_optimal_player_by_stat {
            if self.local_optimal_player_by_stat.addable(&player.stat) {
                let player_trace = self.reconstruct_trace(player);
                self.local_optimal_player_by_stat.add(player_trace, true);
            } else {
                return Ok(());
            }
        }

        if self.local_optimal_player_by_score.addable(&player.score()) {
            let player_trace = self.reconstruct_trace(player);
            if self.search_config.print_new_highscore {
                write!(self.writer, "New High ")?;
                player_trace.write(self.writer)?;
                writeln!(self.writer, "--------------------------------------------------------------------------------")?;
            }
            self.local_optimal_player_by_score.add(player_trace, true);
        }
        Ok(())
    }

    fn remove_player_progress(&mut self, progress: PlayerProgress) {
        let mut progress = progress;
        while progress != self.init_player.progress {
            let rc = self
                .player_progress_rc
                .get_mut(&progress)
                .expect("player_progress_rc missing progress");
            *rc -= 1;
            if *rc > 0 {
                return;
            }
            let diff = self
                .optimal_player
                .get(&progress)
                .expect("optimal_player missing progress")
                .diff
                .clone();
            self.optimal_player.remove_entry(&progress);
            progress -= &diff;
        }
    }

    fn expand(
        &mut self,
        player: &Player,
        location: VertexIDType,
        probe: &ProbeStat,
    ) -> io::Result<()> {
        let mut new_player = player.clone();
        new_player.visit(location, &self.level, probe);

        if location == self.level.exit {
            self.add_exit_player(&new_player)?;
            return Ok(());
        }

        let new_progress = new_player.progress.clone();
        if self.optimal_player.contains_key(&new_progress) {
            let new_objective = new_player.stat.objective();
            let player = self
                .optimal_player
                .get_mut(&new_progress)
                .expect("optimal_player missing new_progress");
            if player.stat.objective().ge(&new_objective) {
                return Ok(());
            }
            match self.player_progress_rc.get_mut(&player.progress) {
                Some(rc) => {
                    *rc += 1;
                }
                None => {
                    self.player_progress_rc.insert(player.progress.clone(), 1);
                }
            }
            player.stat.hp = new_objective.hp;
            player.diff = new_player.diff;
            let progress = player.reverted_progress();
            self.remove_player_progress(progress);
        } else {
            self.optimal_player.insert(new_progress.clone(), new_player);
            match self.player_progress_rc.get_mut(&player.progress) {
                Some(rc) => {
                    *rc += 1;
                }
                None => {
                    self.player_progress_rc.insert(player.progress.clone(), 1);
                }
            }
            self.clones.push_back(new_progress);
            self.search_progress.total_search_count += 1;
        }
        Ok(())
    }

    fn reconstruct_trace(&self, player: &Player) -> PlayerTrace {
        let mut trace = Vec::new();
        let mut diff = player.diff.clone();
        let mut progress = player.progress.clone();
        while progress != self.init_player.progress {
            trace.push(diff.location);
            progress -= &diff;
            diff = self
                .optimal_player
                .get(&progress)
                .expect("optimal_player missing progress")
                .diff
                .clone();
        }
        trace.reverse();
        PlayerTrace {
            level_config: self.level_config,
            level: Rc::clone(&self.level),
            player: player.clone(),
            trace,
        }
    }

    fn estimate_max_combat(&mut self) -> io::Result<()> {
        let mut max_combat = PlayerCombat::with_stat(i16::MAX, i16::MAX);
        let mut stat = PlayerStat::default();
        for i in 0..self.level.next_id {
            let probe = self.level.vertex_of_id(i).probe(&max_combat);
            let mut diff = PlayerStat::default();
            diff.join(probe.diff);
            stat += &diff;
        }
        max_combat = self.init_player.stat.as_ref().clone();
        max_combat += stat.as_ref();
        write!(self.writer, "Estimated {}", max_combat)?;
        write!(self.log_writer, "Estimated {}", max_combat)?;
        self.max_combat_probe_result = self.probe(&max_combat).clone();
        Ok(())
    }

    fn print_progress(&mut self) -> io::Result<()> {
        if self.search_progress.current_search_count % 1000000 == 0
            && self.search_progress.timer_begin.elapsed().as_secs() > 10
        {
            self.search_progress.timer_begin = Instant::now();
            writeln!(
                self.log_writer,
                "Progress: {}m / {}m",
                self.search_progress.current_search_count / 1000000,
                self.search_progress.total_search_count / 1000000
            )?;
        }
        Ok(())
    }

    fn search_config(&mut self, config: i32) -> io::Result<()> {
        self.search_progress = SearchProgress::new();
        self.local_optimal_player_by_score.clear();
        self.local_optimal_player_by_stat.trace.clear();
        self.probe_result.clear();
        self.player_progress_rc.clear();
        self.optimal_player.clear();

        self.level_config = config;
        self.level = Rc::new(self.level_info.build(config));
        // TODO check for errors when building level

        let mut player = self.init_player.clone();
        player.enter(&self.level);
        let player_progress = player.progress.clone();
        self.optimal_player.insert(player_progress.clone(), player);
        self.clones.push_back(player_progress);
        self.search_progress.total_search_count += 1;

        if self.search_config.use_estimated_max_combat {
            self.estimate_max_combat()?;
        }

        while let Some(progress) = self.clones.pop_front() {
            self.search_progress.current_search_count += 1;
            self.print_progress()?;

            let player = self
                .optimal_player
                .get(&progress)
                .expect("optimal_player missing progress")
                .clone();
            self.player_progress_rc.insert(progress.clone(), 0);

            let probe_result = self.probe(player.stat.as_ref()).clone();
            let mut extended_probe_result =
                Vec::with_capacity(player.neighbors.get_weight() as usize);
            let was_intermediate = if player.diff.location == u8::MAX {
                false
            } else {
                self.level
                    .vertex_of_id(player.diff.location)
                    .room_type
                    .contains(RoomType::INTERMEDIATE)
            };

            let mut has_free_priority = false;
            for id in BitSetIter::from(player.neighbors) {
                if was_intermediate
                    && !self.level.neighbors[player.diff.location as usize].get_bit(id as usize)
                {
                    continue;
                }
                let probe = &probe_result[id as usize];
                if !player.ge(&probe.req) {
                    continue;
                }
                let room_type = self.level.vertex_of_id(id).room_type;
                let priority = room_type.contains(RoomType::PRIORITY);
                let intermediate = room_type.contains(RoomType::INTERMEDIATE);
                let mut free = self.search_config.use_estimated_max_combat
                    && id != self.level.exit
                    && !intermediate
                    && (probe.diff.as_ref().flag & player.stat.as_ref().flag).bits() == 0
                    && !room_type.contains(RoomType::DELAYED)
                    && self.max_combat_probe_result[id as usize].diff.objective()
                        == probe.diff.objective()
                    && probe.diff.nonnegative();

                #[cfg(feature = "closed-level")]
                {
                    free = free
                        && !room_type.contains(RoomType::REPEATED)
                        && !self.level.boundary_mask.get_bit(id as usize)
                        && !self
                            .level
                            .boundary_mask
                            .get_bit(player.diff.location as usize);
                }
                if !free
                    && self
                        .level
                        .vertex_of_id(id)
                        .room_type
                        .contains(RoomType::ONLY_WHEN_FREE)
                {
                    continue;
                }
                if free || priority {
                    has_free_priority = true;
                    self.expand(&player, id, &probe)?;
                    break;
                }
                extended_probe_result.push(ExtendedProbeStat {
                    location: id,
                    probe: probe.clone(),
                })
            }
            if !has_free_priority {
                for probe in extended_probe_result {
                    self.expand(&player, probe.location, &probe.probe)?;
                }
            }
            if *self
                .player_progress_rc
                .get(&progress)
                .expect("rc missing progress")
                == 0
            {
                self.remove_player_progress(progress);
            }
        }
        self.global_optimal_player_by_score
            .add_all(&self.local_optimal_player_by_score);
        if self.search_config.calculate_optimal_player_by_stat {
            self.global_optimal_player_by_stat
                .add_all(&self.local_optimal_player_by_stat);
        }
        Ok(())
    }

    pub fn search(&mut self) -> io::Result<()> {
        for config in 0..self.level_info.max_config_number {
            writeln!(self.log_writer, "Config:")?;
            self.level_info.print_config(self.log_writer, config);

            writeln!(
                self.writer,
                "================================================================================\n\
                 Config:"
            )?;
            self.level_info.print_config(self.writer, config);
            self.writer.flush()?;

            let begin = Instant::now();
            self.search_config(config)?;

            let elapsed_secs = begin.elapsed().as_secs();
            writeln!(
                self.log_writer,
                "There are {} situations searched.\n\
                 Finished searching in {} seconds.",
                self.search_progress.total_search_count, elapsed_secs
            )?;

            writeln!(
                self.writer,
                "================================================================================\n\
                 There are {} situations searched.\n\
                 Finished searching in {} seconds.",
                 self.search_progress.total_search_count,
                 elapsed_secs
            )?;

            if self.search_config.print_local_optimal_player_by_score {
                if self.local_optimal_player_by_score.score.score > 0 {
                    writeln!(
                        self.writer,
                        "The local optimal player by score is: \n\
                        --------------------------------------------------------------------------------"
                    )?;
                    self.local_optimal_player_by_score
                        .trace
                        .print(self.writer, &self.init_player)?;
                } else {
                    writeln!(
                        self.writer,
                        "It is impossible to reach exit with this config.\n\
                        ================================================================================"
                    )?;
                }
            }

            if self.search_config.print_local_optimal_player_by_stat {
                writeln!(
                    self.writer,
                    "================================================================================\n\
                     There are {} local optimal players by stats.\n\
                     ================================================================================\n\
                     --------------------------------------------------------------------------------",
                     self.local_optimal_player_by_stat.trace.len()
                )?;

                for (i, trace) in self.local_optimal_player_by_stat.trace.iter().enumerate() {
                    write!(self.writer, "Local optimal player by score [{}] ", i + 1,)?;
                    trace.write(self.writer)?;
                    writeln!(
                        self.writer,
                        "--------------------------------------------------------------------------------"
                    )?;
                }
                writeln!(
                    self.writer,
                    "================================================================================"
                )?;
            }

            self.writer.flush()?;
        }

        if self.search_config.print_global_optimal_player_by_score {
            writeln!(
                self.log_writer,
                "--------------------------------------------------------------------------------\n\
                The global optimal player by score is: "
            )?;
            self.level_info.print_config(
                self.log_writer,
                self.global_optimal_player_by_score.trace.level_config,
            );
            self.global_optimal_player_by_score
                .trace
                .write(self.log_writer)?;
            writeln!(
                self.log_writer,
                "--------------------------------------------------------------------------------"
            )?;

            writeln!(
                self.writer,
                "////////////////////////////////////////////////////////////////////////////////\n\
                The global optimal player by score is: "
            )?;
            self.level_info.print_config(
                self.writer,
                self.global_optimal_player_by_score.trace.level_config,
            );
            writeln!(
                self.writer,
                "--------------------------------------------------------------------------------"
            )?;
            self.global_optimal_player_by_score
                .trace
                .print(self.writer, &self.init_player)?;
        }

        if self.search_config.print_global_optimal_player_by_stat {
            writeln!(
                self.log_writer,
                "There are {} global optimal players by stat.",
                self.global_optimal_player_by_stat.trace.len()
            )?;

            writeln!(
                self.writer,
                "////////////////////////////////////////////////////////////////////////////////\n\
                There are {} global optimal players by stat.\n\
                ////////////////////////////////////////////////////////////////////////////////\n\
                --------------------------------------------------------------------------------",
                self.global_optimal_player_by_stat.trace.len()
            )?;
            for (i, trace) in self.global_optimal_player_by_stat.trace.iter().enumerate() {
                write!(self.writer, "Global optimal player by score [{}] ", i + 1,)?;
                self.level_info
                    .print_config(self.writer, trace.level_config);
                trace.write(self.writer)?;
                writeln!(
                    self.writer,
                    "--------------------------------------------------------------------------------"
                )?;
            }
            writeln!(
                self.writer,
                "////////////////////////////////////////////////////////////////////////////////"
            )?;
        }
        self.writer.flush()?;
        Ok(())
    }
}
