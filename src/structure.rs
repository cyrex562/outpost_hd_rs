use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::agridome::Agridome;
use crate::color;
use crate::color::Color;
use crate::common::{ConnectorDir, DisabledReason, IdleReason, StructureID};
use crate::common::DisabledReason::Chap;
use crate::map_object::MapObject;
use crate::population_requirements::PopulationRequirements;
use crate::storable_resources::StorableResources;
use crate::strings::{STR_Agridome, STR_AirShaft, STR_CargoLander, STR_Chap, STR_ColonistLander, STR_CommandCenter, STR_Commercial, STR_CommTower, STR_FusionReactor, STR_HotLaboratory, STR_Laboratory, STR_MaintenanceFacility, STR_MedicalCenter, STR_MineFacility, STR_MineShaft, STR_Nursery, STR_Park, STR_RecreationCenter, STR_Recycling, STR_RedLightDistrict, STR_Residence, STR_Road, STR_RobotCommand, STR_SeedFactory, STR_SeedLander, STR_SeedPower, STR_SeedSmelter, STR_Smelter, STR_SolarPanel1, STR_SolarPlant, STR_StorageTanks, STR_StructureStateConstruction, STR_StructureStateDestroyed, STR_StructureStateOperational, STR_SurfaceFactory, STR_SurfacePolice, STR_Tube, STR_UndergroundFactory, STR_UndergroundPolice, STR_University, STR_Warehouse};
use crate::structure_type::StructureType;

pub enum StructureState {
    UnderConstruction,
    Operational,
    Idle,
    Disabled,
    Destroyed,
}

pub enum StructureClass {
    Command,
    Communication,
    Commercial,
    EnergyProduction,
    Factory,
    FoodProduction,
    Laboratory,
    Lander,
    LifeSupport,
    Maintenance,
    Mine,
    MedicalCenter,
    Nursery,
    Park,
    Road,
    SurfacePolice,
    UndergroundPolice,
    RecreationCenter,
    Recycling,
    Residence,
    RobotCommand,
    Smelter,
    Storage,
    Tube,
    Undefined,
    University,
    Warehouse,
}


#[derive(Default, Debug, Clone)]
pub struct Structure<'a> {
    pub map_object: MapObject<'a>,
    pub structure_type: StructureType,
    pub structure_id: StructureID,
    pub age: i32,
    pub crime_rate: i32,
    pub integrity: i32,
    pub structure_state: StructureState,
    pub structure_class: StructureClass,
    pub connector_direction: ConnectorDir,
    pub population_available: PopulationRequirements,
    pub production_pool: StorableResources,
    pub storage_pool: StorableResources,
    pub disabled_reason: DisabledReason,
    pub idle_reason: IdleReason,
    pub connected: bool,
    pub forced_idle: bool,
}

impl Structure {
    pub fn new(structure_class: StructureClass, id: StructureID, initial_action: Option<&str>) -> Self {
        let init_act = initial_action.unwrap_or(constants::StructureStateConstruction);
        Self {
            map_object: MapObject::new(&structure_name(id),
                                       StructureCatalogue::get_type(id.clone()).sprite_path,
                                       init_act),
            structure_type: StructureCatalogue::get_type(id.clone()),
            structure_id: id.clone(),
            structure_class,
            ..Default::default()
        }
    }

    pub fn disable(&mut self, reason: DisabledReason) {
        self.map_object.sprite().pause();
        self.map_object.sprite().color(Color::new(255, 0, 0, 185));
        self.structure_state = StructureState::Disabled;
        self.disabled_reason = reason;
        self.idle_reason = IdleReason::None;
        self.disabled_state_set();
    }

    pub fn enable(&mut self) {
        if self.force_idle(false) {
            self.idle(IdleReason::PlayerSet);
            return;
        }

        self.map_object.sprite().resume();
        self.map_object.sprite().color(color::White);
        self.structure_state = StructureState::Operational;
        self.disabled_reason = DisabledReason::None;
        self.idle_reason = Idlereason::None;
    }

    pub fn idle(&mut self, reason: IdleReason) {
        self.force_idle(false);
        if self.structure_state != StructureState::Disabled {
            return;
        }

        self.map_object.sprite().pause();
        self.map_object.sprite().color(Color::new(255, 255, 255, 185));
        self.disabled_reason = DisabledReason::None;
        self.idle_reason = reason;
        self.structure_state = StructureState::Idle;
    }

