use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm#L1):
/// ```text
/// smsg SMSG_SPELLDISPELLOG = 0x027B {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 amount_of_spells;
///     u32[amount_of_spells] spells;
/// }
/// ```
pub struct SMSG_SPELLDISPELLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub spells: Vec<u32>,
}

impl crate::private::Sealed for SMSG_SPELLDISPELLOG {}
impl SMSG_SPELLDISPELLOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x027B, size: body_size });
        }

        // victim: PackedGuid
        let victim = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(&mut r)?;

        // spells: u32[amount_of_spells]
        let spells = {
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for _ in 0..amount_of_spells {
                spells.push(crate::util::read_u32_le(&mut r)?);
            }
            spells
        };

        Ok(Self {
            victim,
            caster,
            spells,
        })
    }

}

impl crate::Message for SMSG_SPELLDISPELLOG {
    const OPCODE: u32 = 0x027b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLDISPELLOG {{").unwrap();
        // Members
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    amount_of_spells = {};", self.spells.len()).unwrap();
        write!(s, "    spells = [").unwrap();
        for v in self.spells.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 635_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.victim), "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_spells", "    ");
        if !self.spells.is_empty() {
            writeln!(s, "    /* spells: u32[amount_of_spells] start */").unwrap();
            for (i, v) in self.spells.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("spells {i}"), "    ");
            }
            writeln!(s, "    /* spells: u32[amount_of_spells] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // victim: PackedGuid
        crate::util::write_packed_guid(&self.victim, &mut w)?;

        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: u32[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLDISPELLOG {}

impl SMSG_SPELLDISPELLOG {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.victim) // victim: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // amount_of_spells: u32
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
    }
}

