use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::io::Write;
use std::ops::{Add, AddAssign, Neg, Sub};
use std::u8;

use rust_dense_bitset::DenseBitSet as BitSet;

// Ways that equipment affect the player
bitflags! {
    #[derive(Default)]
    struct PlayerFlag: u8 {
        const DEAD                      = 0b00001;
        const HAS_WEAPON                = 0b00010;
        const DOUBLE_GR_WEAPON          = 0b00100;
        const DOUBLE_ATK_AGAINST_GOBLIN = 0b01000;
        const DOUBLE_ATK_AGAINST_WYRM   = 0b10000;
        const WEAPON_MASK               = Self::HAS_WEAPON.bits | Self::DOUBLE_GR_WEAPON.bits | Self::DOUBLE_ATK_AGAINST_GOBLIN.bits | Self::DOUBLE_ATK_AGAINST_WYRM.bits;
        const SHIELD_MASK               = 0;
        const ACCESSORY_MASK            = 0;
    }
}

impl Display for PlayerFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut flag_str = String::new();
        if self.contains(PlayerFlag::DEAD) {
            flag_str += " DEAD";
        }
        if self.contains(PlayerFlag::HAS_WEAPON) {
            flag_str += " HAS_WEAPON";
        }
        if self.contains(PlayerFlag::DOUBLE_GR_WEAPON) {
            flag_str += " DOUBLE_GR_WEAPON";
        }
        if self.contains(PlayerFlag::DOUBLE_ATK_AGAINST_GOBLIN) {
            flag_str += " DOUBLE_ATK_AGAINST_GOBLIN";
        }
        if self.contains(PlayerFlag::DOUBLE_ATK_AGAINST_WYRM) {
            flag_str += " DOUBLE_ATK_AGAINST_WYRM";
        }
        write!(f, "{}", flag_str)
    }
}

// Player stats
// HP is shifted by 1 so that 0 is considered alive.
// This change makes code cleaner.
#[derive(Default)]
pub struct PlayerStat {
    pub hp: i32,
    pub flag: PlayerFlag,
    pub atk: i16,
    pub def: i16,
    pub equip_flag: PlayerFlag,
    pub equip_atk: i16,
    pub equip_def: i16,
    pub gr: i16,
    pub yk: i8,
    pub gk: i8,
    pub bk: i8,
    pub counter: i8,
}

impl From<PlayerStat> for PlayerObjective {
    fn from(stat: PlayerStat) -> Self {
        Self {
            hp: stat.hp,
        }
    }
}

impl From<PlayerStat> for PlayerCombat {
    fn from(stat: PlayerStat) -> Self {
        Self {
            hp: stat.hp,
            flag: stat.flag,
            atk: stat.atk,
            def: stat.def,
            equip_flag: stat.equip_flag,
            equip_atk: stat.equip_atk,
            equip_def: stat.equip_def,
            // default counter
            ..Default::default()
        }
    }
}

impl PlayerStat {
    pub fn ge(&self, rhs: &Self) -> bool {
        self.hp >= rhs.hp
            && self.flag.contains(rhs.flag)
            && self.atk >= rhs.atk
            && self.def >= rhs.def
            && self.equip_flag.contains(rhs.equip_flag)
            && self.equip_atk >= rhs.equip_atk
            && self.equip_def >= rhs.equip_def
            && self.gr >= rhs.gr
            && self.yk >= rhs.yk
            && self.gk >= rhs.gk
            && self.bk >= rhs.bk
            // ignore counter
    }

    pub fn nonnegative(&self) -> bool {
        self.hp >= 0
            && self.atk >= 0
            && self.def >= 0
            && self.equip_atk >= 0
            && self.equip_def >= 0
            && self.gr >= 0
            && self.yk >= 0
            && self.gk >= 0
            && self.bk >= 0
            // ignore counter
    }

    // Find the maximum stats of two players
    pub fn join(&mut self, rhs: Self) {
        self.hp = self.hp.max(rhs.hp);
        self.flag |= rhs.flag;
        self.atk = self.atk.max(rhs.atk);
        self.def = self.def.max(rhs.def);
        self.equip_flag |= rhs.equip_flag;
        self.equip_atk = self.equip_atk.max(rhs.equip_atk);
        self.equip_def = self.equip_def.max(rhs.equip_def);
        self.gr = self.gr.max(rhs.gr);
        self.yk = self.yk.max(rhs.yk);
        self.gk = self.gk.max(rhs.gk);
        self.bk = self.bk.max(rhs.bk);
        // ignore counter
    }
}

