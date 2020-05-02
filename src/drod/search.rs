use std::collections::{HashMap, VecDeque};
use std::usize;

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

use super::bitset_iter::BitSetIter;
use super::room::RoomType;
use super::stat::{PlayerStat, ProbeStat, StatDiff};
use super::EssPlayer;
use super::Level;

struct ExtendedProbeStat {
    id: usize,
    probe: ProbeStat,
    loss: bool,
    free: bool,
    priority: bool,
}

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
        let mut init_player = EssPlayer::with_stat(init_stat);
        init_player.enter(&level);
        let mut optimal_player = HashMap::new();
        optimal_player.insert(BitSet::new(), init_player);

        Self {
            level,
            init_stat,
            print_highscore: true,
            prefix_player: init_player,
            // optimal_exit_player: Player::new(),
            optimal_exit_player_score: 0,
            // local_optimal_exit_player: LinkedList::new(),
            optimal_player,
            init_player,
            total_search_count: 0,
            current_search_count: 0,
            prefix: Vec::new(),
            prefix_bitset: BitSet::new(),
            suffix: Vec::new(),
            suffix_bitset: BitSet::new(),
            // probe_result: HashMap::new(),
            clones: VecDeque::new(),
            optimal_needed_count: HashMap::new(),
        }
    }

    pub fn search(&mut self) {
        self.optimal_player
            .insert(self.prefix_player.visited, self.prefix_player);
        self.clones.push_back(self.prefix_player.visited);
        self.total_search_count += 1;

        while let Some(current) = self.clones.pop_front() {
            let current_player = self.optimal_player[&current];
            self.optimal_needed_count.insert(current, 0);
            self.current_search_count += 1;

            let probe_result = self.probe_player(&current_player).clone();
            let mut extended_probe_result = Vec::<ExtendedProbeStat>::with_capacity(
                current_player.neighbors.get_weight() as usize,
            );
            let was_intermediate = if current_player.last_visit == usize::MAX {
                false
            } else {
                self.level
                    .vertex(current_player.last_visit)
                    .room_type
                    .contains(RoomType::INTERMEDIATE)
            };

            let mut has_priority = false;
            let mut has_free = false;
            for neighbor in BitSetIter::from(current_player.neighbors) {
                if was_intermediate
                    && !self.level.neighbours[current_player.last_visit].get_bit(neighbor)
                {
                    continue;
                }

                let probe_stat = probe_result[neighbor];
                let available = current_player.stat.ge(&probe_stat.req);
                if !available {
                    continue;
                }
                let room_type = self.level.vertex(neighbor).room_type;
                let priority = room_type.contains(RoomType::PRIORITY_ROOM);
                let intermediate = room_type.contains(RoomType::INTERMEDIATE);
                let free = neighbor != self.level.exit
                    && !intermediate
                    && probe_stat.loss == 0
                    && probe_stat.diff.ge(&StatDiff::default());
                if !free && room_type.contains(RoomType::ONLY_WHEN_FREE) {
                    continue;
                }
                has_free |= free;
                has_priority |= priority;
                extended_probe_result.push(ExtendedProbeStat {
                    id: neighbor,
                    probe: probe_stat,
                    loss: probe_stat.loss > 0,
                    free,
                    priority,
                });
            }

            if has_priority {
                for extended_probe in extended_probe_result {
                    if extended_probe.priority {
                        self.expand(
                            &current_player,
                            extended_probe.id,
                            &extended_probe.probe,
                            true,
                        );
                        break;
                    }
                }
            } else if has_free {
                for extended_probe in extended_probe_result {
                    if extended_probe.free {
                        self.expand(
                            &current_player,
                            extended_probe.id,
                            &extended_probe.probe,
                            true,
                        );
                        break;
                    }
                }
            } else {
                for extended_probe in extended_probe_result {
                    self.expand(
                        &current_player,
                        extended_probe.id,
                        &extended_probe.probe,
                        true,
                    );
                }
            }
            if self.optimal_needed_count[&current] == 0 {
                self.try_remove_optimal_player(&current);
            }
        }
    }

    // fn probe_stat(&mut self, stat: &EssStat) -> Vec<ProbeStat> {
    //     todo!()
    // }

    // TODO make this return slice?
    fn probe_player(&mut self, player: &EssPlayer) -> Vec<ProbeStat> {
        todo!()
    }

    fn try_remove_optimal_player(&mut self, bitset: &BitSet) {
        todo!()
    }

    fn expand(&mut self, player: &EssPlayer, id: usize, probe: &ProbeStat, push: bool) {
        todo!()
    }
}
