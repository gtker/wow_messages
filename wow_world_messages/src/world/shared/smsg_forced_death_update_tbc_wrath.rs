use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Resets `Release spirit` timer clientside.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_forced_death_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_forced_death_update.wowm#L1):
/// ```text
/// smsg SMSG_FORCED_DEATH_UPDATE = 0x037A {
/// }
/// ```
pub struct SMSG_FORCED_DEATH_UPDATE {
}

impl crate::private::Sealed for SMSG_FORCED_DEATH_UPDATE {}
impl crate::Message for SMSG_FORCED_DEATH_UPDATE {
    const OPCODE: u32 = 0x037a;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x037A, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FORCED_DEATH_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FORCED_DEATH_UPDATE {}