impl AddAssign for PlayerStat {
    fn add_assign(&mut self, rhs: PlayerStat) {
        self.hp += rhs.hp;
        self.flag ^= rhs.flag;
        self.atk += rhs.atk;
        self.def += rhs.def;
        self.equip_flag ^= rhs.equip_flag;
        self.equip_atk += rhs.equip_atk;
        self.equip_def += rhs.equip_def;
        self.gr += rhs.gr;
        self.yk += rhs.yk;
        self.gk += rhs.gk;
        self.bk += rhs.bk;
        // ignore counter
    }
}

impl Sub for PlayerStat {
    type Output = Self;

    fn sub(self, other: PlayerStat) -> Self {
        Self {
            hp: self.hp - other.hp,
            flag: self.flag ^ other.flag,
            atk: self.atk - other.atk,
            def: self.def - other.def,
            equip_flag: self.equip_flag ^ other.equip_flag,
            equip_atk: self.equip_atk - other.equip_atk,
            equip_def: self.equip_def - other.equip_def,
            gr: self.gr - other.gr,
            yk: self.yk - other.yk,
            gk: self.gk - other.gk,
            bk: self.bk - other.bk,
            // default counter
            ..Default::default()
        }
    }
}

impl Neg for PlayerStat {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            hp: -self.hp,
            flag: self.flag,
            atk: -self.atk,
            def: -self.def,
            equip_flag: self.equip_flag,
            equip_atk: -self.equip_atk,
            equip_def: -self.equip_def,
            gr: -self.gr,
            yk: -self.yk,
            gk: -self.gk,
            bk: -self.bk,
            // default counter
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct LevelStat {
    pub counter: i8,
}

#[derive(Default, Eq, PartialEq)]
pub struct PlayerCombat {
    pub hp: i32,
    pub flag: PlayerFlag,
    pub atk: i16,
    pub def: i16,
    pub equip_flag: PlayerFlag,
    pub equip_atk: i16,
    pub equip_def: i16,
    pub counter: i8,
}

impl From<PlayerCombat> for PlayerStat {
    fn from(combat: PlayerCombat) -> Self {
        Self {
            hp: combat.hp,
            flag: combat.flag,
            atk: combat.atk,
            def: combat.def,
            equip_flag: combat.equip_flag,
            equip_atk: combat.equip_atk,
            equip_def: combat.equip_def,
            // default including counter
            ..Default::default()
        }
    }
}

impl AddAssign<PlayerStat> for PlayerCombat {
    fn add_assign(&mut self, rhs: PlayerStat) {
        self.hp += rhs.hp;
        self.flag ^= rhs.flag;
        self.atk += rhs.atk;
        self.def += rhs.def;
        self.equip_flag ^= rhs.equip_flag;
        self.equip_atk += rhs.equip_atk;
        self.equip_def += rhs.equip_def;
        // ignore counter
    }
}

impl PlayerCombat {
    fn write(&self, writer: &mut dyn Write) {
        writeln!(writer, "Combat, hp:{}, flag:{}, atk:{}, def:{}, equip_flag:{}, equip_atk:{}, equip_def:{}",
            self.hp, self.flag, self.atk, self.def, self.equip_flag, self.equip_atk, self.equip_def);
    }
}

#[derive(Default, Eq, PartialEq)]
pub struct PlayerObjective {
    pub hp: i32,
}

impl Add for PlayerObjective {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            hp: self.hp + rhs.hp,
        }
    }
}

impl PlayerObjective {
    fn ge(&self, rhs: &Self) -> bool {
        self.hp >= rhs.hp
    }
}

#[derive(Default)]
pub struct PlayerScore {
    pub score: i32,
}

impl PlayerScore {
    fn ge(&self, rhs: &Self) -> bool {
        self.score >= rhs.score
    }

    fn write(&self, writer: &mut dyn Write) {
        let mut score_str = (self.score / 1000).to_string() + ".";
        if self.score % 1000 < 100 {
            score_str += "0";
        }
        if self.score % 1000 < 10 {
            score_str += "0";
        }
        score_str += &(self.score % 1000).to_string();
        write!(writer, "{}", score_str).expect("error writing PlayerScore");
    }
}

// Results of completing a room element
#[derive(Default)]
pub struct ProbeStat {
    pub diff: PlayerStat,
    pub req: PlayerStat,
}

