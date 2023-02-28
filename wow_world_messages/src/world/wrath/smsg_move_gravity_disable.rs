use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_gravity_disable.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_gravity_disable.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_GRAVITY_DISABLE = 0x04CE {
///     PackedGuid unit;
///     u32 movement_counter;
/// }
/// ```
pub struct SMSG_MOVE_GRAVITY_DISABLE {
    pub unit: Guid,
    pub movement_counter: u32,
}

impl crate::Message for SMSG_MOVE_GRAVITY_DISABLE {
    const OPCODE: u32 = 0x04ce;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04CE, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unit,
            movement_counter,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOVE_GRAVITY_DISABLE {}

impl SMSG_MOVE_GRAVITY_DISABLE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + 4 // movement_counter: u32
    }
}

