#include <unordered_map>
#include <unordered_set>
#include <queue>
#include <bitset>
#include <list>

//constexpr Stat maxStat = {flag: DHasWeapon, HP: 1 << 20, ATK: 27, DEF: 43}; // This depends on altar

constexpr Stat maxStat = {flag: DHasWeapon, HP: 1 << 20, ATK: 1000, DEF: 1000}; // This depends on altar


struct Player {
	Stat stat;
	Map* map;
	std::vector<int> trace;
	std::unordered_set<int> neighbours;
	std::vector<int> inNeighbourCount;
	std::unordered_map<int, bool> available;
	std::unordered_map<int, bool> isFree;
	bool hasFree;
	std::bitset<MaxRoomsPerMap> visitedBitset;
	std::bitset<MaxRoomsPerMap> oldVisitedBitset;

	Player& enter(Map* map, int entrance = -1) {
		this->map = map;
		inNeighbourCount = map->inNeighbourRequirement;
		neighbours.insert(entrance < 0 ? map->entrance : entrance);
		return *this;
	}

	void probe(Stat stat) {
		auto estat = EssStat(stat);
		if (!map->probeResult.contains(estat)) {
			std::vector<RoomStat> r;
			for (int id = 0; id < map->ids.size(); ++id) {
				r.push_back(map->vertices[id]->toRoomStat(stat));
			}
			map->probeResult[estat] = r;
		}
	}

	void probe() {
		auto estat = EssStat(stat);
		probe(stat);
		probe(maxStat);
		auto maxEstat = EssStat(maxStat);
		hasFree = false;
		for (auto id : neighbours) {
			if (map->vertices[id]->hotTile) {
				map->probeResult[estat][id] = map->vertices[id]->toRoomStat(stat);
			}
			stat.HP += (1 << 20);
			available[id] = stat >= map->probeResult[estat][id].req;
			stat.HP -= (1 << 20);
			isFree[id] = available[id] && ((stat.flag & DEnd) && map->probeResult[estat][id].diff.score() >= 0 && id != map->exit || map->priority[id] || map->probeResult[estat][id].diff.YK >= 0 && map->probeResult[estat][id].diff.GK >= 0 && map->probeResult[estat][id].diff.GR >= 0 && map->probeResult[estat][id].diff >= map->probeResult[maxEstat][id].diff && id != map->exit);
			hasFree |= isFree[id];
		}
	}

	void visit(std::string name) {
		visit(map->name2id[name]);
	}

	void visit(int id) {
		stat += map->vertices[id]->toRoomStat(stat).diff;
		oldVisitedBitset = visitedBitset;
		visitedBitset[id] = 1;
		trace.push_back(id);
		if (map->levelEntrance[id]) {
			neighbours.clear();
		}
		neighbours.erase(id);
		for (auto nb : map->neighbours[id]) {
			++inNeighbourCount[nb];
			if (!visitedBitset[nb] && inNeighbourCount[nb] >= 0) {
				neighbours.insert(nb);
			}
		}
	}
};

std::ostream& operator << (std::ostream& os, const Player& p) {
	int score = p.stat.score();
	os << "Score: " << score / 1000 << ".";
	if (score % 1000 < 100) {
		os << "0";
	}
	if (score % 1000 < 10) {
		os << "0";
	}
	os << score % 1000  << "\n";
	os << "Stat: " << p.stat << "\n";
	os << "Neighbours: ";
	bool first = true;
	for (auto id : p.neighbours) {
		if (first) {
			first = false;
		} else {
			os << ", ";
		}
		os << p.map->vertices[id]->name;
	}
	os << "\n";
	os << "Trace: ";
	first = true;
	for (auto id : p.trace) {
		if (first) {
			first = false;
		} else {
			os << ", ";
		}
		os << p.map->vertices[id]->name;
	}
	os << "\n";
	return os;
}

std::unordered_map< std::bitset<MaxRoomsPerMap>, Player > optimalPlayer;

std::unordered_map< std::bitset<MaxRoomsPerMap>, int > optimalNeededCount;

std::queue< std::bitset<MaxRoomsPerMap> > clones;

