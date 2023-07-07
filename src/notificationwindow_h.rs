// #pragma once

// #include<window.h>
// #include<button.h>
// #include<textarea.h>

// #include"notificationarea.h"

// #include"signal.h"


struct MapCoordinate;


class NotificationWindow : public Window
{
public:
	using TakeMeThereSignal = NAS2D::Signal<const MapCoordinate&>;

public:
	NotificationWindow();

	void notification(const NotificationArea::Notification&);

	TakeMeThereSignal::Source& takeMeThere() { return mTakeMeThereClicked; }

	void update() override;

private:
	void onOkayClicked();
	void onTakeMeThereClicked();

	const NAS2D::Image& mIcons;

	NotificationArea::Notification mNotification;
	Button btnOkay{"Okay", {this, &NotificationWindow::onOkayClicked}};
	Button btnTakeMeThere{"Take Me There", {this, &NotificationWindow::onTakeMeThereClicked}};
	TextArea mMessageArea;
	bool mTakeMeThereVisible{false};

	TakeMeThereSignal mTakeMeThereClicked;
};
