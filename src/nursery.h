#pragma once

#include "structure.h"

#include "strings.h"


class Nursery : public Structure
{
public:
	Nursery() : Structure(
		StructureClass::Nursery,
		StructureID::SID_NURSERY)
	{
	}
};