impl AddAssign for ProbeStat {
    fn add_assign(&mut self, rhs: Self) {
        self.req.join(rhs.req - self.diff);
        self.diff += rhs.diff;
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
    atk: i16,
    def: i16,
    gr: i16,
    rep: i32,
}

impl Monster {
    // Results of fighting a monster
    fn probe(&self, stat: &PlayerCombat) -> ProbeStat {
        let player_atk = if stat
            .flag
            .contains(PlayerFlag::DOUBLE_ATK_AGAINST_GOBLIN)
            && self.behavior.contains(MonsterBehavior::GOBLIN_WEAKNESS)
            || stat
                .flag
                .contains(PlayerFlag::DOUBLE_ATK_AGAINST_WYRM)
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
            let mut hits = (self.hp - 1) / (player_atk - monster_def) as i32;

            // Monster attacks first if it has behavior or player attacks without sword and monster lacks attack last behavior
            if self.behavior.contains(MonsterBehavior::ATTACK_FIRST)
                || !self.behavior.contains(MonsterBehavior::ATTACK_LAST)
                    && !stat.flag.contains(PlayerFlag::HAS_WEAPON)
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
                && !stat.flag.contains(PlayerFlag::HAS_WEAPON)
            {
                hits += 1;
            }

            // Stepping on a sword causes damage before starting combat and attacking without sword still cause an extra hit
            if self.behavior.contains(MonsterBehavior::ATTACK_WEAPON) {
                hits += 1;
                if !stat.flag.contains(PlayerFlag::HAS_WEAPON) {
                    hits += 1;
                }
            }

            // Represents taking a single hit from automatic attack or sword without fighting
            if self.behavior.contains(MonsterBehavior::ONE_HIT) {
                hits = 1;
            }
            hits * (monster_atk - player_def) as i32
        };

        let gr_gain = if self.behavior.contains(MonsterBehavior::ONE_HIT) {
            0
        } else if stat
            .flag
            .contains(PlayerFlag::DOUBLE_GR_WEAPON)
        {
            self.gr * 2
        } else {
            self.gr
        };

        let rep_gain = if self.behavior.contains(MonsterBehavior::ONE_HIT) {
            0
        } else {
            self.rep
        };

        let mut diff = PlayerStat::default();
        diff.hp = -damage;
        diff.gr = gr_gain;

        let mut req = PlayerStat::default();
        req.hp = damage;

        ProbeStat { diff, req, }
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
// #[derive(Debug)]
enum RoomElement {
    Resource(PlayerStat),  // Gives player stats
    Cost(PlayerStat),      // Removes player stats
    Requirement(PlayerStat), // Requires certain stats to fight
    Monster(Monster),    // Fight monster to pass
    Equipment(PlayerStat), // Gives player equipment, replaces old equipment
}

impl RoomElement {
    // Test results of going through room element
    fn probe(&self, stat: &PlayerCombat) -> ProbeStat {
        match self {
            RoomElement::Resource(resource) => ProbeStat {
                diff: *resource,
                req: PlayerStat::default(),
            },
            RoomElement::Cost(cost) => ProbeStat {
                diff: -*cost,
                req: (*cost).into(),
            },
            RoomElement::Requirement(req) => ProbeStat {
                diff: PlayerStat::default(),
                req: *req,
            },
            RoomElement::Monster(monster) => monster.probe(stat),
            RoomElement::Equipment(equip) => {
                // Pick up equipment, replace old one
                // TODO handle equipment that can give both atk and def, option for swapping equipment using inventory
                let mut diff = PlayerStat::default();
                diff.atk = 0.max(equip.equip_atk - stat.equip_atk);
                diff.def = 0.max(equip.equip_def - stat.equip_def);
                diff.equip_atk = 0.max(equip.equip_atk - stat.equip_atk);
                diff.equip_def = 0.max(equip.equip_def - stat.equip_def);

                // By default weapons only give atk, shields only give def, accessory does not affect atk or def
                if equip.equip_atk >= stat.equip_atk && equip.equip_atk > 0 {
                    diff.flag = equip.flag;
                } else if equip.equip_atk == 0 && equip.equip_def == 0 {
                    diff.flag = equip.flag;
                }

                ProbeStat {
                    diff,
                    req: PlayerStat::default(),
                }
            } // TODO add support for percentage damage elements like hot tile, Aumtlich beams
              // TODO add support for oremites
        }
    }
}

// Room contains a sequence of elements that must all be completed
// #[derive(Debug)]
pub struct Room {
    pub(super) name: String,
    content: Vec<RoomElement>,
    pub(super) room_type: RoomType,
}

impl Room {
    // Test result of going through each element in order
    pub(super) fn probe(&self, stat: &PlayerCombat) -> ProbeStat {
        let mut player = PlayerStat::from(*stat);
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
// #[derive(Debug)]
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
        // if room.room_type.is_empty() {
        if room.room_type.contains(DRepeatedRoom) {
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