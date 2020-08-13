use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::{AddAssign, Neg, Sub};
use std::u8;

use rust_dense_bitset::DenseBitSet as BitSet;

// Ways that equipment affect the player
bitflags! {
    #[derive(Default)]
    struct PlayerBehavior: u32 {
        const HAS_WEAPON                = 0b000001;
        const DOUBLE_GR_WEAPON          = 0b000010;
        const DOUBLE_GR_ACCESSORY       = 0b000100;
        const DOUBLE_REP_ACCESSORY      = 0b001000;
        const DOUBLE_ATK_AGAINST_GOBLIN = 0b010000;
        const DOUBLE_ATK_AGAINST_WYRM   = 0b100000;
        const WEAPON_ATTR = Self::HAS_WEAPON.bits | Self::DOUBLE_GR_WEAPON.bits | Self::DOUBLE_ATK_AGAINST_GOBLIN.bits | Self::DOUBLE_ATK_AGAINST_WYRM.bits;
        const ACCESSORY_ATTR = Self::DOUBLE_GR_ACCESSORY.bits | Self::DOUBLE_REP_ACCESSORY.bits;
    }
}

impl Display for PlayerBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut behavior_str = String::new();
        if self.contains(PlayerBehavior::HAS_WEAPON) {
            behavior_str += " HAS_WEAPON";
        }
        if self.contains(PlayerBehavior::DOUBLE_GR_WEAPON) {
            behavior_str += " DOUBLE_GR_WEAPON";
        }
        if self.contains(PlayerBehavior::DOUBLE_GR_ACCESSORY) {
            behavior_str += " DOUBLE_GR_ACCESSORY";
        }
        if self.contains(PlayerBehavior::DOUBLE_REP_ACCESSORY) {
            behavior_str += " DOUBLE_REP_ACCESSORY";
        }
        if self.contains(PlayerBehavior::DOUBLE_ATK_AGAINST_GOBLIN) {
            behavior_str += " DOUBLE_ATK_AGAINST_GOBLIN";
        }
        if self.contains(PlayerBehavior::DOUBLE_ATK_AGAINST_WYRM) {
            behavior_str += " DOUBLE_ATK_AGAINST_WYRM";
        }

        write!(f, "Behavior:{}", behavior_str)
    }
}

// Changes that apply to player stats
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct StatDiff {
    added_behavior: PlayerBehavior,
    removed_behavior: PlayerBehavior,
    pub(super) hp: i32,
    atk: i32,
    def: i32,
    equip_atk: i32,
    equip_def: i32,
    gr: i32,
    rep: i32,
    yk: i32,
    gk: i32,
    bk: i32,
    sk: i32,
}

impl StatDiff {
    // Check whether changes only affect player positively
    pub(super) fn is_free(&self) -> bool {
        self.removed_behavior.is_empty()
            && self.hp >= 0
            && self.atk >= 0
            && self.def >= 0
            && self.equip_atk >= 0
            && self.equip_def >= 0
            && self.gr >= 0
            && self.rep >= 0
            && self.yk >= 0
            && self.gk >= 0
            && self.bk >= 0
            && self.sk >= 0
    }
}

impl AddAssign for StatDiff {
    fn add_assign(&mut self, other: Self) {
        self.added_behavior |= other.added_behavior;
        self.added_behavior -= other.removed_behavior;
        self.removed_behavior |= other.removed_behavior;
        self.removed_behavior -= other.added_behavior;
        self.hp += other.hp;
        self.atk += other.atk;
        self.def += other.def;
        self.equip_atk += other.equip_atk;
        self.equip_def += other.equip_def;
        self.gr += other.gr;
        self.rep += other.rep;
        self.yk += other.yk;
        self.gk += other.gk;
        self.bk += other.bk;
        self.sk += other.sk;
    }
}

impl Neg for StatDiff {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            added_behavior: self.removed_behavior,
            removed_behavior: self.added_behavior,
            hp: -self.hp,
            atk: -self.atk,
            def: -self.def,
            equip_atk: -self.equip_atk,
            equip_def: -self.equip_def,
            gr: -self.gr,
            rep: -self.rep,
            yk: -self.yk,
            gk: -self.gk,
            bk: -self.bk,
            sk: -self.sk,
        }
    }
}

// Player stats
// HP is shifted by 1 so that 0 is considered alive.
// This change makes code cleaner.
#[derive(Clone, Copy, Debug, Default)]
pub struct Player {
    behavior: PlayerBehavior,
    pub(super) hp: i32,
    atk: i32,
    def: i32,
    equip_atk: i32,
    equip_def: i32,
    gr: i32,
    rep: i32,
    yk: i32,
    gk: i32,
    bk: i32,
    sk: i32,
}

