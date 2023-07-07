// #pragma once

// #include"orerefining.h"

// #include"strings.h"


class SeedSmelter : public OreRefining
{
public:
	SeedSmelter() : OreRefining(
		StructureClass::Smelter,
		StructureID::SID_SEED_SMELTER)
	{
	}
};
