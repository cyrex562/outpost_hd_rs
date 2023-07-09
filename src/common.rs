// enum class StructureState;

use sdl2::rect::Rect;
use crate::color::Color;
use crate::structure::StructureState;

pub enum Difficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
}
// TODO define strings for enum


/**
 * Digger robot digging direction.
 */
pub enum Direction {
    Up,
    Down,
    East,
    West,
    North,
    South,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

pub fn structure_color_table(state: StructureState) -> Color
{
    match state {
        StructureState::UnderConstruction => Color::new(150, 150, 150,100),
        StructureState::Operational => Color::new(0, 185, 0, 255),
        StructureState::Idle => Color::new(0, 185, 0, 100),
        StructureState::Disabled => Color::new(220, 0, 0, 255),
        StructureState::Destroyed => Color::new(220, 0, 0, 255),
    }
}

pub fn structure_text_color_table(state: StructureState) -> Color {
    match state {
        StructureState::UnderConstruction => Color::new(185, 185, 185,100),
        StructureState::Operational => Color::new(0, 185, 0, 255),
        StructureState::Idle => Color::new(0, 185, 0, 100),
        StructureState::Disabled => Color::new(220, 0, 0, 255),
        StructureState::Destroyed => Color::new(220, 0, 0, 255),
    }
}

pub fn tile_index_translation(tt: TerrainType) -> String {
    match tt {
        TerrainType::Dozed => "dozed".to_string(),
        TerrainType::Clear => "clear".to_string(),
        TerrainType::Rough => "rough".to_string(),
        TerrainType::Difficult => "difficult".to_string(),
        TerrainType::Impassable => "impassable".to_string(),
    }
}

pub fn mine_yield_translation(my: MineProductionRate) -> String {
    match my {
        MineProductionRate::Low => "low".to_string(),
        MineProductionRate::Medium => "medium".to_string(),
        MineProductionRate::High => "high".to_string(),
    }
}

pub fn disabled_reason_table(dr: DisabledReason) -> String {
    match dr {
        DisabledReason::None => "None".to_string(),
        DisabledReason::Chap => "Chap".to_string(),
        DisabledReason::Disconnected => "Disconnected".to_string(),
        DisabledReason::Energy => "Energy".to_string(),
        DisabledReason::Population => "Population".to_string(),
        DisabledReason::RefinedResources => "RefinedResources".to_string(),
        DisabledReason::StructuralIntegrity => "StructuralIntegrity".to_string(),
    }
}

pub fn idle_reason_table(ir: IdleReason) -> String {
    match ir {
        IdleReason::FactoryInsufficientResources => "FactoryInsufficientResources".to_string(),
        IdleReason::FactoryInsufficientRobotCommandCapacity => "FactoryInsufficientRobotCommandCapacity".to_string(),
        IdleReason::FactoryInsufficientWarehouseSpace => "FactoryInsufficientWarehouseSpace".to_string(),
        IdleReason::FactoryProductionComplete => "FactoryProductionComplete".to_string(),
        IdleReason::InternalStorageFull => "InternalStorageFull".to_string(),
        IdleReason::MineExhausted => "MineExhausted".to_string(),
        IdleReason::MineInactive => "MineInactive".to_string(),
        IdleReason::InsufficientLuxuryProduct => "InsufficientLuxuryProduct".to_string(),
        IdleReason::None => "None".to_string(),
        IdleReason::PlayerSet => "PlayerSet".to_string(),
    }
}

pub const MoraleStringTable: [&str; 16] = [
    "Terrible", "Poor", "Fair", "Good", "Excellent", "Morale is", "Births", "Deaths", "No active food production", "parks & arboretums", "recreational facilities", "luxury availability", "residental over capacity", "biowaste overflowing", "structures disabled", "structures destroyed"
];

pub fn morale_string(index: usize) -> String {
    MoraleStringTable[index].to_string()
}

pub fn morale_string_from_enum(morale: Morale) -> String {
    match morale {
        Morale::Terrible => "Terrible".to_string(),
        Morale::Poor => "Poor".to_string(),
        Morale::Fair => "Fair".to_string(),
        Morale::Good => "Good".to_string(),
        Morale::Excellent => "Excellent".to_string(),
        Morale::MoraleIs => "Morale is".to_string(),
        Morale::Births => "Births".to_string(),
        Morale::Deaths => "Deaths".to_string(),
        Morale::NoActiveFoodProduction => "No active food production".to_string(),
        Morale::ParksAndArboretums => "parks & arboretums".to_string(),
        Morale::RecreationalFacilities => "recreational facilities".to_string(),
        Morale::LuxuryAvailability => "luxury availability".to_string(),
        Morale::ResidentalOverCapacity => "residental over capacity".to_string(),
        Morale::BiowasteOverflowing => "biowaste overflowing".to_string(),
        Morale::StructuresDisabled => "structures disabled".to_string(),
        Morale::StructuresDestroyed => "structures destroyed".to_string(),
    }
}

pub const ResourceNamesRefined: [&str;4] = [
    "Common Metals",
    "Common Minerals",
    "Rare Metals",
    "Rare Minerals"
];

pub const ResourceNamesOre: [&str;4] = [
    "Common Metals Ore",
    "Common Minerals Ore",
    "Rare Metals Ore",
    "Rare Minerals Ore"
];

pub const ResourceImageRectsRefined: [Rect<i32>;4] = [
    Rect
]

/**
 * Terrain type enumeration
 */
pub enum TerrainType {
    Dozed,
    Clear,
    Rough,
    Difficult,
    Impassable,
}


pub enum MineProductionRate {
    Low,
    Medium,
    High,
}


pub enum DisabledReason {
    None,
    /**< Not Disabled, default reason. */

