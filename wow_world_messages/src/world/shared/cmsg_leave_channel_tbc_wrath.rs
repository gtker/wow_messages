use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_leave_channel.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_leave_channel.wowm#L7):
/// ```text
/// cmsg CMSG_LEAVE_CHANNEL = 0x0098 {
///     u32 channel_id;
///     CString channel_name;
/// }
/// ```
pub struct CMSG_LEAVE_CHANNEL {
    pub channel_id: u32,
    pub channel_name: String,
}

impl crate::private::Sealed for CMSG_LEAVE_CHANNEL {}
impl CMSG_LEAVE_CHANNEL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // channel_id: u32
        let channel_id = crate::util::read_u32_le(&mut r)?;

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        Ok(Self {
            channel_id,
            channel_name,
        })
    }

}

impl crate::Message for CMSG_LEAVE_CHANNEL {
    const OPCODE: u32 = 0x0098;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LEAVE_CHANNEL {{").unwrap();
        // Members
        writeln!(s, "    channel_id = {};", self.channel_id).unwrap();
        writeln!(s, "    channel_name = \"{}\";", self.channel_name).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 152_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "channel_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.channel_name.len() + 1, "channel_name", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel_id: u32
        w.write_all(&self.channel_id.to_le_bytes())?;

        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().next_back(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(152, "CMSG_LEAVE_CHANNEL", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LEAVE_CHANNEL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LEAVE_CHANNEL {}

impl CMSG_LEAVE_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        4 // channel_id: u32
        + self.channel_name.len() + 1 // channel_name: CString
    }
}

