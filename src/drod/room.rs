use super::monster::Monster;
use super::percent::PercentDamage;
use super::stat::{CombatStat, Player, PlayerBehavior, ProbeStat, StatDiff};

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
    Requirement(Player),
    Monster(Monster),
    Equipment(StatDiff),
    Percent(PercentDamage),
}

impl RoomElement {
    pub(super) fn probe(&self, stat: &CombatStat) -> ProbeStat {
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
                let mut new_behavior = PlayerBehavior::empty();
                if equip.equip_atk >= stat.equip_atk && equip.equip_atk > 0 {
                    new_behavior = stat.behavior & !PlayerBehavior::WEAPON_ATTR | equip.behavior;
                } else if equip.equip_def >= stat.equip_def && equip.equip_def > 0 {
                    new_behavior = stat.behavior
                } else if equip.equip_atk == 0 && equip.equip_def == 0 {
                    new_behavior = stat.behavior & !PlayerBehavior::ACCESSORY_ATTR | equip.behavior;
                }

                let mut diff = StatDiff::default();
                diff.behavior = new_behavior ^ stat.behavior;
                diff.atk = 0.max(equip.equip_atk - stat.equip_atk);
                diff.def = 0.max(equip.equip_def - stat.equip_def);
                diff.equip_atk = 0.max(equip.equip_atk - stat.equip_atk);
                diff.equip_def = 0.max(equip.equip_def - stat.equip_def);

                ProbeStat {
                    diff,
                    req: Player::default(),
                    damage: 0,
                }
            }
            RoomElement::Percent(percent) => percent.probe(stat),
        }
    }
}

#[derive(Debug)]
pub(super) struct Room {
    pub(super) name: String,
    content: Vec<RoomElement>,
    pub(super) room_type: RoomType,
}

impl Room {
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
