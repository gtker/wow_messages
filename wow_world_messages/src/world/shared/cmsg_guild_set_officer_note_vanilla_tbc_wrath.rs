use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_set_officer_note.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_set_officer_note.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_SET_OFFICER_NOTE = 0x0235 {
///     CString player_name;
///     CString note;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_GUILD_SET_OFFICER_NOTE {
    pub player_name: String,
    /// vmangos: Max length 31
    pub note: String,
}

impl crate::private::Sealed for CMSG_GUILD_SET_OFFICER_NOTE {}
impl CMSG_GUILD_SET_OFFICER_NOTE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player_name: CString
        let player_name = {
            let player_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player_name)?
        };

        // note: CString
        let note = {
            let note = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(note)?
        };

        Ok(Self {
            player_name,
            note,
        })
    }

}

impl crate::Message for CMSG_GUILD_SET_OFFICER_NOTE {
    const OPCODE: u32 = 0x0235;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GUILD_SET_OFFICER_NOTE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_SET_OFFICER_NOTE {{").unwrap();
        // Members
        writeln!(s, "    player_name = \"{}\";", self.player_name).unwrap();
        writeln!(s, "    note = \"{}\";", self.note).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 565_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.player_name.len() + 1, "player_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.note.len() + 1, "note", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().next_back(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // note: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.note.as_bytes().iter().next_back(), Some(&0_u8), "String `note` must not be null-terminated.");
        w.write_all(self.note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(565, "CMSG_GUILD_SET_OFFICER_NOTE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_SET_OFFICER_NOTE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_SET_OFFICER_NOTE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_SET_OFFICER_NOTE {}

impl CMSG_GUILD_SET_OFFICER_NOTE {
    pub(crate) fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString
        + self.note.len() + 1 // note: CString
    }
}

