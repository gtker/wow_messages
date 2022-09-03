use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_leave_battlefield.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_leave_battlefield.wowm#L3):
/// ```text
/// cmsg CMSG_LEAVE_BATTLEFIELD = 0x02E1 {
///     u8 unknown1;
///     u8 battle_ground_type_id;
///     u16 unknown2;
/// }
/// ```
pub struct CMSG_LEAVE_BATTLEFIELD {
    pub unknown1: u8,
    /// cmangos/vmangos/mangoszero: BattleGroundTypeId-1 ? - Classic Only
    ///
    pub battle_ground_type_id: u8,
    pub unknown2: u16,
}

impl crate::Message for CMSG_LEAVE_BATTLEFIELD {
    const OPCODE: u32 = 0x02e1;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // battle_ground_type_id: u8
        w.write_all(&self.battle_ground_type_id.to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // battle_ground_type_id: u8
        let battle_ground_type_id = crate::util::read_u8_le(r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        Ok(Self {
            unknown1,
            battle_ground_type_id,
            unknown2,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_LEAVE_BATTLEFIELD {}

