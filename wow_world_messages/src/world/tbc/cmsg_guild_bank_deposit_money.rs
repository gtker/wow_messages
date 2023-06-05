use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_deposit_money.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_deposit_money.wowm#L1):
/// ```text
/// cmsg CMSG_GUILD_BANK_DEPOSIT_MONEY = 0x03EB {
///     Guid bank;
///     Gold money;
/// }
/// ```
pub struct CMSG_GUILD_BANK_DEPOSIT_MONEY {
    pub bank: Guid,
    pub money: Gold,
}

#[cfg(feature = "print-testcase")]
impl CMSG_GUILD_BANK_DEPOSIT_MONEY {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_BANK_DEPOSIT_MONEY {{").unwrap();
        // Members
        writeln!(s, "    bank = {};", self.bank.guid()).unwrap();
        writeln!(s, "    money = {};", self.money.as_int()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1003_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "bank");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_GUILD_BANK_DEPOSIT_MONEY {}
impl crate::Message for CMSG_GUILD_BANK_DEPOSIT_MONEY {
    const OPCODE: u32 = 0x03eb;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EB, size: body_size });
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
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_BANK_DEPOSIT_MONEY {}

