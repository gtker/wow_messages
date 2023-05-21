use std::io::{Read, Write};

use crate::Guid;
use crate::shared::spell_log_miss_vanilla_tbc_wrath::SpellLogMiss;
use wow_world_base::shared::spell_miss_info_vanilla_vanilla_tbc_wrath::SpellMissInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L8):
/// ```text
/// smsg SMSG_SPELLLOGMISS = 0x024B {
///     u32 id;
///     Guid caster;
///     u8 unknown1;
///     u32 amount_of_targets;
///     SpellLogMiss[amount_of_targets] targets;
/// }
/// ```
pub struct SMSG_SPELLLOGMISS {
    pub id: u32,
    pub caster: Guid,
    /// cmangos/mangoszero: can be 0 or 1
    pub unknown1: u8,
    pub targets: Vec<SpellLogMiss>,
}

impl crate::private::Sealed for SMSG_SPELLLOGMISS {}
impl SMSG_SPELLLOGMISS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(17..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(&mut r)?;

        // targets: SpellLogMiss[amount_of_targets]
        let targets = {
            let mut targets = Vec::with_capacity(amount_of_targets as usize);
            for _ in 0..amount_of_targets {
                targets.push(SpellLogMiss::read(&mut r)?);
            }
            targets
        };

        Ok(Self {
            id,
            caster,
            unknown1,
            targets,
        })
    }

}

impl crate::Message for SMSG_SPELLLOGMISS {
    const OPCODE: u32 = 0x024b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLLOGMISS {{").unwrap();
        // Members
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    amount_of_targets = {};", self.targets.len()).unwrap();
        write!(s, "    targets = [").unwrap();
        for v in self.targets.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        target = {};", v.target.guid()).unwrap();
            writeln!(s, "        miss_info = {};", v.miss_info.as_test_case_value()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 587_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_targets", "    ");
        if !self.targets.is_empty() {
            writeln!(s, "    /* targets: SpellLogMiss[amount_of_targets] start */").unwrap();
            for (i, v) in self.targets.iter().enumerate() {
                writeln!(s, "    /* targets: SpellLogMiss[amount_of_targets] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "miss_info", "        ");
                writeln!(s, "    /* targets: SpellLogMiss[amount_of_targets] {i} end */").unwrap();
            }
            writeln!(s, "    /* targets: SpellLogMiss[amount_of_targets] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: SpellLogMiss[amount_of_targets]
        for i in self.targets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(587, "SMSG_SPELLLOGMISS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLLOGMISS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLLOGMISS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLLOGMISS {}

impl SMSG_SPELLLOGMISS {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + 8 // caster: Guid
        + 1 // unknown1: u8
        + 4 // amount_of_targets: u32
        + self.targets.len() * 12 // targets: SpellLogMiss[amount_of_targets]
    }
}

