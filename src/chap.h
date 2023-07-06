#pragma once

#include "structure.h"

#include "strings.h"


class CHAP : public Structure
{
public:
	CHAP() : Structure(StructureClass::LifeSupport, StructureID::SID_CHAP)
	{
	}
};
