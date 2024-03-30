use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_buy_tab.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_buy_tab.wowm#L1):
/// ```text
/// cmsg CMSG_GUILD_BANK_BUY_TAB = 0x03E9 {
///     Guid banker;
///     u8 tab;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_GUILD_BANK_BUY_TAB {
    pub banker: Guid,
    pub tab: u8,
}

impl crate::private::Sealed for CMSG_GUILD_BANK_BUY_TAB {}
impl CMSG_GUILD_BANK_BUY_TAB {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // banker: Guid
        let banker = crate::util::read_guid(&mut r)?;

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            banker,
            tab,
        })
    }

}

impl crate::Message for CMSG_GUILD_BANK_BUY_TAB {
    const OPCODE: u32 = 0x03e9;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GUILD_BANK_BUY_TAB"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_BANK_BUY_TAB {{").unwrap();
        // Members
        writeln!(s, "    banker = {};", self.banker.guid()).unwrap();
        writeln!(s, "    tab = {};", self.tab).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1001_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "banker", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // banker: Guid
        w.write_all(&self.banker.guid().to_le_bytes())?;

        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1001, "CMSG_GUILD_BANK_BUY_TAB", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_BANK_BUY_TAB {}

