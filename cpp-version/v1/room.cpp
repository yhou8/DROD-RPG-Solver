#include <vector>

enum RoomType {
	monster,
	monsterHitFirst,
	monsterHitLate,
	resource,
	cost,
	requirment,
	setStat,
	equipment,
	hotTile,
	saveEquip,
	restoreEquip,
	HPBoost
};

struct RoomStat {
	Stat diff;
	Stat req;
};

struct RoomElement {
	RoomType type;
	Stat stat;

	RoomStat toRoomStat(const Stat& player) const;
};

RoomStat monsterToRoomStat(const Stat& monster, const Stat& player, bool hitFirst = false, bool hitLate = false);

RoomStat equipmentToRoomStat(const Stat& equip, const Stat& player);

RoomStat hotTileToRoomStat(const Stat& hotTile, const Stat& player);

RoomStat saveEquipToRoomStat(const Stat& equip, const Stat& player);

RoomStat restoreEquipToRoomStat(const Stat& equip, const Stat& player);

RoomStat HPBoostToRoomStat(const Stat& ratio, const Stat& player);

Stat setStatDiff(const Stat& target, const Stat& player);

RoomStat RoomElement::toRoomStat(const Stat& player) const {
	switch (type) {
	case monster:
		return monsterToRoomStat(stat, player);
	case monsterHitFirst:
		return monsterToRoomStat(stat, player, true, false);
	case monsterHitLate:
		return monsterToRoomStat(stat, player, false, true);
	case resource:
		return RoomStat{stat, nullStat};
	case cost:
		return RoomStat{-stat, stat};
	case requirment:
		return RoomStat{nullStat, stat};
	case setStat:
		return RoomStat{setStatDiff(stat, player), nullStat};
	case equipment:
		return equipmentToRoomStat(stat, player);
	case hotTile:
		return hotTileToRoomStat(stat, player);
	case saveEquip:
		return saveEquipToRoomStat(stat, player);
	case restoreEquip:
		return restoreEquipToRoomStat(stat, player);
	case HPBoost:
		return HPBoostToRoomStat(stat, player);
	}
	exit(1);
}


constexpr RoomElement DRequirement(const Stat s) {
	return RoomElement{requirment, s};
}


constexpr RoomElement DResource(const Stat s) {
	return RoomElement{resource, s};
}

constexpr RoomElement DFlag(const unsigned int flag) {
	return DResource({flag: flag});
}

constexpr RoomElement DSetStat(const Stat s) {
	return RoomElement{setStat, s};
}

Stat setStatDiff(const Stat& l, const Stat& r) {
	Stat res;
	int* pl = (int*)(&l);
	int* pr = (int*)(&r);
	int* pres = (int*)(&res);
	for (int i = 0; i < sizeof(Stat) / sizeof(int); ++i) {
		pres[i] = pl[i] ? pl[i] - pr[i] : 0;
	}
	return res;
}

constexpr RoomElement DHPBoost(const Stat s) {
	return RoomElement{HPBoost, s};
}

RoomStat HPBoostToRoomStat(const Stat& ratio, const Stat& player) {
	int HPDiff = innerProduct(ratio, player);
	return {{HP: HPDiff}, {}};
}

constexpr auto DYK = DResource({YK: 1});
constexpr auto DGK = DResource({GK: 1});
constexpr auto DBK = DResource({BK: 1});
constexpr auto DSK = DResource({SK: 1});

constexpr RoomElement DHP(int n) {
	return DResource({HP: n});
}

constexpr RoomElement DATK(int n) {
	return DResource({ATK: n});
}

constexpr RoomElement DDEF(int n) {
	return DResource({DEF: n});
}

constexpr RoomElement DGR(int n) {
	return DResource({GR: n});
}

constexpr RoomElement DCost(const Stat s) {
	return RoomElement{cost, s};
}

constexpr auto DYD = DCost({YK: 1});
constexpr auto DGD = DCost({GK: 1});
constexpr auto DBD = DCost({BK: 1});
constexpr auto DSD = DCost({SK: 1});

constexpr RoomElement DGG(int n) {
	return DCost({GR: n});
}

struct Room {
	std::string name;
	std::vector<RoomElement> content;

	bool hotTile = false;

	RoomStat toRoomStat(const Stat& player) const;

	Room& simplify();
};

RoomStat Room::toRoomStat(const Stat& player) const {
	Stat stat = player;
	RoomStat res;
	for (auto r : content) {
		auto [diff, req] = r.toRoomStat(stat);
		res.req = max(res.req, req - res.diff);
		res.diff += diff;
		stat += diff;
	}
	return res;
}

Room& Room::simplify() {
	// TODO: merge adjacent resources
	// TODO: some other reordering
	return *this;
}