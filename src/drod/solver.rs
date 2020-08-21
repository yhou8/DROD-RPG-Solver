use super::model::{Level, PlayerStat, ProbeStat, RoomType};
use super::{Ge, VertexIDType};

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

// use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
use std::ops::{AddAssign, SubAssign};
// use std::rc::Rc;
// use std::u8;

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

#[derive(Clone)]
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

struct PlayerProgressDiff {
    progress: PlayerProgress,
    location: VertexIDType,
}

// Score scaled by 1000
struct PlayerScore {
    score: i32,
}

impl PlayerScore {
    fn new() -> Self {
        Self { score: 0 }
    }
}

impl Ge for PlayerScore {
    fn ge(&self, other: &Self) -> bool {
        self.score >= other.score
    }
}

impl Display for PlayerScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let score_modk = self.score % 1000;
        let tenth = if score_modk < 100 { "0" } else { "" };
        let hundredth = if score_modk < 10 { "0" } else { "" };
        write!(
            f,
            "{}.{}{}{}",
            self.score / 1000,
            tenth,
            hundredth,
            score_modk
        )
    }
}

struct Player {
    stat: PlayerStat,
    progress: PlayerProgress,
    diff: PlayerProgressDiff,
    neighbors: BitSet,

    #[cfg(feature = "closed-level")]
    disabled: BitSet,
}

impl Player {
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

// impl Ge<PlayerStat> for Player {
//     fn ge(&self, stat: &PlayerStat) -> bool {
//         self.stat.ge(stat)
//     }
// }

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

struct PlayerTrace {
    level_config: i32,
    level: Level,
    player: Player,
    trace: Vec<VertexIDType>,
}

impl PlayerTrace {
    fn new() -> Self {
        todo!()
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

    // fn print(&self, writer: &mut dyn Write, init_player: &Player) -> io::Result<()> {
    //     todo!()
    // }
}

// Track pareto frontier of traces by stat
struct OptimalStatSet {
    trace: Vec<PlayerTrace>,
}

impl OptimalStatSet {
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

    fn add_all(&mut self, other: Self) {
        for trace in other.trace {
            self.add(trace, false);
        }
    }
}

// Track trace with optimal score
struct OptimalScore {
    trace: PlayerTrace,
    score: PlayerScore,
}

impl OptimalScore {
    fn addable(&self, score: &PlayerScore) -> bool {
        !self.score.ge(score)
    }

