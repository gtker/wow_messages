use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::SpellCastResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm#L1):
/// ```text
/// smsg SMSG_SPELL_FAILURE = 0x0133 {
///     Guid guid;
///     u32 spell;
///     SpellCastResult result;
/// }
/// ```
pub struct SMSG_SPELL_FAILURE {
    pub guid: Guid,
    pub spell: u32,
    pub result: SpellCastResult,
}

impl crate::private::Sealed for SMSG_SPELL_FAILURE {}
impl SMSG_SPELL_FAILURE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 13 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0133, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // result: SpellCastResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            spell,
            result,
        })
    }

}

impl crate::Message for SMSG_SPELL_FAILURE {
    const OPCODE: u32 = 0x0133;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELL_FAILURE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 307_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELL_FAILURE {}

