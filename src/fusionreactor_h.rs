// #pragma once

// #include"powerstructure.h"

// #include"strings.h"

// #include<string>


pub const FUSION_REACTOR_BASE_PRODUCUCTION: i32 = 1000;


class FusionReactor : public PowerStructure
{
public:
	FusionReactor() : PowerStructure(
		StructureClass::EnergyProduction,
		StructureID::SID_FUSION_REACTOR)
	{
	}

protected:
	int calculateMaxEnergyProduction() override
	{
		return FUSION_REACTOR_BASE_PRODUCUCTION;
	}
};
