#pragma once

#include <window.h>
#include <button.h>

#include "tile.h"


class TileInspector: public Window
{
public:
	TileInspector();

	void tile(Tile& tile) { mTile = &tile; }

	void update() override;

private:
	void onClose();

	Button btnClose;
	Tile* mTile = nullptr;
};
