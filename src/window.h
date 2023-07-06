#pragma once

#include "uicontainer.h"

#include "font.h"
#include "image.h"
#include <rectangle_skin.h>
#include "point.h"
#include "vector.h"

#include <string>


class Window : public UIContainer
{
public:
	Window(std::string newTitle = "", const NAS2D::Font& titleFont = getDefaultFontBold());
	~Window() override;

	void anchored(bool isAnchored);

	void show() override;

	void update() override;
	void draw() const override;

	using TitleChangeSignal = NAS2D::Signal<Window*>;

	void title(const std::string& title);
	const std::string& title() const { return mTitle; }
	TitleChangeSignal::Source& titleChanged() { return mTitleChanged; }

	virtual void onTitleChanged() { mTitleChanged(this); }

protected:
	void onMouseDown(NAS2D::EventHandler::MouseButton button, NAS2D::Point<int> position) override;
	void onMouseUp(NAS2D::EventHandler::MouseButton button, NAS2D::Point<int> position);
	void onMouseMove(NAS2D::Point<int> position, NAS2D::Vector<int> relative);

	static constexpr int sWindowTitleBarHeight = 20;

private:
	const NAS2D::Font& mTitleFont;
	const NAS2D::Image& mTitleBarLeft;
	const NAS2D::Image& mTitleBarCenter;
	const NAS2D::Image& mTitleBarRight;
	NAS2D::RectangleSkin mBody;

	TitleChangeSignal mTitleChanged;

	std::string mTitle;

	bool mMouseDrag = false;
	bool mAnchored = false;
};
