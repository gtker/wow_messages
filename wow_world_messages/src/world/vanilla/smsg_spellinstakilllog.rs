use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellinstakilllog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellinstakilllog.wowm#L1):
/// ```text
/// smsg SMSG_SPELLINSTAKILLLOG = 0x032F {
///     Guid target;
///     Spell spell;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELLINSTAKILLLOG {
    pub target: Guid,
    pub spell: u32,
}

impl crate::private::Sealed for SMSG_SPELLINSTAKILLLOG {}
impl SMSG_SPELLINSTAKILLLOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            target,
            spell,
        })
    }

}

impl crate::Message for SMSG_SPELLINSTAKILLLOG {
    const OPCODE: u32 = 0x032f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELLINSTAKILLLOG"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLINSTAKILLLOG {{").unwrap();
        // Members
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 815_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(815, "SMSG_SPELLINSTAKILLLOG", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLINSTAKILLLOG {}

