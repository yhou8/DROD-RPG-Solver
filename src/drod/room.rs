use super::monster::MonsterStat;
use super::stat::{EssStat, PlayerStat, ProbeStat, StatDiff};

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
    Cost(StatDiff, PlayerStat),
    Requirement(PlayerStat),
    Monster(MonsterStat),
}

impl RoomElement {
    pub(super) fn to_probe_stat(&self, player: &EssStat) -> ProbeStat {
        match self {
            RoomElement::Resource(resource) => ProbeStat {
                diff: *resource,
                req: PlayerStat::default(),
                loss: 0,
            },
            RoomElement::Cost(cost, req) => ProbeStat {
                diff: -*cost,
                req: *req,
                loss: 0,
            },
            RoomElement::Requirement(req) => ProbeStat {
                diff: StatDiff::default(),
                req: *req,
                loss: 0,
            },
            RoomElement::Monster(monster) => monster.to_probe_stat(player),
        }
    }
}

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
