#[macro_use]
extern crate bitflags;
extern crate rust_dense_bitset;

mod drod;
// use drod::{Level, PlayerStat, Solver};

use rust_dense_bitset::DenseBitSet as BitSet;

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::Write;
use std::time::Instant;

type VertexIDType = u8;

struct Level {}

// TODO support multiple configs
struct LevelInfo {
    max_config_number: i32,
}

impl LevelInfo {
    fn new() -> Self {
        Self {
            max_config_number: 1,
        }
    }

    fn build(&self, config: i32) -> Level {
        todo!()
    }

    fn print_config(&self, writer: &mut dyn Write, config: i32) {}
}

struct Player {
    hp: i32,
    // flag: PlayerFlag,
    atk: i16,
    def: i16,
    // equip_flag: PlayerFlag,
    equip_atk: i16,
    equip_def: i16,
    gr: i16,
    yk: i8,
    gk: i8,
    bk: i8,
    visited: BitSet,
    neighbors: BitSet,

    #[cfg(feature = "closed-level")]
    memory: BitSet,
    #[cfg(feature = "closed-level")]
    disabled: BitSet,
}

impl Player {
    fn new(hp: i32, atk: i16, def: i16) -> Self {
        Self {
            hp,
            // flag: PlayerFlag::new(),
            atk,
            def,
            // equip_flag: PlayerFlag::new(),
            equip_atk: 0,
            equip_def: 0,
            gr: 0,
            yk: 0,
            gk: 0,
            bk: 0,
            visited: BitSet::new(),
            neighbors: BitSet::new(),

            #[cfg(feature = "closed-level")]
            memory: BitSet::new(),
            #[cfg(feature = "closed-level")]
            disabled: BitSet::new(),
        }
    }
}

struct PlayerScore {
    score: i32,
}

impl PlayerScore {
    fn new() -> Self {
        Self { score: 0 }
    }
}

struct PlayerTrace {
    level_config: i32,
    // level: Level,
    player: Player,
    trace: Vec<VertexIDType>,
}

impl PlayerTrace {
    fn new() -> Self {
        Self {
            level_config: 0,
            // level: Level::new(),
            player: Player::new(500, 10, 10),
            trace: Vec::new(),
        }
    }

    fn write(&self, writer: &dyn Write) {
        todo!()
    }

    fn print(&self, writer: &dyn Write, player: &Player) {
        todo!()
    }
}

struct SearchConfig<'a> {
    use_estimated_max_combat: bool,
    print_new_highscore: bool,
    calculate_optimal_player_by_stat: bool,
    print_local_optimal_player_by_score: bool,
    print_local_optimal_player_by_stat: bool,
    print_global_optimal_player_by_score: bool,
    print_global_optimal_player_by_stat: bool,
    writer: &'a mut dyn Write,
    log_writer: &'a mut dyn Write,
}

impl<'a> SearchConfig<'a> {
    fn new(writer: &'a mut dyn Write, log_writer: &'a mut dyn Write) -> Self {
        Self {
            use_estimated_max_combat: true,
            print_new_highscore: true,
            calculate_optimal_player_by_stat: true,
            print_local_optimal_player_by_score: true,
            print_local_optimal_player_by_stat: true,
            print_global_optimal_player_by_score: true,
            print_global_optimal_player_by_stat: true,
            writer,
            log_writer,
        }
    }
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

struct OptimalStatSet {}

impl OptimalStatSet {
    fn new() -> Self {
        Self {}
    }

    fn size(&self) -> usize {
        todo!()
    }
}

impl<'a> IntoIterator for &'a OptimalStatSet {
    type Item = &'a PlayerTrace;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

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
}

struct Search<'a> {
    search_config: SearchConfig<'a>,
    level_info: LevelInfo,
    init_player: Player,
    search_progress: SearchProgress,
    level_config: i32,
    // level: Level,
    local_optimal_player_by_score: OptimalScore,
    global_optimal_player_by_score: OptimalScore,
    local_optimal_player_by_stat: OptimalStatSet,
    global_optimal_player_by_stat: OptimalStatSet,
    // probe_result: HashMap<PlayerCombat, Vec<ProbeStat>>,
    // player_progress_rc: HashMap<PlayerProgress, i32>,
    // optimal_player: HashMap<PlayerProgress, Player>,
    // clones: VecDeque<PlayerProgress>,
}

impl<'a> Search<'a> {
    fn new(search_config: SearchConfig<'a>, level_info: LevelInfo, init_player: Player) -> Self {
        Self {
            search_config,
            level_info,
            init_player,
            search_progress: SearchProgress::new(),
            level_config: 0,
            // level: Level::new(),
            local_optimal_player_by_score: OptimalScore::new(),
            global_optimal_player_by_score: OptimalScore::new(),
            local_optimal_player_by_stat: OptimalStatSet::new(),
            global_optimal_player_by_stat: OptimalStatSet::new(),
            // probe_result: HashMap::new(),
            // player_progress_rc: HashMap::new(),
            // optimal_player: HashMap::new(),
            // clones: VecDeque::new(),
        }
    }

