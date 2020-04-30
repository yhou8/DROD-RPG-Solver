#pragma once

#include "stat.h"

enum MonsterFlag : FLAG {
	DHasWeapon = 1 << 0,
	DGoblinWeakness = 1 << 1,
	DWyrmWeakness = 1 << 2,
	DAttackFirst = 1 << 3,
	DAttackLast = 1 << 4,
	DNoEnemyDefense = 1 << 5,
	DSuprisedFromBehind = 1 << 6,
	DBrained = 1 << 7,
	DBrained2 = 1 << 8,
	DOneHit = 1 << 9,
};

struct MonsterStat {
	FLAG Flag = 0;
	int HP	= 0;
	int ATK	= 0;
	int DEF	= 0;
	int GR	= 0;

	ProbeStat ToProbeStat(const EssStat& player) const;
};
