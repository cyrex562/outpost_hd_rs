#pragma once

#include "structure.h"

#include "strings.h"


class SurfacePolice : public Structure
{
public:
	SurfacePolice() : Structure(
		StructureClass::SurfacePolice,
		StructureID::SID_SURFACE_POLICE)
	{
	}

	int getRange() const
	{
		return 5;
	}
};
