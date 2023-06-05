use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_notification.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_notification.wowm#L3):
/// ```text
/// smsg SMSG_NOTIFICATION = 0x01CB {
///     CString notification;
/// }
/// ```
pub struct SMSG_NOTIFICATION {
    pub notification: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_NOTIFICATION {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_NOTIFICATION {{").unwrap();
        // Members
        writeln!(s, "    notification = \"{}\";", self.notification).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 459_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_NOTIFICATION {}
impl crate::Message for SMSG_NOTIFICATION {
    const OPCODE: u32 = 0x01cb;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // notification: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.notification.as_bytes().iter().rev().next(), Some(&0_u8), "String `notification` must not be null-terminated.");
        w.write_all(self.notification.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01CB, size: body_size });
        }

        // notification: CString
        let notification = {
            let notification = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(notification)?
        };

        Ok(Self {
            notification,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_NOTIFICATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_NOTIFICATION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_NOTIFICATION {}

impl SMSG_NOTIFICATION {
    pub(crate) fn size(&self) -> usize {
        self.notification.len() + 1 // notification: CString
    }
}

