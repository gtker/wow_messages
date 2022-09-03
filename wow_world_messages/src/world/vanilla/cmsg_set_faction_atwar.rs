use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm#L3):
/// ```text
/// cmsg CMSG_SET_FACTION_ATWAR = 0x0125 {
///     u32 reputation_list_id;
///     u8 flags;
/// }
/// ```
pub struct CMSG_SET_FACTION_ATWAR {
    pub reputation_list_id: u32,
    pub flags: u8,
}

impl crate::Message for CMSG_SET_FACTION_ATWAR {
    const OPCODE: u32 = 0x0125;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        Ok(Self {
            reputation_list_id,
            flags,
        })
    }

}
impl ClientMessage for CMSG_SET_FACTION_ATWAR {}

