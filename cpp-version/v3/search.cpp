struct SearchConfig {
	bool useEstimatedMaxCombat = true;
	bool printNewHighscore = true;
	bool calculateOptimalPlayerByStat = true;
	bool printLocalOptimalPlayerByScore = true;
	bool printLocalOptimalPlayerByStat = true;
	bool printGlobalOptimalPlayerByScore = true;
	bool printGlobalOptimalPlayerByStat = true;
	std::ostream* osp = &std::cout;
	std::ostream* logosp = &std::cout;
};

struct SearchProgress {
	std::size_t totalSearchCount;
	std::size_t currentSearchCount;
	Timer timer;
};

struct Search {
	SearchConfig searchConfig;
	LevelInfo levelInfo;
	Player initPlayer;

	std::vector<ProbeStat>& probe(const PlayerCombat&);
	void addExitPlayer(const Player& p);
	void removePlayerProgress(PlayerProgress);
	void expand(const Player& p, VertexIDType id, const ProbeStat& probe);
	PlayerTrace reconstructTrace(Player) const;

	std::vector<ProbeStat> maxCombatProbeResult;
	void estimateMaxCombat();

	void printProgress();
	void search(const int);
	void search();

	SearchProgress searchProgress;

	int levelConfig;
	Level* level;

	Optimal<PlayerTrace, PlayerScore> localOptimalPlayerByScore;
	Optimal<PlayerTrace, PlayerScore> globalOptimalPlayerByScore;

	OptimalSet<PlayerTrace, PlayerStat> localOptimalPlayerByStat;
	OptimalSet<PlayerTrace, PlayerStat> globalOptimalPlayerByStat;

	std::unordered_map<PlayerCombat, std::vector<ProbeStat>> probeResult;
	std::unordered_map<PlayerProgress, int> playerProgressRC;
	std::unordered_map<PlayerProgress, Player> optimalPlayer;
	std::deque<PlayerProgress> clones;
};


void Search::estimateMaxCombat() {
	auto maxCombat = PlayerCombat{
		MEMBER_INIT_MAX(PlayerCombat, ATK)
		MEMBER_INIT_MAX(PlayerCombat, DEF)
	};

	auto stat = PlayerStat{};
	for (int i = 0; i < level->N; ++i) {
		auto probe = level->vertex(i)->probe(maxCombat);
		auto diff = PlayerStat{};
		diff.join(probe.diff);
		stat += diff;
	}
	maxCombat = (PlayerCombat) initPlayer;
	maxCombat += stat;
	*(searchConfig.osp) << "Estimated " << maxCombat;
	*(searchConfig.logosp) << "Estimated " << maxCombat;
	maxCombatProbeResult = probe(maxCombat);
};

std::vector<ProbeStat>& Search::probe(const PlayerCombat& combat) {
	if (probeResult.contains(combat)) {
		return probeResult[combat];
	}
	std::vector<ProbeStat> res;
	for (int i = 0; i < level->N; ++i) {
		res.push_back(level->vertex(i)->probe(combat));
	}
	probeResult[combat] = res;
	return probeResult[combat];
}

PlayerTrace Search::reconstructTrace(Player player) const {
	PlayerTrace playerTrace = {
		levelConfig: levelConfig,
		level: level,
		player: player,
	};
	std::vector<int> trace;
	auto progress = (PlayerProgress) player;
	while (!(progress == (PlayerProgress) initPlayer)) {
		trace.push_back(player.diff.location);
		progress -= player.diff;
		player = optimalPlayer.at(progress);
	}
	while (!trace.empty()) {
		playerTrace.trace.push_back(trace.back());
		trace.pop_back();
	}
	return playerTrace;
}

void Search::removePlayerProgress(PlayerProgress progress) {
	while (!(progress == (PlayerProgress) initPlayer) && --playerProgressRC[progress] <= 0) {
		auto diff = optimalPlayer[progress].diff;
		optimalPlayer.erase(progress);
		progress -= diff;
	}
}


