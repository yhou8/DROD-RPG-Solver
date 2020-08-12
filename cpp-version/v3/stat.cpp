enum PlayerFlagEnum : PlayerFlagType {
	DDead							= 1 << 0,
	DHasWeapon						= 1 << 1,
	DDoubleGRWeapon					= 0,
	DDoubleATKAgainstGoblin			= 0,
	DDoubleATKAgainstWyrm			= 0,
	PlayerFlagWeaponMask			= DHasWeapon | DDoubleGRWeapon | DDoubleATKAgainstGoblin | DDoubleATKAgainstWyrm,
	PlayerFlagShieldMask			= 0,
	PlayerFlagAccesoryMask			= 0,
};

// --------------------------------------------------------------------------------

struct PlayerStat {
	STRUCT_MEMBER_DECLARATION(PlayerStat)

	CONVERT_STRUCT_DECLARATION(PlayerObjective)
	CONVERT_STRUCT_DECLARATION(PlayerCombat)

	inline bool operator >=(const PlayerStat&) const;
	inline bool nonnegative() const;
	inline PlayerStat& operator +=(const PlayerStat&);
	inline PlayerStat& join(const PlayerStat&);
	inline PlayerStat operator -(const PlayerStat&) const;
	inline PlayerStat operator -() const;
};

struct LevelStat {
	STRUCT_MEMBER_DECLARATION(LevelStat)
};

struct PlayerCombat {
	STRUCT_MEMBER_DECLARATION(PlayerCombat)

	CONVERT_STRUCT_DECLARATION(PlayerStat)

	inline PlayerCombat& operator +=(const PlayerStat&);
	inline bool operator ==(const PlayerCombat&) const = default;
};

HASH_FUNCTION(PlayerCombat)

struct PlayerObjective {
	STRUCT_MEMBER_DECLARATION(PlayerObjective)
	inline PlayerObjective operator +(const PlayerObjective&);
	inline bool operator >=(const PlayerObjective&) const;
	inline bool operator ==(const PlayerObjective&) const = default;
};

struct PlayerScore {
	STRUCT_MEMBER_DECLARATION(PlayerScore)

	inline bool operator >=(const PlayerScore&) const;
};

CONVERT_STRUCT(PlayerStat, PlayerObjective)
CONVERT_STRUCT(PlayerStat, PlayerCombat)
CONVERT_STRUCT(PlayerCombat, PlayerStat)

// --------------------------------------------------------------------------------

struct ProbeStat {
	PlayerStat diff;
	PlayerStat req;

	inline ProbeStat& operator +=(const ProbeStat&);
};

// --------------------------------------------------------------------------------

