use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm#L1):
/// ```text
/// cmsg CMSG_SEND_MAIL = 0x0238 {
///     Guid mailbox;
///     CString receiver;
///     CString subject;
///     CString body;
///     u32 unknown1;
///     u32 unknown2;
///     Guid item;
///     Gold money;
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
    pub money: Gold,
    pub cash_on_delivery_amount: u32,
    /// cmangos: const 0
    ///
    pub unknown3: u32,
    /// cmangos: const 0
    ///
    pub unknown4: u32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_SEND_MAIL {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SEND_MAIL {{").unwrap();
        // Members
        writeln!(s, "    mailbox = {};", self.mailbox.guid()).unwrap();
        writeln!(s, "    receiver = \"{}\";", self.receiver).unwrap();
        writeln!(s, "    subject = \"{}\";", self.subject).unwrap();
        writeln!(s, "    body = \"{}\";", self.body).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    money = {};", self.money.as_int()).unwrap();
        writeln!(s, "    cash_on_delivery_amount = {};", self.cash_on_delivery_amount).unwrap();
        writeln!(s, "    unknown3 = {};", self.unknown3).unwrap();
        writeln!(s, "    unknown4 = {};", self.unknown4).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 568_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "mailbox", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.receiver.len() + 1, "receiver", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.subject.len() + 1, "subject", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.body.len() + 1, "body", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "cash_on_delivery_amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown3", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown4", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_SEND_MAIL {}
impl crate::Message for CMSG_SEND_MAIL {
    const OPCODE: u32 = 0x0238;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_SEND_MAIL::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        // receiver: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.receiver.as_bytes().iter().rev().next(), Some(&0_u8), "String `receiver` must not be null-terminated.");
        w.write_all(self.receiver.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // subject: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.subject.as_bytes().iter().rev().next(), Some(&0_u8), "String `subject` must not be null-terminated.");
        w.write_all(self.subject.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.body.as_bytes().iter().rev().next(), Some(&0_u8), "String `body` must not be null-terminated.");
        w.write_all(self.body.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // money: Gold
        w.write_all((self.money.as_int()).to_le_bytes().as_slice())?;

        // cash_on_delivery_amount: u32
        w.write_all(&self.cash_on_delivery_amount.to_le_bytes())?;

        // unknown3: u32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(43..=808).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0238, size: body_size });
        }

        // mailbox: Guid
        let mailbox = crate::util::read_guid(&mut r)?;

        // receiver: CString
        let receiver = {
            let receiver = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(receiver)?
        };

        // subject: CString
        let subject = {
            let subject = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(subject)?
        };

        // body: CString
        let body = {
            let body = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(body)?
        };

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // money: Gold
        let money = Gold::new(crate::util::read_u32_le(&mut r)?);

        // cash_on_delivery_amount: u32
        let cash_on_delivery_amount = crate::util::read_u32_le(&mut r)?;

        // unknown3: u32
        let unknown3 = crate::util::read_u32_le(&mut r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(&mut r)?;

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
impl crate::vanilla::ClientMessage for CMSG_SEND_MAIL {}

impl CMSG_SEND_MAIL {
    pub(crate) fn size(&self) -> usize {
        8 // mailbox: Guid
        + self.receiver.len() + 1 // receiver: CString
        + self.subject.len() + 1 // subject: CString
        + self.body.len() + 1 // body: CString
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 8 // item: Guid
        + 4 // money: Gold
        + 4 // cash_on_delivery_amount: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
    }
}

