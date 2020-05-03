use std::collections::{HashMap, VecDeque};
use std::u8;

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

use super::bitset_iter::BitSetIter;
use super::player::Player;
use super::room::RoomType;
use super::stat::{EssStat, PlayerStat, ProbeStat, StatDiff};
use super::EssPlayer;
use super::Level;

struct ExtendedProbeStat {
    room_id: u8,
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
    optimal_exit_player: Player,
    optimal_exit_player_score: i32,
    local_optimal_exit_player: Vec<Player>,
    optimal_player: HashMap<BitSet, EssPlayer>,
    init_player: EssPlayer,
    total_search_count: usize,
    current_search_count: usize,
    prefix: Vec<u8>,
    prefix_bitset: BitSet,
    suffix: Vec<u8>,
    suffix_bitset: BitSet,
    probe_result: HashMap<EssStat, Vec<ProbeStat>>,
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
            optimal_exit_player: Player::new(),
            optimal_exit_player_score: 0,
            local_optimal_exit_player: Vec::new(),
            optimal_player,
            init_player,
            total_search_count: 0,
            current_search_count: 0,
            prefix: Vec::new(),
            prefix_bitset: BitSet::new(),
            suffix: Vec::new(),
            suffix_bitset: BitSet::new(),
            probe_result: HashMap::new(),
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
            let was_intermediate = if current_player.last_visit == u8::MAX {
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
                let idx_neighbor = neighbor as usize;
                let idx_visit = current_player.last_visit as usize;
                if was_intermediate
                    && !self.level.neighbors[idx_visit].get_bit(idx_neighbor)
                {
                    continue;
                }

                let probe_stat = probe_result[idx_neighbor];
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
                    room_id: neighbor,
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
                            current_player,
                            extended_probe.room_id,
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
                            current_player,
                            extended_probe.room_id,
                            &extended_probe.probe,
                            true,
                        );
                        break;
                    }
                }
            } else {
                for extended_probe in extended_probe_result {
                    self.expand(
                        current_player,
                        extended_probe.room_id,
                        &extended_probe.probe,
                        true,
                    );
                }
            }
            if self.optimal_needed_count[&current] == 0 {
                self.try_remove_optimal_player(current);
            }
        }
    }

    fn to_player(&self, mut player: EssPlayer) -> Player {
        let mut new_player = Player::with_stat(self.init_player.stat);
        new_player.enter(&self.level);
        for id in &self.prefix {
            new_player.visit(*id, &self.level);
        }
        let mut trace = Vec::new();
        while player.visited != self.prefix_player.visited {
            trace.push(player.last_visit);
            let bitset = player.previous_visited();
            player = self.optimal_player[&bitset];
        }
        for id in trace.into_iter().rev() {
            new_player.visit(id, &self.level);
        }
        new_player
    }

    // TODO make these return slice?
    fn probe_stat(&mut self, stat: &EssStat) -> Vec<ProbeStat> {
        if let Some(result) = self.probe_result.get(stat) {
            result.clone()
        } else {
            let result: Vec<ProbeStat> = (0..self.level.next_id)
                .map(|i| self.level.vertex(i).to_probe_stat(stat))
                .collect();
            self.probe_result.insert(*stat, result.clone());
            result
        }
    }

    fn probe_player(&mut self, player: &EssPlayer) -> Vec<ProbeStat> {
        self.probe_stat(&player.stat.into())
    }

    fn try_remove_optimal_player(&mut self, bitset: BitSet) {
        let mut bitset = bitset;
        while bitset != self.prefix_player.visited {
            let last_visit = self.optimal_player[&bitset].last_visit;
            let count = *self
                .optimal_needed_count
                .entry(bitset)
                .and_modify(|x| *x -= 1)
                .or_insert(-1);
            if count <= 0 {
                self.optimal_player.remove(&bitset);
                self.optimal_needed_count.remove(&bitset);
                if last_visit == u8::MAX {
                    return;
                } else {
                    let idx = last_visit as usize;
                    bitset.set_bit(idx, false);
                }
            }
        }
    }

    fn expand(&mut self, mut player: EssPlayer, room_id: u8, probe: &ProbeStat, push: bool) {
        let bitset = player.visited;
        if room_id == self.level.exit {
            player.visit(room_id, &self.level, probe);

            let stat = player.stat;
            let mut local_max = true;
            self.local_optimal_exit_player.retain(|local_player| {
                if local_player.stat.ge(&stat) {
                    local_max = false;
                    true
                } else if stat.ge(&local_player.stat) && local_max {
                    false
                } else {
                    true
                }
            });

            if !local_max {
                return;
            }

            let optimal_player = self.to_player(player);
            self.local_optimal_exit_player.push(optimal_player.clone());

            let new_score = player.stat.score();
            if new_score <= self.optimal_exit_player_score {
                return;
            }

            self.try_remove_optimal_player(self.optimal_exit_player.previous_visited);
            self.optimal_exit_player = optimal_player;
            self.optimal_exit_player_score = new_score;
            *self.optimal_needed_count.entry(bitset).or_insert(0) += 1;
            if self.print_highscore {
                todo!()
            }
        } else {
            let idx = room_id as usize;
            let mut new_bitset = bitset;
            new_bitset.set_bit(idx, true);
            if let Some(optimal_player) = self.optimal_player.get_mut(&new_bitset) {
                let new_hp = player.stat.hp + probe.diff.hp;
                if new_hp <= optimal_player.stat.hp {
                    return;
                }
                let previous_visited = optimal_player.previous_visited();
                optimal_player.stat.hp = new_hp;
                optimal_player.last_visit = room_id;
                self.try_remove_optimal_player(previous_visited);
                *self.optimal_needed_count.entry(bitset).or_insert(0) += 1;
            } else {
                player.visit(room_id, &self.level, probe);
                self.optimal_player.insert(new_bitset, player);
                *self.optimal_needed_count.entry(bitset).or_insert(0) += 1;

                if push {
                    self.clones.push_back(new_bitset);
                    self.total_search_count += 1;
                }
            }
        }
    }
}
