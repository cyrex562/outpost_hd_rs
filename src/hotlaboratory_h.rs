// #pragma once

// #include"researchfacility.h"

// #include"strings.h"


class HotLaboratory : public ResearchFacility
{
public:
	HotLaboratory() : ResearchFacility(
		StructureClass::Laboratory,
		StructureID::SID_HOT_LABORATORY)
	{
		maxScientistsAllowed(3);
		hotPointsPerScientist(1.0f);
	}
};
