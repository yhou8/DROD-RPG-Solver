#pragma once

#include <cstdint>
#include <functional>

template<std::size_t N>
struct _Bitset;

template<std::size_t N>
struct _BitsetIterator {
	_Bitset<N> bs;
	int index;

	_BitsetIterator operator++() {
		bs.reset(index - 1);
		index = bs.ffs();
		return *this;
	}

	bool operator != (const _BitsetIterator& it) const {
		return index != it.index;
	}
	const int operator*() const {
		return index - 1;
	}
};

template<>
struct _Bitset<128> {
	typedef uint64_t BlockType;
	BlockType d0 = 0;
	BlockType d1 = 0;

	_Bitset& operator += (const _Bitset& b) {
		d0 |= b.d0;
		d1 |= b.d1;
		return *this;
	}

	_Bitset& operator -= (const _Bitset& b) {
		d0 &= ~b.d0;
		d1 &= ~b.d1;
		return *this;
	}

	bool test(int pos) const {
		if (pos < 64) {
			return d0 & ((uint64_t) 1 << pos);
		} else {
			return d1 & ((uint64_t) 1 << (pos & 63));
		}
	}

	bool set(int pos) {
		if (pos < 64) {
			return d0 |= ((uint64_t) 1 << pos);
		} else {
			return d1 |= ((uint64_t) 1 << (pos & 63));
		}
	}

	bool reset(int pos) {
		if (pos < 64) {
			return d0 &= ~((uint64_t) 1 << pos);
		} else {
			return d1 &= ~((uint64_t) 1 << (pos & 63));
		}
	}

	bool empty() const {
		return d0 == 0 && d1 == 0;
	}

	int count() const {
		return __builtin_popcount(d0) + __builtin_popcount(d1);
	}

	int ffs() const {
		int index = __builtin_ffsll(d0);
		if (index) {
			return index;
		}
		index = __builtin_ffsll(d1);
		if (index) {
			return index + 64;
		}
		return 0;
	}

	_BitsetIterator<128> begin() const {
		return _BitsetIterator<128>{*this, ffs()};
	};

	_BitsetIterator<128> end() const {
		return _BitsetIterator<128>{*this, 0};
	};
};

bool operator == (const _Bitset<128> s0, const _Bitset<128> s1) {
	return s0.d0 == s1.d0 && s0.d1 == s1.d1;
}

namespace std {
	template<> struct hash<_Bitset<128>> {
		std::size_t operator()(_Bitset<128> const& s) const noexcept {
			return (s.d0 * 0xcc9e2d51) ^ s.d1;
		}
	};
}


template<>
struct _Bitset<64> {
	typedef uint64_t BlockType;
	BlockType d0 = 0;

	_Bitset& operator += (const _Bitset& b) {
		d0 |= b.d0;
		return *this;
	}

	_Bitset& operator -= (const _Bitset& b) {
		d0 &= ~b.d0;
		return *this;
	}

	bool test(int pos) const {
		return d0 & ((uint64_t) 1 << pos);
	}

	bool set(int pos) {
		return d0 |= ((uint64_t) 1 << pos);
	}

	bool reset(int pos) {
		return d0 &= ~((uint64_t) 1 << pos);
	}

	bool empty() const {
		return d0 == 0;
	}

	int count() const {
		return __builtin_popcount(d0);
	}

	int ffs() const {
		int index = __builtin_ffsll(d0);
		return index;
	}

	_BitsetIterator<64> begin() const {
		return _BitsetIterator<64>{*this, ffs()};
	};

	_BitsetIterator<64> end() const {
		return _BitsetIterator<64>{*this, 0};
	};
};

bool operator == (const _Bitset<64> s0, const _Bitset<64> s1) {
	return s0.d0 == s1.d0;
}

namespace std {
	template<> struct hash<_Bitset<64>> {
		std::size_t operator()(_Bitset<64> const& s) const noexcept {
			return (s.d0 * 0xcc9e2d51);
		}
	};
}

//typedef _Bitset<128> Bitset;
typedef _Bitset<64> Bitset;
