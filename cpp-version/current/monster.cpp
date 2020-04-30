#pragma once

#include "monster.h"
#include "room.h"

ProbeStat MonsterStat::ToProbeStat(const EssStat& player) const {
	int pATK = (player.Flag & DDoubleATKAgainstGoblin) && (Flag & DGoblinWeakness)
		|| (player.Flag & DDoubleATKAgainstWyrm) && (Flag & DWyrmWeakness)
		? player.ATK * 2
		: player.ATK;
	int pDEF = (Flag & DNoEnemyDefense) ? 0 : player.DEF;

	int mATK = ATK;
	if (Flag & DBrained2) {
		mATK *= 4;
	} else if (Flag & DBrained) {
		mATK *= 2;
	}
	int mDEF = DEF;

	int HPCost;
	if (pATK <= mDEF) {
		HPCost = 1 << 24;
	} else if (pDEF >= mATK) {
		HPCost = 0;
	} else {
		int hits = (HP - 1) / (pATK - mDEF);
		if ((Flag & DAttackFirst) || !(player.Flag & DHasWeapon)) {
			++hits;
		}
		if ((Flag & DAttackLast) && hits) {
			--hits;
		}
		if ((Flag & DSuprisedFromBehind) && hits) {
			--hits;
		}
		if (Flag & DHasWeapon) {
			++hits;
		}
		if (Flag & DOneHit) {
			hits = 1;
		}
		HPCost = hits * (mATK - pDEF);
	}

	int GRGain = (player.Flag & DDoubleGR) ? GR * 2 : GR;
	if (Flag & DOneHit) {
		GRGain = 0;
	}
	StatDiff diff = {HP: -HPCost, GR: GRGain};
	Stat req = {HP: HPCost};
	return {diff, req, HPCost};
}

// --------------------------------------------------------------------------------

constexpr RoomElement DMonster(const MonsterStat m) {
	return {DMonsterType, {.monster = m}};
}

#define NewMonster(name, flag, hp, atk, def, gr) \
	constexpr auto D##name = DMonster({ Flag: flag, HP: hp, ATK: atk, DEF: def, GR: gr}); \
	constexpr RoomElement D##name##Flag(FLAG Flag = 0) { \
		return DMonster({ Flag: flag | Flag, HP: hp, ATK: atk, DEF: def, GR: gr}); \
	}; \

NewMonster(Brain,			0,						35,		9,		1,		1);
NewMonster(Roach,			0,						45,		20,		2,		2);
NewMonster(Wraithwing,		0,						35,		38,		3,		3);
NewMonster(EvilEye,			0,						60,		32,		8,		5);
NewMonster(RoachQueen,		0,						50,		42,		6,		6);
NewMonster(Spider,			0,						55,		52,		12,		8);
NewMonster(MudBaby,			0,						130,	60,		3,		8);
NewMonster(Antlion,			0,						60,		100,	8,		12);
NewMonster(MudMother,		0,						50,		48,		22,		12);
NewMonster(TarMother,		0,						100,	180,	110,	100);
NewMonster(GelMother,		0,						180,	460,	360,	200);
NewMonster(GrayMan,			0,						260,	85,		5,		18);
NewMonster(MadEye,			0,						100,	95,		30,		22);
NewMonster(Neather,			0,						100,	65,		15,		25);
NewMonster(RockGolem,		0,						20,		100,	68,		28);
NewMonster(Goblin,			DGoblinWeakness,		320,	120,	15,		30);
NewMonster(TarBaby,			0,						320,	140,	20,		30);
NewMonster(Soulless,		0,						220,	180,	30,		35);
NewMonster(Mimic,			0,						210,	200,	65,		45);
NewMonster(Swordsman,		0,						100,	680,	50,		55);
NewMonster(RedGuard,		0,						160,	230,	105,	65);
NewMonster(GelBaby,			0,						360,	310,	20,		40);
NewMonster(Fegundo,			0,						200,	390,	90,		50);
NewMonster(WaterSkipper,	0,						220,	370,	110,	80);
NewMonster(Seep,			0,						200,	380,	130,	90);
NewMonster(Pirate,			0,						180,	430,	210,	120);
NewMonster(Aumtlich,		0,						230,	450,	100,	100);
NewMonster(Wubba,			0,						10,		0,		320,	100);
NewMonster(GoblinKing,		DGoblinWeakness,		400,	199,	66,		144);
NewMonster(Slayer,			0,						4500,	560,	310,	1000);
NewMonster(RockGiant,		0,						800,	500,	100,	500);
NewMonster(Rattlesnake,		DWyrmWeakness,			1200,	180,	20,		100);
NewMonster(Adder,			DWyrmWeakness,			1500,	600,	250,	800);
NewMonster(Serpent,			DWyrmWeakness,			2500,	550,	350,	900);


NewMonster(RoachEgg,		0,						1,		0,		23,		0);
NewMonster(EvilEyeBack,		DSuprisedFromBehind,	60,		32,		8,		5);
NewMonster(EvilEyeHit,		DOneHit,				60,		32,		8,		5);
NewMonster(MadEyeBack,		DSuprisedFromBehind,	100,	95,		30,		22);
NewMonster(MadEyeHit,		DOneHit,				100,	95,		30,		22);
