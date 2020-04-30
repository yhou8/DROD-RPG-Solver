#pragma once

#include <functional>
#include <iostream>
#include <string>

typedef unsigned int FLAG;

enum PlayerFlag : FLAG {
//	DHasWeapon = 1 << 0,
	DEnd							= 1 << 1,
	DDoubleGR						= 1 << 2,
	DDoubleATKAgainstGoblin			= 1 << 3,
	DDoubleATKAgainstWyrm			= 1 << 4,
};

// --------------------------------------------------------------------------------

struct Stat;

struct StatDiff {
	FLAG Flag = 0;
	int HP	= 0;
	int ATK	= 0;
	int DEF	= 0;
	int GR	= 0;
	int YK	= 0;
	int GK	= 0;
	int BK	= 0;

	explicit operator bool() const;

	explicit operator Stat() const;
};

StatDiff& operator += (StatDiff& d0, const StatDiff& d1);

StatDiff operator + (const StatDiff& d0, const StatDiff& d1);

StatDiff operator - (const StatDiff& d);

bool operator >= (const StatDiff& d0, const StatDiff& d1);

// --------------------------------------------------------------------------------

/* Warning:
	HP is shifted by 1 so that 0 is considered alive.
	This change makes code cleaner.
*/

struct Stat {
	FLAG Flag = 0;
	int HP	= 0;
	int ATK	= 0;
	int DEF	= 0;
	int GR	= 0;
	int YK	= 0;
	int GK	= 0;
	int BK	= 0;

	bool valid();

	int score() const;

	Stat& operator += (const StatDiff& d);

	Stat& join(const Stat& s);

	Stat operator - (const StatDiff& d) const;
};

std::ostream& operator << (std::ostream& os, const Stat& stat);

bool operator >= (const Stat& s0, const Stat& s1);

// --------------------------------------------------------------------------------

struct EssStat {
	FLAG Flag = 0;
	int ATK = 0;
	int DEF = 0;

	EssStat(Stat s);

	explicit operator Stat() const;

};

bool operator == (const EssStat s0, const EssStat s1);

namespace std {
	template<> struct hash<EssStat> {
		std::size_t operator()(EssStat const& s) const noexcept {
			return s.Flag ^ (s.ATK << 10) ^ (s.DEF << 20);
		}
	};
}

// --------------------------------------------------------------------------------

enum ProbeStatType : FLAG {
	DBad = 1 << 0,
	DNoPotentialLoss = 1 << 1,
	DFree = 1 << 2,
	DPriority = 1 << 3,
};

struct ProbeStat {
	StatDiff diff;
	Stat req;
	int loss = 0;
};

ProbeStat& operator += (ProbeStat& s0, const ProbeStat& s1);
