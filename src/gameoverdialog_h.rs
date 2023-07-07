// #pragma once

// #include<window.h>
// #include<button.h>


class GameOverDialog : public Window
{
public:
	using ClickSignal = NAS2D::Signal<>;

public:
	GameOverDialog();

	ClickSignal::Source& returnToMainMenu() { return mSignal; }

	void update() override;

private:
	void onClose();

	const NAS2D::Image& mHeader;

	Button btnClose;

	ClickSignal mSignal;
};
