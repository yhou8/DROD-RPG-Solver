use super::{Ge, VertexIDType};

use bitflags::bitflags;
use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Neg, Sub};
use std::u8;

// Character behaviors that affect gameplay
bitflags! {
    #[derive(Default)]
    pub(super) struct PlayerFlag: u8 {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let dead = if self.contains(PlayerFlag::DEAD) {
            " DEAD"
        } else {
            ""
        };
        let has_weapon = if self.contains(PlayerFlag::HAS_WEAPON) {
            " HAS_WEAPON"
        } else {
            ""
        };
        let double_gr = if self.contains(PlayerFlag::DOUBLE_GR_WEAPON) {
            " DOUBLE_GR_WEAPON"
        } else {
            ""
        };
        let double_goblin = if self.contains(PlayerFlag::DOUBLE_ATK_AGAINST_GOBLIN) {
            " DOUBLE_ATK_AGAINST_GOBLIN"
        } else {
            ""
        };
        let double_wyrm = if self.contains(PlayerFlag::DOUBLE_ATK_AGAINST_WYRM) {
            " DOUBLE_ATK_AGAINST_WYRM"
        } else {
            ""
        };
        write!(
            f,
            "{}{}{}{}{}",
            dead, has_weapon, double_gr, double_goblin, double_wyrm
        )
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct EquipStat {
    flag: PlayerFlag,
    atk: i16,
    def: i16,
}

impl EquipStat {
    fn nonnegative(&self) -> bool {
        self.atk >= 0 && self.def >= 0
    }

    fn join(&mut self, other: Self) {
        self.flag |= other.flag;
        self.atk = self.atk.max(other.atk);
        self.def = self.def.max(other.def);
    }
}

impl Ge for EquipStat {
    fn ge(&self, other: &Self) -> bool {
        self.flag.contains(other.flag) && self.atk >= other.atk && self.def >= other.def
    }
}

impl AddAssign<&Self> for EquipStat {
    fn add_assign(&mut self, other: &Self) {
        self.flag ^= other.flag;
        self.atk += other.atk;
        self.def += other.def;
    }
}

impl Sub for &EquipStat {
    type Output = EquipStat;

    fn sub(self, other: Self) -> EquipStat {
        EquipStat {
            flag: self.flag ^ other.flag,
            atk: self.atk - other.atk,
            def: self.def - other.def,
        }
    }
}

impl Neg for EquipStat {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            flag: self.flag,
            atk: -self.atk,
            def: -self.def,
        }
    }
}

impl Display for EquipStat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "equip_flag: {}, equip_atk: {}, equip_def: {}",
            self.flag, self.atk, self.def
        )
    }
}

// Player stats that affect combat
#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub(super) struct PlayerCombat {
    pub(super) flag: PlayerFlag,
    pub(super) atk: i16,
    pub(super) def: i16,
    equip: EquipStat,
}

impl PlayerCombat {
    pub(super) fn with_stat(atk: i16, def: i16) -> Self {
        Self {
            atk,
            def,
            ..Default::default()
        }
    }

    fn nonnegative(&self) -> bool {
        self.atk >= 0 && self.def >= 0 && self.equip.nonnegative()
    }

    fn join(&mut self, other: Self) {
        self.flag |= other.flag;
        self.atk = self.atk.max(other.atk);
        self.def = self.def.max(other.def);
        self.equip.join(other.equip);
    }
}

impl Ge for PlayerCombat {
    fn ge(&self, other: &Self) -> bool {
        self.flag.contains(other.flag)
            && self.atk >= other.atk
            && self.def >= other.def
            && self.equip.ge(&other.equip)
    }
}

impl AddAssign<&Self> for PlayerCombat {
    fn add_assign(&mut self, other: &Self) {
        self.flag ^= other.flag;
        self.atk += other.atk;
        self.def += other.def;
        self.equip += &other.equip;
    }
}

impl Sub for &PlayerCombat {
    type Output = PlayerCombat;

    fn sub(self, other: Self) -> PlayerCombat {
        PlayerCombat {
            flag: self.flag ^ other.flag,
            atk: self.atk - other.atk,
            def: self.def - other.def,
            equip: &self.equip - &other.equip,
        }
    }
}

