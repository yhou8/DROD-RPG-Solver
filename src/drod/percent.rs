use super::stat::{CombatStat, Player, ProbeStat, StatDiff};

#[derive(Debug)]
pub(super) struct PercentDamage {
    percent: i32,
    round_up: bool,
}

impl PercentDamage {
    pub(super) fn to_probe_stat(&self, player: &CombatStat) -> ProbeStat {
        todo!()
        // let damage_scaled = (player.hp + 1) * self.percent;
        // let mut hp_cost = damage_scaled / 100;
        // if hp_cost == 0 || self.round_up && damage_scaled % 100 != 0 {
        //     hp_cost += 1;
        // }

        // let mut diff = StatDiff::default();
        // diff.hp = -hp_cost;
        // let mut req = PlayerStat::default();
        // req.hp = hp_cost;
        // ProbeStat {
        //     diff,
        //     req,
        //     loss: hp_cost,
        // }
    }
}
