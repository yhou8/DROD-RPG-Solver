enum MonsterFlagEnum : MonsterFlagType {
	DOneHit					= 1 << 0,
	DAttackFirst			= 1 << 1,
	DSuprisedFromBehind		= 1 << 2,
	DAttackLast				= 0,
	DNoEnemyDefense			= 0,
	DMonsterHasWeapon		= 0,
	DGoblinWeakness			= 0,
	DWyrmWeakness			= 0,
	DBrained				= 0,
	DBrained2				= 0,
};

struct MonsterStat {
	STRUCT_MEMBER_DECLARATION(MonsterStat)

	ProbeStat probe(const PlayerCombat& p) const;
};

// --------------------------------------------------------------------------------

ProbeStat MonsterStat::probe(const PlayerCombat& player) const {
	if (player.Flag & DDead) {
		return {};
	}

	int pATK = (player.Flag & DDoubleATKAgainstGoblin) && (Flag & DGoblinWeakness)
		|| (player.Flag & DDoubleATKAgainstWyrm) && (Flag & DWyrmWeakness)
		? player.ATK * 2
		: player.ATK;

	int pDEF = (Flag & DNoEnemyDefense) ? 0 : player.DEF;

	int mATK = ATK;
	if (Flag & DBrained2) { mATK *= 4; }
	else if (Flag & DBrained) { mATK *= 2; }
	int mDEF = DEF;

	int mHP = HP;

	if (pATK <= mDEF) {
		PlayerStat dead = {Flag: DDead};
		return {dead, dead};
	}

	int HPCost = 0;
	if (pDEF < mATK) {
		int hits = (mHP - 1) / (pATK - mDEF);
		if ((Flag & DAttackFirst) || !(player.Flag & DHasWeapon)) {
			++hits;
		}
		if ((Flag & DAttackLast) && hits) {
			--hits;
		}
		if ((Flag & DSuprisedFromBehind) && hits) {
			--hits;
		}
		if (Flag & DMonsterHasWeapon) {
			++hits;
		}
		if (Flag & DOneHit) {
			hits = 1;
		}
		HPCost = hits * (mATK - pDEF);
	}

	int mGR = (player.Flag & DDoubleGRWeapon)
		? GR * 2
		: GR;
	if (Flag & DOneHit) {
		mGR = 0;
	}

	PlayerStat diff = {
		MEMBER_INIT(PlayerStat, HP, -HPCost)
		MEMBER_INIT(PlayerStat, GR, mGR)
	};
	PlayerStat req = {
		MEMBER_INIT(PlayerStat, HP, HPCost)
	};
	return {diff, req};
}
