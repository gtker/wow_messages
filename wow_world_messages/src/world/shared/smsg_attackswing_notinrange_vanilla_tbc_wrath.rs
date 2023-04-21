use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackswing_notinrange.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackswing_notinrange.wowm#L3):
/// ```text
/// smsg SMSG_ATTACKSWING_NOTINRANGE = 0x0145 {
/// }
/// ```
pub struct SMSG_ATTACKSWING_NOTINRANGE {
}

impl crate::private::Sealed for SMSG_ATTACKSWING_NOTINRANGE {}
impl crate::Message for SMSG_ATTACKSWING_NOTINRANGE {
    const OPCODE: u32 = 0x0145;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0145, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKSWING_NOTINRANGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ATTACKSWING_NOTINRANGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ATTACKSWING_NOTINRANGE {}

