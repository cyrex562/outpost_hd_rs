// #pragma once

// #include<window.h>
// #include<button.h>


class GameOptionsDialog : public Window
{
public:
	using ClickSignal = NAS2D::Signal<>;

	GameOptionsDialog();
	~GameOptionsDialog() override;

	void update() override;

	ClickSignal::Source& SaveGame() { return mSignalSave; }
	ClickSignal::Source& LoadGame() { return mSignalLoad; }
	ClickSignal::Source& returnToGame() { return mSignalReturn; }
	ClickSignal::Source& returnToMainMenu() { return mSignalClose; }

private:
	pub const buttonHeight: i32 = 25;
	pub const buttonWidth: i32 = 200;
	pub const buttonHorizontalMargin: i32 = 5;
	pub const buttonVerticalMargin: i32 = 3;
	
	void onLoad();
	void onSave();
	void onHelp();
	void onReturn();
	void onClose();

	void onEnableChange() override;

	Button btnSave;
	Button btnLoad;
	Button btnHelp;
	Button btnReturn;
	Button btnClose;

	ClickSignal mSignalSave;
	ClickSignal mSignalLoad;
	ClickSignal mSignalReturn;
	ClickSignal mSignalClose;
};
