#pragma once

#include "structure.h"

#include "strings.h"


class Park : public Structure
{
public:
	Park() : Structure(
		StructureClass::Park,
		StructureID::SID_PARK)
	{
	}
};
