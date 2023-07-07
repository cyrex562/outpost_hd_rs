// #pragma once

// #include"structure.h"

// #include"strings.h"

// #include<vector>


class Robot;


/**
 * Implements the Robot Command structure.
 */
class RobotCommand : public Structure
{
public:
	RobotCommand() : Structure(
		StructureClass::RobotCommand,
		StructureID::SID_ROBOT_COMMAND)
	{
	}
};
