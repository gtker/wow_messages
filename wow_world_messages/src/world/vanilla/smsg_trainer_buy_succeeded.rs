use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_buy_succeeded.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_buy_succeeded.wowm#L3):
/// ```text
/// smsg SMSG_TRAINER_BUY_SUCCEEDED = 0x01B3 {
///     Guid guid;
///     u32 id;
/// }
/// ```
pub struct SMSG_TRAINER_BUY_SUCCEEDED {
    pub guid: Guid,
    pub id: u32,
}

impl crate::Message for SMSG_TRAINER_BUY_SUCCEEDED {
    const OPCODE: u32 = 0x01b3;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            id,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_TRAINER_BUY_SUCCEEDED {}

