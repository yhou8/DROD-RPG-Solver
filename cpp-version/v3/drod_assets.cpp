#pragma once

constexpr auto DYK = DResource({
	MEMBER_INIT_CHECK(PlayerStat, YK, 1)
});

constexpr auto DGK = DResource({
	MEMBER_INIT_CHECK(PlayerStat, GK, 1)
});

constexpr auto DBK = DResource({
	MEMBER_INIT_CHECK(PlayerStat, BK, 1)
});

constexpr Element DHP(int n) {
	return DResource({
		MEMBER_INIT_CHECK(PlayerStat, HP, n)
	});
}

constexpr Element DATK(int n) {
	return DResource({
		MEMBER_INIT_CHECK(PlayerStat, ATK, n)
	});
}

constexpr Element DDEF(int n) {
	return DResource({
		MEMBER_INIT_CHECK(PlayerStat, DEF, n)
	});
}

constexpr Element DGR(int n) {
	return DResource({
		MEMBER_INIT_CHECK(PlayerStat, GR, n)
	});
}

constexpr auto DYD = DCost({
	MEMBER_INIT_CHECK(PlayerStat, YK, 1)
});

constexpr auto DGD = DCost({
	MEMBER_INIT_CHECK(PlayerStat, GK, 1)
});

constexpr auto DBD = DCost({
	MEMBER_INIT_CHECK(PlayerStat, BK, 1)
});

constexpr Element DGG(int n) {
	return DCost({
		MEMBER_INIT_CHECK(PlayerStat, GR, n)
	});
}

// --------------------------------------------------------------------------------

#define NewMonster(name, flag, hp, atk, def, gr) \
	constexpr auto D##name = DMonster({ \
		MEMBER_INIT_CHECK(MonsterStat, Flag, flag) \
		MEMBER_INIT_CHECK(MonsterStat, HP, hp) \
		MEMBER_INIT_CHECK(MonsterStat, ATK, atk) \
		MEMBER_INIT_CHECK(MonsterStat, DEF, def) \
		MEMBER_INIT_CHECK(MonsterStat, GR, gr) \
	}); \
	constexpr Element D##name##Flag(u32 Flag = 0) { \
		return DMonster({ \
			MEMBER_INIT_CHECK(MonsterStat, Flag, flag | Flag) \
			MEMBER_INIT_CHECK(MonsterStat, HP, hp) \
			MEMBER_INIT_CHECK(MonsterStat, ATK, atk) \
			MEMBER_INIT_CHECK(MonsterStat, DEF, def) \
			MEMBER_INIT_CHECK(MonsterStat, GR, gr) \
		}); \
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

// --------------------------------------------------------------------------------

#define NewEquipment(name, flag, atk, def) \
	constexpr auto D##name = DEquipment({ \
		MEMBER_INIT_CHECK(EquipStat, EquipFlag, flag) \
		MEMBER_INIT_CHECK(EquipStat, EquipATK, atk) \
		MEMBER_INIT_CHECK(EquipStat, EquipDEF, def) \
	}); \

NewEquipment(WoodenBlade,		DHasWeapon,								10,		0);
NewEquipment(LuckyBlade,		DHasWeapon | DDoubleGRWeapon,			10,		0);
NewEquipment(ShortSword,		DHasWeapon,								30,		0);
NewEquipment(GoblinBiter,		DHasWeapon | DDoubleATKAgainstGoblin,	30,		0);
NewEquipment(LongSword,			DHasWeapon,								70,		0);
NewEquipment(Hook,			 	DHasWeapon,								120,	0);
NewEquipment(WyrmSmiter,		DHasWeapon | DDoubleATKAgainstWyrm,		120,	0);
NewEquipment(ReallyBigSword,	DHasWeapon,								220,	0);

NewEquipment(WoodenShield,		0,										0,		10);
NewEquipment(BronzeShield,		0,										0,		30);
NewEquipment(SteelShield,		0,										0,		70);
NewEquipment(KnightShield,		0,										0,		120);
NewEquipment(OremiteShield,		0,										0,		220);

//NewEquipment(LuckyGreckle,		DDoubleGRAccessory,						0,		0);
//NewEquipment(VIPCard,			DDoubleREPAccessory,					0,		0);

// --------------------------------------------------------------------------------
