#pragma once

#include "structure.h"

#include "strings.h"


class Road : public Structure
{
public:
	Road() : Structure(
		StructureClass::Road,
		StructureID::SID_ROAD)
	{
	}
};
