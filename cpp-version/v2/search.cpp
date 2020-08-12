#pragma once

#include "search.h"
#include "timer.h"

Search::Search(const Level* level, Stat initStat)
		: level(level)
		, initStat(initStat) {
	initPlayer = {initStat};
	initPlayer.enter(level);
	prefixPlayer = initPlayer;
	optimalPlayer[{}] = initPlayer;
}


template<class T>
Search& Search::operator >> (const T t) {
	int id = level->id(t);
	prefix.push_back(id);
	prefixBitset.set(id);
	expand(prefixPlayer, id, probe(prefixPlayer)[id], false);
	prefixPlayer.visit(id, level, probe(prefixPlayer)[id]);
	return *this;
}

template<class T>
Search& Search::operator << (const T t) {
	int id = level->id(t);
	suffix.push_back(id);
	suffixBitset.set(id);
	return *this;
}

const std::vector<ProbeStat>& Search::probe(const EssStat s) {
	if (probeResult.contains(s)) {
		return probeResult.at(s);
	}
	std::vector<ProbeStat> res;
	for (int i = 0; i < level->N; ++i) {
		res.push_back(level->vertex(i)->ToProbeStat(s));
	}
	probeResult[s] = res;
	return probeResult[s];
}

const std::vector<ProbeStat>& Search::probe(const EssPlayer& player) {
	return probe(EssStat(player.stat));
}

struct ExtendedProbeStat {
	int id;
	ProbeStat probe;
	bool loss;
	bool free;
	bool priorty;
};

void Search::tryRemoveOptimalPlayer(Bitset bs) {
	while (!(bs == prefixPlayer.visited) && --optimalNeededCount[bs] <= 0) {
		int lastVisit = optimalPlayer[bs].lastVisit;
		optimalPlayer.erase(bs);
		optimalNeededCount.erase(bs);
		if (lastVisit == -1) {
			return;
		}
		bs.reset(lastVisit);
	}
}

void Search::expand(EssPlayer p, int id, const ProbeStat& probe, bool push) {
	if (id == level->exit) {
		auto bs = p.visited;
		p.visit(id, level, probe);

		Stat pStat = p.stat;
		bool localMax = true;
		for (auto it = localOptimalExitPlayer.begin(); localMax && it != localOptimalExitPlayer.end(); ) {
			if (it->stat >= pStat) {
				localMax = false;
			} else {
				Stat opStat = it->stat;
				if (pStat >= opStat) {
					it = localOptimalExitPlayer.erase(it);
				} else {
					++it;
				}
			}
		}
		if (!localMax) {
			return;
		}

		Player player = ToPlayer(p);

		localOptimalExitPlayer.push_back(player);

		int newScore = p.stat.score();

		if (newScore <= optimalExitPlayerScore) {
			return;
		}

		tryRemoveOptimalPlayer(optimalExitPlayer.previousVisited);
		optimalExitPlayer = player;
		optimalExitPlayerScore = newScore;
		++optimalNeededCount[bs];
		if (printHighscore) {
			std::cout << "New High " << player;
			std::cout << "--------------------------------------------------------------------------------" << std::endl;
		}
	} else {
		auto bs = p.visited;
		auto new_bs = bs;
		new_bs.set(id);
		if (optimalPlayer.contains(new_bs)) {
			int new_HP = p.stat.HP + probe.diff.HP;
			if (new_HP <= optimalPlayer[new_bs].stat.HP) {
				return;
			}
			tryRemoveOptimalPlayer(optimalPlayer[new_bs].previousVisited());
			optimalPlayer[new_bs].stat.HP = new_HP;
			optimalPlayer[new_bs].lastVisit = id;
			++optimalNeededCount[bs];
		} else {
			p.visit(id, level, probe);
			optimalPlayer[new_bs] = p;
			++optimalNeededCount[bs];
			if (push) {
				clones.push(new_bs);
				++totalSearchCount;
			}
		}
	}
}

void Search::search() {
	optimalPlayer[prefixPlayer.visited] = prefixPlayer;
	clones.push(prefixPlayer.visited);
	++totalSearchCount;

	Timer timer;

	while (!clones.empty()) {
		const auto bs = clones.front();
		const EssPlayer p = optimalPlayer[bs];
		clones.pop();
		optimalNeededCount[bs] = 0;

		++currentSearchCount;
		if (currentSearchCount % 1000000 == 0) {
			if (timer.elapsed() > 10000) {
				timer.reset();
				std::cout << "Progress: " << currentSearchCount / 1000000 << "m / " << totalSearchCount / 1000000 << "m" << std::endl;
			}
		}

		auto probeResult = probe(p);

		std::vector<ExtendedProbeStat> extendedProbeResult;
		extendedProbeResult.reserve(p.neighbours.count());

		bool wasIntermediate = p.lastVisit == -1 ? false : (level->vertex(p.lastVisit)->type & DIntermediate);

		bool hasPriority = false;
		bool hasFree = false;
		for (auto nb : p.neighbours) {
			if (wasIntermediate && !level->neighbours[p.lastVisit].test(nb)) {
				continue;
			}
			auto pr = probeResult[nb];
			bool available = p.stat >= pr.req;
			if (!available) {
				continue;
			}
			bool priorty = level->vertex(nb)->type & DPriorityRoom;
			bool intermediate = level->vertex(nb)->type & DIntermediate;
			bool free = (nb != level->exit) && !intermediate && pr.loss == 0 && pr.diff >= StatDiff{};
			if (!free && (level->vertex(nb)->type & DOnlyWhenFree)) {
				continue;
			}
			hasFree |= free;
			hasPriority |= priorty;
			extendedProbeResult.push_back({
				id: nb,
				probe: pr,
				loss: pr.loss > 0,
				free: free,
				priorty: priorty
			});
		}
		if (hasPriority) {
			for (auto& epr : extendedProbeResult) {
				if (epr.priorty) {
					expand(p, epr.id, epr.probe);
					break;
				}
			}
		} else if (hasFree) {
			for (auto& epr : extendedProbeResult) {
				if (epr.free) {
					expand(p, epr.id, epr.probe);
					break;
				}
			}
		} else {
			for (auto& epr : extendedProbeResult) {
				expand(p, epr.id, epr.probe);
			}
		}
		if (!optimalNeededCount[bs]) {
			tryRemoveOptimalPlayer(bs);
		}
	}
}

Player Search::ToPlayer(EssPlayer p) const {
	auto player = new Player{initPlayer.stat};
	player->enter(level);
	for (auto id : prefix) {
		player->visit(id);
	}
	std::vector<int> trace;
	while (! (p.visited == prefixPlayer.visited) ) {
		trace.push_back(p.lastVisit);
		auto bs = p.previousVisited();
		p = optimalPlayer.at(bs);
	}
	while (!trace.empty()) {
		player->visit(trace.back());
		trace.pop_back();
	}
	return *player;
}
