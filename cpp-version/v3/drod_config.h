#define FlagMemberID						0
#define ATKMemberID							1
#define DEFMemberID							2
#define EquipFlagMemberID					3
#define EquipATKMemberID					4
#define EquipDEFMemberID					5
#define HPMemberID							6
#define GRMemberID							7
#define YKMemberID							8
#define GKMemberID							9
#define BKMemberID							10
#define CounterMemberID						11
#define visitedMemberID						12
#define locationMemberID					13
#define neighboursMemberID					14
#define memoryMemberID						15
#define scoreMemberID						16
#define disabledMemberID					17

#define VertexIDType						u8
#define PlayerFlagType						u8
#define MonsterFlagType						u8

#define ScoreFunction						(HP + 1) * 25 + (ATK * 5 + DEF * 3 + YK * 10 + GK * 20 + BK * 30) * 1000

#define Bitset								_Bitset<64>
#define LevelModel							Digraph<Room*>

#define PlayerProgressHash					std::hash<Bitset>()(t.visited)
#define PlayerCombatHash					t.Flag ^ (t.ATK << 10) ^ (t.DEF << 20) ^ (t.EquipATK << 5) ^ (t.EquipDEF << 15)

#define MEMBER_LIST \
	((																																(MonsterStat)				)			(MonsterFlagType)	(Flag)			(0)		(0)					)	\
	((	(Player)												(PlayerObjective)					(PlayerStat)					(MonsterStat)				)			(i32)				(HP)			(0)		(0)					)	\
	((	(Player)																					(PlayerStat)	(PlayerCombat)								)			(PlayerFlagType)	(Flag)			(0)		(0)					)	\
	((	(Player)																					(PlayerStat)	(PlayerCombat)	(MonsterStat)				)			(i16)				(ATK)			(0)		(0)					)	\
	((	(Player)																					(PlayerStat)	(PlayerCombat)	(MonsterStat)				)			(i16)				(DEF)			(0)		(0)					)	\
	((	(Player)																					(PlayerStat)	(PlayerCombat)					(EquipStat)	)			(u8)				(EquipFlag)		(0)		(0)					)	\
	((	(Player)																					(PlayerStat)	(PlayerCombat)					(EquipStat)	)			(i16)				(EquipATK)		(0)		(0)					)	\
	((	(Player)																					(PlayerStat)	(PlayerCombat)					(EquipStat)	)			(i16)				(EquipDEF)		(0)		(0)					)	\
	((	(Player)																					(PlayerStat)					(MonsterStat)				)			(i16)				(GR)			(0)		(0)					)	\
	((	(Player)																					(PlayerStat)												)			(i8)				(YK)			(0)		(0)					)	\
	((	(Player)																					(PlayerStat)												)			(i8)				(GK)			(0)		(0)					)	\
	((	(Player)																					(PlayerStat)												)			(i8)				(BK)			(0)		(0)					)	\
	((	(Player)	(PlayerProgress)	(PlayerProgressDiff)						(LevelStat)		(PlayerStat)	(PlayerCombat)								)			(i8)				(Counter)		(0)		(IGNORE)			)	\
	((	(Player)	(PlayerProgress)	(PlayerProgressDiff)																									)			(Bitset)			(visited)		({})	(0)					)	\
	((	(Player)	(PlayerProgress)	(PlayerProgressDiff)																									)			(Bitset)			(memory)		({})	(IGNORE)			)	\
	((	(Player)																																				)			(Bitset)			(disabled)		({})	(IGNORE)			)	\
	((	(Player)																																				)			(Bitset)			(neighbours)	({})	(0)					)	\
	((									(PlayerProgressDiff)																									)			(VertexIDType)		(location)		(-1)	(0)					)	\
	((	(PlayerScore)																																			)			(i32)				(score)			(0)		(0)					)	\

