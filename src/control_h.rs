// #pragma once

// #include"signal.h"
// #include"point.h"
// #include"vector.h"
// #include"rectangle.h"
// #include"resourcecache.h"


// namespace NAS2D
// {
// 	class Font;
// 	class Image;
// }


/**
 * Implements a base for all GUI Controls to derive from.
 * 
 * The Control class is the base class from which all GUI controls inherit
 * from.
 */
// class Control
pub struct Control
{
// public:
	// using ResizeSignal = NAS2D::Signal<Control*>;
	// using OnMoveSignal = NAS2D::Signal<NAS2D::Vector<int>>;

	// using ControlImageCache = NAS2D::ResourceCache<NAS2D::Image, std::string>;

	// static void setDefaultFont(const NAS2D::Font& font);
	// static void setDefaultFontBold(const NAS2D::Font& font);
	// static void setImageCache(ControlImageCache& controlImageCache);

	// static const NAS2D::Font& getDefaultFont();
	// static const NAS2D::Font& getDefaultFontBold();
	// static const NAS2D::Image& getImage(const std::string& filename);


	// Control() = default;
	// Control(Control&) = default;
	// Control(Control&&) = default;
	// virtual ~Control() = default;

	// const NAS2D::Rectangle<int>& area() const { return mRect; }
	// void area(const NAS2D::Rectangle<int>& area);

	// NAS2D::Point<int> position() const { return mRect.position; }
	// void position(NAS2D::Point<int> pos);

	// int positionX() const;
	// int positionY() const;

	// OnMoveSignal::Source& moved();

	// void highlight(bool highlight);
	// bool highlight() const;

	// void enabled(bool enabled);
	// bool enabled() const;

	// void visible(bool visible);
	// bool visible() const;

	// virtual void hide() { visible(false); }
	// virtual void show() { visible(true); }

	// const NAS2D::Rectangle<int>& rect() const;

	// virtual void hasFocus(bool focus);
	// bool hasFocus() const;

	// NAS2D::Vector<int> size() const { return mRect.size; }
	// void size(NAS2D::Vector<int> newSize);
	// void size(int newSize);

	// void width(int w);
	// void height(int h);

	// ResizeSignal::Source& resized();

	// virtual void update() {}

// protected:
	/**
	 * Called whenever the Control's position is changed.
	 * 
	 * \param	displacement	Difference in position.
	 */
	// virtual void onMove(NAS2D::Vector<int> displacement) { mOnMoveSignal(displacement); }

	// virtual void onResize() { mOnResizeSignal(this); }

	// virtual void onVisibilityChange(bool /*visible*/) {}

	// virtual void onEnableChange() {}

	// virtual void onFocusChange() {}

	// OnMoveSignal mOnMoveSignal; /**< Signal fired whenever the position of the Control changes. */
	pub mOnMoveSignal: OnMoveSignal,

	// ResizeSignal mOnResizeSignal;
	pub mOnResizeSignal: ResizeSignal,

	// NAS2D::Rectangle<int> mRect; /**< Area of the Control. */
	pub mRect: Rectangle<i32>,

	// bool mVisible = true; /**< Flag indicating visibility of the Control. */
	pub mVisible: bool,
	
	// bool mEnabled = true; /**< Flag indicating whether or not the Control is enabled. */
	pub mEnabled: bool,

	// bool mHasFocus = false; /**< Flag indicating that the Control has input focus. */
	pub mHasFocus: bool,
	// bool mHighlight = false; /**< Flag indicating that this Control is highlighted. */
	pub mHighlight: bool,

// private:
	// virtual void draw() const {}
}

