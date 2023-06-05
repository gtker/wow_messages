use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::ThreatUpdateUnit;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/threat/smsg_threat_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/threat/smsg_threat_update.wowm#L1):
/// ```text
/// smsg SMSG_THREAT_UPDATE = 0x0483 {
///     PackedGuid unit;
///     u32 amount_of_units;
///     ThreatUpdateUnit[amount_of_units] units;
/// }
/// ```
pub struct SMSG_THREAT_UPDATE {
    pub unit: Guid,
    pub units: Vec<ThreatUpdateUnit>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_THREAT_UPDATE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_THREAT_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    amount_of_units = {};", self.units.len()).unwrap();
        write!(s, "    units = [").unwrap();
        for v in self.units.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    unit = {};", v.unit.guid()).unwrap();
            writeln!(s, "    threat = {};", v.threat).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1155_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_THREAT_UPDATE {}
impl crate::Message for SMSG_THREAT_UPDATE {
    const OPCODE: u32 = 0x0483;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // amount_of_units: u32
        w.write_all(&(self.units.len() as u32).to_le_bytes())?;

        // units: ThreatUpdateUnit[amount_of_units]
        for i in self.units.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0483, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

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
            units,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_THREAT_UPDATE {}

impl SMSG_THREAT_UPDATE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + 4 // amount_of_units: u32
        + self.units.iter().fold(0, |acc, x| acc + x.size()) // units: ThreatUpdateUnit[amount_of_units]
    }
}

