constexpr RoomElement newEquipment(const char* name, unsigned int flag, int ATK, int DEF) {
	auto s = Stat{
		.flag = flag,
		.equipATK = ATK,
		.equipDEF = DEF
	};
	return RoomElement{type: equipment, stat: s};
}

constexpr auto DWoodenBlade = newEquipment("Wooden Blade", DHasWeapon, 10, 0);
constexpr auto DLuckyBlade = newEquipment("Lucky Blade", DHasWeapon | DDoubleGRWeapon, 10, 0);
constexpr auto DShortSword = newEquipment("Short Sword", DHasWeapon, 30, 0);
constexpr auto DGoblinBiter = newEquipment("Goblin Biter", DHasWeapon | DDoubleATKAgainstGoblin, 30, 0);
constexpr auto DLongSword = newEquipment("Long Sword", DHasWeapon, 70, 0);
constexpr auto DHook = newEquipment("Hook", DHasWeapon, 120, 0);
constexpr auto DWyrmSmiter = newEquipment("Wyrm Smiter", DHasWeapon | DDoubleATKAgainstWyrm, 120, 0);
constexpr auto DReallyBigSword = newEquipment("Really Big Sword", DHasWeapon, 220, 0);

constexpr auto DWoodenShield = newEquipment("Wooden Shield", 0, 0, 10);
constexpr auto DBronzeShield = newEquipment("Bronze Shield", 0, 0, 30);
constexpr auto DSteelShield = newEquipment("Steel Shield", 0, 0, 70);
constexpr auto DKnightShield = newEquipment("Knight Shield", 0, 0, 120);
constexpr auto DOremiteShield = newEquipment("Oremite Shield", 0, 0, 220);

constexpr auto DLuckyGreckle = newEquipment("Lucky Greckle", DDoubleGRAccessory, 0, 0);
constexpr auto DVIPCard = newEquipment("VIP Card", DDoubleREPAccessory, 0, 0);

RoomStat equipmentToRoomStat(const Stat& equip, const Stat& player) {
	unsigned int newFlag = 0;
	if (equip.equipATK >= player.equipATK && equip.equipATK) {
		newFlag = (player.flag & ~DWeaponAttr) | equip.flag;
	} else if (equip.equipDEF >= player.equipDEF && equip.equipDEF) {
		newFlag = player.flag;
	} else if (equip.equipATK == 0 && equip.equipDEF == 0) {
		newFlag = (player.flag & ~DAccessoryAttr) | equip.flag;
	}
	Stat diff = {
		flag: newFlag ^ player.flag,
		ATK: equip.equipATK > player.equipATK ? equip.equipATK - player.equipATK : 0,
		DEF: equip.equipDEF > player.equipDEF ? equip.equipDEF - player.equipDEF : 0,
		equipATK: equip.equipATK > player.equipATK ? equip.equipATK - player.equipATK : 0,
		equipDEF: equip.equipDEF > player.equipDEF ? equip.equipDEF - player.equipDEF : 0,
	};
	Stat req = {};
	return RoomStat{diff, req};
}

RoomStat saveEquipToRoomStat(const Stat& equip, const Stat& player) {
	Stat diff = {
		flag: DHasWeapon,
		ATK: -player.equipATK,
		DEF: -player.equipDEF
	};
	Stat req = {};
	return RoomStat{diff, req};
};

RoomStat restoreEquipToRoomStat(const Stat& equip, const Stat& player) {
	Stat diff = {
		flag: DHasWeapon,
		ATK: player.equipATK,
		DEF: player.equipDEF
	};
	Stat req = {};
	return RoomStat{diff, req};
}


constexpr RoomElement DSaveEquipment = RoomElement{type: saveEquip};
constexpr RoomElement DRestoreEquipment = RoomElement{type: restoreEquip};
