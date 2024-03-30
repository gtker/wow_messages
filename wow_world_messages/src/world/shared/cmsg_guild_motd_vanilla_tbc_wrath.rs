use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_motd.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_motd.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_MOTD = 0x0091 {
///     CString message_of_the_day;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_GUILD_MOTD {
    pub message_of_the_day: String,
}

impl crate::private::Sealed for CMSG_GUILD_MOTD {}
impl CMSG_GUILD_MOTD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // message_of_the_day: CString
        let message_of_the_day = {
            let message_of_the_day = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message_of_the_day)?
        };

        Ok(Self {
            message_of_the_day,
        })
    }

}

impl crate::Message for CMSG_GUILD_MOTD {
    const OPCODE: u32 = 0x0091;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GUILD_MOTD"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_MOTD {{").unwrap();
        // Members
        writeln!(s, "    message_of_the_day = \"{}\";", self.message_of_the_day).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 145_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.message_of_the_day.len() + 1, "message_of_the_day", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // message_of_the_day: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message_of_the_day.as_bytes().iter().next_back(), Some(&0_u8), "String `message_of_the_day` must not be null-terminated.");
        w.write_all(self.message_of_the_day.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(145, "CMSG_GUILD_MOTD", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_MOTD {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_MOTD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_MOTD {}

impl CMSG_GUILD_MOTD {
    pub(crate) fn size(&self) -> usize {
        self.message_of_the_day.len() + 1 // message_of_the_day: CString
    }
}

