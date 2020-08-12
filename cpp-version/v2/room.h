#pragma once

#include <vector>
#include <string>
#include "stat.h"
#include "monster.h"

enum RoomElementType {
	DResourceType,
	DCostType,
	DRequirementType,
	DMonsterType
};

struct RoomElement {
	RoomElementType type;
	union {
		StatDiff resource; // resource >= 0
		StatDiff cost; // cost.Flag = 0, cost >= 0
		Stat req; // req >= 0
		MonsterStat monster;
	};

	ProbeStat ToProbeStat(const EssStat& player) const;
};

// --------------------------------------------------------------------------------

enum RoomType {
	DIntermediate = 1 << 0,
	DOnlyWhenFree = 1 << 1,
	DPriorityRoom = 1 << 3,
};

struct Room {
	std::string name;
	std::vector<RoomElement> content;
	RoomType type;

	ProbeStat ToProbeStat(const EssStat& player) const;
};
