#pragma once

#include <map>
#include <vector>
#include <cassert>
#include <utility>
#include "bitset.h"

template<class Vertex>
class Digraph {
public:
	Digraph& add(const Vertex v) {
		if (vertex2id.contains(v)) {
			currentVertexID = vertex2id.at(v);
			return *this;
		}
		assert(!name2id.contains(v->name));
		currentVertexID = N++;
		vertex2id[v] = currentVertexID;
		name2id[v->name] = currentVertexID;
		vertices.push_back(v);
		neighbours.push_back({});
		excludeNeighbours.push_back({});
		excludeNeighbours.back().set(currentVertexID);
		return *this;
	}

	Digraph& select(const Vertex v) {
		return add(v);
	}

	template<class T>
	Digraph& select(const T t) {
		currentVertexID = id(t);
		return *this;
	}

	Digraph& addArc(const int id0, const int id1) {
		if (id0 >= 0 && id1 >= 0 && id0 != id1) {
			neighbours[id0].set(id1);
		}
		return *this;
	}

	template <class T0, class T1>
	Digraph& addArc(const T0 t0, const T1 t1) {
		int id0 = select(t0).currentVertexID;
		int id1 = select(t1).currentVertexID;
		return addArc(id0, id1);
	}

	template<class T>
	Digraph& operator << (const T t1) {
		int id0 = currentVertexID;
		int id1 = select(t1).currentVertexID;
		return addArc(id0, id1);
	}

	template <class T0, class T1>
	Digraph& exclude(const T0 t0, const T1 t1) {
		int id0 = select(t0).currentVertexID;
		int id1 = select(t1).currentVertexID;
		if (id0 >= 0 && id1 >= 0 && id0 != id1) {
			excludeNeighbours[id0].set(id1);
			excludeNeighbours[id1].set(id0);
		}
		return *this;
	}

	int id(const int id) const {
		return id;
	}

	int id(const Vertex vertex) const {
		return vertex2id.at(vertex);
	}

	int id(const std::string name) const {
		return name2id.at(name);
	}

	Digraph& reset() {
		currentVertexID = -1;
		return *this;
	}

	Vertex vertex() const {
		return vertices[currentVertexID];
	}

	template<class T>
	Vertex vertex(T t) const {
		return vertices[id(t)];
	}

	int N = 0;

	std::vector<Bitset> neighbours;
	std::vector<Bitset> excludeNeighbours;

protected:
	int currentVertexID = -1;
	std::map<std::string, int> name2id;
	std::map<Vertex, int> vertex2id;
	std::vector<Vertex> vertices;
};
