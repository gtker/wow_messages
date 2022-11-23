mod tbc;
mod vanilla;
mod wrath;

use crate::base_printer::position::RawPosition;

pub(crate) use tbc::TBC_AREA_TRIGGERS;
pub(crate) use vanilla::VANILLA_AREA_TRIGGERS;
pub(crate) use wrath::WRATH_AREA_TRIGGERS;

pub(crate) enum AreaTrigger {
    Circle {
        position: RawPosition,
        radius: f32,
    },
    Square {
        position: RawPosition,
        /// Size along the x axis.
        length: f32,
        /// Size along the y axis.
        width: f32,
        /// Size along the z axis.
        height: f32,
        /// Rotation about the Z axis
        yaw: f32,
    },
}

impl AreaTrigger {
    pub const fn position(&self) -> &RawPosition {
        match self {
            AreaTrigger::Circle { position, .. } => position,
            AreaTrigger::Square { position, .. } => position,
        }
    }
}
