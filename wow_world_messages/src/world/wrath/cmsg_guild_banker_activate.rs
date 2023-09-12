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

impl crate::private::Sealed for CMSG_GUILD_BANKER_ACTIVATE {}
impl CMSG_GUILD_BANKER_ACTIVATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for CMSG_GUILD_BANKER_ACTIVATE {
    const OPCODE: u32 = 0x03e6;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GUILD_BANKER_ACTIVATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_BANKER_ACTIVATE {{").unwrap();
        // Members
        writeln!(s, "    bank = {};", self.bank.guid()).unwrap();
        writeln!(s, "    full_update = {};", if self.full_update { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 998_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "bank", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "full_update", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(998, "CMSG_GUILD_BANKER_ACTIVATE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANKER_ACTIVATE {}

