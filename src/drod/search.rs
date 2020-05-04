use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::u8;

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as RoomSet;

use super::bitset_iter::BitSetIter;
use super::room::RoomType;
use super::route::Route;
use super::stat::{CombatStat, Player, ProbeStat};
use super::Level;
use super::RouteState;

struct ExtendedProbeStat {
    room_id: u8,
    probe: ProbeStat,
    free: bool,
    priority: bool,
}

pub struct Search {
    level: Rc<Level>,
    optimal_route: Route,
    optimal_route_score: i32,
    local_optimal_routes: Vec<Route>,
    optimal_visit_states: HashMap<RoomSet, RouteState>,
    init_state: RouteState,
    total_search_count: usize,
    current_search_count: usize,
    probe_result: HashMap<CombatStat, Vec<ProbeStat>>,
    remaining_visits: VecDeque<RoomSet>,
    optimal_visit_rc: HashMap<RoomSet, i32>,
}

impl Search {
    pub fn new(level: Level, init_player: Player) -> Self {
        let mut init_state = RouteState::with_player(init_player);
        init_state.enter(&level);
        let mut optimal_states = HashMap::new();
        optimal_states.insert(RoomSet::new(), init_state);
        let level = Rc::new(level);

        Self {
            level: Rc::clone(&level),
            optimal_route: Route::new(init_player, Rc::clone(&level)),
            optimal_route_score: 0,
            local_optimal_routes: Vec::new(),
            optimal_visit_states: optimal_states,
            init_state,
            total_search_count: 0,
            current_search_count: 0,
            probe_result: HashMap::new(),
            remaining_visits: VecDeque::new(),
            optimal_visit_rc: HashMap::new(),
        }
    }

    pub fn search(&mut self) {
        self.optimal_visit_states
            .insert(self.init_state.visited, self.init_state);
        self.remaining_visits.push_back(self.init_state.visited);
        self.total_search_count += 1;

        while let Some(rooms) = self.remaining_visits.pop_front() {
            let state = self.optimal_visit_states[&rooms];
            self.optimal_visit_rc.insert(rooms, 0);
            self.current_search_count += 1;

            let probe_result = self.probe_state(&state).clone();
            let mut extended_probe_result =
                Vec::<ExtendedProbeStat>::with_capacity(state.neighbors.get_weight() as usize);
            let was_intermediate = if state.last_visit == u8::MAX {
                false
            } else {
                self.level
                    .vertex(state.last_visit)
                    .room_type
                    .contains(RoomType::INTERMEDIATE)
            };

            let mut has_priority = false;
            let mut has_free = false;
            for neighbor in BitSetIter::from(state.neighbors) {
                let idx_neighbor = neighbor as usize;
                let idx_visit = state.last_visit as usize;
                if was_intermediate && !self.level.neighbors[idx_visit].get_bit(idx_neighbor) {
                    continue;
                }

                let probe_stat = probe_result[idx_neighbor];
                let available = state.player.dominate(&probe_stat.req);
                if !available {
                    continue;
                }

                let room_type = self.level.vertex(neighbor).room_type;
                let priority = room_type.contains(RoomType::PRIORITY_ROOM);
                let intermediate = room_type.contains(RoomType::INTERMEDIATE);
                let free = neighbor != self.level.exit
                    && !intermediate
                    && probe_stat.damage == 0
                    && probe_stat.diff.is_free();
                if !free && room_type.contains(RoomType::ONLY_WHEN_FREE) {
                    continue;
                }
                has_free |= free;
                has_priority |= priority;
                extended_probe_result.push(ExtendedProbeStat {
                    room_id: neighbor,
                    probe: probe_stat,
                    free,
                    priority,
                });
            }

            if has_priority {
                for extended_probe in extended_probe_result {
                    if extended_probe.priority {
                        self.expand(state, extended_probe.room_id, &extended_probe.probe);
                        break;
                    }
                }
            } else if has_free {
                for extended_probe in extended_probe_result {
                    if extended_probe.free {
                        self.expand(state, extended_probe.room_id, &extended_probe.probe);
                        break;
                    }
                }
            } else {
                for extended_probe in extended_probe_result {
                    self.expand(state, extended_probe.room_id, &extended_probe.probe);
                }
            }
            if self.optimal_visit_rc[&rooms] == 0 {
                self.try_remove_optimal_state(rooms);
            }
        }
    }

