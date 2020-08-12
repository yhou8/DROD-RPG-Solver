#include <map>
#include <vector>

template<class Node>
class Graph {
public:
	int add(Node n) {
		auto it = ids.find(n);
		if (it != ids.end()) {
			return it->second;
		}
		int id = ids.size();
		ids[n] = id;
		name2id[n->name] = id;
		vertices.push_back(n);
		inNeighbourRequirement.push_back(0);
		neighbours.push_back( std::vector<int>() );
		levelEntrance.push_back(false);
		priority.push_back(false);
		return id;
	}

	void addArc(Node n0, Node n1) {
		int id0 = add(n0);
		int id1 = add(n1);
		neighbours[id0].push_back(id1);
	}

	void add(Node n0, Node n1) {
		int id0 = add(n0);
		if (n0 == n1) {
			return;
		}
		int id1 = add(n1);
		neighbours[id0].push_back(id1);
		neighbours[id1].push_back(id0);
	}

	void remove(Node n0, Node n1) {
		int id0 = add(n0);
		int id1 = add(n1);
		exclude[id0].push_back(id1);
		exclude[id1].push_back(id0);
	}

	std::map<std::string, int> name2id;
	std::map<Node, int> ids;
	std::vector<Node> vertices;
	std::vector< std::vector<int> > neighbours;
	std::vector< std::vector<int> > exclude;
	std::vector<int> inNeighbourRequirement;
	std::vector<bool> levelEntrance;
	std::vector<bool> priority;
	std::vector<bool> intermediate;

	// assume stat diff only depends on atk and def, equipatk, equipdef
	std::map< EssStat, std::vector<RoomStat> > probeResult;
};

enum RoomContentType {
	Key2HP = 1 << 0,
	HP2Key = 1 << 1,
	
};

class Map : public Graph<Room*> {
public:
	void setEntrance(Room* room) {
		entrance = ids[room];
	}

	void setExit(Room* room) {
		exit = ids[room];
	}

	void analyze();

	int entrance;
	int exit;
};

constexpr int MaxRoomsPerMap = 128;
