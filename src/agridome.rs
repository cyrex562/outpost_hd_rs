// #pragma once

// #include"foodproduction.h"

// #include"strings.h"

// #include<algorithm>


use crate::common::{IdleReason, StructureID};
use crate::foodproduction::FoodProduction;
use crate::structure::StructureClass;

pub const AGRIDOME_CAPACITY: i32 = 1000;
pub const AGRIDOME_BASE_PRODUCUCTION: i32 = 10;
//
// class Agridome : public FoodProduction
// {
// public:
// 	Agridome() : FoodProduction(StructureClass::FoodProduction, StructureID::SID_AGRIDOME)
// 	{
// 	}
//
// protected:
// 	void think() override
// 	{
// 		if (isIdle()) { return; }
//
// 		mFoodLevel = std::clamp(mFoodLevel + calculateProduction(), 0, AGRIDOME_CAPACITY);
//
// 		if (isStorageFull())
// 		{
// 			idle(IdleReason::InternalStorageFull);
// 		}
// 	}
//
// 	void disabledStateSet() override
// 	{
// 		mFoodLevel = 0;
// 	}
//
// 	virtual int foodCapacity() override
// 	{
// 		return AGRIDOME_CAPACITY;
// 	}
//
// private:
// 	virtual int calculateProduction() override
// 	{
// 		if (!operational())
// 		{
// 			return 0;
// 		}
//
// 		return std::min(AGRIDOME_BASE_PRODUCUCTION, AGRIDOME_CAPACITY - mFoodLevel);
// 	}
//
// 	bool isStorageFull()
// 	{
// 		return mFoodLevel >= AGRIDOME_CAPACITY;
// 	}
// };


pub struct Agridome
{
	pub food_production: FoodProduction
}

impl Agridome
{
	pub fn new() -> Self {
		Self {
			food_production: FoodProduction::new(StructureClass::FoodProduction, StructureID::SID_AGRIDOME)
		}
	}

	pub fn think(&self)
	{
		if is_idle() == true {
			return;
		}

		food_level = i32::clamp(food_level + calculate_production(), 0, AGRIDOME_CAPACITY);

		if is_storage_full(){
			idle(IdleReason::InternalStorageFull)
		}
	}

	pub fn disabled_state_set(&mut self) {
		food_level = 0;
	}

	pub fn food_capacity(&mut self) -> i32 {
		AGRIDOME_CAPACITY
	}

	pub fn calculate_production(&mut self) -> i32 {
		if operational() == false {
			return 0;
		}

		return i32::min(AGRIDOME_BASE_PRODUCUCTION, AGRIDOME_CAPACITY - food_level);
	}

	pub fn is_storage_full(&mut self) -> bool {
		food_level >= AGRIDOME_CAPACITY
	}
}