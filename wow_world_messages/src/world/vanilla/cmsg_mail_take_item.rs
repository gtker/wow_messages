use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_take_item.wowm#L1):
/// ```text
/// cmsg CMSG_MAIL_TAKE_ITEM = 0x0246 {
///     Guid mailbox;
///     u32 mail_id;
/// }
/// ```
pub struct CMSG_MAIL_TAKE_ITEM {
    pub mailbox: Guid,
    pub mail_id: u32,
}

impl crate::private::Sealed for CMSG_MAIL_TAKE_ITEM {}
impl CMSG_MAIL_TAKE_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // mailbox: Guid
        let mailbox = crate::util::read_guid(&mut r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            mailbox,
            mail_id,
        })
    }

}

impl crate::Message for CMSG_MAIL_TAKE_ITEM {
    const OPCODE: u32 = 0x0246;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_MAIL_TAKE_ITEM {{").unwrap();
        // Members
        writeln!(s, "    mailbox = {};", self.mailbox.guid()).unwrap();
        writeln!(s, "    mail_id = {};", self.mail_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 582_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "mailbox", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(582, "CMSG_MAIL_TAKE_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MAIL_TAKE_ITEM {}

