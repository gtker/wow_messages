use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_get_mail_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_get_mail_list.wowm#L3):
/// ```text
/// cmsg CMSG_GET_MAIL_LIST = 0x023A {
///     Guid mailbox;
/// }
/// ```
pub struct CMSG_GET_MAIL_LIST {
    pub mailbox: Guid,
}

impl crate::private::Sealed for CMSG_GET_MAIL_LIST {}
impl crate::Message for CMSG_GET_MAIL_LIST {
    const OPCODE: u32 = 0x023a;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x023A, size: body_size });
        }

        // mailbox: Guid
        let mailbox = crate::util::read_guid(&mut r)?;

        Ok(Self {
            mailbox,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GET_MAIL_LIST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GET_MAIL_LIST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GET_MAIL_LIST {}

