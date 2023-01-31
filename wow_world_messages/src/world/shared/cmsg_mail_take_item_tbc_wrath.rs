use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm#L8):
/// ```text
/// cmsg CMSG_MAIL_TAKE_ITEM = 0x0246 {
///     Guid mailbox;
///     u32 mail_id;
///     u32 item;
/// }
/// ```
pub struct CMSG_MAIL_TAKE_ITEM {
    pub mailbox: Guid,
    pub mail_id: u32,
    pub item: u32,
}

impl crate::Message for CMSG_MAIL_TAKE_ITEM {
    const OPCODE: u32 = 0x0246;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0246, size: body_size as u32 });
        }

        // mailbox: Guid
        let mailbox = Guid::read(r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox,
            mail_id,
            item,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MAIL_TAKE_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MAIL_TAKE_ITEM {}