    Chap,
    /**< Requires atmosphere, no atmosphere available. */
    Disconnected,
    /**< Not connected to Command Center */
    Energy,
    /**< Not enough Energy to operate. */
    Population,
    /**< Insufficient workers or scientists (or both) */
    RefinedResources,
    /**< Insufficient mined and refined resources */
    StructuralIntegrity,
    /**< Structural integrity out of operating tolerances (damaged structure) */
}


pub enum IdleReason {
    None,

    PlayerSet,
    InternalStorageFull,
    FactoryProductionComplete,
    FactoryInsufficientResources,
    FactoryInsufficientRobotCommandCapacity,
    FactoryInsufficientWarehouseSpace,
    MineExhausted,
    MineInactive,
    InsufficientLuxuryProduct,
}

/**
 * Connector Direction.
 *
 * \note	CONNECTOR_INTERSECTION is explicitely set to '1' to prevent
 *    		breaking changes with save files.
 */
pub enum ConnectorDir {
    CONNECTOR_INTERSECTION = 1,
    CONNECTOR_RIGHT,
    CONNECTOR_LEFT,
    CONNECTOR_VERTICAL, // Functions as an intersection
}


/**
 * Unique identifier code for each structure.
 *
 * \note	Each individual structure is identified using a SID_ code as opposed
 *    		the structure Class code which is used to group like structures into
 *    		lists for structure updates.
 */
pub enum StructureID {
    SID_NONE,

    SID_AGRIDOME,
    SID_AIR_SHAFT,
    SID_CARGO_LANDER,
    SID_CHAP,
    SID_COLONIST_LANDER,
    SID_COMMAND_CENTER,
    SID_COMMERCIAL,
    SID_COMM_TOWER,
    SID_FUSION_REACTOR,
    SID_HOT_LABORATORY,
    SID_LABORATORY,
    SID_MEDICAL_CENTER,
    SID_MINE_FACILITY,
    SID_MINE_SHAFT,
    SID_NURSERY,
    SID_PARK,
    SID_RECREATION_CENTER,
    SID_RED_LIGHT_DISTRICT,
    SID_RESIDENCE,
    SID_ROAD,
    SID_ROBOT_COMMAND,
    SID_SEED_FACTORY,
    SID_SEED_LANDER,
    SID_SEED_POWER,
    SID_SEED_SMELTER,
    SID_SMELTER,
    SID_SOLAR_PANEL1,
    SID_SOLAR_PLANT,
    SID_STORAGE_TANKS,
    SID_SURFACE_FACTORY,
    SID_SURFACE_POLICE,
    SID_TUBE,
    SID_UNDERGROUND_FACTORY,
    SID_UNDERGROUND_POLICE,
    SID_UNIVERSITY,
    SID_WAREHOUSE,
    SID_RECYCLING,
    SID_MAINTENANCE_FACILITY,

    SID_COUNT,
}


/**
 * Factory Product enumeration
 *
 * \note	Products are arranged to match the order in which they appear
 *    		in the icon atlas (data/ui/factory.png). In order to allow
 *    		for easy future additions, the icons are grouped into two
 *    		sets of 32 icons. The first 32 are for above ground products,
 *    		the second set for underground products.
 *
 *    		To easily map to icons in the atlas, padding entries with a
 *    		'reserved' naming convention have been added. These can be
 *    		replaced as additional products are added.
 *
 * \remark	ASSUMPTION: Factories will never have more than 32 individual
 *    		products that they can produce.
 */
pub enum ProductType {
    PRODUCT_NONE = -1,