void Search::addExitPlayer(const Player& player) {
	if (searchConfig.calculateOptimalPlayerByStat) {
		bool localMaxByStat = localOptimalPlayerByStat.addable((PlayerStat) player);
		if (!localMaxByStat) {
			return;
		}
	}
	if (searchConfig.calculateOptimalPlayerByStat) {
		PlayerTrace playerTrace = reconstructTrace(player);
		localOptimalPlayerByStat.add(playerTrace, true);
	}
	bool localMaxByScore = localOptimalPlayerByScore.addable((PlayerScore) player);
	if (!localMaxByScore) {
		return;
	}
	PlayerTrace playerTrace = reconstructTrace(player);
	localOptimalPlayerByScore.add(playerTrace, localMaxByScore);
	if (searchConfig.printNewHighscore) {
		auto& os = *(searchConfig.osp);
		os << "New High ";
		os << playerTrace;
		os << "--------------------------------------------------------------------------------" << std::endl;
	}
}

void Search::expand(const Player& player, const VertexIDType location, const ProbeStat& probe) {
	// This approach is quite slow. But, it is much easier to write in this way.
	Player newPlayer = player;
	newPlayer.visit(location, level, probe);

	if (location == level->exit) {
		addExitPlayer(newPlayer);
		return;
	}
	auto newProgress = (PlayerProgress) newPlayer;
	if (optimalPlayer.contains(newProgress)) {
		auto newObjective = (PlayerObjective) newPlayer;
		if ((PlayerObjective) optimalPlayer[newProgress] >= newObjective) {
			return;
		}
		++playerProgressRC[(PlayerProgress) player];
		removePlayerProgress(optimalPlayer[newProgress].revertedProgress());
		optimalPlayer[newProgress] = newObjective;
		optimalPlayer[newProgress].diff = newPlayer.diff;
	} else {
		optimalPlayer[newProgress] = newPlayer;
		++playerProgressRC[(PlayerProgress) player];
		clones.push_back(newProgress);
		++searchProgress.totalSearchCount;
	}
}

void Search::printProgress() {
	if (searchProgress.currentSearchCount % 1000000 == 0 && searchProgress.timer.elapsed() > 10000) {
		searchProgress.timer.reset();
		*(searchConfig.logosp) << "Progress: " << searchProgress.currentSearchCount / 1000000 << "m / " << searchProgress.totalSearchCount / 1000000 << "m" << std::endl;
	}
}

struct ExtendedProbeStat {
	VertexIDType location;
	ProbeStat probe;
};

void Search::search(const int config) {
	searchProgress = {};
	localOptimalPlayerByScore.clear();
	localOptimalPlayerByStat.clear();
	probeResult.clear();
	playerProgressRC.clear();
	optimalPlayer.clear();

	levelConfig = config;
	level = levelInfo.build(levelConfig);
	if (!level) {
		*(searchConfig.logosp) << "Invalid config." << std::endl;
		*(searchConfig.osp) << "Invalid config." << std::endl;
		return;
	}
	Player player = initPlayer;
	player.enter(level);
	auto playerProgress = (PlayerProgress) player;
	optimalPlayer[playerProgress] = player;
	clones.push_back(playerProgress);
	++searchProgress.totalSearchCount;

	if (searchConfig.useEstimatedMaxCombat) {
		estimateMaxCombat();
	}

	while (!clones.empty()) {
		++searchProgress.currentSearchCount;
		printProgress();

		auto progress = clones.front();
		auto player = optimalPlayer[progress];
		clones.pop_front();
		playerProgressRC[progress] = 0;

		auto probeResult = probe((PlayerCombat) player);
		std::vector<ExtendedProbeStat> extendedProbeResult;
		extendedProbeResult.reserve(player.neighbours.count());

		bool wasIntermediate = player.diff.location == (VertexIDType) -1 ? false : (level->vertex(player.diff.location)->type & DIntermediateRoom);
		bool hasFreePriority = false;
		for (auto room : player.neighbours) {
			if (wasIntermediate && !level->neighbours[player.diff.location].test(room)) {
				continue;
			}
			auto probe = probeResult[room];
			bool available = (player >= probe.req);
			if (!available) {
				continue;
			}
			auto roomType = level->vertex(room)->type;
			bool priorty = roomType & DPriorityRoom;
			bool intermediate = roomType & DIntermediateRoom;
			bool free = searchConfig.useEstimatedMaxCombat
				&& (room != level->exit)
				&& !intermediate
				&& !(probe.diff.Flag & player.Flag)
				&& !(roomType & DRepeatedRoom)
				&& !(roomType & DDelayedRoom)
				#ifdef ClosedLevel
					&& !level->boundaryMask.test(room)
					&& !level->boundaryMask.test(player.diff.location)
				#endif
				&& (PlayerObjective) maxCombatProbeResult[room].diff == (PlayerObjective) probe.diff
				&& probe.diff.nonnegative();
			if (!free && (level->vertex(room)->type & DOnlyWhenFreeRoom)) {
				continue;
			}
			if (free || priorty) {
				hasFreePriority = true;
				expand(player, room, probe);
				break;
			}
			extendedProbeResult.push_back({
				location: (VertexIDType) room,
				probe: probe,
			});
		}
		if (!hasFreePriority) {
			for (auto& probe : extendedProbeResult) {
				expand(player, probe.location, probe.probe);
			}
		}
		if (!playerProgressRC[progress]) {
			removePlayerProgress(progress);
		}
	}

	globalOptimalPlayerByScore.add(localOptimalPlayerByScore);
	if (searchConfig.calculateOptimalPlayerByStat) {
		globalOptimalPlayerByStat.add(localOptimalPlayerByStat);
	}
}

