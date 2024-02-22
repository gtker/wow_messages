use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_update_chain_targets.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_update_chain_targets.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_UPDATE_CHAIN_TARGETS = 0x0330 {
///     Guid caster;
///     Spell spell;
///     u32 amount_of_targets;
///     Guid[amount_of_targets] targets;
/// }
/// ```
pub struct SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub caster: Guid,
    pub spell: u32,
    pub targets: Vec<Guid>,
}

impl crate::private::Sealed for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}
impl SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(&mut r)?;

        // targets: Guid[amount_of_targets]
        let targets = {
            let mut targets = Vec::with_capacity(amount_of_targets as usize);

            let allocation_size = u64::from(amount_of_targets) * 8;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_targets {
                targets.push(crate::util::read_guid(&mut r)?);
            }
            targets
        };

        Ok(Self {
            caster,
            spell,
            targets,
        })
    }

}

impl crate::Message for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    const OPCODE: u32 = 0x0330;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELL_UPDATE_CHAIN_TARGETS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELL_UPDATE_CHAIN_TARGETS {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    amount_of_targets = {};", self.targets.len()).unwrap();
        writeln!(s, "    targets = [").unwrap();
        for v in self.targets.as_slice() {
            write!(s, "{v:#08X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 816_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_targets", "    ");
        if !self.targets.is_empty() {
            writeln!(s, "    /* targets: Guid[amount_of_targets] start */").unwrap();
            for (i, v) in self.targets.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("targets {i}"), "    ");
            }
            writeln!(s, "    /* targets: Guid[amount_of_targets] end */").unwrap();
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
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: Guid[amount_of_targets]
        for i in self.targets.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(816, "SMSG_SPELL_UPDATE_CHAIN_TARGETS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

impl SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub(crate) fn size(&self) -> usize {
        8 // caster: Guid
        + 4 // spell: Spell
        + 4 // amount_of_targets: u32
        + self.targets.len() *  8 // targets: Guid[amount_of_targets]
    }
}

