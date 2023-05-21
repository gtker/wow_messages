use std::io::{Read, Write};

use crate::Guid;
use crate::shared::mail_item_tbc_wrath::MailItem;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm#L32):
/// ```text
/// cmsg CMSG_SEND_MAIL = 0x0238 {
///     Guid mailbox;
///     CString receiver;
///     CString subject;
///     CString body;
///     u32 unknown1;
///     u32 unknown2;
///     u8 amount_of_items;
///     MailItem[amount_of_items] items;
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
    pub unknown1: u32,
    /// cmangos: 0x00000000
    pub unknown2: u32,
    pub items: Vec<MailItem>,
    pub money: Gold,
    pub cash_on_delivery_amount: u32,
    /// mangosone: const 0
    pub unknown3: u32,
    /// mangosone: const 0
    pub unknown4: u32,
}

impl crate::private::Sealed for CMSG_SEND_MAIL {}
impl CMSG_SEND_MAIL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(36..=3105).contains(&body_size) {
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

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(&mut r)?;

        // items: MailItem[amount_of_items]
        let items = {
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for _ in 0..amount_of_items {
                items.push(MailItem::read(&mut r)?);
            }
            items
        };

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
            items,
            money,
            cash_on_delivery_amount,
            unknown3,
            unknown4,
        })
    }

}

impl crate::Message for CMSG_SEND_MAIL {
    const OPCODE: u32 = 0x0238;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
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
        writeln!(s, "    amount_of_items = {};", self.items.len()).unwrap();
        write!(s, "    items = [").unwrap();
        for v in self.items.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item.guid()).unwrap();
            writeln!(s, "        slot = {};", v.slot).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
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
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_items", "    ");
        if !self.items.is_empty() {
            writeln!(s, "    /* items: MailItem[amount_of_items] start */").unwrap();
            for (i, v) in self.items.iter().enumerate() {
                writeln!(s, "    /* items: MailItem[amount_of_items] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "        ");
                writeln!(s, "    /* items: MailItem[amount_of_items] {i} end */").unwrap();
            }
            writeln!(s, "    /* items: MailItem[amount_of_items] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "cash_on_delivery_amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown3", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown4", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
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

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: MailItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(&mut w)?;
        }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SEND_MAIL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SEND_MAIL {}

impl CMSG_SEND_MAIL {
    pub(crate) fn size(&self) -> usize {
        8 // mailbox: Guid
        + self.receiver.len() + 1 // receiver: CString
        + self.subject.len() + 1 // subject: CString
        + self.body.len() + 1 // body: CString
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 1 // amount_of_items: u8
        + self.items.len() * 9 // items: MailItem[amount_of_items]
        + 4 // money: Gold
        + 4 // cash_on_delivery_amount: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
    }
}

