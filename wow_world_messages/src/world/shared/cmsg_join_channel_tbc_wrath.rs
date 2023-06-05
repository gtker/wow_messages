use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm#L8):
/// ```text
/// cmsg CMSG_JOIN_CHANNEL = 0x0097 {
///     u32 channel_id;
///     u8 unknown1;
///     u8 unknown2;
///     CString channel_name;
///     CString channel_password;
/// }
/// ```
pub struct CMSG_JOIN_CHANNEL {
    pub channel_id: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub channel_name: String,
    pub channel_password: String,
}

#[cfg(feature = "print-testcase")]
impl CMSG_JOIN_CHANNEL {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_JOIN_CHANNEL {{").unwrap();
        // Members
        writeln!(s, "    channel_id = {};", self.channel_id).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    channel_name = \"{}\";", self.channel_name).unwrap();
        writeln!(s, "    channel_password = \"{}\";", self.channel_password).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 151_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "channel_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.channel_name.len() + 1, "channel_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.channel_password.len() + 1, "channel_password", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_JOIN_CHANNEL {}
impl crate::Message for CMSG_JOIN_CHANNEL {
    const OPCODE: u32 = 0x0097;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_JOIN_CHANNEL::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel_id: u32
        w.write_all(&self.channel_id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

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
        if !(8..=518).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0097, size: body_size });
        }

        // channel_id: u32
        let channel_id = crate::util::read_u32_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

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
            channel_id,
            unknown1,
            unknown2,
            channel_name,
            channel_password,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_JOIN_CHANNEL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_JOIN_CHANNEL {}

impl CMSG_JOIN_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        4 // channel_id: u32
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + self.channel_name.len() + 1 // channel_name: CString
        + self.channel_password.len() + 1 // channel_password: CString
    }
}

