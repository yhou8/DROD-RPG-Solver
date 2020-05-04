use super::stat::{CombatStat, Player, PlayerBehavior, ProbeStat, StatDiff};

bitflags! {
    struct MonsterBehavior: u32 {
        const HAS_WEAPON            = 0b0000000001;
        const GOBLIN_WEAKNESS       = 0b0000000010;
        const WYRM_WEAKNESS         = 0b0000000100;
        const ATTACK_FIRST          = 0b0000001000;
        const ATTACK_LAST           = 0b0000010000;
        const NO_ENEMY_DEFENSE      = 0b0000100000;
        const SURPRISED_FROM_BEHIND = 0b0001000000;
        const BRAINED               = 0b0010000000;
        const BRAINED_2             = 0b0100000000;  // TODO support arbitrary number of brains?
        const ONE_HIT               = 0b1000000000;
    }
}

#[derive(Debug)]
pub(super) struct Monster {
    behavior: MonsterBehavior,
    hp: i32,
    atk: i32,
    def: i32,
    gr: i32,
    rep: i32,
}

impl Monster {
    pub(super) fn probe(&self, stat: &CombatStat) -> ProbeStat {
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
            let mut hits = (self.hp - 1) / player_atk - monster_def;
            if self.behavior.contains(MonsterBehavior::ATTACK_FIRST)
                || !stat.behavior.contains(PlayerBehavior::HAS_WEAPON)
            {
                hits += 1;
            }
            if self.behavior.contains(MonsterBehavior::ATTACK_LAST) && hits > 0 {
                hits -= 1;
            }
            if self
                .behavior
                .contains(MonsterBehavior::SURPRISED_FROM_BEHIND)
                && hits > 0
            {
                hits -= 1;
            }
            if self.behavior.contains(MonsterBehavior::HAS_WEAPON) {
                hits += 1;
            }
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
