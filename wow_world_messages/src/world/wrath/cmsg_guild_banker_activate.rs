use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_banker_activate.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_banker_activate.wowm#L8):
/// ```text
/// cmsg CMSG_GUILD_BANKER_ACTIVATE = 0x03E6 {
///     Guid bank;
///     Bool full_update;
/// }
/// ```
pub struct CMSG_GUILD_BANKER_ACTIVATE {
    pub bank: Guid,
    pub full_update: bool,
}

#[cfg(feature = "print-testcase")]
impl CMSG_GUILD_BANKER_ACTIVATE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_BANKER_ACTIVATE {{").unwrap();
        // Members
        writeln!(s, "    bank = {};", self.bank.guid()).unwrap();
        writeln!(s, "    full_update = {};", if self.full_update { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 998_u16.to_le_bytes();
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
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_GUILD_BANKER_ACTIVATE {}
impl crate::Message for CMSG_GUILD_BANKER_ACTIVATE {
    const OPCODE: u32 = 0x03e6;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // full_update: Bool
        w.write_all(u8::from(self.full_update).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E6, size: body_size });
        }

        // bank: Guid
        let bank = crate::util::read_guid(&mut r)?;

        // full_update: Bool
        let full_update = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            bank,
            full_update,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANKER_ACTIVATE {}

