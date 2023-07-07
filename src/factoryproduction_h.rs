// #pragma once

// #include<window.h>
// #include<button.h>
// #include<checkbox.h>
// #include"icongrid.h"

// #include"uiconstants.h"
// #include"common.h"
// #include"productioncost.h"


class Factory;


/**
 * Implements a Factory Production dialog interface.
 */
class FactoryProduction : public Window
{
public:
	FactoryProduction();

	void factory(Factory* newFactory);
	Factory* factory() { return mFactory; }

	void hide() override;

	void update() override;

private:
	void onOkay();
	void onCancel();
	void onClearSelection();
	void onCheckBoxIdleChange();
	void onApply();

	void clearProduct();

	void onProductSelectionChange(const IconGrid::Item*);

	Factory* mFactory = nullptr;

	ProductType mProduct = ProductType::PRODUCT_NONE;
	ProductionCost mProductCost;

	IconGrid mProductGrid{"ui/factory.png", 32, constants::MarginTight};

	Button btnOkay{"Okay", {this, &FactoryProduction::onOkay}};
	Button btnCancel{"Cancel", {this, &FactoryProduction::onCancel}};
	Button btnClearSelection{"Clear Selection", {this, &FactoryProduction::onClearSelection}};
	Button btnApply{"Apply", {this, &FactoryProduction::onApply}};

	CheckBox chkIdle{"Idle"};
};
