#pragma once

#include <cassert>
#include "room.h"

ProbeStat RoomElement::ToProbeStat(const EssStat& player) const {
	switch (type) {
		case DResourceType:
			return {resource, {}, 0};
		case DCostType:
			return {-cost, (Stat) req, 0};
		case DRequirementType:
			return {{}, req, 0};
		case DMonsterType:
			return monster.ToProbeStat(player);
	};
	assert(0);
};

// --------------------------------------------------------------------------------

constexpr RoomElement DResource(const StatDiff resource) {
	return {DResourceType, {.resource = resource}};
}

constexpr RoomElement DCost(const StatDiff cost) {
	return {DCostType, {.cost = cost}};
}

constexpr RoomElement DRequirement(const Stat req) {
	return {DResourceType, {.req = req}};
}

// --------------------------------------------------------------------------------

constexpr auto DYK = DResource({YK: 1});
constexpr auto DGK = DResource({GK: 1});
constexpr auto DBK = DResource({BK: 1});

constexpr RoomElement DHP(int n) {
	return DResource({HP: n});
}

constexpr RoomElement DATK(int n) {
	return DResource({ATK: n});
}

constexpr RoomElement DDEF(int n) {
	return DResource({DEF: n});
}

constexpr RoomElement DGR(int n) {
	return DResource({GR: n});
}

constexpr auto DYD = DCost({YK: 1});
constexpr auto DGD = DCost({GK: 1});
constexpr auto DBD = DCost({BK: 1});

constexpr RoomElement DGG(int n) {
	return DCost({GR: n});
}

// --------------------------------------------------------------------------------

ProbeStat Room::ToProbeStat(const EssStat& player) const {
	auto stat = (Stat) player;
	ProbeStat res;
	for (auto r : content) {
		auto probe = r.ToProbeStat(stat);
		res += probe;
		stat += probe.diff;
	}
	return res;
}
