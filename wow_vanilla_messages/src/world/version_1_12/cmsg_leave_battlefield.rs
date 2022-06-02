use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

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
    /// # Comment
    ///
    /// cmangos/vmangos/mangoszero: BattleGroundTypeId-1 ? - Classic Only
    pub battle_ground_type_id: u8,
    pub unknown2: u16,
}

impl ClientMessage for CMSG_LEAVE_BATTLEFIELD {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // battle_ground_type_id: u8
        w.write_all(&self.battle_ground_type_id.to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02e1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

