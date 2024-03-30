use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm#L15):
/// ```text
/// smsg MSG_GUILD_BANK_MONEY_WITHDRAWN_Server = 0x03FE {
///     u32 remaining_withdraw_amount;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    pub remaining_withdraw_amount: u32,
}

impl crate::private::Sealed for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {}
impl MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // remaining_withdraw_amount: u32
        let remaining_withdraw_amount = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            remaining_withdraw_amount,
        })
    }

}

impl crate::Message for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    const OPCODE: u32 = 0x03fe;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_GUILD_BANK_MONEY_WITHDRAWN_Server"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {{").unwrap();
        // Members
        writeln!(s, "    remaining_withdraw_amount = {};", self.remaining_withdraw_amount).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1022_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "remaining_withdraw_amount", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // remaining_withdraw_amount: u32
        w.write_all(&self.remaining_withdraw_amount.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1022, "MSG_GUILD_BANK_MONEY_WITHDRAWN_Server", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {}

