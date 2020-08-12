#pragma once

#include <vector>
#include <unordered_set>

#include "stat.h"
#include "level.h"

struct Player {
	Stat stat;
	const Level* level;
	std::vector<int> trace;
	Bitset neighbours;
	Bitset visited;
	Bitset previousVisited;

	Player& enter(const Level* level) {
		this->level = level;
		neighbours.set(level->entrance);
		return *this;
	}

	Player& visit(const int id) {
		return *this >> id;
	}

	Player& operator >> (const int id) {
		assert(neighbours.test(id));
		auto pr = level->vertex(id)->ToProbeStat(stat);
		assert(stat >= pr.req);
		stat += pr.diff;
		trace.push_back(id);
		neighbours += level->neighbours[id];
		neighbours -= level->excludeNeighbours[id];
		neighbours -= visited;
		previousVisited = visited;
		visited.set(id);
		return *this;
	}

	template<class T>
	Player& operator >> (const T t) {
		return *this >> level->id(t);
	}
};

std::ostream& operator << (std::ostream& os, const Player& p) {
	os << p.stat;
	os << "Neighbours: ";
	bool first = true;
	for (auto id : p.neighbours) {
		if (first) {
			first = false;
		} else {
			os << ", ";
		}
		os << p.level->vertex(id)->name;
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
		os << p.level->vertex(id)->name;
	}
	os << "\n";
	return os;
}


void printTrace(const Stat& initStat, const Player& player) {
	Player p{initStat};
	p.enter(player.level);
	std::cout << p;
	std::cout << "--------------------------------------------------------------------------------\n";
	for (auto id : player.trace) {
		p >> id;
		std::cout << p;
		std::cout << "--------------------------------------------------------------------------------\n";
	}
	std::cout.flush();
}