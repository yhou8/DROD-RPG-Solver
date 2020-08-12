/* Warning:
	Player's HP is shifted by 1 so that 0 HP is considered alive.
	This change makes code cleaner.
*/

enum Attribute {
	// Player Attribute
	DHasWeapon = 1 << 0,
	DDoubleATKAgainstGoblin = 1 << 1,
	DWeakenZombie = 1 << 14,
	DDoubleATKAgainstWyrm = 1 << 2,
	DDoubleGRWeapon = 1 << 9,
	DDoubleGRAccessory = 1 << 10,
	DDoubleREPAccessory = 1 << 11,
	// Monster Attribute
	DBrained = 1 << 3,
	DIgnoreDEF = 1 << 4,
	DAttackFirst = 1 << 5,
	DAttackLate = 1 << 6,
	DIsGoblin = 1 << 7,
	DIsWyrm = 1 << 8,
	DIsZombie = 1 << 13,
	// Other Attribute
	DEnd = 1 << 12
};

unsigned int DWeaponAttr = DHasWeapon | DDoubleATKAgainstGoblin | DDoubleATKAgainstWyrm | DDoubleGRWeapon;
unsigned int DAccessoryAttr = DDoubleGRAccessory | DDoubleREPAccessory | DWeakenZombie | DEnd;

struct Stat {
	unsigned int flag = 0;
	int HP	= 0;
	int ATK	= 0;
	int DEF	= 0;
	int GR	= 0;
	int REP	= 0;
	int YK	= 0;
	int GK	= 0;
	int BK	= 0;
	int SK = 0;

	int equipATK = 0;
	int equipDEF = 0;

	bool valid() const;
	int score() const;
};

struct EssStat {
	unsigned int flag = 0;
	int ATK = 0;
	int DEF = 0;
	int equipATK = 0;
	int equipDEF = 0;

	EssStat(Stat s) {
		flag = s.flag;
		ATK = s.ATK;
		DEF = s.DEF;
		equipATK = s.equipATK;
		equipDEF = s.equipDEF;
	}

	bool operator < (const EssStat r) const {
		if (flag < r.flag) {
			return true;
		} else if (flag > r.flag) {
			return false;
		}
		if (ATK < r.ATK) {
			return true;
		} else if (ATK > r.ATK) {
			return false;
		}
		if (DEF < r.DEF) {
			return true;
		} else if (DEF > r.DEF) {
			return false;
		}
		if (equipATK < r.equipATK) {
			return true;
		} else if (equipATK > r.equipATK) {
			return false;
		}
		if (equipDEF < r.equipDEF) {
			return true;
		} else if (equipDEF > r.equipDEF) {
			return false;
		}
		return false;
	}
};

#ifdef TOTS
int Stat::score() const {
		return (HP + 1) * 25 + (ATK * 5 + YK * 10 + GK * 20 + BK * 30 + SK * 30) * 1000 + DEF * 10000 / 3;
}
#else
int Stat::score() const {
		return (HP + 1) * 25 + (ATK * 5 + DEF * 3 + YK * 10 + GK * 20 + BK * 30 + SK * 30) * 1000;
}
#endif

typedef Stat StatDiff;

std::string flag2String(unsigned int flag) {
	std::string res = "";
	if (flag & DHasWeapon) {
		res += " HasWeapon";
	}
	if (flag & DDoubleATKAgainstGoblin) {
		res += " DoubleATKAgainstGoblin";
	}
	if (flag & DDoubleATKAgainstWyrm) {
		res += " DoubleATKAgainstWyrm";
	}
	if (flag & DDoubleGRWeapon) {
		res += " DoubleGRWeapon";
	}
	if (flag & DDoubleGRAccessory) {
		res += " DoubleGRAccessory";
	}
	if (flag & DDoubleREPAccessory) {
		res += " DoubleREPAccessory";
	}
	if (flag & DWeakenZombie) {
		res += " WeakenZombie";
	}
	return res;
}

int innerProduct(const Stat& s0, const Stat& s1) {
	return
		s0.HP * s1.HP +
		s0.ATK * s1.ATK +
		s0.DEF * s1.DEF +
		s0.GR * s1.GR +
		s0.REP * s1.REP +
		s0.YK * s1.YK +
		s0.GK * s1.GK +
		s0.BK * s1.BK +
		s0.SK * s1.SK +
		s0.equipATK * s1.equipATK +
		s0.equipDEF * s1.equipDEF;
}

std::ostream& operator << (std::ostream& os, const Stat& stat) {
	return os
		<< "{ flag:" << flag2String(stat.flag)
		<< ", HP: " << stat.HP + 1
		<< ", ATK: " << stat.ATK
		<< ", DEF: " << stat.DEF
		<< ", GR: " << stat.GR
		<< ", REP: " << stat.REP
		<< ", YK: " << stat.YK
		<< ", GK: " << stat.GK
		<< ", BK: " << stat.BK
		<< ", SK: " << stat.SK
		<< "}";
}


