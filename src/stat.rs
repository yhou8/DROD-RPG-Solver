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

// HP is shifted by 1 so that 0 is considered alive.
// This change makes code cleaner.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerStat {
    behavior: PlayerBehavior,
    hp: i32,
    atk: i32,
    def: i32,
    gr: i32,
    yk: i32,
    gk: i32,
    bk: i32,
}
