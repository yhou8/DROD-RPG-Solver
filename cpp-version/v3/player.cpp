struct PlayerProgressDiff {
	STRUCT_MEMBER_DECLARATION(PlayerProgressDiff)

	inline PlayerProgressDiff& operator =(const PlayerStat&);
};

struct Player {
	STRUCT_MEMBER_DECLARATION(Player)

	CONVERT_STRUCT_DECLARATION(PlayerScore)
	CONVERT_STRUCT_DECLARATION(PlayerProgress)
	CONVERT_STRUCT_DECLARATION(PlayerObjective)
	CONVERT_STRUCT_DECLARATION(PlayerStat)
	CONVERT_STRUCT_DECLARATION(PlayerCombat)

	PlayerProgressDiff diff;

	inline bool operator >=(const PlayerStat &) const;
	inline Player& operator += (const PlayerStat &);
	inline Player& operator = (const PlayerObjective &);

	inline PlayerProgress revertedProgress() const;
	inline Player& enter(Level*);
	inline Player& visit(const VertexIDType, Level*, const ProbeStat&);

	std::ostream& print(std::ostream&, Level*) const;
};

struct PlayerProgress {
	STRUCT_MEMBER_DECLARATION(PlayerProgress)

	static Level* level;

	inline bool operator ==(const PlayerProgress&) const = default;
	inline PlayerProgress& operator -=(const PlayerProgressDiff&);
};

Level* PlayerProgress::level = 0;

HASH_FUNCTION(PlayerProgress)

struct PlayerTrace {
	int levelConfig;
	Level* level;
	Player player;
	std::vector<VertexIDType> trace;

	CONVERT_STRUCT_DECLARATION(PlayerScore)
	CONVERT_STRUCT_DECLARATION(PlayerStat)

	inline PlayerTrace& visit(const VertexIDType);
	std::ostream& print(std::ostream&, const Player&) const;
};

CONVERT_STRUCT(Player, PlayerProgress)
CONVERT_STRUCT(Player, PlayerObjective)
CONVERT_STRUCT(Player, PlayerStat)
CONVERT_STRUCT(Player, PlayerCombat)

// --------------------------------------------------------------------------------

Player::operator PlayerScore() const {
	return PlayerScore{
		MEMBER_INIT(PlayerScore, score, ScoreFunction)
	};
}

bool Player::operator >=(const PlayerStat& p) const {
	return STRUCT_ACTION_CODE(Player, PlayerStat, geq, &&) true;
}

Player& Player::operator +=(const PlayerStat& p) {
	STRUCT_ACTION_CODE(Player, PlayerStat, selfAdd, ;) return *this;
}

Player& Player::operator =(const PlayerObjective& p) {
	STRUCT_ACTION_CODE(Player, PlayerObjective, assign, ;) return *this;
}

Player& Player::enter(Level* level) {
	neighbours.set(level->entrance);
	PlayerProgress::level = level;
	return *this;
}

Player& Player::visit(const VertexIDType location, Level* level, const ProbeStat& probe) {
	#ifdef ClosedLevel
		*this += probe.diff;
		diff = probe.diff;
		Bitset oldMemory = memory;
		Bitset oldVisited = visited;
		disabled ^= level->toggleNeighbours[location];
		if ((level->boundaryMask & ~disabled).test(location) || (level->boundaryMask & ~disabled).test(diff.location) || (level->vertex(location)->type & DClearNeighboursRoom)) {
			memory.join(visited);
			memory.substract((level->boundaryMask & ~disabled));
			neighbours = {};
			visited = {};
		}
		Bitset explore;
		explore.set(location);
		while (!explore.empty()) {
			VertexIDType v = explore.ffs() - 1;
			visited.set(v);
			neighbours.join(level->neighbours[v] &~disabled);
			Bitset memoryVisited = memory & neighbours;
			memory.substract(memoryVisited);
			explore.join(memoryVisited);
			explore.substract(visited);
		}
		neighbours.substract(visited);
		neighbours.substract(disabled);
		diff.location = location;
		diff.memory = oldMemory ^ memory;
		diff.visited = oldVisited ^ visited;
		return *this;
	#else
		*this += probe.diff;
		diff = probe.diff;
		visited.set(location);
		neighbours.join(level->neighbours[location]);
		neighbours.substract(level->toggleNeighbours[location]);
		neighbours.substract(visited);
		MEMBER_CODE(PlayerProgressDiff, location, diff.location = location;)
		return *this;
	#endif
}

