struct HPBoostStat {
	STRUCT_MEMBER_DECLARATION(PlayerStat)

	ProbeStat probe(const PlayerCombat&) const;
};

// --------------------------------------------------------------------------------

ProbeStat HPBoostStat::probe(const PlayerCombat& p) const {
	int HPDiff = STRUCT_BINARY_OPERATOR_CODE(PlayerCombat, percentFloor, +) 0
	MEMBER_CODE(PlayerCombat, HP, - IntFlagOperator::percentFloor(HP, p.HP) + IntFlagOperator::percentFloor((int) HP, (int) p.HP + 1));
	if (HPDiff >= 0) {
		return ProbeStat{{
			MEMBER_INIT(PlayerStat, HP, HPDiff)
		}};
	} else {
		return ProbeStat{{
			MEMBER_INIT(PlayerStat, HP, HPDiff)
		}, {
			MEMBER_INIT(PlayerStat, HP, -HPDiff)
		}};
	}
}
