bitflags! {
    #[derive(Default)]
    struct PlayerBehavior: u32 {
        const HAS_WEAPON                = 0b00001;
        const END                       = 0b00010;      // TODO remove?
        const DOUBLE_GR                 = 0b00100;
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
    gr: i32,
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
            && self.yk >= other.yk
            && self.gk >= other.gk
            && self.bk >= other.bk
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
    yk: i32,
    gk: i32,
    bk: i32,
}

impl PlayerStat {
    pub(super) fn score(&self) -> i32 {
        todo!()
    }

    pub(super) fn ge(&self, other: &Self) -> bool {
        self.behavior.contains(other.behavior)
            && self.hp >= other.hp
            && self.atk >= other.atk
            && self.def >= other.def
            && self.gr >= other.gr
            && self.yk >= other.yk
            && self.gk >= other.gk
            && self.bk >= other.bk
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(super) struct EssStat {
    behavior: PlayerBehavior,
    atk: i32,
    def: i32,
}

impl EssStat {
    pub(super) fn new(stat: PlayerStat) -> Self {
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
