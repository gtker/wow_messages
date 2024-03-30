use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/threat/smsg_highest_threat_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/threat/smsg_highest_threat_update.wowm#L1):
/// ```text
/// struct ThreatUpdateUnit {
///     PackedGuid unit;
///     u32 threat;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ThreatUpdateUnit {
    pub unit: Guid,
    pub threat: u32,
}

impl ThreatUpdateUnit {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // threat: u32
        w.write_all(&self.threat.to_le_bytes())?;

        Ok(())
    }
}

impl ThreatUpdateUnit {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // threat: u32
        let threat = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unit,
            threat,
        })
    }

}

impl ThreatUpdateUnit {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + 4 // threat: u32
    }
}

