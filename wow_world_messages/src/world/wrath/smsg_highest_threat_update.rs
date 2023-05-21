use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::ThreatUpdateUnit;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/threat/smsg_highest_threat_update.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/threat/smsg_highest_threat_update.wowm#L8):
/// ```text
/// smsg SMSG_HIGHEST_THREAT_UPDATE = 0x0482 {
///     PackedGuid unit;
///     PackedGuid new_victim;
///     u32 amount_of_units;
///     ThreatUpdateUnit[amount_of_units] units;
/// }
/// ```
pub struct SMSG_HIGHEST_THREAT_UPDATE {
    pub unit: Guid,
    pub new_victim: Guid,
    pub units: Vec<ThreatUpdateUnit>,
}

impl crate::private::Sealed for SMSG_HIGHEST_THREAT_UPDATE {}
impl SMSG_HIGHEST_THREAT_UPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0482, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // new_victim: PackedGuid
        let new_victim = crate::util::read_packed_guid(&mut r)?;

        // amount_of_units: u32
        let amount_of_units = crate::util::read_u32_le(&mut r)?;

        // units: ThreatUpdateUnit[amount_of_units]
        let units = {
            let mut units = Vec::with_capacity(amount_of_units as usize);
            for _ in 0..amount_of_units {
                units.push(ThreatUpdateUnit::read(&mut r)?);
            }
            units
        };

        Ok(Self {
            unit,
            new_victim,
            units,
        })
    }

}

impl crate::Message for SMSG_HIGHEST_THREAT_UPDATE {
    const OPCODE: u32 = 0x0482;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_HIGHEST_THREAT_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    new_victim = {};", self.new_victim.guid()).unwrap();
        writeln!(s, "    amount_of_units = {};", self.units.len()).unwrap();
        write!(s, "    units = [").unwrap();
        for v in self.units.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        unit = {};", v.unit.guid()).unwrap();
            writeln!(s, "        threat = {};", v.threat).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1154_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.new_victim), "new_victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_units", "    ");
        if !self.units.is_empty() {
            writeln!(s, "    /* units: ThreatUpdateUnit[amount_of_units] start */").unwrap();
            for (i, v) in self.units.iter().enumerate() {
                writeln!(s, "    /* units: ThreatUpdateUnit[amount_of_units] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.unit), "unit", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "threat", "        ");
                writeln!(s, "    /* units: ThreatUpdateUnit[amount_of_units] {i} end */").unwrap();
            }
            writeln!(s, "    /* units: ThreatUpdateUnit[amount_of_units] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // new_victim: PackedGuid
        crate::util::write_packed_guid(&self.new_victim, &mut w)?;

        // amount_of_units: u32
        w.write_all(&(self.units.len() as u32).to_le_bytes())?;

        // units: ThreatUpdateUnit[amount_of_units]
        for i in self.units.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_HIGHEST_THREAT_UPDATE {}

impl SMSG_HIGHEST_THREAT_UPDATE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + crate::util::packed_guid_size(&self.new_victim) // new_victim: PackedGuid
        + 4 // amount_of_units: u32
        + self.units.iter().fold(0, |acc, x| acc + x.size()) // units: ThreatUpdateUnit[amount_of_units]
    }
}

