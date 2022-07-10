use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_accept_quest.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_accept_quest.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_ACCEPT_QUEST = 0x0189 {
///     Guid guid;
///     u32 quest_id;
/// }
/// ```
pub struct CMSG_QUESTGIVER_ACCEPT_QUEST {
    pub guid: Guid,
    pub quest_id: u32,
}

impl ClientMessage for CMSG_QUESTGIVER_ACCEPT_QUEST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0189;

    fn client_size(&self) -> u16 {
        18
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            quest_id,
        })
    }

}

