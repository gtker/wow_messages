use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm#L3):
/// ```text
/// smsg SMSG_SPELLORDAMAGE_IMMUNE = 0x0263 {
///     Guid caster;
///     Guid target;
///     u32 id;
///     Bool debug_log_format;
/// }
/// ```
pub struct SMSG_SPELLORDAMAGE_IMMUNE {
    pub caster: Guid,
    pub target: Guid,
    pub id: u32,
    pub debug_log_format: bool,
}

impl crate::private::Sealed for SMSG_SPELLORDAMAGE_IMMUNE {}
impl crate::Message for SMSG_SPELLORDAMAGE_IMMUNE {
    const OPCODE: u32 = 0x0263;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLORDAMAGE_IMMUNE {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    debug_log_format = {};", if self.debug_log_format { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 23_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 611_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "debug_log_format", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // debug_log_format: Bool
        w.write_all(u8::from(self.debug_log_format).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 21 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0263, size: body_size });
        }

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // debug_log_format: Bool
        let debug_log_format = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            caster,
            target,
            id,
            debug_log_format,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {}

