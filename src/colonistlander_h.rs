// #pragma once

// #include"structure.h"

// #include"strings.h"
// #include"tile.h"


class ColonistLander : public Structure
{
public:
	using Signal = NAS2D::Signal<>;

public:

	ColonistLander(Tile* tile) : Structure(
		StructureClass::Lander,
		StructureID::SID_COLONIST_LANDER),
		mTile(tile)
	{
		enable();
	}

	Signal::Source& deploySignal() { return mDeploy; }

protected:
	void think() override
	{
		if (age() == turnsToBuild())
		{
			mDeploy();
			mTile->index(TerrainType::Dozed);
		}
	}

private:
	Signal mDeploy;

	Tile* mTile;
};