Stat& operator += (Stat& s, const StatDiff& d) {
	s.flag ^= d.flag;
	s.HP += d.HP;
	s.ATK += d.ATK;
	s.DEF += d.DEF;
	s.GR += d.GR;
	s.REP += d.REP;
	s.YK += d.YK;
	s.GK += d.GK;
	s.BK += d.BK;
	s.SK += d.SK;
	s.equipATK += d.equipATK;
	s.equipDEF += d.equipDEF;
	return s;
}

StatDiff operator - (const StatDiff& d) {
	return StatDiff{
		flag : d.flag,
		HP : - d.HP,
		ATK : - d.ATK,
		DEF : - d.DEF,
		GR : - d.GR,
		REP : - d.REP,
		YK : - d.YK,
		GK : - d.GK,
		BK : - d.BK,
		SK : - d.SK,
		equipATK: -d.equipATK,
		equipDEF: -d.equipDEF
	};
}

StatDiff operator + (const StatDiff& d0, const StatDiff& d1) {
	return StatDiff{
		flag : d0.flag ^ d1.flag,
		HP : d0.HP + d1.HP,
		ATK : d0.ATK + d1.ATK,
		DEF : d0.DEF + d1.DEF,
		GR : d0.GR + d1.GR,
		REP : d0.REP + d1.REP,
		YK : d0.YK + d1.YK,
		GK : d0.GK + d1.GK,
		BK : d0.BK + d1.BK,
		SK : d0.SK + d1.SK,
		equipATK : d0.equipATK + d1.equipATK,
		equipDEF : d0.equipDEF + d1.equipDEF
	};
}

StatDiff operator - (const StatDiff& d0, const StatDiff& d1) {
	return StatDiff{
		flag : d0.flag & ~d1.flag,
		HP : d0.HP - d1.HP,
		ATK : d0.ATK - d1.ATK,
		DEF : d0.DEF - d1.DEF,
		GR : d0.GR - d1.GR,
		REP : d0.REP - d1.REP,
		YK : d0.YK - d1.YK,
		GK : d0.GK - d1.GK,
		BK : d0.BK - d1.BK,
		SK : d0.SK - d1.SK,
		equipATK : d0.equipATK - d1.equipATK,
		equipDEF : d0.equipDEF - d1.equipDEF
	};
}

bool operator >= (const Stat& s0, const Stat& s1) {
	return (s0.flag & s1.flag) == s1.flag
		&& s0.HP >= s1.HP
		&& s0.ATK >= s1.ATK
		&& s0.DEF >= s1.DEF
		&& s0.GR >= s1.GR
		&& s0.REP >= s1.REP
		&& s0.YK >= s1.YK
		&& s0.GK >= s1.GK
		&& s0.BK >= s1.BK
		&& s0.SK >= s1.SK
		&& s0.equipATK >= s1.equipATK
		&& s0.equipDEF >= s1.equipDEF;
}

constexpr Stat nullStat;

bool Stat::valid() const {
	return *this >= nullStat;
}

StatDiff min(const StatDiff& d0, const StatDiff& d1) {
	return StatDiff{
		flag : d0.flag & d1.flag,
		HP : d0.HP < d1.HP ? d0.HP : d1.HP,
		ATK : d0.ATK < d1.ATK ? d0.ATK : d1.ATK,
		DEF : d0.DEF < d1.DEF ? d0.DEF : d1.DEF,
		GR : d0.GR < d1.GR ? d0.GR : d1.GR,
		REP : d0.REP < d1.REP ? d0.REP : d1.REP,
		YK : d0.YK < d1.YK ? d0.YK : d1.YK,
		GK : d0.GK < d1.GK ? d0.GK : d1.GK,
		BK : d0.BK < d1.BK ? d0.BK : d1.BK,
		SK : d0.SK < d1.SK ? d0.SK : d1.SK,
		equipATK : d0.equipATK < d1.equipATK ? d0.equipATK : d1.equipATK,
		equipDEF : d0.equipDEF < d1.equipDEF ? d0.equipDEF : d1.equipDEF
	};
}

StatDiff max(const StatDiff& d0, const StatDiff& d1) {
	return StatDiff{
		flag : d0.flag | d1.flag,
		HP : d0.HP > d1.HP ? d0.HP : d1.HP,
		ATK : d0.ATK > d1.ATK ? d0.ATK : d1.ATK,
		DEF : d0.DEF > d1.DEF ? d0.DEF : d1.DEF,
		GR : d0.GR > d1.GR ? d0.GR : d1.GR,
		REP : d0.REP > d1.REP ? d0.REP : d1.REP,
		YK : d0.YK > d1.YK ? d0.YK : d1.YK,
		GK : d0.GK > d1.GK ? d0.GK : d1.GK,
		BK : d0.BK > d1.BK ? d0.BK : d1.BK,
		SK : d0.SK > d1.SK ? d0.SK : d1.SK,
		equipATK : d0.equipATK > d1.equipATK ? d0.equipATK : d1.equipATK,
		equipDEF : d0.equipDEF > d1.equipDEF ? d0.equipDEF : d1.equipDEF
	};
}