    pub fn force_idle(&mut self, force: bool) {
        if self.disabled() || self.destroyed() { reeturn; }
        if force {
            self.idle(IdleReason::PlayerSet);
            self.forced_idle = true;
        } else {
            self.forced_idle = false;
            self.enable();
        }
    }

    pub fn resources_in(&mut self) -> &StorableResources {
        &self.structure_type.operational_cost
    }

    pub fn population_requirements(&mut self) -> &PopulationRequirements {
        &self.structure_type.population_requirements
    }

    pub fn state_description(&mut self) -> String {
        STRUCTURE_STATE_TRANSLATION(self.structure_state.clone()).to_string()
    }

    pub fn state_descritpion2(&mut self, state: StructureState) -> String {
        STRUCTURE_STATE_TRANSLATION(state).to_string()
    }

    pub fn class_description(&mut self) -> String {
        STRUCTURE_CLASS_TRANSLATION(self.structure_class.clone()).to_string()
    }

    pub fn class_description2(&mut self, structure_class: StructureClass) -> String {
        STRUCTURE_CLASS_TRANSLATION(structure_class).to_string()
    }

    pub fn turns_to_build(&mut self) -> i32 {
        self.structure_type.turns_to_build
    }

    pub fn max_age(&mut self) -> i32 {
        self.structure_type.max_age
    }

    pub fn energy_requirement(&mut self) -> i32 {
        self.structure_type.energy_requirement
    }

    pub fn storage_capacity(&mut self) -> i32 {
        self.structure_type.ore_storage_capacity
    }

    pub fn has_crime(&mut self) -> bool {
        self.structure_type.is_crime_target
    }

    pub fn integrity_decay_rate(&mut self) -> i32 {
        self.structure_type.integrity_decay_rate
    }

    pub fn requires_chap(&mut self) -> bool {
        self.structure_type.requires_chap
    }

    pub fn self_sustained(&mut self) -> bool {
        self.structure_type.self_sustained
    }

    pub fn repairable(&mut self) -> bool {
        self.structure_type.repairable && (self.structure_state != StructureState::Destroyed)
    }

    pub fn activate(&mut self) {
        self.sprite().play(STR_StructureStateOperational);
        self.enable();
        self.activated();
    }

    pub fn rebuild(&mut self) {
        self.sprite().play(STR_StructureStateConstruction);
        self.state(StructureState::UnderConstruction);
        self.age(1)
    }

    pub fn update(&mut self) {
        if (self.destroyed()) { return; }
        self.increment_age();
        self.update_integrity_decay();
    }

    pub fn increment_age(&mut self) {
        self.age += 1;
        if self.age == self.turns_to_build() {
            self.activate();
        } else if (self.max_age() == 0) {
            // TODO
        } else if (self.age == self.max_age()) {
            self.destroy();
        }
    }

    pub fn update_integrity_decay(&mut self) {
        if self.structure_state == StructureState::UnderConstruction {
            return;
        }

        self.integrity = i32::clamp(self.integrity - self.integrity_decay_rate(), 0, self.integrity);
        if self.integrity <= 35 && self.disabled_reason() == false {
            self.disable(DisabledReason::StructuralIntegrity);
        } else if self.integrity <= 20 && self.destroyed() == false {
            if random_number.generate(0, 1000) < 100 {
                self.destroy();
            }
        } else if self.integrity <= 0 {
            self.integrity = 0;
            self.destroy();
        }
    }

    pub fn destroy(&mut self) {
        self.sprite().play(STR_StructureStateDestroyed);
        self.structure_state = StructureState::Destroyed;
    }

    pub fn forced_state_change(&mut self, structure_state: StructureState, disabled_reason: DisabledReason, idle_reason: IdleReason) {
        if self.age >= self.turns_to_build() {
            self.sprite().play(STR_StructureStateOperational);
        }
        if self.structure_state == StructureState::Operational { self.enable(); } else if self.structure_state == StructureState::Idle { self.idle(idle_reason); } else if self.structure_state == StructureState::Disabled { self.disable(disabled_reason); } else if self.structure_state == StructureState::Destroyed { self.destroy(); } else if self.structure_state == StructureState::UnderConstruction { self.structure_state = StructureState::UnderConstruction }
    }

    pub fn die(&mut self) {
        self.map_object.die();
        panic!("Structure died");
    }

    pub fn crime_rate(&mut self, crime_rate: i32) {
        self.crime_rate = i32::clamp(crime_rate, 0, 100);
    }

