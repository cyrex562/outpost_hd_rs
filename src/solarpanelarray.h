#pragma once

#include "powerstructure.h"

#include "common.h"
#include "strings.h"


const int SOLAR_PANEL_BASE_PRODUCUCTION = 50;


class SolarPanelArray : public PowerStructure
{
public:
	SolarPanelArray() :
		PowerStructure
		{
			StructureClass::EnergyProduction,
			StructureID::SID_SOLAR_PANEL1
		}
	{
	}

protected:
	int calculateMaxEnergyProduction() override
	{
		return static_cast<int>(SOLAR_PANEL_BASE_PRODUCUCTION / getMeanSolarDistance());
	}
};