impl Neg for PlayerCombat {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            flag: self.flag,
            atk: -self.atk,
            def: -self.def,
            equip: -self.equip,
        }
    }
}

impl Display for PlayerCombat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Combat, flag: {}, atk: {}, def: {}, {}",
            self.flag, self.atk, self.def, self.equip
        )
    }
}

#[derive(Eq, PartialEq)]
pub(super) struct PlayerObjective {
    pub(super) hp: i32,
}

impl Ge for PlayerObjective {
    fn ge(&self, other: &Self) -> bool {
        self.hp >= other.hp
    }
}

// HP is shifted by 1 so that 0 is considered alive.
// This change makes code cleaner.
#[derive(Clone, Default)]
pub(super) struct PlayerStat {
    pub(super) hp: i32,
    combat: PlayerCombat,
    gr: i16,
    pub(super) yk: i8,
    pub(super) gk: i8,
    pub(super) bk: i8,
}

impl PlayerStat {
    pub(super) fn with_stat(hp: i32, atk: i16, def: i16) -> Self {
        Self {
            hp: hp - 1,
            combat: PlayerCombat::with_stat(atk, def),
            ..Default::default()
        }
    }

    pub(super) fn nonnegative(&self) -> bool {
        self.hp >= 0
            && self.combat.nonnegative()
            && self.gr >= 0
            && self.yk >= 0
            && self.gk >= 0
            && self.bk >= 0
    }

    // Find max of two player stats
    pub(super) fn join(&mut self, other: Self) {
        self.hp = self.hp.max(other.hp);
        self.combat.join(other.combat);
        self.gr = self.gr.max(other.gr);
        self.yk = self.yk.max(other.yk);
        self.gk = self.gk.max(other.gk);
        self.bk = self.bk.max(other.bk);
    }

    pub(super) fn objective(&self) -> PlayerObjective {
        PlayerObjective { hp: self.hp }
    }
}

impl AsRef<PlayerCombat> for PlayerStat {
    fn as_ref(&self) -> &PlayerCombat {
        &self.combat
    }
}

impl Ge for PlayerStat {
    fn ge(&self, other: &Self) -> bool {
        self.hp >= other.hp
            && self.combat.ge(&other.combat)
            && self.gr >= other.gr
            && self.yk >= other.yk
            && self.gk >= other.gk
            && self.bk >= other.bk
    }
}

impl AddAssign<&Self> for PlayerStat {
    fn add_assign(&mut self, other: &Self) {
        self.hp += other.hp;
        self.combat += &other.combat;
        self.gr += other.gr;
        self.yk += other.yk;
        self.gk += other.gk;
        self.bk += other.bk;
    }
}

impl Sub for &PlayerStat {
    type Output = PlayerStat;

    fn sub(self, other: Self) -> PlayerStat {
        PlayerStat {
            hp: self.hp - other.hp,
            combat: &self.combat - &other.combat,
            gr: self.gr - other.gr,
            yk: self.yk - other.yk,
            gk: self.gk - other.gk,
            bk: self.bk - other.bk,
        }
    }
}

impl Neg for PlayerStat {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            hp: -self.hp,
            combat: -self.combat,
            gr: -self.gr,
            yk: -self.yk,
            gk: -self.gk,
            bk: -self.bk,
        }
    }
}

impl Display for PlayerStat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "hp: {}, flag: {}, atk: {}, def: {}, {}, gr: {}, yk: {}, gk: {}, bk: {}",
            self.hp + 1,
            self.combat.flag,
            self.combat.atk,
            self.combat.def,
            self.combat.equip,
            self.gr,
            self.yk,
            self.gk,
            self.bk
        )
    }
}

// Result of completing a room element
#[derive(Clone, Default)]
pub(super) struct ProbeStat {
    pub(super) diff: PlayerStat,
    pub(super) req: PlayerStat,
}

impl AddAssign<&Self> for ProbeStat {
    fn add_assign(&mut self, other: &Self) {
        self.req.join(&other.req - &self.diff);
        self.diff += &other.diff;
    }
}

