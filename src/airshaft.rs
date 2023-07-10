use crate::common::{ConnectorDir, DisabledReason, IdleReason, StructureID};
use crate::strings::{STR_StructureStateOperational, STR_StructureStateOperationalUg};
use crate::structure::{Structure, StructureClass, StructureState};
use crate::structure_type::StructureType;

pub struct Airshaft<'a>
{
    pub structure: Structure<'a>,
    pub is_underground: bool,
}

impl Airshaft
{
    pub fn new() -> Self {
        let mut out = Self {
            structure: Default::default(),
            is_underground: false,
        };
        out.structure.structure_type = StructureType::AirShaft;
        out.structure.structure_class = StructureClass::AirShaft;
        out.structure.id = StructureID::SID_AIR_SHAFT;
        out.structure.connector_direction = ConnectorDir::CONNECTOR_VERTICAL;
        out.structure.structure_state = StructureState::Operational;
        out
    }

    pub fn ug(&mut self) {
        self.structure.map_object.sprite.play(STR_StructureStateOperationalUg);
        self.is_underground = true;
    }

    pub fn forced_state_change(&mut self, state: StructureState, disabled_reason: DisabledReason, idle_reason: IdleReason) {
        if self.is_underground { return;}
        self.structure.structure_state = state;
        self.structure.disabled_reason = disabled_reason;
        self.structure.idle_reason = idle_reason;
    }
}