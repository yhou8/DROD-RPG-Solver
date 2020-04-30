#pragma once

#include "stat.h"
#include <cassert>

StatDiff::operator bool() const {
	return Flag != 0
		|| HP != 0
		|| ATK != 0
		|| DEF != 0
		|| GR != 0
		|| YK != 0
		|| GK != 0
		|| BK != 0;
}

StatDiff::operator Stat() const {
	return {
		Flag : Flag,
		HP : HP,
		ATK : ATK,
		DEF : DEF,
		GR : GR,
		YK : YK,
		GK : GK,
		BK : BK
	};
}

bool operator >= (const StatDiff& d0, const StatDiff& d1) {
	return (d0.Flag & d1.Flag) == d1.Flag
		&& d0.HP >= d1.HP
		&& d0.ATK >= d1.ATK
		&& d0.DEF >= d1.DEF
		&& d0.GR >= d1.GR
		&& d0.YK >= d1.YK
		&& d0.GK >= d1.GK
		&& d0.BK >= d1.BK;
}

StatDiff& operator += (StatDiff& d0, const StatDiff& d1) {
	d0.Flag |= d1.Flag;
	d0.HP += d1.HP;
	d0.ATK += d1.ATK;
	d0.DEF += d1.DEF;
	d0.GR += d1.GR;
	d0.YK += d1.YK;
	d0.GK += d1.GK;
	d0.BK += d1.BK;
	return d0;
}

StatDiff operator + (const StatDiff& d0, const StatDiff& d1) {
	return StatDiff{
		Flag : d0.Flag | d1.Flag,
		HP : d0.HP + d1.HP,
		ATK : d0.ATK + d1.ATK,
		DEF : d0.DEF + d1.DEF,
		GR : d0.GR + d1.GR,
		YK : d0.YK + d1.YK,
		GK : d0.GK + d1.GK,
		BK : d0.BK + d1.BK
	};
}

StatDiff operator - (const StatDiff& d) {
	assert(!d.Flag);
	return StatDiff{
		Flag : d.Flag,
		HP : - d.HP,
		ATK : - d.ATK,
		DEF : - d.DEF,
		GR : - d.GR,
		YK : - d.YK,
		GK : - d.GK,
		BK : - d.BK
	};
}

// --------------------------------------------------------------------------------

bool Stat::valid() {
	return HP >= 0
		&& ATK >= 0
		&& DEF >= 0
		&& GR >= 0
		&& YK >= 0
		&& GK >= 0
		&& BK >= 0;
}

int Stat::score() const {
	#ifdef TOTS
		return (HP + 1) * 25 + (ATK * 5 + YK * 10 + GK * 20 + BK * 30) * 1000 + DEF * 1000 * 10 / 3;
	#else
		return (HP + 1) * 25 + (ATK * 5 + DEF * 3 + YK * 10 + GK * 20 + BK * 30) * 1000;
	#endif
}

Stat& Stat::operator += (const StatDiff& d) {
	Flag |= d.Flag;
	HP += d.HP;
	ATK += d.ATK;
	DEF += d.DEF;
	GR += d.GR;
	YK += d.YK;
	GK += d.GK;
	BK += d.BK;
	return *this;
}

Stat& Stat::join(const Stat& s) {
	Flag |= s.Flag;
	if (HP < s.HP) { HP = s.HP; };
	if (ATK < s.ATK) { ATK = s.ATK; };
	if (DEF < s.DEF) { DEF = s.DEF; };
	if (GR < s.GR) { GR = s.GR; };
	if (YK < s.YK) { YK = s.YK; };
	if (GK < s.GK) { GK = s.GK; };
	if (BK < s.BK) { BK = s.BK; };
	return *this;
}

Stat Stat::operator - (const StatDiff& d) const {
	return Stat{
		Flag : Flag & ~d.Flag,
		HP : HP - d.HP,
		ATK : ATK - d.ATK,
		DEF : DEF - d.DEF,
		GR : GR - d.GR,
		YK : YK - d.YK,
		GK : GK - d.GK,
		BK : BK - d.BK
	};
}

std::ostream& printScore(std::ostream& os, const Stat& stat) {
	int score = stat.score();
	os << "Score: " << score / 1000 << ".";
	if (score % 1000 < 100) {
		os << "0";
	}
	if (score % 1000 < 10) {
		os << "0";
	}
	return os << score % 1000;
}

std::ostream& printPlayerFlag(std::ostream& os, const FLAG Flag) {
	os << "Flag:";
	if (Flag & DDoubleGR) {
		os << " DoubleGR";
	}
	if (Flag & DDoubleATKAgainstGoblin) {
		os << " DoubleATKAgainstGoblin";
	}
	if (Flag & DDoubleATKAgainstWyrm) {
		os << " DoubleATKAgainstWyrm";
	}
	return os;
}

std::ostream& operator << (std::ostream& os, const Stat& stat) {
	printScore(os, stat) << "\n{ ";
	return printPlayerFlag(os, stat.Flag)
		<< ", HP: " << stat.HP + 1
		<< ", ATK: " << stat.ATK
		<< ", DEF: " << stat.DEF
		<< ", GR: " << stat.GR
		<< ", YK: " << stat.YK
		<< ", GK: " << stat.GK
		<< ", BK: " << stat.BK
		<< "}\n";
}

bool operator >= (const Stat& s0, const Stat& s1) {
	return (s0.Flag & s1.Flag) == s1.Flag
		&& s0.HP >= s1.HP
		&& s0.ATK >= s1.ATK
		&& s0.DEF >= s1.DEF
		&& s0.GR >= s1.GR
		&& s0.YK >= s1.YK
		&& s0.GK >= s1.GK
		&& s0.BK >= s1.BK;
}

// --------------------------------------------------------------------------------

EssStat::EssStat(Stat s) {
	Flag = s.Flag;
	ATK = s.ATK;
	DEF = s.DEF;
}

bool operator == (const EssStat s0, const EssStat s1) {
	return s0.Flag == s1.Flag && s0.ATK == s1.ATK && s0.DEF == s1.DEF;
}

EssStat::operator Stat() const {
	return {
		Flag : Flag,
		ATK : ATK,
		DEF : DEF
	};
}

// --------------------------------------------------------------------------------

ProbeStat& operator += (ProbeStat& s0, const ProbeStat& s1) {
	s0.req.join(s1.req - s0.diff);
	s0.diff += s1.diff;
	s0.loss += s1.loss;
	return s0;
}
