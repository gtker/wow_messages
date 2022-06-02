use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm#L3):
/// ```text
/// cmsg CMSG_SET_FACTION_INACTIVE = 0x0317 {
///     u32 reputation_list_id;
///     u8 inactive;
/// }
/// ```
pub struct CMSG_SET_FACTION_INACTIVE {
    pub reputation_list_id: u32,
    pub inactive: u8,
}

impl ClientMessage for CMSG_SET_FACTION_INACTIVE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // inactive: u8
        w.write_all(&self.inactive.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0317;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // inactive: u8
        let inactive = crate::util::read_u8_le(r)?;

        Ok(Self {
            reputation_list_id,
            inactive,
        })
    }

}

