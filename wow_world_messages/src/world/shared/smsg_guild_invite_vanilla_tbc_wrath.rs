use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_invite.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_invite.wowm#L3):
/// ```text
/// smsg SMSG_GUILD_INVITE = 0x0083 {
///     CString player_name;
///     CString guild_name;
/// }
/// ```
pub struct SMSG_GUILD_INVITE {
    pub player_name: String,
    pub guild_name: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_GUILD_INVITE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_INVITE {{").unwrap();
        // Members
        writeln!(s, "    player_name = \"{}\";", self.player_name).unwrap();
        writeln!(s, "    guild_name = \"{}\";", self.guild_name).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 131_u32.to_le_bytes();
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

impl crate::private::Sealed for SMSG_GUILD_INVITE {}
impl crate::Message for SMSG_GUILD_INVITE {
    const OPCODE: u32 = 0x0083;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_name` must not be null-terminated.");
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0083, size: body_size });
        }

        // player_name: CString
        let player_name = {
            let player_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player_name)?
        };

        // guild_name: CString
        let guild_name = {
            let guild_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_name)?
        };

        Ok(Self {
            player_name,
            guild_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GUILD_INVITE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GUILD_INVITE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GUILD_INVITE {}

impl SMSG_GUILD_INVITE {
    pub(crate) fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString
        + self.guild_name.len() + 1 // guild_name: CString
    }
}

