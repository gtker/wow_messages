use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_mail_create_text_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_create_text_item.wowm#L1):
/// ```text
/// cmsg CMSG_MAIL_CREATE_TEXT_ITEM = 0x024A {
///     Guid mailbox;
///     u32 mail_id;
///     u32 mail_template_id;
/// }
/// ```
pub struct CMSG_MAIL_CREATE_TEXT_ITEM {
    pub mailbox: Guid,
    pub mail_id: u32,
    /// mangoszero/cmangos/vmangos: mailTemplateId, non need, Mail store own 100% correct value anyway
    ///
    pub mail_template_id: u32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_MAIL_CREATE_TEXT_ITEM {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_MAIL_CREATE_TEXT_ITEM {{").unwrap();
        // Members
        writeln!(s, "    mailbox = {};", self.mailbox.guid()).unwrap();
        writeln!(s, "    mail_id = {};", self.mail_id).unwrap();
        writeln!(s, "    mail_template_id = {};", self.mail_template_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 586_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "mailbox", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_template_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_MAIL_CREATE_TEXT_ITEM {}
impl crate::Message for CMSG_MAIL_CREATE_TEXT_ITEM {
    const OPCODE: u32 = 0x024a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_MAIL_CREATE_TEXT_ITEM::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024A, size: body_size });
        }

        // mailbox: Guid
        let mailbox = crate::util::read_guid(&mut r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(&mut r)?;

        // mail_template_id: u32
        let mail_template_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            mailbox,
            mail_id,
            mail_template_id,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MAIL_CREATE_TEXT_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MAIL_CREATE_TEXT_ITEM {}

