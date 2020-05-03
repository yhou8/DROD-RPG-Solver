use std::ops::{AddAssign, Neg, Sub};

bitflags! {
    #[derive(Default)]
    pub(super) struct PlayerBehavior: u32 {
        const HAS_WEAPON                = 0b00001;
        const DOUBLE_GR                 = 0b00010;
        const DOUBLE_REP                = 0b00100;
        const DOUBLE_ATK_AGAINST_GOBLIN = 0b01000;
        const DOUBLE_ATK_AGAINST_WYRM   = 0b10000;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct StatDiff {
    behavior: PlayerBehavior,
    pub(super) hp: i32,
    atk: i32,
    def: i32,
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
            && self.gr >= other.gr
            && self.rep >= other.rep
            && self.yk >= other.yk
            && self.gk >= other.gk
            && self.bk >= other.bk
    }
}

impl AddAssign for StatDiff {
    fn add_assign(&mut self, other: Self) {
        self.behavior |= other.behavior;
        self.hp += other.hp;
        self.atk += other.atk;
        self.def += other.def;
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
            && self.gr >= other.gr
            && self.rep >= other.rep
            && self.yk >= other.yk
            && self.gk >= other.gk
            && self.bk >= other.bk
    }
}

impl From<EssStat> for PlayerStat {
    fn from(stat: EssStat) -> Self {
        Self {
            behavior: stat.behavior,
            atk: stat.atk,
            def: stat.def,
            ..Self::default()
        }
    }
}

impl AddAssign<StatDiff> for PlayerStat {
    fn add_assign(&mut self, other: StatDiff) {
        self.behavior |= other.behavior;
        self.hp += other.hp;
        self.atk += other.atk;
        self.def += other.def;
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
            behavior: self.behavior - other.behavior,
            hp: self.hp - other.hp,
            atk: self.atk - other.atk,
            def: self.def - other.def,
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
    pub(super) atk: i32,
    pub(super) def: i32,
}

impl From<PlayerStat> for EssStat {
    fn from(stat: PlayerStat) -> Self {
        Self {
            behavior: stat.behavior,
            atk: stat.atk,
            def: stat.def,
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
