// #pragma once

// #include"structure.h"

use crate::structure::{Structure, StructureClass};

/**
* Virtual class for structures whose primary purpose is agricultural production
*
* \note	FoodProduction is an abstract class
*/
// class FoodProduction : public Structure
// {
// public:
// 	FoodProduction(StructureClass structureClass, StructureID id) :
// 		Structure(structureClass, id) {}
//
// 	StringTable createInspectorViewTable() override
// 	{
// 		StringTable stringTable(2, 2);
//
// 		stringTable[{0, 0}].text = "Food Stored:";
// 		stringTable[{1, 0}].text = std::to_string(mFoodLevel) + " / " + std::to_string(foodCapacity());
//
// 		stringTable[{0, 1}].text = "Production Rate:";
// 		stringTable[{1, 1}].text = std::to_string(calculateProduction());
//
// 		return stringTable;
// 	}
//
// 	int foodLevel() const { return mFoodLevel; }
// 	void foodLevel(int level) { mFoodLevel = std::clamp(level, 0, foodCapacity()); }
//
// 	virtual int foodCapacity() = 0;
//
// protected:
// 	virtual int calculateProduction() = 0;
//
// 	int mFoodLevel = 0;
// };

pub struct FoodProduction
{
	pub structure: Structure,
	pub food_level: i32,
}

impl FoodProduction
{
	pub fn new(structure_class: StructureClass, id: StructureId) -> Self {
		Self {
			structure: Structure::new(structure_class, id),
			food_level: 0,
		}
	}

	pub fn create_inspector_view_table(&self) -> StringTable {
		let mut string_table = StringTable::new(2,2);
		string_table[0][0].text = "Food Stored:";
		string_table[1][0].text = format!("{}/{}", self.food_level, self.food_capacity());
		string_table[0][1].text = "Production Rate:";
		string_table[1][1].text = format!("{}", self.calculate_production());
		string_table
	}

	pub fn set_food_level(&mut self, level: i32) {
		self.food_level = i32::clamp(level, 0, self.food_capacity());
	}

	pub fn food_capacity(&mut self) -> i32 {
		todo!()
	}

	pub fn calculate_productino(&mut self) -> i32 {
		todo!()
	}
}