impl EquipStat {
    // Test result of changing equipment
    fn probe(&self, player: &EquipStat) -> ProbeStat {
        if self.atk > 0 && self.atk > player.atk {
            let flag = (player.flag & PlayerFlag::WEAPON_MASK) ^ self.flag;
            let atk = self.atk - player.atk;
            let equip = EquipStat {
                flag,
                atk,
                ..Default::default()
            };
            let combat = PlayerCombat {
                flag,
                atk,
                equip,
                ..Default::default()
            };
            let diff = PlayerStat {
                combat,
                ..Default::default()
            };
            ProbeStat {
                diff,
                ..Default::default()
            }
        } else if self.def > 0 && self.def > player.def {
            let flag = (player.flag & PlayerFlag::SHIELD_MASK) ^ self.flag;
            let def = self.def - player.def;
            let equip = EquipStat {
                flag,
                def,
                ..Default::default()
            };
            let combat = PlayerCombat {
                flag,
                def,
                equip,
                ..Default::default()
            };
            let diff = PlayerStat {
                combat,
                ..Default::default()
            };
            ProbeStat {
                diff,
                ..Default::default()
            }
        } else if !self.flag.is_empty() {
            let flag = (player.flag & PlayerFlag::ACCESSORY_MASK) ^ self.flag;
            let equip = EquipStat {
                flag,
                ..Default::default()
            };
            let combat = PlayerCombat {
                flag,
                equip,
                ..Default::default()
            };
            let diff = PlayerStat {
                combat,
                ..Default::default()
            };
            ProbeStat {
                diff,
                ..Default::default()
            }
        } else {
            ProbeStat::default()
        }
    }

    // TODO make this a method
    // Test result of unequipping equipment
    fn unequip(player: &EquipStat, weapon: bool, shield: bool, accessory: bool) -> ProbeStat {
        let mut flag = PlayerFlag::empty();
        let mut atk = 0;
        let mut def = 0;
        if weapon {
            flag |= player.flag & PlayerFlag::WEAPON_MASK;
            atk = -player.atk;
        }
        if shield {
            flag |= player.flag & PlayerFlag::SHIELD_MASK;
            def = -player.def;
        }
        if accessory {
            flag |= player.flag & PlayerFlag::ACCESSORY_MASK;
        }
        let combat = PlayerCombat {
            flag,
            atk,
            def,
            ..Default::default()
        };
        let diff = PlayerStat {
            combat,
            ..Default::default()
        };
        ProbeStat {
            diff,
            ..Default::default()
        }
    }

    // Test result of reequipping equipment
    fn reequip(player: &EquipStat, weapon: bool, shield: bool, accessory: bool) -> ProbeStat {
        let mut flag = PlayerFlag::empty();
        let mut atk = 0;
        let mut def = 0;
        if weapon {
            flag |= player.flag & PlayerFlag::WEAPON_MASK;
            atk = player.atk;
        }
        if shield {
            flag |= player.flag & PlayerFlag::SHIELD_MASK;
            def = player.def;
        }
        if accessory {
            flag |= player.flag & PlayerFlag::ACCESSORY_MASK;
        }
        let combat = PlayerCombat {
            flag,
            atk,
            def,
            ..Default::default()
        };
        let diff = PlayerStat {
            combat,
            ..Default::default()
        };
        ProbeStat {
            diff,
            ..Default::default()
        }
    }
}

// TODO support percent damage
// Boost health by applying multiplier to existing stats
#[derive(Clone)]
struct HpBoostStat {
    mult: PlayerCombat,
}

impl HpBoostStat {
    fn percent_floor<T: Into<i32>>(mult: T, num: T) -> i32 {
        let prod = mult.into() * num.into();
        if prod >= 0 {
            prod / 100
        } else {
            (prod - 99) / 100
        }
    }