    fn add_all(&mut self, other: Self) -> bool {
        if self.score.ge(&other.score) {
            false
        } else {
            *self = other;
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

// // Represents state of player and choices after visiting a set of rooms
// #[derive(Clone, Copy, Debug)]
// struct RouteState {
//     player: Player,
//     neighbors: RoomSet,
//     visited: RoomSet,
//     last_visit: u8,
// }

// impl RouteState {
//     fn new(player: Player, level: &Level) -> Self {
//         let neighbors = RoomSet::from_integer(1 << level.entrance);
//         Self {
//             player,
//             neighbors,
//             visited: RoomSet::new(),
//             last_visit: u8::MAX,
//         }
//     }

//     // Find route state after visiting a room
//     fn visit(&mut self, room_id: u8, level: &Level, probe: &ProbeStat) {
//         let idx = room_id as usize;
//         self.player += probe.diff;
//         self.neighbors |= level.neighbors[idx];
//         self.neighbors &= !level.toggle_neighbors[idx];
//         self.neighbors &= !self.visited;
//         self.last_visit = room_id;
//         self.visited.set_bit(idx, true);
//     }

//     fn previous_visited(&self) -> RoomSet {
//         let idx = self.last_visit as usize;
//         let mut rooms = self.visited;
//         rooms.set_bit(idx, false);
//         rooms
//     }
// }

// // Represents full route through a level
// #[derive(Clone, Debug)]
// pub struct Route {
//     player: Player,
//     level: Rc<Level>,
//     trace: Vec<u8>,
//     neighbors: RoomSet,
//     visited: RoomSet,
//     previous_visited: RoomSet,
// }

// impl Route {
//     fn new(player: Player, level: Rc<Level>) -> Self {
//         let neighbors = RoomSet::from_integer(1 << level.entrance);
//         Self {
//             player,
//             level,
//             trace: Vec::new(),
//             neighbors,
//             visited: RoomSet::new(),
//             previous_visited: RoomSet::new(),
//         }
//     }

//     // Add room to route
//     fn visit(&mut self, room_id: u8) {
//         let idx = room_id as usize;
//         assert!(self.neighbors.get_bit(idx));
//         let probe = self.level.vertex_of_id(room_id).probe(&self.player.into());
//         assert!(self.player.dominate(&probe.req));

//         self.player += probe.diff;
//         self.trace.push(room_id);
//         self.neighbors |= self.level.neighbors[idx];
//         self.neighbors &= !self.level.toggle_neighbors[idx];
//         self.neighbors &= !self.visited;
//         self.previous_visited = self.visited;
//         self.visited.set_bit(idx, true);
//     }
// }

// impl Display for Route {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let mut trace_str = String::new();
//         let mut first = true;
//         for id in &self.trace {
//             if first {
//                 first = false;
//             } else {
//                 trace_str += ", ";
//             }
//             trace_str += &self.level.vertex_of_id(*id).name;
//         }
//         write!(f, "{}\n\nTrace: {}", self.player, trace_str)
//     }
// }

// // Stores auxillary data about probing a room
// struct ExtendedProbeStat {
//     probe: ProbeStat,
//     room_id: u8,
//     free: bool,
//     priority: bool,
// }

// // Algorithm to search for optimal solution to level
// pub struct Solver {
//     level: Rc<Level>,
//     init_state: RouteState,
//     optimal_route: Route,
//     optimal_route_score: i32,
//     local_optimal_routes: Vec<Route>,
//     optimal_visit_states: HashMap<RoomSet, RouteState>,
//     optimal_visit_rc: HashMap<RoomSet, i32>,
//     remaining_visits: VecDeque<RoomSet>,
//     probe_cache: HashMap<CombatStat, Vec<ProbeStat>>,
// }

// impl Solver {
//     pub fn new(level: Level, init_player: Player) -> Self {
//         let level = Rc::new(level);
//         let init_state = RouteState::new(init_player, &level);
//         let mut optimal_states = HashMap::new();
//         optimal_states.insert(RoomSet::new(), init_state);

//         Self {
//             level: Rc::clone(&level),
//             init_state,
//             optimal_route: Route::new(init_player, Rc::clone(&level)),
//             optimal_route_score: 0,
//             local_optimal_routes: Vec::new(),
//             optimal_visit_states: optimal_states,
//             optimal_visit_rc: HashMap::new(),
//             remaining_visits: VecDeque::new(),
//             probe_cache: HashMap::new(),
//         }
//     }

//     // Solve for optimal solution to level if one exists
//     pub fn find_solution(mut self) -> Option<Solution> {
//         self.optimal_visit_states
//             .insert(self.init_state.visited, self.init_state);
//         self.remaining_visits.push_back(self.init_state.visited);

//         while let Some(rooms_visited) = self.remaining_visits.pop_front() {
//             let state = self.optimal_visit_states[&rooms_visited];
//             self.optimal_visit_rc.insert(rooms_visited, 0);

//             let stat = &CombatStat::from(state.player);
//             self.cache_room_probes(stat);
//             let probe_result = &self.probe_cache[stat];

//             let mut extended_probes =
//                 Vec::<ExtendedProbeStat>::with_capacity(state.neighbors.get_weight() as usize);
//             let was_intermediate = if state.last_visit == u8::MAX {
//                 false
//             } else {
//                 self.level
//                     .vertex_of_id(state.last_visit)
//                     .room_type
//                     .contains(RoomType::INTERMEDIATE)
//             };

//             let mut has_priority = false;
//             let mut has_free = false;
//             for neighbor in BitSetIter::from(state.neighbors) {
//                 let idx_neighbor = neighbor as usize;
//                 let idx_visit = state.last_visit as usize;
//                 if was_intermediate && !self.level.neighbors[idx_visit].get_bit(idx_neighbor) {
//                     continue;
//                 }

//                 let probe_stat = probe_result[idx_neighbor];
//                 let available = state.player.dominate(&probe_stat.req);
//                 if !available {
//                     continue;
//                 }

//                 let room_type = self.level.vertex_of_id(neighbor).room_type;
//                 let priority = room_type.contains(RoomType::PRIORITY_ROOM);
//                 let intermediate = room_type.contains(RoomType::INTERMEDIATE);
//                 let free = neighbor != self.level.exit
//                     && !intermediate
//                     && probe_stat.damage == 0
//                     && probe_stat.diff.is_free();
//                 if !free && room_type.contains(RoomType::ONLY_WHEN_FREE) {
//                     continue;
//                 }

//                 has_free |= free;
//                 has_priority |= priority;
//                 extended_probes.push(ExtendedProbeStat {
//                     room_id: neighbor,
//                     probe: probe_stat,
//                     free,
//                     priority,
//                 });
//             }

//             if has_priority {
//                 for room in extended_probes {
//                     if room.priority {
//                         self.update_optimal_route(room.room_id, &state, &room.probe);
//                         break;
//                     }
//                 }
//             } else if has_free {
//                 for room in extended_probes {
//                     if room.free {
//                         self.update_optimal_route(room.room_id, &state, &room.probe);
//                         break;
//                     }
//                 }
//             } else {
//                 for room in extended_probes {
//                     self.update_optimal_route(room.room_id, &state, &room.probe);
//                 }
//             }
//             if self.optimal_visit_rc[&rooms_visited] == 0 {
//                 self.clean_visit_states(rooms_visited);
//             }
//         }

//         if self.optimal_route_score == 0 {
//             None
//         } else {
//             Some(Solution {
//                 optimal_route: self.optimal_route,
//                 local_optimal_routes: self.local_optimal_routes,
//             })
//         }
//     }

//     // Construct full route used to reach state
//     fn to_route(&self, state: &RouteState) -> Route {
//         let mut state = *state;
//         let mut trace = Vec::new();
//         while state.visited != self.init_state.visited {
//             trace.push(state.last_visit);
//             let rooms = state.previous_visited();
//             state = self.optimal_visit_states[&rooms];
//         }

//         let mut route = Route::new(self.init_state.player, Rc::clone(&self.level));
//         for room_id in trace.into_iter().rev() {
//             route.visit(room_id);
//         }
//         route
//     }

//     // Cache the results of probing each room with given stats
//     fn cache_room_probes(&mut self, stat: &CombatStat) {
//         if !self.probe_cache.contains_key(stat) {
//             let result: Vec<ProbeStat> = (0..self.level.next_id)
//                 .map(|i| self.level.vertex_of_id(i).probe(stat))
//                 .collect();
//             self.probe_cache.insert(*stat, result);
//         }
//     }

//     // Clean up state related to the set of rooms visited
//     fn clean_visit_states(&mut self, rooms: RoomSet) {
//         let mut rooms = rooms;
//         while rooms != self.init_state.visited {
//             let last_visit = self.optimal_visit_states[&rooms].last_visit;
//             let rc = *self
//                 .optimal_visit_rc
//                 .entry(rooms)
//                 .and_modify(|x| *x -= 1)
//                 .or_insert(-1);
//             if rc <= 0 {
//                 self.optimal_visit_states.remove(&rooms);
//                 self.optimal_visit_rc.remove(&rooms);
//                 if last_visit == u8::MAX {
//                     return;
//                 } else {
//                     let idx = last_visit as usize;
//                     rooms.set_bit(idx, false);
//                 }
//             }
//         }
//     }

//     // Check whether current route is most optimal way to visit set of rooms
//     fn update_optimal_route(&mut self, room_id: u8, state: &RouteState, probe: &ProbeStat) {
//         let mut state = *state;
//         let rooms = state.visited;
//         if room_id == self.level.exit {
//             state.visit(room_id, &self.level, probe);

//             let player = state.player;
//             let mut local_max = true;
//             self.local_optimal_routes.retain(|local_route| {
//                 if local_route.player.dominate(&player) {
//                     local_max = false;
//                     true
//                 } else if player.dominate(&local_route.player) && local_max {
//                     false
//                 } else {
//                     true
//                 }
//             });

//             if !local_max {
//                 return;
//             }

//             let route = self.to_route(&state);
//             self.local_optimal_routes.push(route.clone());

//             let score = state.player.score();
//             if score <= self.optimal_route_score {
//                 return;
//             }

//             // Print high score
//             println!("New High Score {}", route);
//             println!(
//                 "--------------------------------------------------------------------------------"
//             );

//             self.clean_visit_states(self.optimal_route.previous_visited);
//             self.optimal_route = route;
//             self.optimal_route_score = score;
//             *self.optimal_visit_rc.entry(rooms).or_insert(0) += 1;
//         } else {
//             let idx = room_id as usize;
//             let mut new_rooms = rooms;
//             new_rooms.set_bit(idx, true);
//             if let Some(optimal_state) = self.optimal_visit_states.get_mut(&new_rooms) {
//                 let new_hp = state.player.hp + probe.diff.hp;
//                 if new_hp <= optimal_state.player.hp {
//                     return;
//                 }
//                 let previous_visited = optimal_state.previous_visited();
//                 optimal_state.player.hp = new_hp;
//                 optimal_state.last_visit = room_id;
//                 self.clean_visit_states(previous_visited);
//                 *self.optimal_visit_rc.entry(rooms).or_insert(0) += 1;
//             } else {
//                 state.visit(room_id, &self.level, probe);
//                 self.optimal_visit_states.insert(new_rooms, state);
//                 *self.optimal_visit_rc.entry(rooms).or_insert(0) += 1;
//                 self.remaining_visits.push_back(new_rooms);
//             }
//         }
//     }
// }

// pub struct Solution {
//     optimal_route: Route,
//     local_optimal_routes: Vec<Route>,
// }

// impl Display for Solution {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let mut local_route_str = String::new();
//         for route in &self.local_optimal_routes {
//             local_route_str += &route.to_string();
//             local_route_str += "\n--------------------------------------------------------------------------------\n";
//         }

//         write!(
//             f,
//             "Local Optimal Routes:\n{}Most Optimal Route:\n{}",
//             local_route_str, self.optimal_route
//         )
//     }
// }
