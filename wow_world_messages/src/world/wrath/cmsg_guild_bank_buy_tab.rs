use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_buy_tab.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_buy_tab.wowm#L8):
/// ```text
/// cmsg CMSG_GUILD_BANK_BUY_TAB = 0x03EA {
///     Guid banker;
///     u8 tab;
/// }
/// ```
pub struct CMSG_GUILD_BANK_BUY_TAB {
    pub banker: Guid,
    pub tab: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_GUILD_BANK_BUY_TAB {
    pub fn to_test_case_string(&self) -> Option<String> {
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
        let [a, b, c, d] = 1002_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "banker", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_GUILD_BANK_BUY_TAB {}
impl crate::Message for CMSG_GUILD_BANK_BUY_TAB {
    const OPCODE: u32 = 0x03ea;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_GUILD_BANK_BUY_TAB::to_test_case_string(self)
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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EA, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANK_BUY_TAB {}

