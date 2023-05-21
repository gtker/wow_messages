use std::io::{Read, Write};

use crate::DateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_info.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_info.wowm#L12):
/// ```text
/// smsg SMSG_GUILD_INFO = 0x0088 {
///     CString guild_name;
///     DateTime created;
///     u32 amount_of_characters_in_guild;
///     u32 amount_of_accounts_in_guild;
/// }
/// ```
pub struct SMSG_GUILD_INFO {
    pub guild_name: String,
    pub created: DateTime,
    pub amount_of_characters_in_guild: u32,
    pub amount_of_accounts_in_guild: u32,
}

impl crate::private::Sealed for SMSG_GUILD_INFO {}
impl SMSG_GUILD_INFO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(13..=268).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guild_name: CString
        let guild_name = {
            let guild_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_name)?
        };

        // created: DateTime
        let created = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // amount_of_characters_in_guild: u32
        let amount_of_characters_in_guild = crate::util::read_u32_le(&mut r)?;

        // amount_of_accounts_in_guild: u32
        let amount_of_accounts_in_guild = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guild_name,
            created,
            amount_of_characters_in_guild,
            amount_of_accounts_in_guild,
        })
    }

}

impl crate::Message for SMSG_GUILD_INFO {
    const OPCODE: u32 = 0x0088;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_INFO {{").unwrap();
        // Members
        writeln!(s, "    guild_name = \"{}\";", self.guild_name).unwrap();
        writeln!(s, "    created = {};", self.created.as_int()).unwrap();
        writeln!(s, "    amount_of_characters_in_guild = {};", self.amount_of_characters_in_guild).unwrap();
        writeln!(s, "    amount_of_accounts_in_guild = {};", self.amount_of_accounts_in_guild).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 136_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.guild_name.len() + 1, "guild_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "created", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_characters_in_guild", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_accounts_in_guild", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guild_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_name` must not be null-terminated.");
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // created: DateTime
        w.write_all(&self.created.as_int().to_le_bytes())?;

        // amount_of_characters_in_guild: u32
        w.write_all(&self.amount_of_characters_in_guild.to_le_bytes())?;

        // amount_of_accounts_in_guild: u32
        w.write_all(&self.amount_of_accounts_in_guild.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(136, "SMSG_GUILD_INFO", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GUILD_INFO {}

impl SMSG_GUILD_INFO {
    pub(crate) fn size(&self) -> usize {
        self.guild_name.len() + 1 // guild_name: CString
        + 4 // created: DateTime
        + 4 // amount_of_characters_in_guild: u32
        + 4 // amount_of_accounts_in_guild: u32
    }
}