    fn to_route(&self, state: RouteState) -> Route {
        let mut route = Route::new(self.init_state.player, Rc::clone(&self.level));
        let mut trace = Vec::new();
        let mut state = state;
        while state.visited != self.init_state.visited {
            trace.push(state.last_visit);
            let rooms = state.previous_visited();
            state = self.optimal_visit_states[&rooms];
        }
        for room_id in trace.into_iter().rev() {
            route.visit(room_id, &self.level);
        }
        route
    }

    // TODO make these return slice?
    fn probe_stat(&mut self, stat: &CombatStat) -> Vec<ProbeStat> {
        if let Some(result) = self.probe_result.get(stat) {
            result.clone()
        } else {
            let result: Vec<ProbeStat> = (0..self.level.next_id)
                .map(|i| self.level.vertex(i).probe(stat))
                .collect();
            self.probe_result.insert(*stat, result.clone());
            result
        }
    }

    fn probe_state(&mut self, state: &RouteState) -> Vec<ProbeStat> {
        self.probe_stat(&state.player.into())
    }

    fn try_remove_optimal_state(&mut self, rooms: RoomSet) {
        let mut rooms = rooms;
        while rooms != self.init_state.visited {
            let last_visit = self.optimal_visit_states[&rooms].last_visit;
            let rc = *self
                .optimal_visit_rc
                .entry(rooms)
                .and_modify(|x| *x -= 1)
                .or_insert(-1);
            if rc <= 0 {
                self.optimal_visit_states.remove(&rooms);
                self.optimal_visit_rc.remove(&rooms);
                if last_visit == u8::MAX {
                    return;
                } else {
                    let idx = last_visit as usize;
                    rooms.set_bit(idx, false);
                }
            }
        }
    }

    fn expand(&mut self, state: RouteState, room_id: u8, probe: &ProbeStat) {
        let rooms = state.visited;
        let mut state = state;
        if room_id == self.level.exit {
            state.visit(room_id, &self.level, probe);

            let player = state.player;
            let mut local_max = true;
            self.local_optimal_routes.retain(|local_route| {
                if local_route.player.dominate(&player) {
                    local_max = false;
                    true
                } else if player.dominate(&local_route.player) && local_max {
                    false
                } else {
                    true
                }
            });

            if !local_max {
                return;
            }

            let route = self.to_route(state);
            self.local_optimal_routes.push(route.clone());

            let score = state.player.score();
            if score <= self.optimal_route_score {
                return;
            }

            // Print high score
            println!("New High Score {}", route);
            println!(
                "--------------------------------------------------------------------------------"
            );

            self.try_remove_optimal_state(self.optimal_route.previous_visited);
            self.optimal_route = route;
            self.optimal_route_score = score;
            *self.optimal_visit_rc.entry(rooms).or_insert(0) += 1;
        } else {
            let idx = room_id as usize;
            let mut new_rooms = rooms;
            new_rooms.set_bit(idx, true);
            if let Some(optimal_state) = self.optimal_visit_states.get_mut(&new_rooms) {
                let new_hp = state.player.hp + probe.diff.hp;
                if new_hp <= optimal_state.player.hp {
                    return;
                }
                let previous_visited = optimal_state.previous_visited();
                optimal_state.player.hp = new_hp;
                optimal_state.last_visit = room_id;
                self.try_remove_optimal_state(previous_visited);
                *self.optimal_visit_rc.entry(rooms).or_insert(0) += 1;
            } else {
                state.visit(room_id, &self.level, probe);
                self.optimal_visit_states.insert(new_rooms, state);
                *self.optimal_visit_rc.entry(rooms).or_insert(0) += 1;
                self.remaining_visits.push_back(new_rooms);
                self.total_search_count += 1;
            }
        }
    }
}
