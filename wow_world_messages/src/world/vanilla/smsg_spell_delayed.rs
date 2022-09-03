use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_delayed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_delayed.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_DELAYED = 0x01E2 {
///     Guid guid;
///     u32 delay_time;
/// }
/// ```
pub struct SMSG_SPELL_DELAYED {
    pub guid: Guid,
    pub delay_time: u32,
}

impl crate::Message for SMSG_SPELL_DELAYED {
    const OPCODE: u32 = 0x01e2;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // delay_time: u32
        w.write_all(&self.delay_time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // delay_time: u32
        let delay_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            delay_time,
        })
    }

}
impl ServerMessage for SMSG_SPELL_DELAYED {}