impl Player {
    // Calculates score scaled by 1000
    pub(super) fn score(&self) -> i32 {
        // Default score formula for ToTS floors 25 and 49
        // (self.hp + 1) * 25
        //     + (self.atk * 5 + self.yk * 10 + self.gk * 20 + self.bk * 30 + self.sk * 30) * 1000
        //     + self.def * 1000 / 10 * 3;

        // Default score formula for DROD RPG
        (self.hp + 1) * 25
            + (self.atk * 5
                + self.def * 3
                + self.yk * 10
                + self.gk * 20
                + self.bk * 30
                + self.sk * 30)
                * 1000
    }

    // Find the maximum stats of two players
    fn join(&mut self, other: Self) {
        self.behavior |= other.behavior;
        self.hp = self.hp.max(other.hp);
        self.atk = self.atk.max(other.atk);
        self.def = self.def.max(other.def);
        self.equip_atk = self.equip_atk.max(other.equip_atk);
        self.equip_def = self.equip_def.max(other.equip_def);
        self.gr = self.gr.max(other.gr);
        self.rep = self.rep.max(other.rep);
        self.yk = self.yk.max(other.yk);
        self.gk = self.gk.max(other.gk);
        self.bk = self.bk.max(other.bk);
        self.sk = self.sk.max(other.sk);
        // TODO account for how sk can be used as bk?
    }

    // Check whether player only had better stats than other player
    pub(super) fn dominate(&self, other: &Self) -> bool {
        self.behavior.contains(other.behavior)
            && self.hp >= other.hp
            && self.atk >= other.atk
            && self.def >= other.def
            && self.equip_atk >= other.equip_atk
            && self.equip_def >= other.equip_def
            && self.gr >= other.gr
            && self.rep >= other.rep
            && self.yk >= other.yk
            && self.gk >= other.gk
            && self.bk + self.sk >= other.bk + other.sk
            && self.sk >= other.sk
    }
}

impl From<StatDiff> for Player {
    fn from(stat: StatDiff) -> Self {
        Self {
            behavior: stat.added_behavior,
            hp: stat.hp,
            atk: stat.atk,
            def: stat.def,
            equip_atk: stat.equip_atk,
            equip_def: stat.equip_def,
            gr: stat.gr,
            rep: stat.rep,
            yk: stat.yk,
            gk: stat.gk,
            bk: stat.bk,
            sk: stat.sk,
        }
    }
}

impl From<CombatStat> for Player {
    fn from(stat: CombatStat) -> Self {
        Self {
            behavior: stat.behavior,
            atk: stat.atk,
            def: stat.def,
            equip_atk: stat.equip_atk,
            equip_def: stat.equip_def,
            ..Self::default()
        }
    }
}

impl AddAssign<StatDiff> for Player {
    // Apply stat changes to player
    fn add_assign(&mut self, other: StatDiff) {
        self.behavior &= other.added_behavior;
        self.behavior -= other.removed_behavior;
        self.hp += other.hp;
        self.atk += other.atk;
        self.def += other.def;
        self.equip_atk += other.equip_atk;
        self.equip_def += other.equip_def;
        self.gr += other.gr;
        self.rep += other.rep;
        self.yk += other.yk;
        self.gk += other.gk;
        self.bk += other.bk;
        self.sk += other.sk;

        // Check whether sk should be used as bk
        if self.bk < 0 && -self.sk <= self.bk {
            self.sk += self.bk;
            self.bk = 0;
        }
    }
}

impl Sub<StatDiff> for Player {
    type Output = Self;

