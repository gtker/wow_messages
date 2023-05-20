use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_show_mailbox.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_show_mailbox.wowm#L1):
/// ```text
/// smsg SMSG_SHOW_MAILBOX = 0x0297 {
///     Guid guid;
/// }
/// ```
pub struct SMSG_SHOW_MAILBOX {
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_SHOW_MAILBOX {}
impl crate::Message for SMSG_SHOW_MAILBOX {
    const OPCODE: u32 = 0x0297;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0297, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SHOW_MAILBOX {}

