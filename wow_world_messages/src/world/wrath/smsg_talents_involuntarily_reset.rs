use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Only exists as comment in azerothcore/trinitycore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_talents_involuntarily_reset.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_talents_involuntarily_reset.wowm#L1):
/// ```text
/// smsg SMSG_TALENTS_INVOLUNTARILY_RESET = 0x04FA {
///     u8 unknown;
/// }
/// ```
pub struct SMSG_TALENTS_INVOLUNTARILY_RESET {
    pub unknown: u8,
}

impl crate::Message for SMSG_TALENTS_INVOLUNTARILY_RESET {
    const OPCODE: u32 = 0x04fa;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04FA, size: body_size as u32 });
        }

        // unknown: u8
        let unknown = crate::util::read_u8_le(r)?;

        Ok(Self {
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TALENTS_INVOLUNTARILY_RESET {}

