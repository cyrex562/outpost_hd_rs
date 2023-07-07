// #pragma once

// #include"powerstructure.h"

// #include"common.h"
// #include"strings.h"


pub const SOLAR_PLANT_BASE_PRODUCUCTION: i32 = 2000;


class SolarPlant : public PowerStructure
{
public:
	SolarPlant() :
		PowerStructure
		{
			StructureClass::EnergyProduction,
			StructureID::SID_SOLAR_PLANT
		}
	{
	}

protected:
	int calculateMaxEnergyProduction() override
	{
		return static_cast<int>(SOLAR_PLANT_BASE_PRODUCUCTION / getMeanSolarDistance());
	}
};
