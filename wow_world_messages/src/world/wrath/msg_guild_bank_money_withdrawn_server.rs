use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm#L15):
/// ```text
/// smsg MSG_GUILD_BANK_MONEY_WITHDRAWN_Server = 0x03FE {
///     u32 remaining_withdraw_amount;
/// }
/// ```
pub struct MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    pub remaining_withdraw_amount: u32,
}

#[cfg(feature = "print-testcase")]
impl MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {{").unwrap();
        // Members
        writeln!(s, "    remaining_withdraw_amount = {};", self.remaining_withdraw_amount).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1022_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "remaining_withdraw_amount");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {}
impl crate::Message for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    const OPCODE: u32 = 0x03fe;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // remaining_withdraw_amount: u32
        w.write_all(&self.remaining_withdraw_amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FE, size: body_size });
        }

        // remaining_withdraw_amount: u32
        let remaining_withdraw_amount = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            remaining_withdraw_amount,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {}

