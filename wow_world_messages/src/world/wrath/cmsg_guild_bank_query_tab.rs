use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_query_tab.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_query_tab.wowm#L9):
/// ```text
/// cmsg CMSG_GUILD_BANK_QUERY_TAB = 0x03E7 {
///     Guid bank;
///     u8 tab;
///     Bool full_update;
/// }
/// ```
pub struct CMSG_GUILD_BANK_QUERY_TAB {
    pub bank: Guid,
    pub tab: u8,
    pub full_update: bool,
}

impl crate::private::Sealed for CMSG_GUILD_BANK_QUERY_TAB {}
impl CMSG_GUILD_BANK_QUERY_TAB {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 10 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // bank: Guid
        let bank = crate::util::read_guid(&mut r)?;

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        // full_update: Bool
        let full_update = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            bank,
            tab,
            full_update,
        })
    }

}

impl crate::Message for CMSG_GUILD_BANK_QUERY_TAB {
    const OPCODE: u32 = 0x03e7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_BANK_QUERY_TAB {{").unwrap();
        // Members
        writeln!(s, "    bank = {};", self.bank.guid()).unwrap();
        writeln!(s, "    tab = {};", self.tab).unwrap();
        writeln!(s, "    full_update = {};", if self.full_update { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 999_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "bank", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "full_update", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        10
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        // full_update: Bool
        w.write_all(u8::from(self.full_update).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(999, "CMSG_GUILD_BANK_QUERY_TAB", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANK_QUERY_TAB {}

