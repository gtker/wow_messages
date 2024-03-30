use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_decline.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_decline.wowm#L1):
/// ```text
/// smsg SMSG_GUILD_DECLINE = 0x0086 {
///     CString player;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_GUILD_DECLINE {
    pub player: String,
}

impl crate::private::Sealed for SMSG_GUILD_DECLINE {}
impl SMSG_GUILD_DECLINE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: CString
        let player = {
            let player = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player)?
        };

        Ok(Self {
            player,
        })
    }

}

impl crate::Message for SMSG_GUILD_DECLINE {
    const OPCODE: u32 = 0x0086;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_GUILD_DECLINE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_DECLINE {{").unwrap();
        // Members
        writeln!(s, "    player = \"{}\";", self.player).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 134_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.player.len() + 1, "player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player.as_bytes().iter().next_back(), Some(&0_u8), "String `player` must not be null-terminated.");
        w.write_all(self.player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(134, "SMSG_GUILD_DECLINE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GUILD_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GUILD_DECLINE {}

impl SMSG_GUILD_DECLINE {
    pub(crate) fn size(&self) -> usize {
        self.player.len() + 1 // player: CString
    }
}

