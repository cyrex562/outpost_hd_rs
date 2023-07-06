#pragma once

#include "orerefining.h"

#include "strings.h"


class Smelter : public OreRefining
{
public:
	Smelter() : OreRefining(
		StructureClass::Smelter,
		StructureID::SID_SMELTER)
	{
	}
};
