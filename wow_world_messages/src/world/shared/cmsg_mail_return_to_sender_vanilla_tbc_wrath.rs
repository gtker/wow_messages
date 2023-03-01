use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_mail_return_to_sender.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_return_to_sender.wowm#L3):
/// ```text
/// cmsg CMSG_MAIL_RETURN_TO_SENDER = 0x0248 {
///     Guid mailbox_id;
///     u32 mail_id;
/// }
/// ```
pub struct CMSG_MAIL_RETURN_TO_SENDER {
    pub mailbox_id: Guid,
    pub mail_id: u32,
}

impl crate::Message for CMSG_MAIL_RETURN_TO_SENDER {
    const OPCODE: u32 = 0x0248;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // mailbox_id: Guid
        w.write_all(&self.mailbox_id.guid().to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0248, size: body_size as u32 });
        }

        // mailbox_id: Guid
        let mailbox_id = Guid::read(&mut r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            mailbox_id,
            mail_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MAIL_RETURN_TO_SENDER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MAIL_RETURN_TO_SENDER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MAIL_RETURN_TO_SENDER {}

