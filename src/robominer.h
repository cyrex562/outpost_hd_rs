#pragma once

#include "robot.h"


struct MapCoordinate;
class TileMap;
class MineFacility;


class Robominer : public Robot
{
public:
	Robominer();

	MineFacility& buildMine(TileMap& tileMap, const MapCoordinate& position);
};
