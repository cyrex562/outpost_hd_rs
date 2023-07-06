#pragma once

#include "researchfacility.h"

#include "strings.h"


class Laboratory : public ResearchFacility
{
public:
	Laboratory() : ResearchFacility(
		StructureClass::Laboratory,
		StructureID::SID_LABORATORY)
	{
		maxScientistsAllowed(3);
		regularPointsPerScientist(1);
	}
};