void tryRemoveOptimalPlayer(std::bitset<MaxRoomsPerMap> visitedBitset) {
	if (!optimalNeededCount.contains(visitedBitset) || optimalNeededCount[visitedBitset] > 0) {
		return;
	}
	auto p = optimalPlayer[visitedBitset];
	if (p.oldVisitedBitset.any()) {
		--optimalNeededCount[p.oldVisitedBitset];
		tryRemoveOptimalPlayer(p.oldVisitedBitset);
	}
	optimalPlayer.erase(visitedBitset);
	optimalNeededCount.erase(visitedBitset);
}

int highScore = -1;

Player optimalExitPlayer;

std::list< Player > localOptimalExitPlayer;

void tryUpdateOptimalExitPlayer(const Player& p, bool printHighscore = true) {
	Stat pStat = p.stat;
	pStat.REP -= 1 << 20;
	bool localMax = true;
	for (auto it = localOptimalExitPlayer.begin(); it != localOptimalExitPlayer.end(); ) {
		if (it->stat >= pStat) {
			localMax = false;
			break;
		}
		Stat opStat = it->stat;
		opStat.REP -= 1 << 20;
		if (p.stat >= opStat) {
			it = localOptimalExitPlayer.erase(it);
		} else {
			++it;
		}
	}
	if (localMax) {
		localOptimalExitPlayer.push_back(p);
	}

	int newScore = p.stat.score();

	if (newScore <= highScore) {
		return;
	}

	auto oldVisitedBitset = p.visitedBitset;
	--optimalNeededCount[oldVisitedBitset];
	highScore = newScore;
	optimalExitPlayer = p;
	++optimalNeededCount[p.visitedBitset];

	if (!printHighscore) {
		return;
	}
	std::cout << "New High ";
	std::cout << p;
	std::cout << "--------------------------------------------------------------------------------\n";
	std::cout.flush();
}

void newOptimalSearch() {
	optimalPlayer.clear();
	optimalNeededCount.clear();
	clones = {};
	highScore = -1;
	optimalExitPlayer = {};
}


/* ASSUMPTION: the stat of player only depends on initial stat and visited vertices, but does NOT depend on the order of those vertices */

Player searchOptimalSolutions(Player initPlayer, bool printHighscore = true) {
	newOptimalSearch();

	int goal = initPlayer.map->exit;
	initPlayer.probe();
	optimalPlayer[initPlayer.visitedBitset] = initPlayer;

	clones.push(initPlayer.visitedBitset);

	while (!clones.empty()) {
		Player p = optimalPlayer[clones.front()];
		optimalNeededCount[p.visitedBitset] = 0;
		clones.pop();

		if (!p.visitedBitset[goal]) {
			p.probe();

			for (auto id : p.neighbours) {
				if (p.hasFree && !p.isFree[id]) {
					continue;
				}

				auto estat = EssStat(p.stat);
				if (! (p.stat.HP >= p.map->probeResult[estat][id].req.HP && p.available[id]) ) {
					if (p.isFree[id]) {
						break;
					}
					continue;
				}

				auto newVisitedBitset = p.visitedBitset;
				newVisitedBitset[id] = 1;

				auto newTrace = p.trace;
				newTrace.push_back(id);

				if (optimalPlayer.contains(newVisitedBitset)) {
					int newHP = p.stat.HP + p.map->probeResult[estat][id].diff.HP;
					if (optimalPlayer[newVisitedBitset].stat.HP < newHP) {

						auto oldVisitedBitset = optimalPlayer[newVisitedBitset].oldVisitedBitset;
						--optimalNeededCount[oldVisitedBitset];
						tryRemoveOptimalPlayer(oldVisitedBitset);

						optimalPlayer[newVisitedBitset].stat.HP = newHP;
						optimalPlayer[newVisitedBitset].trace = newTrace;
						optimalPlayer[newVisitedBitset].oldVisitedBitset = p.visitedBitset;
						++optimalNeededCount[p.visitedBitset];
					} else {
						if (p.isFree[id]) {
							break;
						}
						continue;
					}
				} else {
					Player clone = {
						stat: p.stat + p.map->probeResult[estat][id].diff,
						map: p.map,
						trace: newTrace,
						neighbours: p.neighbours,
						inNeighbourCount: p.inNeighbourCount,
						visitedBitset: newVisitedBitset,
						oldVisitedBitset: p.visitedBitset
					};

					if (p.map->levelEntrance[id]) {
						clone.neighbours.clear();
					}

					clone.neighbours.erase(id);
					for (auto nb : clone.map->neighbours[id]) {
						++clone.inNeighbourCount[nb];
						if (!clone.visitedBitset[nb] && clone.inNeighbourCount[nb] >= 0) {
							clone.neighbours.insert(nb);
						}
					}
					optimalPlayer[clone.visitedBitset] = clone;
					++optimalNeededCount[p.visitedBitset];
					clones.push(newVisitedBitset);
				}

				if (id == goal) {
					tryUpdateOptimalExitPlayer(optimalPlayer[newVisitedBitset], printHighscore);
				}
				if (p.isFree[id]) {
					break;
				}
			}
		}

		tryRemoveOptimalPlayer(p.visitedBitset);
	}

	return optimalExitPlayer;
}

