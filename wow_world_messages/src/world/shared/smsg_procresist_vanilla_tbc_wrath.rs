use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::log_format_vanilla_tbc_wrath::LogFormat;

/// According to cmangos/azerothcore/trinitycore/mangostwo. Not present in vmangos.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_procresist.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_procresist.wowm#L9):
/// ```text
/// smsg SMSG_PROCRESIST = 0x0260 {
///     Guid caster;
///     Guid target;
///     Spell id;
///     LogFormat log_format;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PROCRESIST {
    pub caster: Guid,
    pub target: Guid,
    pub id: u32,
    pub log_format: LogFormat,
}

impl crate::private::Sealed for SMSG_PROCRESIST {}
impl SMSG_PROCRESIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 21 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // id: Spell
        let id = crate::util::read_u32_le(&mut r)?;

        // log_format: LogFormat
        let log_format = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            caster,
            target,
            id,
            log_format,
        })
    }

}

impl crate::Message for SMSG_PROCRESIST {
    const OPCODE: u32 = 0x0260;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PROCRESIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PROCRESIST {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    log_format = {};", self.log_format.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 23_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 608_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "log_format", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3".to_string())).unwrap();
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

        // id: Spell
        w.write_all(&self.id.to_le_bytes())?;

        // log_format: LogFormat
        w.write_all(&(self.log_format.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(608, "SMSG_PROCRESIST", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PROCRESIST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PROCRESIST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PROCRESIST {}

