use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::wrath::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L3):
/// ```text
/// struct TransportInfo {
///     PackedGuid guid;
///     Vector3d position;
///     f32 orientation;
///     u32 timestamp;
///     u8 seat;
/// }
/// ```
pub struct TransportInfo {
    pub guid: Guid,
    pub position: Vector3d,
    pub orientation: f32,
    pub timestamp: u32,
    pub seat: u8,
}

impl TransportInfo {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // seat: u8
        w.write_all(&self.seat.to_le_bytes())?;

        Ok(())
    }
}

impl TransportInfo {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;
        // timestamp: u32
        let timestamp = crate::util::read_u32_le(r)?;

        // seat: u8
        let seat = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            position,
            orientation,
            timestamp,
            seat,
        })
    }

}

impl TransportInfo {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 12 // position: Vector3d
        + 4 // orientation: f32
        + 4 // timestamp: u32
        + 1 // seat: u8
    }
}

