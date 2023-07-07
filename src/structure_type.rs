use crate::population_requirements::PopulationRequirements;
use crate::storable_resources::StorableResources;

#[derive(Default,Debug,Clone)]
pub struct StructureType
{
    pub name: String,
    pub sprite_path: String,
    pub build_cost: StorableResources,
    pub operational_cost: StorableResources,
    pub population_requirements: PopulationRequirements,
    pub priority: i32,
    pub turns_to_build: i32,
    pub max_age: i32,
    pub energy_required: i32,
    pub energy_produced: i32,
    pub food_produced: i32,
    pub food_storage_capacity: i32,
    pub ore_storage_capacity: i32,
    pub integrity_decay_rate: i32,
    pub is_self_sustained: bool,
    pub is_repairable: bool,
    pub is_chap_required: bool,
    pub is_crime_target: bool
}