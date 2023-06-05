use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_mail_create_text_item.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_create_text_item.wowm#L11):
/// ```text
/// cmsg CMSG_MAIL_CREATE_TEXT_ITEM = 0x024A {
///     Guid mailbox;
///     u32 mail_id;
/// }
/// ```
pub struct CMSG_MAIL_CREATE_TEXT_ITEM {
    pub mailbox: Guid,
    pub mail_id: u32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_MAIL_CREATE_TEXT_ITEM {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_MAIL_CREATE_TEXT_ITEM {{").unwrap();
        // Members
        writeln!(s, "    mailbox = {};", self.mailbox.guid()).unwrap();
        writeln!(s, "    mail_id = {};", self.mail_id).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 586_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "mailbox");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_MAIL_CREATE_TEXT_ITEM {}
impl crate::Message for CMSG_MAIL_CREATE_TEXT_ITEM {
    const OPCODE: u32 = 0x024a;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024A, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MAIL_CREATE_TEXT_ITEM {}