    fn probe(&self, player: &PlayerCombat) -> ProbeStat {
        let hp = Self::percent_floor(self.mult.atk, player.atk)
            + Self::percent_floor(self.mult.def, player.def)
            + Self::percent_floor(self.mult.equip.atk, player.equip.atk)
            + Self::percent_floor(self.mult.equip.def, player.equip.def);

        let diff = PlayerStat {
            hp,
            ..Default::default()
        };

        if hp >= 0 {
            ProbeStat {
                diff,
                ..Default::default()
            }
        } else {
            ProbeStat {
                diff: diff.clone(),
                req: -diff,
            }
        }
    }
}

// Monster behavior that affect combat
bitflags! {
    struct MonsterFlag: u16 {
        const ONE_HIT               = 0b0000000001;
        const ATTACK_FIRST          = 0b0000000010;
        const SURPRISED_FROM_BEHIND = 0b0000000100;
        const ATTACK_LAST           = 0b0000001000;
        const NO_ENEMY_DEFENSE      = 0b0000010000;
        const HAS_WEAPON            = 0b0000100000;
        const GOBLIN_WEAKNESS       = 0b0001000000;
        const WYRM_WEAKNESS         = 0b0010000000;
        const BRAINED               = 0b0100000000;
        const BRAINED2              = 0b1000000000;
    }
}

// Stats of a monster
#[derive(Clone)]
struct MonsterStat {
    flag: MonsterFlag,
    hp: i32,
    atk: i16,
    def: i16,
    gr: i16,
}

impl MonsterStat {
    // Test result of fighting monster
    fn probe(&self, player: &PlayerCombat) -> ProbeStat {
        if player.flag.contains(PlayerFlag::DEAD) {
            return ProbeStat::default();
        }

        let player_atk = if player.flag.contains(PlayerFlag::DOUBLE_ATK_AGAINST_GOBLIN)
            && self.flag.contains(MonsterFlag::GOBLIN_WEAKNESS)
            || player.flag.contains(PlayerFlag::DOUBLE_ATK_AGAINST_WYRM)
                && self.flag.contains(MonsterFlag::WYRM_WEAKNESS)
        {
            player.atk as i32 * 2
        } else {
            player.atk as i32
        };

        let player_def = if self.flag.contains(MonsterFlag::NO_ENEMY_DEFENSE) {
            0
        } else {
            player.def as i32
        };

        let monster_atk = if self.flag.contains(MonsterFlag::BRAINED2) {
            self.atk as i32 * 4
        } else if self.flag.contains(MonsterFlag::BRAINED) {
            self.atk as i32 * 2
        } else {
            self.atk as i32
        };

        let monster_def = self.def as i32;
        let monster_hp = self.hp;

        // Cannot fight monster
        if player_atk <= monster_def {
            let combat = PlayerCombat {
                flag: PlayerFlag::DEAD,
                ..Default::default()
            };
            let dead = PlayerStat {
                combat,
                ..Default::default()
            };
            return ProbeStat {
                diff: dead.clone(),
                req: dead,
            };
        }

        let hp_cost = if player_def >= monster_atk {
            // Monster cannot hurt player
            0
        } else {
            // Number of hits when attacking with sword against default monster
            let mut hits = (monster_hp - 1) / (player_atk - monster_def);

            // Monster attacks first if it has behavior or player attacks without sword
            if self.flag.contains(MonsterFlag::ATTACK_FIRST)
                || !player.flag.contains(PlayerFlag::HAS_WEAPON)
            {
                hits += 1;
            }

            if self.flag.contains(MonsterFlag::ATTACK_LAST) && hits > 0 {
                hits -= 1;
            }

            // Monster attacked from behind loses its first hit
            if self.flag.contains(MonsterFlag::SURPRISED_FROM_BEHIND) && hits > 0 {
                hits -= 1;
            }

            // Stepping on a sword causes damage before starting combat
            if self.flag.contains(MonsterFlag::HAS_WEAPON) {
                hits += 1;
            }

            // Take a single hit from automatic attack or sword without fighting
            if self.flag.contains(MonsterFlag::ONE_HIT) {
                hits = 1;
            }
            hits * (monster_atk - player_def)
        };

        let monster_gr = if self.flag.contains(MonsterFlag::ONE_HIT) {
            0
        } else if player.flag.contains(PlayerFlag::DOUBLE_GR_WEAPON) {
            self.gr * 2
        } else {
            self.gr
        };

        let diff = PlayerStat {
            hp: -hp_cost,
            gr: monster_gr,
            ..Default::default()
        };
        let req = PlayerStat {
            hp: hp_cost,
            ..Default::default()
        };
        ProbeStat { diff, req }
    }
}