    fn sub(self, other: StatDiff) -> Self {
        Self {
            behavior: (self.behavior - other.added_behavior) | other.removed_behavior,
            hp: self.hp - other.hp,
            atk: self.atk - other.atk,
            def: self.def - other.def,
            equip_atk: self.equip_atk - other.equip_atk,
            equip_def: self.equip_def - other.equip_def,
            gr: self.gr - other.gr,
            rep: self.rep - other.rep,
            yk: self.yk - other.yk,
            gk: self.gk - other.gk,
            bk: self.bk - other.bk,
            sk: self.sk - other.sk,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let score = self.score();
        let mut score_str = (score / 1000).to_string();
        score_str += ".";
        if score % 1000 < 100 {
            score_str += "0";
        }
        if score % 1000 < 10 {
            score_str += "0";
        }
        score_str += &(score % 1000).to_string();

        write!(
            f,
            "{{Score: {}\n{}, HP: {}, ATK: {}, DEF: {}, GR: {}, REP: {}, YK: {}, GK: {}, BK: {}, SK: {}}}",
            score_str,
            self.behavior,
            self.hp + 1,
            self.atk,
            self.def,
            self.gr,
            self.rep,
            self.yk,
            self.gk,
            self.bk,
            self.sk
        )
    }
}

// Stats that affect combat
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(super) struct CombatStat {
    behavior: PlayerBehavior,
    atk: i32,
    def: i32,
    equip_atk: i32,
    equip_def: i32,
}

impl From<Player> for CombatStat {
    fn from(stat: Player) -> Self {
        Self {
            behavior: stat.behavior,
            atk: stat.atk,
            def: stat.def,
            equip_atk: stat.equip_atk,
            equip_def: stat.equip_def,
        }
    }
}

// Results of completing a room element
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ProbeStat {
    pub(super) diff: StatDiff,
    pub(super) req: Player,
    pub(super) damage: i32,
}

impl AddAssign for ProbeStat {
    fn add_assign(&mut self, other: Self) {
        self.req.join(other.req - self.diff);
        self.diff += other.diff;
        self.damage += other.damage;
    }
}

// Monster behavior that affect combat
bitflags! {
    struct MonsterBehavior: u32 {
        const GOBLIN_WEAKNESS       = 0b00000000001;
        const WYRM_WEAKNESS         = 0b00000000010;
        const ATTACK_FIRST          = 0b00000000100;
        const ATTACK_LAST           = 0b00000001000;
        const ATTACK_AUTO           = 0b00000010000;    // Monster is attacked from square it automatically attacks
        const ATTACK_WEAPON         = 0b00000100000;    // Monster is attacked from square its sword is on
        const NO_ENEMY_DEFENSE      = 0b00001000000;
        const SURPRISED_FROM_BEHIND = 0b00010000000;
        const BRAINED               = 0b00100000000;
        const BRAINED_2             = 0b01000000000;    // TODO support arbitrary number of brains?
        const ONE_HIT               = 0b10000000000;
    }
}

// Stats of a monster
#[derive(Debug)]
struct Monster {
    behavior: MonsterBehavior,
    hp: i32,
    atk: i32,
    def: i32,
    gr: i32,
    rep: i32,
}

impl Monster {
    // Results of fighting a monster
    fn probe(&self, stat: &CombatStat) -> ProbeStat {
        let player_atk = if stat
            .behavior
            .contains(PlayerBehavior::DOUBLE_ATK_AGAINST_GOBLIN)
            && self.behavior.contains(MonsterBehavior::GOBLIN_WEAKNESS)
            || stat
                .behavior
                .contains(PlayerBehavior::DOUBLE_ATK_AGAINST_WYRM)
                && self.behavior.contains(MonsterBehavior::WYRM_WEAKNESS)
        {
            stat.atk * 2
        } else {
            stat.atk
        };

        let player_def = if self.behavior.contains(MonsterBehavior::NO_ENEMY_DEFENSE) {
            0
        } else {
            stat.def
        };

        let monster_atk = if self.behavior.contains(MonsterBehavior::BRAINED_2) {
            self.atk * 4
        } else if self.behavior.contains(MonsterBehavior::BRAINED) {
            self.atk * 2
        } else {
            self.atk
        };

        let monster_def = self.def;

        let damage = if player_atk <= monster_def {
            // TODO shift by another amount?
            1 << 24
        } else if player_def >= monster_atk {
            0
        } else {
            // Number of hits when attacking with sword against default monster
            let mut hits = (self.hp - 1) / (player_atk - monster_def);

            // Monster attacks first if it has behavior or player attacks without sword and monster lacks attack last behavior
            if self.behavior.contains(MonsterBehavior::ATTACK_FIRST)
                || !self.behavior.contains(MonsterBehavior::ATTACK_LAST)
                    && !stat.behavior.contains(PlayerBehavior::HAS_WEAPON)
            {
                hits += 1;
            }

            // Monsters attacked from behind loses its first hit if any
            if self
                .behavior
                .contains(MonsterBehavior::SURPRISED_FROM_BEHIND)
                && hits > 0
            {
                hits -= 1;
            }

            // Automatic damage is avoided if player can start fight using sword on same turn as stepping on attacked square
            if self.behavior.contains(MonsterBehavior::ATTACK_AUTO)
                && !stat.behavior.contains(PlayerBehavior::HAS_WEAPON)
            {
                hits += 1;
            }

            // Stepping on a sword causes damage before starting combat and attacking without sword still cause an extra hit
            if self.behavior.contains(MonsterBehavior::ATTACK_WEAPON) {
                hits += 1;
                if !stat.behavior.contains(PlayerBehavior::HAS_WEAPON) {
                    hits += 1;
                }
            }

            // Represents taking a single hit from automatic attack or sword without fighting
            if self.behavior.contains(MonsterBehavior::ONE_HIT) {
                hits = 1;
            }
            hits * (monster_atk - player_def)
        };

        let gr_gain = if self.behavior.contains(MonsterBehavior::ONE_HIT) {
            0
        } else if stat
            .behavior
            .contains(PlayerBehavior::DOUBLE_GR_WEAPON | PlayerBehavior::DOUBLE_GR_ACCESSORY)
        {
            self.gr * 4
        } else if stat
            .behavior
            .intersects(PlayerBehavior::DOUBLE_GR_WEAPON | PlayerBehavior::DOUBLE_GR_ACCESSORY)
        {
            self.gr * 2
        } else {
            self.gr
        };

        let rep_gain = if self.behavior.contains(MonsterBehavior::ONE_HIT) {
            0
        } else if stat.behavior.contains(PlayerBehavior::DOUBLE_REP_ACCESSORY) {
            self.rep * 2
        } else {
            self.rep
        };

        let mut diff = StatDiff::default();
        diff.hp = -damage;
        diff.gr = gr_gain;
        diff.rep = rep_gain;

        let mut req = Player::default();
        req.hp = damage;

        ProbeStat { diff, req, damage }
    }
}

// Special ways room should be treated when visiting
bitflags! {
    pub(super) struct RoomType: u32 {
        const INTERMEDIATE   = 0b001;   // Must leave through neighboring rooms
        const ONLY_WHEN_FREE = 0b010;
        const PRIORITY_ROOM  = 0b100;
    }
}

// Types of elements in a room
#[derive(Debug)]
enum RoomElement {
    Resource(StatDiff),  // Gives player stats
    Cost(StatDiff),      // Removes player stats
    Requirement(Player), // Requires certain stats to fight
    Monster(Monster),    // Fight monster to pass
    Equipment(StatDiff), // Gives player equipment, replaces old equipment
}

impl RoomElement {
    // Test results of going through room element
    fn probe(&self, stat: &CombatStat) -> ProbeStat {
        match self {
            RoomElement::Resource(resource) => ProbeStat {
                diff: *resource,
                req: Player::default(),
                damage: 0,
            },
            RoomElement::Cost(cost) => ProbeStat {
                diff: -*cost,
                req: (*cost).into(),
                damage: 0,
            },
            RoomElement::Requirement(req) => ProbeStat {
                diff: StatDiff::default(),
                req: *req,
                damage: 0,
            },
            RoomElement::Monster(monster) => monster.probe(stat),
            RoomElement::Equipment(equip) => {
                // Pick up equipment, replace old one
                // TODO handle equipment that can give both atk and def, option for swapping equipment using inventory
                let mut diff = StatDiff::default();
                diff.atk = 0.max(equip.equip_atk - stat.equip_atk);
                diff.def = 0.max(equip.equip_def - stat.equip_def);
                diff.equip_atk = 0.max(equip.equip_atk - stat.equip_atk);
                diff.equip_def = 0.max(equip.equip_def - stat.equip_def);

                // By default weapons only give atk, shields only give def, accessory does not affect atk or def
                if equip.equip_atk >= stat.equip_atk && equip.equip_atk > 0 {
                    diff.added_behavior = equip.added_behavior;
                    diff.removed_behavior = PlayerBehavior::WEAPON_ATTR - equip.added_behavior;
                } else if equip.equip_atk == 0 && equip.equip_def == 0 {
                    diff.added_behavior = equip.added_behavior;
                    diff.removed_behavior = PlayerBehavior::ACCESSORY_ATTR - equip.added_behavior;
                }

                ProbeStat {
                    diff,
                    req: Player::default(),
                    damage: 0,
                }
            } // TODO add support for percentage damage elements like hot tile, Aumtlich beams
              // TODO add support for oremites
        }
    }
}

// Room contains a sequence of elements that must all be completed
#[derive(Debug)]
pub struct Room {
    pub(super) name: String,
    content: Vec<RoomElement>,
    pub(super) room_type: RoomType,
}

impl Room {
    // Test result of going through each element in order
    pub(super) fn probe(&self, stat: &CombatStat) -> ProbeStat {
        let mut player = Player::from(*stat);
        let mut res = ProbeStat::default();
        for element in &self.content {
            let probe = element.probe(&player.into());
            res += probe;
            player += probe.diff;
        }
        res
    }
}

// TODO split into builder
// Represent level as a graph of rooms
#[derive(Debug)]
pub struct Level {
    pub next_id: u8,
    pub vertices_mask: BitSet,
    pub boundary_mask: BitSet,
    pub neighbors: Vec<BitSet>,
    pub toggle_neighbors: Vec<BitSet>,
    pub use_edge: bool,
    pub entrance: u8,
    pub exit: u8,

