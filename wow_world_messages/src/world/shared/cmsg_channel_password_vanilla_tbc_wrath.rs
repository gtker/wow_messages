use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_channel_password.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_channel_password.wowm#L3):
/// ```text
/// cmsg CMSG_CHANNEL_PASSWORD = 0x009C {
///     CString channel_name;
///     CString channel_password;
/// }
/// ```
pub struct CMSG_CHANNEL_PASSWORD {
    pub channel_name: String,
    pub channel_password: String,
}

#[cfg(feature = "print-testcase")]
impl CMSG_CHANNEL_PASSWORD {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CHANNEL_PASSWORD {{").unwrap();
        // Members
        writeln!(s, "    channel_name = \"{}\";", self.channel_name).unwrap();
        writeln!(s, "    channel_password = \"{}\";", self.channel_password).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 156_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.channel_name.len() + 1, "channel_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.channel_password.len() + 1, "channel_password", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_CHANNEL_PASSWORD {}
impl crate::Message for CMSG_CHANNEL_PASSWORD {
    const OPCODE: u32 = 0x009c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_CHANNEL_PASSWORD::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_password: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_password.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_password` must not be null-terminated.");
        w.write_all(self.channel_password.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x009C, size: body_size });
        }

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        // channel_password: CString
        let channel_password = {
            let channel_password = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_password)?
        };

        Ok(Self {
            channel_name,
            channel_password,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_CHANNEL_PASSWORD {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CHANNEL_PASSWORD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHANNEL_PASSWORD {}

impl CMSG_CHANNEL_PASSWORD {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
        + self.channel_password.len() + 1 // channel_password: CString
    }
}

