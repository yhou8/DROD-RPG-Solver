#include "drod_config.h"
#include "optimizer.h"
#include "drod_assets.cpp"

#define LEVEL_NAME DarkOozerDefeated

struct LEVEL_NAME {
	LevelPtr build() const;

	constexpr static Player initPlayer = {HP: 500 - 1, ATK: 10, DEF: 10};
};

LevelPtr LEVEL_NAME::build() const {
	LevelPtr level = {new Level()};

	level >> new Room{"O", {DYK}};
	level()->setEntrance("O");

	level >> new Room{"exit", {}};
	level()->setExit("exit");


	constexpr auto DFireSpider = DMonster({HP: 52, ATK: 32, DEF: 8, GR: 3});
	constexpr auto DDarkOozer = DMonster({HP: 66, ATK: 82, DEF: 20, GR: 10});

	level >> "O" >> new Room{"U1", {DYD, DRoach, {DATK(2), DDEF(1), DHP(15), DYK}}};
	level >> "O" >> new Room{"U2", {DYD, DWraithwing, {DATK(3), DHP(50), DYK}}};
	level >> "O" >> new Room{"U3", {DYD, DRoach, {DATK(1), DHP(200), DYK}}};
	level >> "O" >> new Room{"U4", {DYD, DWraithwing, {DATK(2), DDEF(1), DHP(200), DYK}}};
	level >> "O" >> new Room{"U5", {DYD, DRoach, {DATK(1), DDEF(1), DYK}}};
	level >> "O" >> new Room{"U6", {DYD, DWraithwing, {DATK(1), DDEF(2), DYK}}};

	level >> "O" >> new Room{"L1", {DYD, DFireSpider, {DATK(2), DDEF(1), DHP(200), DYK}}};
	level >> "O" >> new Room{"L2", {DYD, DWraithwing, {DATK(1), DHP(50), DYK}}};
	level >> "O" >> new Room{"L3", {DYD, DFireSpider, {DHP(50), DDEF(2), DYK}}};
	level >> "O" >> new Room{"L4", {DYD, DSpider, {DATK(3), DYK})}};
	level >> "O" >> new Room{"L5", {DYD, DFireSpider, {DDEF(2), DHP(200), DYK}}};

	level >> "O" >> new Room{"Boss", {DGG(30), DRoach, DRoach, DRoach, DGG(10), DYD, DDarkOozer}}};
	level()->setExit("Boss");

	std::cout << "size of level: " << level()->N << std::endl;
	assert(level()->N <= Bitset::capacity);
	return level;
}

int main() {
	std::ofstream os(std::string(BOOST_PP_STRINGIZE(LEVEL_NAME)) + ".txt");
	Search s = {
		searchConfig: {
			useEstimatedMaxCombat: true,
			printNewHighscore: false,
			calculateOptimalPlayerByStat: false,
			printLocalOptimalPlayerByScore: false,
			printLocalOptimalPlayerByStat: false,
			printGlobalOptimalPlayerByScore: true,
			printGlobalOptimalPlayerByStat: false,
			osp: &os,
			logosp: &std::cout,
		},
		levelInfo: LevelInfo::create<LEVEL_NAME>(),
		initPlayer: LEVEL_NAME::initPlayer,
	};
	s.search();
}
