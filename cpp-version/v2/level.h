#pragma once

#include "bitset.h"
#include "digraph.h"
#include "room.h"

class Level : public Digraph<Room*> {
public:
	int entrance = -1;
	int exit = -1;

	Level& setEntrance() {
		entrance = currentVertexID;
		return *this;
	}

	template <class T>
	Level& setEntrance(T t) {
		entrance = id(t);
		return *this;
	}

	Level& setExit() {
		exit = currentVertexID;
		return *this;
	}

	template <class T>
	Level& setExit(T t) {
		exit = id(t);
		return *this;
	}

	ProbeStat ToProbeStat(const Stat& player) const {
		return vertex()->ToProbeStat(player);
	}
};

struct LevelPtr {
	Level* operator()() const {
		return ptr;
	}
	operator Level* () const {
		return ptr;
	}
	Level* ptr;
};

template <class T>
auto& operator << (LevelPtr level, T t) {
	return level()->select(t);
}
