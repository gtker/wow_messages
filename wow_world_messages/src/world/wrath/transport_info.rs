use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Vector4d;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L3):
/// ```text
/// struct TransportInfo {
///     PackedGuid guid;
///     Vector4d position;
///     u32 timestamp;
///     u8 seat;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct TransportInfo {
    pub guid: Guid,
    pub position: Vector4d,
    pub timestamp: u32,
    pub seat: u8,
}

impl TransportInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // position: Vector4d
        crate::util::vanilla_tbc_wrath_vector4d_write_into_vec(&self.position, &mut w)?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // seat: u8
        w.write_all(&self.seat.to_le_bytes())?;

        Ok(())
    }
}

impl TransportInfo {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // position: Vector4d
        let position = crate::util::vanilla_tbc_wrath_vector4d_read(&mut r)?;

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(&mut r)?;

        // seat: u8
        let seat = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            guid,
            position,
            timestamp,
            seat,
        })
    }

}

impl TransportInfo {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 16 // position: Vector4d
        + 4 // timestamp: u32
        + 1 // seat: u8
    }
}

