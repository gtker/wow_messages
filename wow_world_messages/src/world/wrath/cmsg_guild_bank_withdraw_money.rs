use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_withdraw_money.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_withdraw_money.wowm#L8):
/// ```text
/// cmsg CMSG_GUILD_BANK_WITHDRAW_MONEY = 0x03ED {
///     Guid bank;
///     Gold money;
/// }
/// ```
pub struct CMSG_GUILD_BANK_WITHDRAW_MONEY {
    pub bank: Guid,
    pub money: Gold,
}

impl crate::private::Sealed for CMSG_GUILD_BANK_WITHDRAW_MONEY {}
impl CMSG_GUILD_BANK_WITHDRAW_MONEY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // bank: Guid
        let bank = crate::util::read_guid(&mut r)?;

        // money: Gold
        let money = Gold::new(crate::util::read_u32_le(&mut r)?);

        Ok(Self {
            bank,
            money,
        })
    }

}

impl crate::Message for CMSG_GUILD_BANK_WITHDRAW_MONEY {
    const OPCODE: u32 = 0x03ed;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_BANK_WITHDRAW_MONEY {{").unwrap();
        // Members
        writeln!(s, "    bank = {};", self.bank.guid()).unwrap();
        writeln!(s, "    money = {};", self.money.as_int()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1005_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "bank", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // money: Gold
        w.write_all((self.money.as_int()).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1005, "CMSG_GUILD_BANK_WITHDRAW_MONEY", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANK_WITHDRAW_MONEY {}