    pub fn increase_crime_rate(&mut self, delta_crime_rate: i32) {
        self.crime_rate = i32::clamp(self.crime_rate + delta_crime_rate, 0, 100);
    }

    pub fn integrity(&mut self, integrity: i32) {
        self.integrity = integrity;
    }

    // TODO
    // pub fn get_data_dict(&mut self) -> HashMap<>
    // NAS2D::Dictionary Structure::getDataDict() const
    // {
    // 	NAS2D::Dictionary dictionary =
    // 	{{
    // 		{"age", mAge},
    // 		{"state", static_cast<int>(mStructureState)},
    // 		{"forced_idle", mForcedIdle},
    // 		{"disabled_reason", static_cast<int>(mDisabledReason)},
    // 		{"idle_reason", static_cast<int>(mIdleReason)},
    // 		{"type", mStructureId},
    // 		{"direction", mConnectorDirection},
    // 		{"integrity", mIntegrity},
    // 		{"pop0", mPopulationAvailable.workers},
    // 		{"pop1", mPopulationAvailable.scientists},
    // 	}};
    //
    // 	if (hasCrime())
    // 	{
    // 		dictionary.set("crime_rate", mCrimeRate);
    // 	}
    //
    // 	return dictionary;
    // }
}

pub fn STRUCTURE_STATE_TRANSLATION(struct_state: StructureState) -> &'static str {
    match struct_state {
        StructureState::UnderConstruction => "Under Construction",
        StructureState::Operational => "Operational",
        StructureState::Idle => "Idle",
        StructureState::Disabled => "Disabled",
        StructureState::Destroyed => "Destroyed",
    }
}

pub fn STRUCTURE_CLASS_TRANSLATION(struct_class: StructureClass) -> &'static str {
    match struct_class {
        StructureClass::Command => "Command",
        StructureClass::Communication => "Communication",
        StructureClass::Commercial => "Commercial",
        StructureClass::EnergyProduction => "Energy Production",
        StructureClass::Factory => "Factory",
        StructureClass::FoodProduction => "Food Production",
        StructureClass::Laboratory => "Laboratory",
        StructureClass::Lander => "Lander",
        StructureClass::LifeSupport => "Life Support",
        StructureClass::Maintenance => "Maintenance Facility",
        StructureClass::Mine => "Mine Facility",
        StructureClass::MedicalCenter => "Medical Center",
        StructureClass::Nursery => "Nursery",
        StructureClass::Park => "Park",
        StructureClass::Road => "Road",
        StructureClass::SurfacePolice => "Surface Police",
        StructureClass::UndergroundPolice => "Underground Police",
        StructureClass::RecreationCenter => "Recreation Center",
        StructureClass::Recycling => "Recycling",
        StructureClass::Residence => "Residence",
        StructureClass::RobotCommand => "Robot Command",
        StructureClass::Smelter => "Smelter",
        StructureClass::Storage => "Storage",
        StructureClass::Tube => "Tube",
        StructureClass::Undefined => "Undefined",
        StructureClass::University => "University",
        StructureClass::Warehouse => "Warehouse",
    }
}


pub const StructureNameTable: [&str; 39] = [
    "Not a Structure",
    STR_Agridome,
    STR_AirShaft,
    STR_CargoLander,
    STR_Chap,
    STR_ColonistLander,
    STR_CommandCenter,
    STR_Commercial,
    STR_CommTower,
    STR_FusionReactor,
    STR_HotLaboratory,
    STR_Laboratory,
    STR_MedicalCenter,
    STR_MineFacility,
    STR_MineShaft,
    STR_Nursery,
    STR_Park,
    STR_RecreationCenter,
    STR_RedLightDistrict,
    STR_Residence,
    STR_Road,
    STR_RobotCommand,
    STR_SeedFactory,
    STR_SeedLander,
    STR_SeedPower,
    STR_SeedSmelter,
    STR_Smelter,
    STR_SolarPanel1,
    STR_SolarPlant,
    STR_StorageTanks,
    STR_SurfaceFactory,
    STR_SurfacePolice,
    STR_Tube,
    STR_UndergroundFactory,
    STR_UndergroundPolice,
    STR_University,
    STR_Warehouse,
    STR_Recycling,
    STR_MaintenanceFacility
];

pub fn structure_name(id: StructureID) -> String {
    StructureNameTable[id]
}

pub fn all_structure_classes() -> Vec<StructureClass> {
    STRUCTURE_CLASS_TRANSLATION.values().clone()
}