// Elements in a room that affect player
#[derive(Clone)]
enum Element {
    Resource(PlayerStat),    // Give player resources
    Cost(PlayerStat),        // Remove player resources
    Requirement(PlayerStat), // Require certain stats
    Monster(MonsterStat),    // Fight monster
    Equipment(EquipStat),    // Swap equipment
    Inventory {
        // Equip/Unequip equipment
        equip: bool,
        weapon: bool,
        shield: bool,
        accessory: bool,
    },
    HpBoost(HpBoostStat),
}

impl Element {
    // Test results of passing room element
    fn probe(&self, player: &PlayerCombat) -> ProbeStat {
        match self {
            Self::Resource(resource) => ProbeStat {
                diff: resource.clone(),
                req: PlayerStat::default(),
            },
            Self::Cost(cost) => ProbeStat {
                diff: -cost.clone(),
                req: cost.clone(),
            },
            Self::Requirement(req) => ProbeStat {
                diff: PlayerStat::default(),
                req: req.clone(),
            },
            Self::Monster(monster) => monster.probe(player),
            Self::Equipment(equip) => equip.probe(&player.equip),
            Self::Inventory {
                equip,
                weapon,
                shield,
                accessory,
            } => {
                if *equip {
                    EquipStat::reequip(&player.equip, *weapon, *shield, *accessory)
                } else {
                    EquipStat::unequip(&player.equip, *weapon, *shield, *accessory)
                }
            }
            Self::HpBoost(boost) => boost.probe(player),
        }
    }
}

// Special ways room should be treated when visiting
bitflags! {
    pub(super) struct RoomType: u8 {
        const INTERMEDIATE      = 0b000001;
        const ONLY_WHEN_FREE    = 0b000010;
        const PRIORITY          = 0b000100;
        const DELAYED           = 0b001000;

        #[cfg(feature = "closed-level")]
        const REPEATED          = 0b010000;
        #[cfg(feature = "closed-level")]
        const CLEAR_NEIGHBORS   = 0b100000;
    }
}

// Sequence of elements that must all be completed
pub(super) struct Room {
    pub(super) name: String,
    content: Vec<Element>,
    pub(super) room_type: RoomType,
}

impl Room {
    pub(super) fn new(name: String) -> Self {
        Self {
            name,
            content: Vec::new(),
            room_type: RoomType::empty(),
        }
    }

    // Test result of going through each element
    pub(super) fn probe(&self, player: &PlayerCombat) -> ProbeStat {
        let mut stat = player.clone();
        let mut res = ProbeStat::default();
        for element in &self.content {
            let probe = element.probe(&stat);
            res += &probe;
            stat += &probe.diff.combat;
        }
        res
    }
}

// TODO split into builder
// Represent level as a graph of rooms
pub(super) struct Level {
    pub(super) next_id: VertexIDType,
    vertices_mask: BitSet,
    pub(super) neighbors: Vec<BitSet>,
    pub(super) toggle_neighbors: Vec<BitSet>,
    use_edge: bool,
    current_vertex_id: VertexIDType,
    name2id: HashMap<String, VertexIDType>,
    vertices: Vec<Room>,
    pub(super) entrance: VertexIDType,
    pub(super) exit: VertexIDType,

    #[cfg(feature = "closed-level")]
    pub(super) boundary_mask: BitSet,
}

impl Level {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            vertices_mask: BitSet::new(),
            neighbors: Vec::new(),
            toggle_neighbors: Vec::new(),
            use_edge: false,
            entrance: u8::MAX,
            exit: u8::MAX,
            current_vertex_id: u8::MAX,
            name2id: HashMap::new(),
            vertices: Vec::new(),

