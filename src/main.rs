// #[macro_use]
// extern crate bitflags;
extern crate rust_dense_bitset;

// mod drod;
// use drod::{Level, PlayerStat, Solver};

use rust_dense_bitset::DenseBitSet as BitSet;

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::Write;

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

struct Search<'a> {
    search_config: SearchConfig<'a>,
    level_info: LevelInfo,
    init_player: Player,
    // search_progress: SearchProgress,
    level_config: i32,
    // level: Level,
    // local_optimal_player_by_score: Optimal<PlayerTrace, PlayerScore>,
    // global_optimal_player_by_score: Optimal<PlayerTrace, PlayerScore>,
    // local_optimal_player_by_stat: OptimalSet<PlayerTrace, PlayerStat>,
    // global_optimal_player_by_stat: OptimalSet<PlayerTrace, PlayerStat>,
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
            // search_progress: SearchProgress::new(),
            level_config: 0,
            // level: Level::new(),
            // local_optimal_player_by_score: Optimal::new(),
            // global_optimal_player_by_score: Optimal::new(),
            // local_optimal_player_by_stat: OptimalSet::new(),
            // global_optimal_player_by_stat: OptimalSet::new(),
            // probe_result: HashMap::new(),
            // player_progress_rc: HashMap::new(),
            // optimal_player: HashMap::new(),
            // clones: VecDeque::new(),
        }
    }

    fn search(&self) {
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

    let search = Search::new(search_config, level_info, init_player);
    search.search();
    Ok(())
}
