#pragma once

#include <vector>
#include <unordered_set>

#include "stat.h"
#include "level.h"

struct EssPlayer {
	Stat stat;
	Bitset neighbours;
	Bitset visited;
	int lastVisit = -1;

	Bitset previousVisited() const {
		Bitset bs = visited;
		bs.reset(lastVisit);
		return bs;
	}

	EssPlayer& visit(const int id, const Level* level, const ProbeStat probe) {
		stat += probe.diff;
		neighbours += level->neighbours[id];
		neighbours -= level->excludeNeighbours[id];
		neighbours -= visited;
		lastVisit = id;
		visited.set(id);
		return *this;
	}

	EssPlayer& enter(const Level* level) {
		neighbours.set(level->entrance);
		return *this;
	}
};

void printEssPlayer(std::ostream& os, const EssPlayer& p, const Level* level, bool printVisited = true) {
	os << p.stat;
	os << "Neighbours: ";
	bool first = true;
	for (auto id : p.neighbours) {
		if (first) {
			first = false;
		} else {
			os << ", ";
		}
		os << level->vertex(id)->name;
	}
	os << "\n";
	if (!printVisited) {
		return;
	}
	os << "Visited: ";
	first = true;
	for (auto id : p.visited) {
		if (first) {
			first = false;
		} else {
			os << ", ";
		}
		os << level->vertex(id)->name;
	}
	os << "\n";
}
