// #pragma once

// #include<control.h>

// #include"rectangle.h"
// #include"vector.h"
// #include"eventhandler.h"


namespace NAS2D
{
	class Image;
}

class TileMap;
class MapView;
class MapViewState;


class NavControl : public Control
{
public:
	NavControl(MapView& mapView, TileMap& tileMap);

	void draw() const override;

protected:
	void onMove(NAS2D::Vector<int> displacement) override;

	friend MapViewState;
	void onClick(NAS2D::Point<int> mousePosition);

private:
	MapView& mMapView;
	TileMap& mTileMap;
	const NAS2D::Image& mUiIcons;

	NAS2D::Rectangle<int> mMoveNorthIconRect;
	NAS2D::Rectangle<int> mMoveSouthIconRect;
	NAS2D::Rectangle<int> mMoveEastIconRect;
	NAS2D::Rectangle<int> mMoveWestIconRect;
	NAS2D::Rectangle<int> mMoveUpIconRect;
	NAS2D::Rectangle<int> mMoveDownIconRect;
	NAS2D::Rectangle<int> mBottomUiRect;
};
