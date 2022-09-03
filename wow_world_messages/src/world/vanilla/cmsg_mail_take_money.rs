use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_mail_take_money.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_take_money.wowm#L3):
/// ```text
/// cmsg CMSG_MAIL_TAKE_MONEY = 0x0245 {
///     Guid mailbox_guid;
///     u32 mail_id;
/// }
/// ```
pub struct CMSG_MAIL_TAKE_MONEY {
    pub mailbox_guid: Guid,
    pub mail_id: u32,
}

impl crate::Message for CMSG_MAIL_TAKE_MONEY {
    const OPCODE: u32 = 0x0245;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // mailbox_guid: Guid
        w.write_all(&self.mailbox_guid.guid().to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // mailbox_guid: Guid
        let mailbox_guid = Guid::read(r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox_guid,
            mail_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_MAIL_TAKE_MONEY {}

