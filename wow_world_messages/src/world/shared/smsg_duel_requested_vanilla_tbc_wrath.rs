use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_REQUESTED = 0x0167 {
///     Guid initiator;
///     Guid target;
/// }
/// ```
pub struct SMSG_DUEL_REQUESTED {
    pub initiator: Guid,
    pub target: Guid,
}

impl crate::Message for SMSG_DUEL_REQUESTED {
    const OPCODE: u32 = 0x0167;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // initiator: Guid
        w.write_all(&self.initiator.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0167, size: body_size as u32 });
        }

        // initiator: Guid
        let initiator = Guid::read(r)?;

        // target: Guid
        let target = Guid::read(r)?;

        Ok(Self {
            initiator,
            target,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_DUEL_REQUESTED {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_DUEL_REQUESTED {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_DUEL_REQUESTED {}

