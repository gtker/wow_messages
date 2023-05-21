use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_add_rune_power.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_add_rune_power.wowm#L1):
/// ```text
/// smsg SMSG_ADD_RUNE_POWER = 0x0488 {
///     u32 rune;
/// }
/// ```
pub struct SMSG_ADD_RUNE_POWER {
    /// Emus bitshifts 1 by the rune index instead of directly sending the index.
    /// mangostwo: mask (0x00-0x3F probably)
    pub rune: u32,
}

impl crate::private::Sealed for SMSG_ADD_RUNE_POWER {}
impl SMSG_ADD_RUNE_POWER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // rune: u32
        let rune = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            rune,
        })
    }

}

impl crate::Message for SMSG_ADD_RUNE_POWER {
    const OPCODE: u32 = 0x0488;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ADD_RUNE_POWER {{").unwrap();
        // Members
        writeln!(s, "    rune = {};", self.rune).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1160_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "rune", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // rune: u32
        w.write_all(&self.rune.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1160, "SMSG_ADD_RUNE_POWER", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ADD_RUNE_POWER {}

