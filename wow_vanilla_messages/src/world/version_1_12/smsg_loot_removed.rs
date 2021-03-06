use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Notify a looting player that an item has been taken.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_removed.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_removed.wowm#L5):
/// ```text
/// smsg SMSG_LOOT_REMOVED = 0x0162 {
///     u8 slot;
/// }
/// ```
pub struct SMSG_LOOT_REMOVED {
    pub slot: u8,
}

impl ServerMessage for SMSG_LOOT_REMOVED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0162;

    fn server_size(&self) -> u16 {
        5
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot,
        })
    }

}

