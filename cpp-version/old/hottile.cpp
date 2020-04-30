constexpr RoomElement DHotTile(int p, bool ceil = true) {
	return RoomElement{hotTile, {flag: (ceil ? 1u : 0u), HP: p}};
}

constexpr auto DAumtlichBeam = DHotTile(50, false);

RoomStat hotTileToRoomStat(const Stat& hotTile, const Stat& player) {
	int HPCost = (player.HP + 1) * hotTile.HP / 100;
	if (!HPCost || hotTile.flag && ((player.HP + 1) * hotTile.HP) % 100) {
		++HPCost;
	}
	Stat diff = {HP: - HPCost};
	Stat req = {HP: HPCost};
	return RoomStat{diff, req};
}
