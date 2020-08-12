#include "digraph.h"

struct Level : public LevelModel {
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

template <class T>
auto& operator >> (LevelPtr level, T t) {
	return level()->select(t);
}

template<class T>
T CreateLevelConfig(int n) {
	T t;
	for (int i = 0; i < sizeof(T) / sizeof(bool); ++i) {
		*((bool*)(&t) + i) = n & (1 << i);
	}
	return t;
}

struct LevelInfo {
	int maxConfigNumber; // exclude
	LevelPtr (*build) (int);
	void (*printConfig) (std::ostream&, int);

	template<typename T>
	static LevelInfo create() {
		return LevelInfo{
			maxConfigNumber: (std::is_empty<T>::value ? 1 : 1 << sizeof(T)),
			build: [](int config) -> LevelPtr {
				return CreateLevelConfig<T>(config).build();
			},
			printConfig: [](std::ostream& os, int config) {
				CreateLevelConfig<T>(config).print(os);
			},
		};

	}
};