    current_vertex_id: u8,
    name2id: HashMap<String, u8>,
    vertices: Vec<Room>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            vertices_mask: BitSet::new(),
            boundary_mask: BitSet::new(),
            neighbors: Vec::new(),
            toggle_neighbors: Vec::new(),
            use_edge: false,
            entrance: u8::MAX,
            exit: u8::MAX,
            current_vertex_id: u8::MAX,
            name2id: HashMap::new(),
            vertices: Vec::new(),
        }
    }

    pub fn select_id(&mut self, id: u8) -> &mut Self {
        self.current_vertex_id = id;
        self
    }

    pub fn select_name(&mut self, name: &str) -> &mut Self {
        self.select_id(self.id(name))
    }

    pub fn select_room(&mut self, room: Room) -> &mut Self {
        if self.name2id.contains_key(&room.name) {
            // TODO improve error reporting
            panic!(String::from("the room has aleady been added: ") + &room.name);
        }
        self.vertices_mask.insert(self.next_id as usize, 1, 1);
        if room.room_type.is_empty() {
        // if room.room_type.contains(DRepeatedRoom) {
            self.boundary_mask.insert(self.next_id as usize, 1, 1);
        }
        self.current_vertex_id = self.next_id;
        self.next_id += 1;
        self.name2id.insert(room.name.clone(), self.current_vertex_id);
        self.vertices.push(room);
        self.neighbors.push(BitSet::new());
        self.toggle_neighbors.push(BitSet::new());
        self
    }

    pub fn add_arc(&mut self, id0: u8, id1: u8) -> &mut Self {
        if id0 < self.next_id && id1 < self.next_id {
            self.neighbors[id0 as usize].insert(id1 as usize, 1, 1)
        }
        self
    }

    pub fn add_id(&mut self, id: u8) -> &mut Self {
        let id0 = self.current_vertex_id;
        let id1 = self.select_id(id).current_vertex_id;
        if self.use_edge {
            self.add_arc(id1, id0);
        }
        self.add_arc(id0, id1)
    }

    pub fn add_name(&mut self, name: &str) -> &mut Self {
        self.add_id(self.id(name))
    }

    pub fn add_room(&mut self, room: Room) -> &mut Self {
        let id0 = self.current_vertex_id;
        let id1 = self.select_room(room).current_vertex_id;
        if self.use_edge {
            self.add_arc(id1, id0);
        }
        self.add_arc(id0, id1)
    }

    pub fn toggle(&mut self, id0: u8, id1: u8) -> &mut Self {
        if id0 < self.next_id && id1 < self.next_id {
            self.toggle_neighbors[id0 as usize].insert(id1 as usize, 1, 1)
        }
        self
    }

    pub fn toggle_name(&mut self, name0: &str, name1: &str) -> &mut Self {
        self.toggle(self.id(name0), self.id(name1))
    }

    pub fn id(&self, name: &str) -> u8 {
        let id = self.name2id.get(name);
        *id.expect(&(String::from("cannot find vertex with given name: ") + name))
    }

    pub fn reset(&mut self) -> &mut Self {
        self.current_vertex_id = u8::MAX;
        self
    }

    pub fn vertex(&self) -> &Room {
        &self.vertices[self.current_vertex_id as usize]
    }

    pub fn vertex_of_id(&self, id: u8) -> &Room {
        &self.vertices[id as usize]
    }

    pub fn vertex_of_name(&self, name: &str) -> &Room {
        self.vertex_of_id(self.id(name))
    }

    pub fn set_entrance(&mut self) -> &mut Self {
        self.entrance = self.current_vertex_id;
        self
    }

    pub fn set_entrance_id(&mut self, id: u8) -> &mut Self {
        self.entrance = id;
        self
    }

    pub fn set_entrance_name(&mut self, name: &str) -> &mut Self {
        self.set_entrance_id(self.id(name))
    }

    pub fn set_exit(&mut self) -> &mut Self {
        self.exit = self.current_vertex_id;
        self
    }

    pub fn set_exit_id(&mut self, id: u8) -> &mut Self {
        self.exit = id;
        self
    }

    pub fn set_exit_name(&mut self, name: &str) -> &mut Self {
        self.set_exit_id(self.id(name))
    }
}