    fn search(&mut self) -> io::Result<()> {
        for config in 0..self.level_info.max_config_number {
            writeln!(self.search_config.log_writer, "Config:")?;
            self.level_info
                .print_config(self.search_config.log_writer, config);

            writeln!(
                self.search_config.writer,
                "================================================================================\n\
                 Config:"
            )?;
            self.level_info
                .print_config(self.search_config.writer, config);
            self.search_config.writer.flush()?;

            let begin = Instant::now();
            self.search_config(config);

            let elapsed_secs = begin.elapsed().as_secs();
            writeln!(
                self.search_config.log_writer,
                "There are {} situations searched.\n\
                 Finished searching in {} seconds.",
                self.search_progress.total_search_count, elapsed_secs
            )?;

            writeln!(
                self.search_config.writer,
                "================================================================================\n\
                 There are {} situations searched.\n\
                 Finished searching in {} seconds.",
                 self.search_progress.total_search_count,
                 elapsed_secs
            )?;

            if self.search_config.print_local_optimal_player_by_score {
                if self.local_optimal_player_by_score.score.score > 0 {
                    writeln!(
                        self.search_config.writer,
                        "The local optimal player by score is: \n\
                        --------------------------------------------------------------------------------"
                    )?;
                    self.local_optimal_player_by_score
                        .trace
                        .print(self.search_config.writer, &self.init_player);
                } else {
                    writeln!(
                        self.search_config.writer,
                        "It is impossible to reach exit with this config.\n\
                        ================================================================================"
                    )?;
                }
            }

            if self.search_config.print_local_optimal_player_by_stat {
                writeln!(
                    self.search_config.writer,
                    "================================================================================\n\
                     There are {} local optimal players by stats.\n\
                     ================================================================================\n\
                     --------------------------------------------------------------------------------",
                     self.local_optimal_player_by_stat.size()
                )?;

                for (i, trace) in self.local_optimal_player_by_stat.into_iter().enumerate() {
                    write!(
                        self.search_config.writer,
                        "Local optimal player by score [{}] ",
                        i + 1,
                    )?;
                    trace.write(self.search_config.writer);
                    writeln!(
                        self.search_config.writer,
                        "--------------------------------------------------------------------------------"
                    )?;
                }
                writeln!(
                    self.search_config.writer,
                    "================================================================================"
                )?;
            }

            self.search_config.writer.flush()?;
        }

        if self.search_config.print_global_optimal_player_by_score {
            writeln!(
                self.search_config.log_writer,
                "--------------------------------------------------------------------------------\n\
                The global optimal player by score is: "
            )?;
            self.level_info.print_config(
                self.search_config.log_writer,
                self.global_optimal_player_by_score.trace.level_config,
            );
            self.global_optimal_player_by_score
                .trace
                .write(self.search_config.log_writer);
            writeln!(
                self.search_config.log_writer,
                "--------------------------------------------------------------------------------"
            )?;

            writeln!(
                self.search_config.writer,
                "////////////////////////////////////////////////////////////////////////////////\n\
                The global optimal player by score is: "
            )?;
            self.level_info.print_config(
                self.search_config.writer,
                self.global_optimal_player_by_score.trace.level_config,
            );
            writeln!(
                self.search_config.writer,
                "--------------------------------------------------------------------------------"
            )?;
            self.global_optimal_player_by_score
                .trace
                .print(self.search_config.writer, &self.init_player);
        }

        if self.search_config.print_global_optimal_player_by_stat {
            writeln!(
                self.search_config.log_writer,
                "There are {} global optimal players by stat.",
                self.global_optimal_player_by_stat.size()
            )?;

            writeln!(
                self.search_config.writer,
                "////////////////////////////////////////////////////////////////////////////////\n\
                There are {} global optimal players by stat.\n\
                ////////////////////////////////////////////////////////////////////////////////\n\
                --------------------------------------------------------------------------------",
                self.global_optimal_player_by_stat.size()
            )?;
            for (i, trace) in self.global_optimal_player_by_stat.into_iter().enumerate() {
                write!(
                    self.search_config.writer,
                    "Global optimal player by score [{}] ",
                    i + 1,
                )?;
                self.level_info
                    .print_config(self.search_config.writer, trace.level_config);
                trace.write(self.search_config.writer);
                writeln!(
                    self.search_config.writer,
                    "--------------------------------------------------------------------------------"
                )?;
            }
            writeln!(
                self.search_config.writer,
                "////////////////////////////////////////////////////////////////////////////////"
            )?;
        }
        self.search_config.writer.flush()?;
        Ok(())
    }

    fn search_config(&self, config: i32) {
        todo!()
    }
}

fn main() -> io::Result<()> {
    // TODO read output path from args
    let output_path = "output.txt";
    let mut output_file = File::create(output_path)?;
    let mut stdout = io::stdout();

    // TODO read config from args
    let mut search_config = SearchConfig::new(&mut output_file, &mut stdout);
    search_config.print_new_highscore = false;
    search_config.calculate_optimal_player_by_stat = false;
    search_config.print_local_optimal_player_by_score = false;
    search_config.print_local_optimal_player_by_stat = false;
    search_config.print_global_optimal_player_by_stat = false;
    let search_config = search_config;

    // TODO read level layout from file
    let level_info = LevelInfo::new();

    // TODO read initial stats from file
    let init_player = Player::new(500, 10, 10);

    let mut search = Search::new(search_config, level_info, init_player);
    search.search()?;
    Ok(())
}
