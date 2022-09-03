use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm#L3):
/// ```text
/// cmsg CMSG_BUY_STABLE_SLOT = 0x0272 {
///     Guid npc_guid;
/// }
/// ```
pub struct CMSG_BUY_STABLE_SLOT {
    pub npc_guid: Guid,
}

impl crate::Message for CMSG_BUY_STABLE_SLOT {
    const OPCODE: u32 = 0x0272;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc_guid: Guid
        w.write_all(&self.npc_guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        Ok(Self {
            npc_guid,
        })
    }

}
impl ClientMessage for CMSG_BUY_STABLE_SLOT {}

