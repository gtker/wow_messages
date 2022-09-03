use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questlog_swap_quest.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questlog_swap_quest.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTLOG_SWAP_QUEST = 0x0193 {
///     u8 slot1;
///     u8 slot2;
/// }
/// ```
pub struct CMSG_QUESTLOG_SWAP_QUEST {
    pub slot1: u8,
    pub slot2: u8,
}

impl crate::Message for CMSG_QUESTLOG_SWAP_QUEST {
    const OPCODE: u32 = 0x0193;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot1: u8
        w.write_all(&self.slot1.to_le_bytes())?;

        // slot2: u8
        w.write_all(&self.slot2.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // slot1: u8
        let slot1 = crate::util::read_u8_le(r)?;

        // slot2: u8
        let slot2 = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot1,
            slot2,
        })
    }

}
impl ClientMessage for CMSG_QUESTLOG_SWAP_QUEST {}