void Search::search() {
	auto& os = *(searchConfig.osp);
	auto& logos = *(searchConfig.logosp);
	for (int config = 0; config < (levelInfo.maxConfigNumber); ++config) {
		os << "================================================================================\n";
		os << "Config:\n";
		levelInfo.printConfig(os, config);
		logos << "Config:\n";
		levelInfo.printConfig(logos, config);
		os.flush();


		Timer t;
		search(config);

		os << "================================================================================\n";
		os << "There are " << searchProgress.totalSearchCount << " situations searched.\n" << std::endl;
		logos << "There are " << searchProgress.totalSearchCount << " situations searched.\n" << std::endl;
		os << "Finished searching in " << t.elapsed() / 1000 << " seconds.\n";
		logos << "Finished searching in " << t.elapsed() / 1000 << " seconds.\n";

		if (searchConfig.printLocalOptimalPlayerByScore) {
			if (!localOptimalPlayerByScore.value.score) {
				os << "It is impossible to reach exit with this config.\n";
				os << "================================================================================\n";
			} else {
				os << "The local optimal player by score is: \n";
				os << "--------------------------------------------------------------------------------\n";
				localOptimalPlayerByScore.item.print(os, initPlayer);
			}
		}

		if (searchConfig.printLocalOptimalPlayerByStat) {
			os << "================================================================================\n";
			os << "There are " << localOptimalPlayerByStat.size() << " local optimal players by stats.\n";
			os << "================================================================================\n";
			os << "--------------------------------------------------------------------------------\n";
			int i = 0;
			for (auto& p: localOptimalPlayerByStat) {
				++i;
				os << "Local optiaml player by score [" << i << "] ";
				os << p;
				os << "--------------------------------------------------------------------------------\n";
			}
			os << "================================================================================\n";
			os.flush();
		}
	}

	if (searchConfig.printGlobalOptimalPlayerByScore) {
		logos << "--------------------------------------------------------------------------------\n";
		logos << "The global optimal player by score is: \n";
		levelInfo.printConfig(logos, globalOptimalPlayerByScore.item.levelConfig);
		logos << globalOptimalPlayerByScore.item;
		logos << "--------------------------------------------------------------------------------\n";
		os << "////////////////////////////////////////////////////////////////////////////////\n";
		os << "The global optimal player by score is: \n";
		levelInfo.printConfig(os, globalOptimalPlayerByScore.item.levelConfig);
		os << "--------------------------------------------------------------------------------\n";
		globalOptimalPlayerByScore.item.print(os, initPlayer);
	}

	if (searchConfig.printGlobalOptimalPlayerByStat) {
		os << "////////////////////////////////////////////////////////////////////////////////\n";
		logos << "There are " << globalOptimalPlayerByStat.size() << " global optimal players by stat.\n";
		os << "There are " << globalOptimalPlayerByStat.size() << " global optimal players by stat.\n";
		os << "////////////////////////////////////////////////////////////////////////////////\n";
		os << "--------------------------------------------------------------------------------\n";
		int i = 0;
		for (auto p: globalOptimalPlayerByStat) {
			++i;
			os << "Local optiaml player [" << i << "]\n";
			levelInfo.printConfig(os, p.levelConfig);
			os << p;
			os << "--------------------------------------------------------------------------------\n";
		}
		os << "////////////////////////////////////////////////////////////////////////////////\n";
		os.flush();
	}
}
