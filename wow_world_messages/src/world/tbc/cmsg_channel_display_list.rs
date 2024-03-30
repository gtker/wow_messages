use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_channel_display_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_channel_display_list.wowm#L1):
/// ```text
/// cmsg CMSG_CHANNEL_DISPLAY_LIST = 0x03D1 {
///     CString channel;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_CHANNEL_DISPLAY_LIST {
    pub channel: String,
}

impl crate::private::Sealed for CMSG_CHANNEL_DISPLAY_LIST {}
impl CMSG_CHANNEL_DISPLAY_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // channel: CString
        let channel = {
            let channel = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel)?
        };

        Ok(Self {
            channel,
        })
    }

}

impl crate::Message for CMSG_CHANNEL_DISPLAY_LIST {
    const OPCODE: u32 = 0x03d1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CHANNEL_DISPLAY_LIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CHANNEL_DISPLAY_LIST {{").unwrap();
        // Members
        writeln!(s, "    channel = \"{}\";", self.channel).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 977_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.channel.len() + 1, "channel", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel.as_bytes().iter().next_back(), Some(&0_u8), "String `channel` must not be null-terminated.");
        w.write_all(self.channel.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(977, "CMSG_CHANNEL_DISPLAY_LIST", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CHANNEL_DISPLAY_LIST {}

impl CMSG_CHANNEL_DISPLAY_LIST {
    pub(crate) fn size(&self) -> usize {
        self.channel.len() + 1 // channel: CString
    }
}

