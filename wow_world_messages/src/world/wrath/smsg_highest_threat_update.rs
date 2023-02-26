use crate::Guid;
use crate::wrath::ThreatUpdateUnit;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_HIGHEST_THREAT_UPDATE {
    const OPCODE: u32 = 0x0482;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w)?;

        // new_victim: PackedGuid
        self.new_victim.write_packed_guid_into_vec(w)?;

        // amount_of_units: u32
        w.write_all(&(self.units.len() as u32).to_le_bytes())?;

        // units: ThreatUpdateUnit[amount_of_units]
        for i in self.units.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0482, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        // new_victim: PackedGuid
        let new_victim = Guid::read_packed(r)?;

        // amount_of_units: u32
        let amount_of_units = crate::util::read_u32_le(r)?;

        // units: ThreatUpdateUnit[amount_of_units]
        let units = {
            let mut units = Vec::with_capacity(amount_of_units as usize);
            for i in 0..amount_of_units {
                units.push(ThreatUpdateUnit::read(r)?);
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_HIGHEST_THREAT_UPDATE {}

impl SMSG_HIGHEST_THREAT_UPDATE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + self.new_victim.size() // new_victim: Guid
        + 4 // amount_of_units: u32
        + self.units.iter().fold(0, |acc, x| acc + x.size()) // units: ThreatUpdateUnit[amount_of_units]
    }
}

