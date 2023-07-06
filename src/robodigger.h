#pragma once

#include "robot.h"


enum class Direction;


class Robodigger : public Robot
{
public:
	Robodigger();

	void direction(Direction dir);
	Direction direction() const;

	NAS2D::Dictionary getDataDict() const override;

private:
	Direction mDirection;
};
