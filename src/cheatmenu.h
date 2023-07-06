#pragma once

#include <window.h>
#include <button.h>
#include <textfield.h>
#include <label.h>

#include "signal.h"

#include <map>

class CheatMenu : public Window
{
public:
	enum class CheatCode
	{
		AddResources,
		AddFood,
		AddRobots,
		AddChildren,
		AddStudents,
		AddWorkers,
		AddScientists,
		AddRetired,
		RemoveChildren,
		RemoveStudents,
		RemoveWorkers,
		RemoveScientists,
		RemoveRetired,
		Invalid
	};

	using CheatSignal = NAS2D::Signal<const std::string&>;
	
	void onOkay();
	void update() override;

	CheatMenu();
	
	static CheatMenu::CheatCode stringToCheatCode(const std::string& cheatCode);
	
	CheatSignal::Source& cheatCodeEntered() { return mSignal; }

private:
	CheatSignal mSignal;

	Label mLabelCheatCode;

	Button btnOkay;

	TextField txtCheatCode;
};
