use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_info.wowm#L1):
/// ```text
/// smsg SMSG_GUILD_INFO = 0x0088 {
///     CString guild_name;
///     u32 created_day;
///     u32 created_month;
///     u32 created_year;
///     u32 amount_of_characters_in_guild;
///     u32 amount_of_accounts_in_guild;
/// }
/// ```
pub struct SMSG_GUILD_INFO {
    pub guild_name: String,
    pub created_day: u32,
    pub created_month: u32,
    pub created_year: u32,
    pub amount_of_characters_in_guild: u32,
    pub amount_of_accounts_in_guild: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_GUILD_INFO {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_INFO {{").unwrap();
        // Members
        writeln!(s, "    guild_name = \"{}\";", self.guild_name).unwrap();
        writeln!(s, "    created_day = {};", self.created_day).unwrap();
        writeln!(s, "    created_month = {};", self.created_month).unwrap();
        writeln!(s, "    created_year = {};", self.created_year).unwrap();
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
        crate::util::write_bytes(&mut s, &mut bytes, 4, "created_day", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "created_month", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "created_year", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_characters_in_guild", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_accounts_in_guild", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_GUILD_INFO {}
impl crate::Message for SMSG_GUILD_INFO {
    const OPCODE: u32 = 0x0088;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_GUILD_INFO::to_test_case_string(self)
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

        // created_day: u32
        w.write_all(&self.created_day.to_le_bytes())?;

        // created_month: u32
        w.write_all(&self.created_month.to_le_bytes())?;

        // created_year: u32
        w.write_all(&self.created_year.to_le_bytes())?;

        // amount_of_characters_in_guild: u32
        w.write_all(&self.amount_of_characters_in_guild.to_le_bytes())?;

        // amount_of_accounts_in_guild: u32
        w.write_all(&self.amount_of_accounts_in_guild.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(21..=276).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0088, size: body_size });
        }

        // guild_name: CString
        let guild_name = {
            let guild_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_name)?
        };

        // created_day: u32
        let created_day = crate::util::read_u32_le(&mut r)?;

        // created_month: u32
        let created_month = crate::util::read_u32_le(&mut r)?;

        // created_year: u32
        let created_year = crate::util::read_u32_le(&mut r)?;

        // amount_of_characters_in_guild: u32
        let amount_of_characters_in_guild = crate::util::read_u32_le(&mut r)?;

        // amount_of_accounts_in_guild: u32
        let amount_of_accounts_in_guild = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guild_name,
            created_day,
            created_month,
            created_year,
            amount_of_characters_in_guild,
            amount_of_accounts_in_guild,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GUILD_INFO {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GUILD_INFO {}

impl SMSG_GUILD_INFO {
    pub(crate) fn size(&self) -> usize {
        self.guild_name.len() + 1 // guild_name: CString
        + 4 // created_day: u32
        + 4 // created_month: u32
        + 4 // created_year: u32
        + 4 // amount_of_characters_in_guild: u32
        + 4 // amount_of_accounts_in_guild: u32
    }
}