            #[cfg(feature = "closed-level")]
            boundary_mask: BitSet::new(),
        }
    }

    fn select_id(&mut self, id: VertexIDType) -> &mut Self {
        self.current_vertex_id = id;
        self
    }

    //     fn select_name(&mut self, name: &str) -> &mut Self {
    //         self.select_id(self.id(name))
    //     }

    fn select_room(&mut self, room: Room) -> &mut Self {
        if self.name2id.contains_key(&room.name) {
            // TODO improve error reporting
            panic!(String::from("the room has aleady been added: ") + &room.name);
        }
        self.vertices_mask.set_bit(self.next_id as usize, true);

        #[cfg(feature = "closed-level")]
        {
            if room.room_type.contains(RoomType::REPEATED) {
                self.boundary_mask.set_bit(self.next_id as usize, true);
            }
        }

        self.current_vertex_id = self.next_id;
        self.next_id += 1;
        self.name2id
            .insert(room.name.clone(), self.current_vertex_id);
        self.vertices.push(room);
        self.neighbors.push(BitSet::new());
        self.toggle_neighbors.push(BitSet::new());
        self
    }

    fn add_arc(&mut self, id0: VertexIDType, id1: VertexIDType) -> &mut Self {
        if id0 < self.next_id && id1 < self.next_id {
            self.neighbors[id0 as usize].set_bit(id1 as usize, true)
        }
        self
    }

    fn add_id(&mut self, id: VertexIDType) -> &mut Self {
        let id0 = self.current_vertex_id;
        let id1 = self.select_id(id).current_vertex_id;
        if self.use_edge {
            self.add_arc(id1, id0);
        }
        self.add_arc(id0, id1)
    }

    pub(super) fn add_name(&mut self, name: &str) -> &mut Self {
        self.add_id(self.id(name))
    }

    pub(super) fn add_room(&mut self, room: Room) -> &mut Self {
        let id0 = self.current_vertex_id;
        let id1 = self.select_room(room).current_vertex_id;
        if self.use_edge {
            self.add_arc(id1, id0);
        }
        self.add_arc(id0, id1)
    }

    //     fn toggle(&mut self, id0: VertexIDType, id1: VertexIDType) -> &mut Self {
    //         if id0 < self.next_id && id1 < self.next_id {
    //             self.toggle_neighbors[id0 as usize].set_bit(id1 as usize, true)
    //         }
    //         self
    //     }

    //     fn toggle_name(&mut self, name0: &str, name1: &str) -> &mut Self {
    //         self.toggle(self.id(name0), self.id(name1))
    //     }

    fn id(&self, name: &str) -> VertexIDType {
        let id = self.name2id.get(name);
        *id.expect(&(String::from("cannot find vertex with given name: ") + name))
    }

    //     fn reset(&mut self) -> &mut Self {
    //         self.current_vertex_id = u8::MAX;
    //         self
    //     }

    //     fn vertex(&self) -> &Room {
    //         &self.vertices[self.current_vertex_id as usize]
    //     }

    pub(super) fn vertex_of_id(&self, id: VertexIDType) -> &Room {
        &self.vertices[id as usize]
    }

    //     fn vertex_of_name(&self, name: &str) -> &Room {
    //         self.vertex_of_id(self.id(name))
    //     }

    //     fn set_entrance(&mut self) -> &mut Self {
    //         self.entrance = self.current_vertex_id;
    //         self
    //     }

    fn set_entrance_id(&mut self, id: VertexIDType) -> &mut Self {
        self.entrance = id;
        self
    }

    pub(super) fn set_entrance_name(&mut self, name: &str) -> &mut Self {
        self.set_entrance_id(self.id(name))
    }

    //     fn set_exit(&mut self) -> &mut Self {
    //         self.exit = self.current_vertex_id;
    //         self
    //     }

    fn set_exit_id(&mut self, id: VertexIDType) -> &mut Self {
        self.exit = id;
        self
    }

    pub(super) fn set_exit_name(&mut self, name: &str) -> &mut Self {
        self.set_exit_id(self.id(name))
    }
}

// Score scaled by 1000
#[derive(Clone)]
pub(super) struct PlayerScore {
    pub(super) score: i32,
}

impl PlayerScore {
    pub(super) fn new() -> Self {
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

