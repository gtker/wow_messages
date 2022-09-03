use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_complete.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_COMPLETE = 0x0198 {
///     u32 quest_id;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_COMPLETE {
    pub quest_id: u32,
}

impl crate::Message for SMSG_QUESTUPDATE_COMPLETE {
    const OPCODE: u32 = 0x0198;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            quest_id,
        })
    }

}
impl ServerMessage for SMSG_QUESTUPDATE_COMPLETE {}

