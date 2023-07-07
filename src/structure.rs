use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::color;
use crate::color::Color;
use crate::storable_resources::StorableResources;
use crate::structure_type::StructureType;

pub enum StructureState {
    UnderConstruction,
    Operational,
    Idle,
    Disabled,
    Destroyed
}

pub enum StructureClass
{
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
Warehouse
}


#[derive(Default,Debug,Clone)]
pub struct Structure
{
    pub map_object: MapObject,
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
            map_object: MapObject::new(structure_name(id), StructureCatalogue::get_type(id).sprite_path, init_act),
            structure_type: StructureCatalogue::get_type(id),
            structure_id: id,
            structure_class,
            ..Default::default()
        }
    }

    pub fn disable(&mut self, reason: DisabledReason) {
        self.map_object.sprite().pause();
        self.map_object.sprite().color(Color::new(255,0,0,185));
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
        if  self.structure_state != StructureState::Disabled {
            return;
        }

        self.map_object.sprite().pause();
        self.map_object.sprite().color(Color::new(255,255,255,185));
        self.disabled_reason = DisabledReason::None;
        self.idle_reason = reason;
        self.structure_state = StructureState::Idle;
    }

    pub fn force_idle(&mut self, force: bool) {
        if self.disabled() || self.destroyed() {reeturn;}
        if force {
            self.idle(IdleReason::PlayerSet);
            self.forced_idle = true;
        } else {
            self.forced_idle = false;
            self.enable();
        }
    }

    pub fn resources_in(&mut self) -> &StorableResources {
        self.structure_type.operational_cost
    }
}

lazy_static!{
    static ref STRUCTURE_STATE_TRANSLATION: HashMap<StructureState,&'static str> = {
        let mut m = HashMap::new();
        m.insert(StructureState::UnderConstruction, "Under Construction");
        m.insert(StructureState::Operational, "Operational");
        m.insert(StructureState::Idle, "Idle");
        m.insert(StructureState::Disabled, "Disabled");
        m.insert(StructureState::Destroyed, "Destroyed");
        m
    };
}

lazy_static!{
    static ref STRUCTURE_CLASS_TRANSLATION: HashMap<StructureClass,&'static str> = {
        let mut m = HashMap::new();
        m.insert(StructureClass::Command, "Command"),
	m.insert(StructureClass::Communication, "Communication"),
	m.insert(StructureClass::Commercial, "Commercial"),
	m.insert(StructureClass::EnergyProduction, "Energy Production"),
	m.insert(StructureClass::Factory, "Factory"),
	m.insert(StructureClass::FoodProduction, "Food Production"),
	m.insert(StructureClass::Laboratory, "Laboratory"),
	m.insert(StructureClass::Lander, "Lander"),
	m.insert(StructureClass::LifeSupport, "Life Support"),
	m.insert(StructureClass::Maintenance, "Maintenance Facility"),
	m.insert(StructureClass::Mine, "Mine Facility"),
	m.insert(StructureClass::MedicalCenter, "Mine Facility"),
	m.insert(StructureClass::Nursery, "Mine Facility"),
	m.insert(StructureClass::Park, "Park / Reservoir"),
	m.insert(StructureClass::Road, "Road"),
	m.insert(StructureClass::SurfacePolice, "Police"),
	m.insert(StructureClass::UndergroundPolice, "Police"),
	m.insert(StructureClass::RecreationCenter, "Recreation Center"),
	m.insert(StructureClass::Recycling, "Recycling"),
	m.insert(StructureClass::Residence, "Residential"),
	m.insert(StructureClass::RobotCommand, "Robot Command Center"),
	m.insert(StructureClass::Smelter, "Raw Ore Processing"),
	m.insert(StructureClass::Storage, "Storage"),
	m.insert(StructureClass::Tube, "Tube"),
	m.insert(StructureClass::Undefined, "UNDEFINED"),
	m.insert(StructureClass::University, "University"),
	m.insert(StructureClass::Warehouse, "Warehouse")
        m
    };
}

pub const StructureNameTable: [&str;10] = [
    "Not a Structure",
    constants::Agridome,
    constants::AirShaft,
    constants::CargoLander,
    constants::Chap,
    constants::ColonistLander,
    constants::CommandCenter,
    constants::Commercial,
    constants::CommTower,
    constants::FusionReactor,
    constants::HotLaboratory,
    constants::Laboratory,
    constants::MedicalCenter,
    constants::MineFacility,
    constants::MineShaft,
    constants::Nursery,
    constants::Park,
    constants::RecreationCenter,
    constants::RedLightDistrict,
    constants::Residence,
    constants::Road,
    constants::RobotCommand,
    constants::SeedFactory,
    constants::SeedLander,
    constants::SeedPower,
    constants::SeedSmelter,
    constants::Smelter,
    constants::SolarPanel1,
    constants::SolarPlant,
    constants::StorageTanks,
    constants::SurfaceFactory,
    constants::SurfacePolice,
    constants::Tube,
    constants::UndergroundFactory,
    constants::UndergroundPolice,
    constants::University,
    constants::Warehouse,
    constants::Recycling,
    constants::MaintenanceFacility
];

pub fn structure_name(id: StructureID) -> String {
    StructureNameTable[id]
}

pub fn all_structure_classes() -> Vec<StructureClass>
{
   STRUCTURE_CLASS_TRANSLATION.values().clone()
}



