use std::collections::{HashMap, VecDeque};

use rust_dense_bitset::DenseBitSet as BitSet;

use crate::essplayer::EssPlayer;
use crate::level::Level;
use crate::stat::PlayerStat;

pub struct Search {
    level: Level,
    init_stat: PlayerStat,
    print_highscore: bool,
    prefix_player: EssPlayer,
    // optimal_exit_player: Player,
    optimal_exit_player_score: i32,
    // local_optimal_exit_player: LinkedList<Player>,
    optimal_player: HashMap<BitSet, EssPlayer>,
    init_player: EssPlayer,
    total_search_count: usize,
    current_search_count: usize,
    prefix: Vec<usize>,
    prefix_bitset: BitSet,
    suffix: Vec<usize>,
    suffix_bitset: BitSet,
    // probe_result: HashMap<EssStat, Vec<ProbeStat>>,
    clones: VecDeque<BitSet>,
    optimal_needed_count: HashMap<BitSet, i32>,
}

impl Search {
    pub fn new(level: Level, init_stat: PlayerStat) -> Self {
        Self {
            level,
            init_stat,
            print_highscore: true,
            prefix_player: EssPlayer::new(),
            // optimal_exit_player: Player::new(),
            optimal_exit_player_score: 0,
            // local_optimal_exit_player: LinkedList::new(),
            optimal_player: HashMap::new(),
            init_player: EssPlayer::new(),
            total_search_count: 0,
            current_search_count: 0,
            prefix: Vec::new(),
            prefix_bitset: BitSet::new(),
            suffix: Vec::new(),
            suffix_bitset: BitSet::new(),
            // probe_result:: HashMap::new(),
            clones: VecDeque::new(),
            optimal_needed_count: HashMap::new(),
        }
    }

    pub fn search(&mut self) {
        todo!()
    }
}
