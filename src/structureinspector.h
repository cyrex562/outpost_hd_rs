// #pragma once

// #include<window.h>
// #include<button.h>

// #include"renderer.h"
// #include"point.h"


class Structure;
class StringTable;


class StructureInspector : public Window
{
public:
	StructureInspector();

	void structure(Structure* structure);
	Structure* structure() { return mStructure; }

	void update() override;

private:
	void onClose();
	std::string getDisabledReason() const;
	void drawStructureSpecificTable(NAS2D::Point<int> position, NAS2D::Renderer& renderer);
	std::string formatAge() const;

	StringTable buildStringTable() const;

	Button btnClose;
	const NAS2D::Image& mIcons;
	Structure* mStructure = nullptr;
};
