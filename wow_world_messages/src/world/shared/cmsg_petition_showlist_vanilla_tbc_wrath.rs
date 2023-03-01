use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_showlist.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_showlist.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_SHOWLIST = 0x01BB {
///     Guid guid;
/// }
/// ```
pub struct CMSG_PETITION_SHOWLIST {
    pub guid: Guid,
}

impl crate::Message for CMSG_PETITION_SHOWLIST {
    const OPCODE: u32 = 0x01bb;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BB, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PETITION_SHOWLIST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PETITION_SHOWLIST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PETITION_SHOWLIST {}

