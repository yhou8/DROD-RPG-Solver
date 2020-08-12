struct EquipStat {
	STRUCT_MEMBER_DECLARATION(EquipStat)

	ProbeStat probe(const PlayerCombat&) const;

	static ProbeStat Unequip(const PlayerCombat&, bool weapon, bool shield, bool accesory);
	static ProbeStat Reequip(const PlayerCombat&, bool weapon, bool shield, bool accesory);
};

// --------------------------------------------------------------------------------

ProbeStat EquipStat::probe(const PlayerCombat& player) const {
	if (EquipATK && EquipATK >= player.EquipATK) {
		PlayerStat diff = {
			MEMBER_INIT(PlayerStat, Flag,		(player.EquipFlag & PlayerFlagWeaponMask) ^ EquipFlag)
			MEMBER_INIT(PlayerStat, ATK,		EquipATK - player.EquipATK)
			MEMBER_INIT(PlayerStat, EquipFlag,	(player.EquipFlag & PlayerFlagWeaponMask) ^ EquipFlag)
			MEMBER_INIT(PlayerStat, EquipATK,	EquipATK - player.EquipATK)
		};
		return ProbeStat{diff};
	} else if (EquipDEF && EquipDEF >= player.EquipDEF) {
		PlayerStat diff = {
			MEMBER_INIT(PlayerStat, Flag,		(player.EquipFlag & PlayerFlagShieldMask) ^ EquipFlag)
			MEMBER_INIT(PlayerStat, DEF,		EquipDEF - player.EquipDEF)
			MEMBER_INIT(PlayerStat, EquipFlag,	(player.EquipFlag & PlayerFlagShieldMask) ^ EquipFlag)
			MEMBER_INIT(PlayerStat, EquipDEF,	EquipDEF - player.EquipDEF)
		};
		return ProbeStat{diff};
	} else if (EquipFlag) {
		PlayerStat diff = {
			MEMBER_INIT(PlayerStat, Flag,		(player.EquipFlag & PlayerFlagAccesoryMask) ^ EquipFlag)
			MEMBER_INIT(PlayerStat, EquipFlag,	(player.EquipFlag & PlayerFlagAccesoryMask) ^ EquipFlag)
		};
		return ProbeStat{diff};
	} else {
		return ProbeStat{};
	}
}

// --------------------------------------------------------------------------------

ProbeStat EquipStat::Unequip(const PlayerCombat& player, bool weapon, bool shield, bool accesory) {
	PlayerStat diff = {
		MEMBER_INIT(PlayerStat, Flag,
			  (weapon ? (player.EquipFlag & PlayerFlagWeaponMask) : 0)
			| (shield ? (player.EquipFlag & PlayerFlagShieldMask) : 0)
			| (accesory ? (player.EquipFlag & PlayerFlagAccesoryMask) : 0)
		)
		MEMBER_INIT(PlayerStat, ATK,		weapon ? - player.EquipATK : 0)
		MEMBER_INIT(PlayerStat, DEF,		shield ? - player.EquipDEF : 0)
	};
	return ProbeStat{diff};
}

ProbeStat EquipStat::Reequip(const PlayerCombat& player, bool weapon, bool shield, bool accesory) {
	PlayerStat diff = {
		MEMBER_INIT(PlayerStat, Flag,
			  (weapon ? (player.EquipFlag & PlayerFlagWeaponMask) : 0)
			| (shield ? (player.EquipFlag & PlayerFlagShieldMask) : 0)
			| (accesory ? (player.EquipFlag & PlayerFlagAccesoryMask) : 0)
		)
		MEMBER_INIT(PlayerStat, ATK,		weapon ? player.EquipATK : 0)
		MEMBER_INIT(PlayerStat, DEF,		shield ? player.EquipDEF : 0)
	};
	return ProbeStat{diff};
}
