enum ElementEnum {
	DResourceElement,
	DCostElement,
	DRequirementElement,
	DSetCounterElement,
	DMonsterElement,
	DEquipmentElement,
	DInventoryElement,
	DHPBoostElement,
	DRoomElement,
//	DChoiceElement,
};

struct Element {
	ElementEnum type;
	union {
		PlayerStat		resource;		// resource >= 0
		PlayerStat		cost;			// cost.Flag = 0, cost >= 0
		PlayerStat		req;			// req >= 0
		int				set;			// set more stats
		HPBoostStat		boost;
		MonsterStat		monster;
		EquipStat		equip;			// Swap and equip
		std::tuple<bool, bool, bool, bool> inventory; // unequip/requip, weapon, shield, accessory
		Room*			room;
	};

	ProbeStat probe(const PlayerCombat& player) const;
};

// --------------------------------------------------------------------------------

enum RoomType {
	DIntermediateRoom		= 1 << 0,
	DOnlyWhenFreeRoom		= 1 << 1,
	DPriorityRoom			= 1 << 2,
	DDelayedRoom			= 1 << 3,
	DRepeatedRoom			= 1 << 4,	// very dangerous. currently, cannot visit the same room consecutively
	DClearNeighboursRoom	= 1 << 5,	// very dangerous. currently, cannot visit the same room consecutively
};

struct Room {
	std::string name;
	std::vector<Element> content;
	RoomType type;

	ProbeStat probe(const PlayerCombat& player) const;
};

// --------------------------------------------------------------------------------

ProbeStat Element::probe(const PlayerCombat& player) const {
	switch (type) {
		case DResourceElement:
			return {resource, {}};
		case DCostElement:
			return {-cost, cost};
		case DRequirementElement:
			return {{}, req};
		case DSetCounterElement:
			return {{
				MEMBER_INIT(PlayerStat, Counter, set - player.Counter)
			}, {}};
		case DMonsterElement:
			return monster.probe(player);
		case DEquipmentElement:
			return equip.probe(player);
		case DInventoryElement:
			if (std::get<0>(inventory)) {
				return EquipStat::Reequip(player, std::get<1>(inventory), std::get<2>(inventory), std::get<3>(inventory));
			} else {
				return EquipStat::Unequip(player, std::get<1>(inventory), std::get<2>(inventory), std::get<3>(inventory));
			}
		case DHPBoostElement:
			return boost.probe(player);
		case DRoomElement:
			return room->probe(player);
	};
	assert(0);
	return {};
};

// --------------------------------------------------------------------------------
ProbeStat Room::probe(const PlayerCombat& player) const {
	auto stat = player;
	ProbeStat res;
	int i = 0;
	for (auto r : content) {
		i += 1;
		auto probe = r.probe(stat);
		res += probe;
		stat += probe.diff;
	}
	return res;
}

// --------------------------------------------------------------------------------

constexpr Element DResource(const PlayerStat resource) {
	return {DResourceElement, {.resource = resource}};
}

constexpr Element DCost(const PlayerStat cost) {
	return {DCostElement, {.cost = cost}};
}

constexpr Element DRequirement(const PlayerStat req) {
	return {DRequirementElement, {.req = req}};
}

constexpr Element DSetCounter(const int set) {
	return {DSetCounterElement, {.set = set}};
}

constexpr Element DMonster(const MonsterStat m) {
	return {DMonsterElement, {.monster = m}};
}

constexpr Element DEquipment(const EquipStat e) {
	return {DEquipmentElement, {.equip = e}};
}

constexpr Element DInventory(bool reequip, bool weapon, bool shield, bool accesory) {
	return {DInventoryElement, {.inventory = {reequip, weapon, shield, accesory}}};
}

constexpr Element DHPBoost(const HPBoostStat boost) {
	return {DHPBoostElement, {.boost = boost}};
}

Element DRoom(Room* room) {
	return {DRoomElement, {.room = room}};
}
