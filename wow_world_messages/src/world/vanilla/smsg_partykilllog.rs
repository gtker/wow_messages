use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_partykilllog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_partykilllog.wowm#L3):
/// ```text
/// smsg SMSG_PARTYKILLLOG = 0x01F5 {
///     Guid player_with_killing_blow;
///     Guid victim;
/// }
/// ```
pub struct SMSG_PARTYKILLLOG {
    pub player_with_killing_blow: Guid,
    pub victim: Guid,
}

impl crate::Message for SMSG_PARTYKILLLOG {
    const OPCODE: u32 = 0x01f5;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player_with_killing_blow: Guid
        w.write_all(&self.player_with_killing_blow.guid().to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // player_with_killing_blow: Guid
        let player_with_killing_blow = Guid::read(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        Ok(Self {
            player_with_killing_blow,
            victim,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PARTYKILLLOG {}