    // =====================================
    // = SURFACE FACTORIES
    // =====================================
    PRODUCT_DIGGER,
    PRODUCT_DOZER,
    PRODUCT_MINER,
    PRODUCT_EXPLORER,
    PRODUCT_TRUCK,

    PRODUCT_RESERVED_AG_05,
    PRODUCT_RESERVED_AG_06,
    PRODUCT_RESERVED_AG_07,

    PRODUCT_RESERVED_AG_08,
    PRODUCT_MAINTENANCE_PARTS,

    PRODUCT_RESERVED_AG_10,
    PRODUCT_RESERVED_AG_11,
    PRODUCT_RESERVED_AG_12,
    PRODUCT_RESERVED_AG_13,
    PRODUCT_RESERVED_AG_14,
    PRODUCT_RESERVED_AG_15,

    PRODUCT_RESERVED_AG_16,
    PRODUCT_RESERVED_AG_17,
    PRODUCT_RESERVED_AG_18,
    PRODUCT_RESERVED_AG_19,
    PRODUCT_RESERVED_AG_20,
    PRODUCT_RESERVED_AG_21,
    PRODUCT_RESERVED_AG_22,
    PRODUCT_RESERVED_AG_23,

    PRODUCT_RESERVED_AG_24,
    PRODUCT_RESERVED_AG_25,
    PRODUCT_RESERVED_AG_26,
    PRODUCT_RESERVED_AG_27,
    PRODUCT_RESERVED_AG_28,
    PRODUCT_RESERVED_AG_29,
    PRODUCT_RESERVED_AG_30,
    PRODUCT_RESERVED_AG_31,

    // =====================================
    // = UNDERGROUND FACTORIES
    // =====================================
    PRODUCT_CLOTHING,
    PRODUCT_MEDICINE,
    PRODUCT_RESERVED_UG_34,
    PRODUCT_RESERVED_UG_35,
    PRODUCT_RESERVED_UG_36,
    PRODUCT_RESERVED_UG_37,
    PRODUCT_RESERVED_UG_38,
    PRODUCT_RESERVED_UG_39,

    PRODUCT_RESERVED_UG_40,
    PRODUCT_RESERVED_UG_41,
    PRODUCT_RESERVED_UG_42,
    PRODUCT_RESERVED_UG_43,
    PRODUCT_RESERVED_UG_44,
    PRODUCT_RESERVED_UG_45,
    PRODUCT_RESERVED_UG_46,
    PRODUCT_RESERVED_UG_47,

    PRODUCT_RESERVED_UG_48,
    PRODUCT_RESERVED_UG_49,
    PRODUCT_RESERVED_UG_50,
    PRODUCT_RESERVED_UG_51,
    PRODUCT_RESERVED_UG_52,
    PRODUCT_RESERVED_UG_53,
    PRODUCT_RESERVED_UG_54,
    PRODUCT_RESERVED_UG_55,

    PRODUCT_RESERVED_UG_56,
    PRODUCT_RESERVED_UG_57,
    PRODUCT_RESERVED_UG_58,
    PRODUCT_RESERVED_UG_59,
    PRODUCT_RESERVED_UG_60,
    PRODUCT_RESERVED_UG_61,
    PRODUCT_RESERVED_UG_62,
    PRODUCT_RESERVED_UG_63,

    PRODUCT_COUNT,
}


pub enum Morale {
    Terrible,
    Poor,
    Fair,
    Good,
    Excellent,

    Description,
    Births,
    Deaths,
    NoFoodProduction,
    Parks,
    Recreation,
    Commercial,
    ResidentialOverflow,
    BiowasteOverflow,
    StructuresDisabled,
    StructuresDestroyed,
}


// extern const std::map<TerrainType, std::string> TILE_INDEX_TRANSLATION;
// extern const std::map<MineProductionRate, std::string> MINE_YIELD_TRANSLATION;
//
// extern const std::array<std::string, 4> ResourceNamesRefined;
// extern const std::array<std::string, 4> ResourceNamesOre;
//
// extern const std::array<NAS2D::Rectangle<int>, 4> ResourceImageRectsRefined;
// extern const std::array<NAS2D::Rectangle<int>, 4> ResourceImageRectsOre;

// extern const std::map<std::array<bool, 4>, std::string> IntersectionPatternTable;


// const auto formatDiff = [](int diff)
// {
// return ((diff > 0) ? "+" : "") + std::to_string(diff);
// };
