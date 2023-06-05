use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl crate::private::Sealed for SMSG_PARTYKILLLOG {}
impl crate::Message for SMSG_PARTYKILLLOG {
    const OPCODE: u32 = 0x01f5;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player_with_killing_blow: Guid
        w.write_all(&self.player_with_killing_blow.guid().to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F5, size: body_size });
        }

        // player_with_killing_blow: Guid
        let player_with_killing_blow = crate::util::read_guid(&mut r)?;

        // victim: Guid
        let victim = crate::util::read_guid(&mut r)?;

        Ok(Self {
            player_with_killing_blow,
            victim,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PARTYKILLLOG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PARTYKILLLOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PARTYKILLLOG {}