namespace IntFlagOperator {
	template <typename T>
	inline T& assign(T& x, const T y) {
		return x = y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T&> selfAdd(T& x, const T y) {
		return x += y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T&> selfAdd(T& x, const T y) {
		return x ^= y;
	}

	inline Bitset& selfAdd(Bitset& x, const Bitset y) {
		return x ^= y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T> add(const T x, const T y) {
		return x + y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T> add(const T x, const T y) {
		return x ^ y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T&> selfSub(T& x, const T y) {
		return x -= y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T&> selfSub(T& x, const T y) {
		return x ^= y;
	}

	inline Bitset& selfSub(Bitset& x, const Bitset y) {
		return x ^= y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T> sub(const T x, const T y) {
		return x - y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T> sub(const T x, const T y) {
		return x ^ y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T> neg(const T x) {
		return -x;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T> neg(const T x) {
		return x;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T> nonnegative(const T x) {
		return x >= 0;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T> nonnegative(const T x) {
		return true;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T&> join(T& x, const T y) {
		return (x < y) ? (x = y) : x;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T&> join(T& x, const T y) {
		return x |= y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, bool> geq(const T x, const T y) {
		return x >= y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, bool> geq(const T x, const T y) {
		return (x & y) == y;
	}

	template <typename T>
	inline std::enable_if_t<std::is_signed<T>::value, T> percentFloor(const T x, const T y) {
		int prod = x * y;
		return (prod >= 0) ? (prod / 100) : (prod - 99) / 100;
	}

	template <typename T>
	inline std::enable_if_t<std::is_unsigned<T>::value, T> percentFloor(const T x, const T y) {
		return 0;
	}
}

// --------------------------------------------------------------------------------

bool PlayerStat::operator >=(const PlayerStat& p) const {
	return STRUCT_BINARY_OPERATOR_CODE(PlayerStat, geq, &&) true;
}

bool PlayerStat::nonnegative() const {
	return STRUCT_UNARY_OPERATOR_CODE(PlayerStat, nonnegative, &&) true;
}

PlayerStat& PlayerStat::operator +=(const PlayerStat& p) {
	STRUCT_BINARY_OPERATOR_CODE(PlayerStat, selfAdd, ;) return *this;
}

PlayerStat& PlayerStat::join(const PlayerStat& p) {
	STRUCT_BINARY_OPERATOR_CODE(PlayerStat, join, ;) return *this;
}

PlayerStat PlayerStat::operator -(const PlayerStat& p) const {
	return PlayerStat{ STRUCT_BINARY_OPERATOR_INIT_CODE(PlayerStat, sub) };
}

PlayerStat PlayerStat::operator -() const {
	return PlayerStat{ STRUCT_UNARY_OPERATOR_INIT_CODE(PlayerStat, neg) };
}

// --------------------------------------------------------------------------------

PlayerCombat& PlayerCombat::operator +=(const PlayerStat& p) {
	STRUCT_ACTION_CODE(PlayerCombat, PlayerStat, selfAdd, ;) return *this;
}

PlayerObjective PlayerObjective::operator +(const PlayerObjective& p) {
	return PlayerObjective{ STRUCT_BINARY_OPERATOR_INIT_CODE(PlayerObjective, add) };
}

bool PlayerObjective::operator >=(const PlayerObjective& p) const {
	return STRUCT_BINARY_OPERATOR_CODE(PlayerObjective, geq, &&) true;
}

// --------------------------------------------------------------------------------

bool PlayerScore::operator >=(const PlayerScore& p) const {
	return STRUCT_BINARY_OPERATOR_CODE(PlayerScore, geq, &&) true;
}

std::ostream& operator << (std::ostream&os, PlayerScore s) {
	os << s.score / 1000 << ".";
	if (s.score % 1000 < 100) {
		os << "0";
	}
	if (s.score % 1000 < 10) {
		os << "0";
	}
	os << s.score % 1000;
	return os;
}

// --------------------------------------------------------------------------------

ProbeStat& ProbeStat::operator +=(const ProbeStat& s) {
	req.join(s.req - diff);
	diff += s.diff;
	return *this;
}

std::ostream& operator << (std::ostream& os, const PlayerFlagEnum Flag) {
	if (Flag & DDead) { os << " DDead"; }
	if (Flag & DHasWeapon) { os << " DHasWeapon"; }
	if (Flag & DDoubleGRWeapon) { os << " DDoubleGRWeapon"; }
	if (Flag & DDoubleATKAgainstGoblin) { os << " DDoubleATKAgainstGoblin"; }
	if (Flag & DDoubleATKAgainstWyrm) { os << " DDoubleATKAgainstWyrm"; }
	return os;
}

std::ostream& operator << (std::ostream& os, PlayerCombat& player) {
	return os << "Combat"
	#define GEN_CODE_HELPER(R, DATA, ELEM) \
		<< ", " << BOOST_PP_STRINGIZE(M_IDENTIFIER(ELEM)) << ": " << \
		BOOST_PP_IF( \
				BOOST_PP_OR(MEMBER_EQUAL(M_IDENTIFIER(ELEM), Flag), MEMBER_EQUAL(M_IDENTIFIER(ELEM), EquipFlag)), \
				(PlayerFlagEnum), \
				(int) \
		) player.M_IDENTIFIER(ELEM)
	BOOST_PP_SEQ_FOR_EACH(GEN_CODE_HELPER, _, FILTER_MEMBER_BY_IGNORE(FILTER_MEMBER_BY_STRUCT(PlayerCombat, MEMBER_LIST))) << "\n";
	#undef GEN_CODE_HELPER
}
