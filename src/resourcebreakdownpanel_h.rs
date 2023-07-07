// #pragma once

// #include<control.h>
// #include"storableresources.h"

// #include"font.h"
// #include"image.h"
// #include<rectangle_skin.h>


class ResourceBreakdownPanel : public Control
{
public:
	ResourceBreakdownPanel();

	void playerResources(const StorableResources* resources) { mPlayerResources = resources; }
	void previousResources(const StorableResources& resources) { mPreviousResources = resources; }
	StorableResources& previousResources() { return mPreviousResources; }
	void update() override;

private:
	const NAS2D::Font& mFont;
	const NAS2D::Image& mIcons;
	NAS2D::RectangleSkin mSkin;

	StorableResources mPreviousResources;
	const StorableResources* mPlayerResources = nullptr;
};
