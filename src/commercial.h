// #pragma once

// #include"structure.h"

// #include"strings.h"


class Commercial : public Structure
{
public:
	Commercial() : Structure(
		StructureClass::Commercial,
		StructureID::SID_COMMERCIAL)
	{
	}
};
