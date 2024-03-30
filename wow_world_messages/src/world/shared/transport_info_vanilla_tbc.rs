use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement.wowm#L3):
/// ```text
/// struct TransportInfo {
///     PackedGuid guid;
///     Vector3d position;
///     f32 orientation;
///     u32 timestamp;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct TransportInfo {
    pub guid: Guid,
    pub position: Vector3d,
    pub orientation: f32,
    pub timestamp: u32,
}

impl TransportInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // position: Vector3d
        crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        Ok(())
    }
}

impl TransportInfo {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            position,
            orientation,
            timestamp,
        })
    }

}

impl TransportInfo {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 12 // position: Vector3d
        + 4 // orientation: f32
        + 4 // timestamp: u32
    }
}

