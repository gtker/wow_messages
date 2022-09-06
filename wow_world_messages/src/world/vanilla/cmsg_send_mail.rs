use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm#L3):
/// ```text
/// cmsg CMSG_SEND_MAIL = 0x0238 {
///     Guid mailbox;
///     CString receiver;
///     CString subject;
///     CString body;
///     u32 unknown1;
///     u32 unknown2;
///     Guid item;
///     u32 money;
///     u32 cash_on_delivery_amount;
///     u32 unknown3;
///     u32 unknown4;
/// }
/// ```
pub struct CMSG_SEND_MAIL {
    pub mailbox: Guid,
    pub receiver: String,
    pub subject: String,
    pub body: String,
    /// cmangos: stationery?
    ///
    pub unknown1: u32,
    /// cmangos: 0x00000000
    ///
    pub unknown2: u32,
    pub item: Guid,
    pub money: u32,
    pub cash_on_delivery_amount: u32,
    /// cmangos: const 0
    ///
    pub unknown3: u32,
    /// cmangos: const 0
    ///
    pub unknown4: u32,
}

impl crate::Message for CMSG_SEND_MAIL {
    const OPCODE: u32 = 0x0238;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        // receiver: CString
        w.write_all(self.receiver.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // subject: CString
        w.write_all(self.subject.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body: CString
        w.write_all(self.body.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // money: u32
        w.write_all(&self.money.to_le_bytes())?;

        // cash_on_delivery_amount: u32
        w.write_all(&self.cash_on_delivery_amount.to_le_bytes())?;

        // unknown3: u32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // mailbox: Guid
        let mailbox = Guid::read(r)?;

        // receiver: CString
        let receiver = crate::util::read_c_string_to_vec(r)?;
        let receiver = String::from_utf8(receiver)?;

        // subject: CString
        let subject = crate::util::read_c_string_to_vec(r)?;
        let subject = String::from_utf8(subject)?;

        // body: CString
        let body = crate::util::read_c_string_to_vec(r)?;
        let body = String::from_utf8(body)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // money: u32
        let money = crate::util::read_u32_le(r)?;

        // cash_on_delivery_amount: u32
        let cash_on_delivery_amount = crate::util::read_u32_le(r)?;

        // unknown3: u32
        let unknown3 = crate::util::read_u32_le(r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox,
            receiver,
            subject,
            body,
            unknown1,
            unknown2,
            item,
            money,
            cash_on_delivery_amount,
            unknown3,
            unknown4,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SEND_MAIL {}

impl CMSG_SEND_MAIL {
    pub(crate) fn size(&self) -> usize {
        8 // mailbox: Guid
        + self.receiver.len() + 1 // receiver: CString
        + self.subject.len() + 1 // subject: CString
        + self.body.len() + 1 // body: CString
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 8 // item: Guid
        + 4 // money: u32
        + 4 // cash_on_delivery_amount: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
    }
}

