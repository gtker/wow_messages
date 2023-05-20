use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_unlearn_talents.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_unlearn_talents.wowm#L1):
/// ```text
/// cmsg CMSG_UNLEARN_TALENTS = 0x0213 {
/// }
/// ```
pub struct CMSG_UNLEARN_TALENTS {
}

impl crate::private::Sealed for CMSG_UNLEARN_TALENTS {}
impl crate::Message for CMSG_UNLEARN_TALENTS {
    const OPCODE: u32 = 0x0213;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0213, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_UNLEARN_TALENTS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UNLEARN_TALENTS {}

