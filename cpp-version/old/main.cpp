//#define TOTS


#include <iostream>
#include <string>

#include "stat.cpp"

#include "room.cpp"

#include "monster.cpp"

#include "equipment.cpp"

#include "hottile.cpp"

#include "map.cpp"

#include "player.cpp"

/* Warning:
	Player's HP is shifted by 1 so that 0 HP is considered alive.
	This change makes code cleaner.
*/


	#include "TendrysTale.cpp"

int main() {
	std::cout << "--------------------------------------------------------------------------------" << std::endl;

	TendrysTale();

	std::cout << "--------------------------------------------------------------------------------" << std::endl;
}