impl Control 
{
	pub fn setDefaultFont(&mut self, ctx: &mut AppContext, font: &mut Font)
{
	ctx.defaultFont = &font;
}


void Control::setDefaultFontBold(const NAS2D::Font& font)
{
	defaultFontBold = &font;
}


void Control::setImageCache(ControlImageCache& controlImageCache)
{
	defaultImageCache = &controlImageCache;
}


const NAS2D::Font& Control::getDefaultFont()
{
	if (!defaultFont)
	{
		throw std::runtime_error("No default font set");
	}
	return *defaultFont;
}


const NAS2D::Font& Control::getDefaultFontBold()
{
	if (!defaultFontBold)
	{
		throw std::runtime_error("No default bold font set");
	}
	return *defaultFontBold;
}


const NAS2D::Image& Control::getImage(const std::string& filename)
{
	if (!defaultImageCache)
	{
		throw std::runtime_error("No default image cache set");
	}
	return defaultImageCache->load(filename);
}


// Control member functions

void Control::area(const NAS2D::Rectangle<int>& newRect)
{
	const auto displacement = newRect.position - mRect.position;
	mRect = newRect;
	onMove(displacement);
	onResize();
}


/**
 * Sets the position of the Control.
 * 
 * \param pos	2D Coordinate to position the Control at.
 */
void Control::position(NAS2D::Point<int> pos)
{
	const auto displacement = pos - mRect.position;
	mRect.startPoint(pos);
	onMove(displacement);
}


/**
 * Gets the X Position of the Control.
 */
int Control::positionX() const
{
	return mRect.position.x;
}


/**
 * Gets the Y Position of the Control.
 */
int Control::positionY() const
{
	return mRect.position.y;
}


/**
 * Signal fired whenever the Control's position changes.
 */
Control::OnMoveSignal::Source& Control::moved()
{
	return mOnMoveSignal;
}


void Control::size(NAS2D::Vector<int> newSize)
{
	mRect.size = newSize;
	onResize();
}


void Control::size(int newSize)
{
	size({newSize, newSize});
}


/**
 * Sets the width of the Control.
 * 
 * \note	This is an internal function and may not be
 *			called outside of the Control class.
 */
void Control::width(int w)
{
	size({w, size().y});
}


/**
 * Sets the height of the Control.
 * 
 * \note	This is an internal function and may not be
 *			called outside of the Control class.
 */
void Control::height(int h)
{
	size({size().x, h});
}


Control::ResizeSignal::Source& Control::resized()
{
	return mOnResizeSignal;
}


/**
 * Gets the rectangular area that the Control occupies.
 * 
 * \return	A const reference to a Rectangle<int> object.
 */
const NAS2D::Rectangle<int>& Control::rect() const
{
	return mRect;
}


/**
 * Sets the focus of the Control.
 */
void Control::hasFocus(bool focus)
{
	mHasFocus = focus;
	onFocusChange();
}


/**
 * Gets whether the Control is in focus or not.
 */
bool Control::hasFocus() const
{
	return mHasFocus;
}


/**
 * Sets highlighting of the Control.
 * 
 * \param	highlight		True highlights the Control. False turns off highlighting.
 * 
 * \note	Some controls may ignore this setting.
 */
void Control::highlight(bool highlight)
{
	mHighlight = highlight;
}


/**
 * Gets whether or not the Control is highlighed.
 */
bool Control::highlight() const
{
	return mHighlight;
}


/**
 * Enables/Disabled the Control.
 * 
 * \param	enabled		True enables the Control. False disables the Control.
 * 
 * \note	Some controls may ignore this setting.
 */
void Control::enabled(bool enabled)
{
	mEnabled = enabled;
	onEnableChange();
}


/**
 * Gets whether the Control is disabled or not.
 */
bool Control::enabled() const
{
	return mEnabled;
}


/**
 * Sets vibility of the Control.
 * 
 * \param	visible	True sets Control to visible. False hides the Control.
 * 
 * \note	Some controls may ignore this setting.
 */
void Control::visible(bool visible)
{
	mVisible = visible;
	onVisibilityChange(mVisible);
}


/**
 * Gets visibility of the Control.
 */
bool Control::visible() const
{
	return mVisible;
}
}