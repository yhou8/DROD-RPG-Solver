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
    // Monster(MonsterStat),
}

pub(super) struct Room {
    name: String,
    content: Vec<RoomElement>,
    pub(super) room_type: RoomType,
}

impl Room {
    pub(super) fn to_probe_stat(&self, player: &EssStat) -> ProbeStat {
        todo!()
    }
}
