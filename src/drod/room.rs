use super::monster::MonsterStat;
use super::percent::PercentDamage;
use super::stat::{EssStat, PlayerBehavior, PlayerStat, ProbeStat, StatDiff};

bitflags! {
    pub(super) struct RoomType: u32 {
        const INTERMEDIATE   = 0b001;
        const ONLY_WHEN_FREE = 0b010;
        const PRIORITY_ROOM  = 0b100;
    }
}

#[derive(Debug)]
enum RoomElement {
    Resource(StatDiff),
    Cost(StatDiff),
    Requirement(PlayerStat),
    Monster(MonsterStat),
    Equipment(StatDiff),
    Percent(PercentDamage),
}

impl RoomElement {
    pub(super) fn to_probe_stat(&self, player: &EssStat) -> ProbeStat {
        match self {
            RoomElement::Resource(resource) => ProbeStat {
                diff: *resource,
                req: PlayerStat::default(),
                loss: 0,
            },
            RoomElement::Cost(cost) => ProbeStat {
                diff: -*cost,
                req: (*cost).into(),
                loss: 0,
            },
            RoomElement::Requirement(req) => ProbeStat {
                diff: StatDiff::default(),
                req: *req,
                loss: 0,
            },
            RoomElement::Monster(monster) => monster.to_probe_stat(player),
            RoomElement::Equipment(equip) => {
                let mut new_behavior = PlayerBehavior::empty();
                if equip.equip_atk >= player.equip_atk && equip.equip_atk > 0 {
                    new_behavior = player.behavior & !PlayerBehavior::WEAPON_ATTR | equip.behavior;
                } else if equip.equip_def >= player.equip_def && equip.equip_def > 0 {
                    new_behavior = player.behavior
                } else if equip.equip_atk == 0 && equip.equip_def == 0 {
                    new_behavior =
                        player.behavior & !PlayerBehavior::ACCESSORY_ATTR | equip.behavior;
                }

                let mut diff = StatDiff::default();
                diff.behavior = new_behavior ^ player.behavior;
                diff.atk = 0.max(equip.equip_atk - player.equip_atk);
                diff.def = 0.max(equip.equip_def - player.equip_def);
                diff.equip_atk = 0.max(equip.equip_atk - player.equip_atk);
                diff.equip_def = 0.max(equip.equip_def - player.equip_def);

                ProbeStat {
                    diff,
                    req: PlayerStat::default(),
                    loss: 0,
                }
            }
            RoomElement::Percent(percent) => percent.to_probe_stat(player),
        }
    }
}

#[derive(Debug)]
pub(super) struct Room {
    name: String,
    content: Vec<RoomElement>,
    pub(super) room_type: RoomType,
}

impl Room {
    pub(super) fn to_probe_stat(&self, player: &EssStat) -> ProbeStat {
        let mut stat = PlayerStat::from(*player);
        let mut res = ProbeStat::default();
        for element in &self.content {
            let probe = element.to_probe_stat(&stat.into());
            res += probe;
            stat += probe.diff;
        }
        res
    }
}
