template<class Vertex>
class Digraph {
public:
	Digraph& select(const Vertex v) {
		if (vertex2id.contains(v)) {
			currentVertexID = vertex2id.at(v);
			return *this;
		}
		ASSERT_WITH_MSG(!name2id.contains(v->name), "the room has aleady been added: " << v->name << std::endl);
		verticesMask.set(N);
		if (v->type & DRepeatedRoom) {
			boundaryMask.set(N);
		}
		currentVertexID = N++;
		vertex2id[v] = currentVertexID;
		name2id[v->name] = currentVertexID;
		vertices.push_back(v);
		neighbours.push_back({});
		toggleNeighbours.push_back({});
		return *this;
	}

	template<class T>
	Digraph& select(const T t) {
		currentVertexID = id(t);
		return *this;
	}

	Digraph& addArc(const int id0, const int id1) {
		if (id0 >= 0 && id1 >= 0) {
			neighbours[id0].set(id1);
		}
		return *this;
	}

	template<class T>
	Digraph& operator >> (const T t1) {
		int id0 = currentVertexID;
		int id1 = select(t1).currentVertexID;
		if (useEdge) {
			addArc(id1, id0);
		}
		return addArc(id0, id1);
	}

	template<class T>
	Digraph& operator << (const T t1) {
		int id0 = currentVertexID;
		int id1 = select(t1).currentVertexID;
		select(id0);
		if (useEdge) {
			addArc(id1, id0);
		}
		addArc(id0, id1);
		return *this;
	}

	template <class T0, class T1>
	Digraph& toggle(const T0 t0, const T1 t1) {
		int id0 = id(t0);
		int id1 = id(t1);
		if (id0 >= 0 && id1 >= 0) {
			toggleNeighbours[id0].set(id1);
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
		ASSERT_WITH_MSG(name2id.contains(name), "cannot find vertex with given name: " << name << std::endl);
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

	Bitset verticesMask;
	Bitset boundaryMask;
	std::vector<Bitset> neighbours;
	std::vector<Bitset> toggleNeighbours;

	bool useEdge = false;

protected:
	int currentVertexID = -1;
	std::map<std::string, int> name2id;
	std::map<Vertex, int> vertex2id;
	std::vector<Vertex> vertices;
};
