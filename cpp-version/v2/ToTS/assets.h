#pragma once

constexpr auto DCross = DResource({Flag: DDoubleATKAgainstGoblin});
constexpr auto DIronSword = DResource({ATK: 10});
constexpr auto DIronShield = DResource({DEF: 10});
constexpr auto DSilverSword = DResource({ATK: 20});
constexpr auto DSilverShield = DResource({DEF: 20});

NewMonster(GreenSlime,		0,					35,		18,		1,		1);
NewMonster(RedSlime,		0,					45,		20,		2,		2);
NewMonster(Bat,				0,					35,		38,		3,		3);
NewMonster(Priest,			0,					60,		32,		8,		5);
NewMonster(SkeletonC,		0,					50,		42,		6,		6);
NewMonster(SkeletonB,		0,					55,		52,		12,		8);
NewMonster(GateKeeperC,		0,					50,		48,		22,		12);
NewMonster(SkeletonA,		0,					100,	65,		15,		30);


NewMonster(BigSlime,		0,					130,	60,		3,		8);
NewMonster(BigBat,			0,					60,		100,	8,		12);
NewMonster(Zombie, 			DGoblinWeakness,	260,	85,		5,		18);
NewMonster(SuperionPriest,	0,					100,	95,		30,		22);
NewMonster(Rock,			0,					20,		100,	68,		28);
NewMonster(ZombieKnight,	DGoblinWeakness,	320,	120,	15,		30);
NewMonster(Vampire,			DGoblinWeakness,	444,	199,	66,		144);

NewMonster(SlimeMan,		0,					320,	140,	20,		30);
NewMonster(GhostSoldier,	0,					220,	180,	30,		35);
NewMonster(Soldier,			0,					210,	200,	65,		45);
NewMonster(Knight,			0,					160,	230,	105,	65);
NewMonster(GoldenKnight,	0,					120,	150,	50,		100);
NewMonster(GateKeeperB,		0,					100,	180,	110,	100);
