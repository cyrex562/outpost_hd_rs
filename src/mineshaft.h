#pragma once

#include "structure.h"

#include "strings.h"


class MineShaft : public Structure
{
public:
	MineShaft() : Structure(
		StructureClass::Undefined,
		StructureID::SID_MINE_SHAFT)
	{
	}
};
