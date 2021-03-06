use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Emote;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_emote.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_emote.wowm#L3):
/// ```text
/// cmsg CMSG_EMOTE = 0x0102 {
///     Emote emote;
/// }
/// ```
pub struct CMSG_EMOTE {
    pub emote: Emote,
}

impl ClientMessage for CMSG_EMOTE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0102;

    fn client_size(&self) -> u16 {
        10
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            emote,
        })
    }

}

