use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::SpellCastResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm#L9):
/// ```text
/// smsg SMSG_SPELL_FAILURE = 0x0133 {
///     Guid guid;
///     u8 extra_casts;
///     u32 spell;
///     SpellCastResult result;
/// }
/// ```
pub struct SMSG_SPELL_FAILURE {
    pub guid: Guid,
    pub extra_casts: u8,
    pub spell: u32,
    pub result: SpellCastResult,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SPELL_FAILURE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELL_FAILURE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    extra_casts = {};", self.extra_casts).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 307_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "extra_casts", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_SPELL_FAILURE {}
impl crate::Message for SMSG_SPELL_FAILURE {
    const OPCODE: u32 = 0x0133;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_SPELL_FAILURE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        14
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // extra_casts: u8
        w.write_all(&self.extra_casts.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 14 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0133, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // extra_casts: u8
        let extra_casts = crate::util::read_u8_le(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // result: SpellCastResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            extra_casts,
            spell,
            result,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_FAILURE {}

