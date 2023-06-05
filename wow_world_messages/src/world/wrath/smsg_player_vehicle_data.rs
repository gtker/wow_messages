use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/smsg_player_vehicle_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/smsg_player_vehicle_data.wowm#L1):
/// ```text
/// smsg SMSG_PLAYER_VEHICLE_DATA = 0x04A7 {
///     PackedGuid target;
///     u32 vehicle_id;
/// }
/// ```
pub struct SMSG_PLAYER_VEHICLE_DATA {
    pub target: Guid,
    pub vehicle_id: u32,
}

impl crate::private::Sealed for SMSG_PLAYER_VEHICLE_DATA {}
impl crate::Message for SMSG_PLAYER_VEHICLE_DATA {
    const OPCODE: u32 = 0x04a7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // vehicle_id: u32
        w.write_all(&self.vehicle_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04A7, size: body_size });
        }

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // vehicle_id: u32
        let vehicle_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            target,
            vehicle_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAYER_VEHICLE_DATA {}

impl SMSG_PLAYER_VEHICLE_DATA {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 4 // vehicle_id: u32
    }
}

