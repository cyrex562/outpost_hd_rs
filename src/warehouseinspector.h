// #pragma once

// #include<window.h>
// #include<button.h>


class Warehouse;


/**
* Implements a Factory Production dialog interface.
*/
class WarehouseInspector : public Window
{
public:
	WarehouseInspector();

	void warehouse(Warehouse* w);

	void hide() override;
	void update() override;

private:
	void onClose();

	Warehouse* mWarehouse = nullptr;
	Button btnClose;
};
