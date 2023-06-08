use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm#L1):
/// ```text
/// smsg SMSG_SUPERCEDED_SPELL = 0x012C {
///     u16 new_spell_id;
///     u16 old_spell_id;
/// }
/// ```
pub struct SMSG_SUPERCEDED_SPELL {
    pub new_spell_id: u16,
    pub old_spell_id: u16,
}

impl crate::private::Sealed for SMSG_SUPERCEDED_SPELL {}
impl crate::Message for SMSG_SUPERCEDED_SPELL {
    const OPCODE: u32 = 0x012c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SUPERCEDED_SPELL {{").unwrap();
        // Members
        writeln!(s, "    new_spell_id = {};", self.new_spell_id).unwrap();
        writeln!(s, "    old_spell_id = {};", self.old_spell_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 300_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 2, "new_spell_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "old_spell_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // new_spell_id: u16
        w.write_all(&self.new_spell_id.to_le_bytes())?;

        // old_spell_id: u16
        w.write_all(&self.old_spell_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x012C, size: body_size });
        }

        // new_spell_id: u16
        let new_spell_id = crate::util::read_u16_le(&mut r)?;

        // old_spell_id: u16
        let old_spell_id = crate::util::read_u16_le(&mut r)?;

        Ok(Self {
            new_spell_id,
            old_spell_id,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SUPERCEDED_SPELL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SUPERCEDED_SPELL {}

