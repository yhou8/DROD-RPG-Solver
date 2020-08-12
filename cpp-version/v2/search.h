#pragma once

#include <vector>
#include <unordered_map>
#include <utility>
#include <queue>
#include <list>
#include "level.h"
#include "EssPlayer.cpp"
#include "Player.cpp"

class Search {
public:
	Search(const Level* level, Stat initStat);

	template<class T>
	Search& operator >> (const T t);

	template<class T>
	Search& operator << (const T t);

	void search();

	void printOptimalPlayer(std::ostream& os, EssPlayer p) const;

	const Level* level;
	const Stat initStat;
	bool printHighscore = true;


	EssPlayer prefixPlayer;

	Player optimalExitPlayer = {};
	int optimalExitPlayerScore = 0;
	std::list<Player> localOptimalExitPlayer;
	std::unordered_map<Bitset, EssPlayer> optimalPlayer;
	EssPlayer initPlayer;

	Player ToPlayer(EssPlayer player) const;

private:
	std::size_t totalSearchCount = 0;
	std::size_t currentSearchCount = 0;

	std::vector<int> prefix;
	Bitset prefixBitset;
	std::vector<int> suffix;
	Bitset suffixBitset;
	std::unordered_map< EssStat, std::vector<ProbeStat> > probeResult;
	std::queue<Bitset> clones;
	std::unordered_map<Bitset, int> optimalNeededCount;

	const std::vector<ProbeStat>& probe(const EssStat s);

	const std::vector<ProbeStat>& probe(const EssPlayer& player);

	void tryRemoveOptimalPlayer(Bitset);

	void removeOptimalPlayer(const Bitset);

	void expand(EssPlayer p, int id, const ProbeStat& probe, bool push = true);
};