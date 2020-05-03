use std::ops::{AddAssign, Neg, Sub};

bitflags! {
    #[derive(Default)]
    pub(super) struct PlayerBehavior: u32 {
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

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct StatDiff {
    pub(super) behavior: PlayerBehavior,
    pub(super) hp: i32,
    pub(super) atk: i32,
    pub(super) def: i32,
    pub(super) equip_atk: i32,
    pub(super) equip_def: i32,
    pub(super) gr: i32,
    pub(super) rep: i32,
    yk: i32,
    gk: i32,
    bk: i32,
}

impl StatDiff {
    pub(super) fn ge(&self, other: &Self) -> bool {
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
            && self.bk >= other.bk
    }
}

impl AddAssign for StatDiff {
    fn add_assign(&mut self, other: Self) {
        self.behavior ^= other.behavior;
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
    }
}

impl Neg for StatDiff {
    type Output = Self;

    fn neg(self) -> Self {
        assert!(self.behavior.is_empty());
        Self {
            behavior: self.behavior,
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
        }
    }
}

// HP is shifted by 1 so that 0 is considered alive.
// This change makes code cleaner.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerStat {
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
}

impl PlayerStat {
    // Calculates score scaled by 1000
    pub(super) fn score(&self) -> i32 {
        // Default score formula for ToTS floors 25 and 49
        // (self.hp + 1) * 25
        //     + (self.atk * 5 + self.yk * 10 + self.gk * 20 + self.bk + 30) * 1000
        //     + self.def * 1000 / 10 * 3;

        // Default score formula for DROD RPG
        (self.hp + 1) * 25
            + (self.atk * 5 + self.def * 3 + self.yk * 10 + self.gk * 20 + self.bk + 30) * 1000
    }

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
    }

    pub(super) fn ge(&self, other: &Self) -> bool {
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
            && self.bk >= other.bk
    }
}

impl From<StatDiff> for PlayerStat {
    fn from(stat: StatDiff) -> Self {
        Self {
            behavior: stat.behavior,
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
        }
    }
}

impl From<EssStat> for PlayerStat {
    fn from(stat: EssStat) -> Self {
        Self {
            behavior: stat.behavior,
            hp: stat.hp,
            atk: stat.atk,
            def: stat.def,
            equip_atk: stat.equip_atk,
            equip_def: stat.equip_def,
            ..Self::default()
        }
    }
}

impl AddAssign<StatDiff> for PlayerStat {
    fn add_assign(&mut self, other: StatDiff) {
        self.behavior &= other.behavior;
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
    }
}

impl Sub<StatDiff> for PlayerStat {
    type Output = Self;

    fn sub(self, other: StatDiff) -> Self {
        Self {
            behavior: self.behavior ^ other.behavior,
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
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(super) struct EssStat {
    pub(super) behavior: PlayerBehavior,
    pub(super) hp: i32,
    pub(super) atk: i32,
    pub(super) def: i32,
    pub(super) equip_atk: i32,
    pub(super) equip_def: i32,
}

impl From<PlayerStat> for EssStat {
    fn from(stat: PlayerStat) -> Self {
        Self {
            behavior: stat.behavior,
            hp: stat.hp,
            atk: stat.atk,
            def: stat.def,
            equip_atk: stat.equip_atk,
            equip_def: stat.equip_def,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ProbeStat {
    pub(super) diff: StatDiff,
    pub(super) req: PlayerStat,
    pub(super) loss: i32,
}

impl AddAssign for ProbeStat {
    fn add_assign(&mut self, other: Self) {
        self.req.join(other.req - self.diff);
        self.diff += other.diff;
        self.loss += other.loss;
    }
}