// --------------------------------------------------------------------------------

PlayerProgressDiff& PlayerProgressDiff::operator =(const PlayerStat& p) {
	STRUCT_ACTION_CODE(PlayerProgressDiff, PlayerStat, assign, ;) return *this;
}

// --------------------------------------------------------------------------------

PlayerProgress& PlayerProgress::operator -=(const PlayerProgressDiff& p) {
	#ifndef ClosedLevel
		visited.reset(p.location);
	#endif
	STRUCT_ACTION_CODE(PlayerProgress, PlayerProgressDiff, selfSub, ;)
	return *this;
}

PlayerProgress Player::revertedProgress() const {
	auto progress = (PlayerProgress) *this;
	progress -= diff;
	return progress;
}

// --------------------------------------------------------------------------------

PlayerTrace::operator PlayerStat() const {
	return (PlayerStat) player;
}

PlayerTrace::operator PlayerScore() const {
	return (PlayerScore) player;
}

template<typename T>
std::ostream& printRoomList(std::ostream& os, Level* level, T list) {
	bool first = true;
	for (auto room : list) {
		if (first) {
			first = false;
		} else {
			os << ", ";
		}
		os << level->vertex(room)->name;
	}
	return os << "\n";
}


// --------------------------------------------------------------------------------

std::ostream& Player::print(std::ostream& os, Level* level) const {
	os << "Score: " << (PlayerScore) *this << "\n{"
	#define GEN_CODE_HELPER(R, DATA, ELEM) \
		BOOST_PP_EXPR_IF(BOOST_PP_NOT(MEMBER_EQUAL(M_IDENTIFIER(ELEM), HP)), << ", ") << BOOST_PP_STRINGIZE(M_IDENTIFIER(ELEM)) << ": " << \
		BOOST_PP_IF( \
				BOOST_PP_OR(MEMBER_EQUAL(M_IDENTIFIER(ELEM), Flag), MEMBER_EQUAL(M_IDENTIFIER(ELEM), EquipFlag)), \
				(PlayerFlagEnum), \
				(int) \
		) M_IDENTIFIER(ELEM) \
		BOOST_PP_EXPR_IF(MEMBER_EQUAL(M_IDENTIFIER(ELEM), HP), + 1)
	BOOST_PP_SEQ_FOR_EACH(GEN_CODE_HELPER, _, FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(PlayerStat, MEMBER_LIST))) << "}\n";
	#undef GEN_CODE_HELPER
	os << "Neighbours: ";
	printRoomList(os, level, neighbours);
	#ifdef ClosedLevel
		os << "Visited: ";
		printRoomList(os, level, visited);
		os << "Memory: ";
		printRoomList(os, level, memory);
	#endif
	return os;
}

std::ostream& operator <<(std::ostream& os, PlayerTrace p) {
	p.player.print(os, p.level);
	os << "Trace: ";
	printRoomList(os, p.level, p.trace);
	return os;
}

PlayerTrace& PlayerTrace::visit(const VertexIDType room) {
	player.visit(room, level, level->vertex(room)->probe((PlayerCombat) player));
	trace.push_back(room);
	return *this;
}

std::ostream& PlayerTrace::print(std::ostream& os, const Player& initPlayer) const {
	PlayerTrace player = {
		levelConfig: levelConfig,
		level: level,
		player: initPlayer,
	};
	os << player;
	os << "--------------------------------------------------------------------------------\n";
	for (auto room : trace) {
		player.visit((VertexIDType) room);
		os << player;
		os << "--------------------------------------------------------------------------------\n";
	}
	return os;
}

// --------------------------------------------------------------------------------