Player searchOptimalSolutions(Player initPlayer, std::bitset<MaxRoomsPerMap> targetBitset) {

	int highScore = -1;

	Player optimalTargetPlayer;

	initPlayer.probe();
	optimalPlayer[initPlayer.visitedBitset] = initPlayer;

	clones.push(initPlayer.visitedBitset);

	while (!clones.empty()) {
		Player p = optimalPlayer[clones.front()];
		optimalNeededCount[p.visitedBitset] = 0;
		clones.pop();

		for (auto id : p.neighbours) {
			auto estat = EssStat(p.stat);
			if (! (p.stat.HP >= p.map->probeResult[estat][id].req.HP && p.available[id]) ) {
				continue;
			}

			auto newVisitedBitset = p.visitedBitset;
			newVisitedBitset[id] = 1;

			auto newTrace = p.trace;
			newTrace.push_back(id);

			if (optimalPlayer.contains(newVisitedBitset)) {
				int newHP = p.stat.HP + p.map->probeResult[estat][id].diff.HP;
				if (optimalPlayer[newVisitedBitset].stat.HP < newHP) {
					tryRemoveOptimalPlayer(optimalPlayer[newVisitedBitset].oldVisitedBitset);
					optimalPlayer[newVisitedBitset].stat.HP = newHP;
					optimalPlayer[newVisitedBitset].trace = newTrace;
					optimalPlayer[newVisitedBitset].oldVisitedBitset = p.visitedBitset;
				} else {
					continue;
				}
			} else {
				Player clone = {
					stat: p.stat + p.map->probeResult[estat][id].diff,
					map: p.map,
					trace: newTrace,
					neighbours: p.neighbours,
					inNeighbourCount: p.inNeighbourCount,
					visitedBitset: newVisitedBitset,
					oldVisitedBitset: p.visitedBitset
				};

				clone.neighbours.erase(id);
				for (auto nb : clone.map->neighbours[id]) {
					++clone.inNeighbourCount[nb];
					if (!clone.visitedBitset[nb] && clone.inNeighbourCount[nb] >= 0) {
						clone.neighbours.insert(nb);
					}
				}
				clone.probe();
				optimalPlayer[clone.visitedBitset] = clone;
				++optimalNeededCount[p.visitedBitset];

				if (newVisitedBitset != targetBitset) {
					clones.push(newVisitedBitset);
				}
			}

			if (newVisitedBitset == targetBitset) {
				int newHighScore = optimalPlayer[newVisitedBitset].stat.score();

				if (newHighScore > highScore) {
					highScore = newHighScore;
					optimalTargetPlayer = optimalPlayer[newVisitedBitset];
				}
			}
		}

		tryRemoveOptimalPlayer(p.visitedBitset);
	}

	return optimalTargetPlayer;
}

void printTrace(const Player& initPlayer, const Player& player) {
	Player p(initPlayer);
	std::cout << p;
	std::cout << "--------------------------------------------------------------------------------\n";
	for (auto id : player.trace) {
		p.visit(id);
		std::cout << p;
		std::cout << "--------------------------------------------------------------------------------\n";
	}
	std::cout.flush();
}