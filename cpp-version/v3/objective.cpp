template<typename T, typename V = T>
struct OptimalSet : std::list<T> {
	bool addable(const V& v) {
		for (auto it = this->begin(); it != this->end(); ) {
			if (V(*it) >= v) {
				return false;
			} else if (v >= V(*it)) {
				it = this->erase(it);
			} else {
				++it;
			}
		}
		return true;
	}

	bool add(const T& t, bool force = false) {
		if (force || addable(V(t))) {
			this->push_back(t);
			return true;
		}
		return false;
	}

	void add(const OptimalSet& t) {
		for (auto& it : t) {
			add(it);
		}
	}
};

template<typename T, typename V = T>
struct Optimal {
	T item;
	V value;

	bool addable(const V& v) {
		return !(value >= v);
	}

	bool add(const Optimal& t, bool force = false) {
		if (value >= t.value) {
			return false;
		} else {
			*this = t;
			return true;
		}
	}

	bool add(const T& t, bool force = false) {
		auto v = (V) t;
		if (force || addable(v)) {
			item = t;
			value = v;
			return true;
		} else {
			return false;
		}
	}

	void clear() {
		item = {};
		value = {};
	}
};
