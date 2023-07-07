// #pragma once

// #include"structure.h"

// #include"strings.h"
// #include"productpool.h"


class Warehouse : public Structure
{
public:
	Warehouse() : Structure(
		StructureClass::Warehouse,
		StructureID::SID_WAREHOUSE)
	{
	}

	ProductPool& products() { return mProducts; }

private:
	ProductPool mProducts;
};
