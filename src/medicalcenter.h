// #pragma once

// #include"structure.h"

// #include"strings.h"


class MedicalCenter : public Structure
{
public:
	MedicalCenter() : Structure(
		StructureClass::MedicalCenter,
		StructureID::SID_MEDICAL_CENTER)
	{
	}
};
