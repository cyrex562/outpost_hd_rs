#pragma once

#include "foodproduction.h"

#include "numbers.h"
#include "strings.h"


/**
 * Implements the Command Center structure.
 */
class CommandCenter : public FoodProduction
{
public:
	CommandCenter() : FoodProduction(
		StructureClass::Command,
		StructureID::SID_COMMAND_CENTER)
	{
	}

	int foodCapacity() override
	{
		return constants::BaseStorageCapacity;
	}

	int getRange() const
	{
		return operational() ? constants::RobotCommRange : 0;
	}

protected:
	int calculateProduction() override
	{
		return 0;
	}
};
