use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/threat/smsg_threat_remove.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/threat/smsg_threat_remove.wowm#L1):
/// ```text
/// smsg SMSG_THREAT_REMOVE = 0x0484 {
///     PackedGuid unit;
///     PackedGuid victim;
/// }
/// ```
pub struct SMSG_THREAT_REMOVE {
    pub unit: Guid,
    pub victim: Guid,
}

impl crate::private::Sealed for SMSG_THREAT_REMOVE {}
impl crate::Message for SMSG_THREAT_REMOVE {
    const OPCODE: u32 = 0x0484;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // victim: PackedGuid
        self.victim.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=18).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0484, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // victim: PackedGuid
        let victim = Guid::read_packed(&mut r)?;

        Ok(Self {
            unit,
            victim,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_THREAT_REMOVE {}

impl SMSG_THREAT_REMOVE {
    pub(crate) const fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
        + self.victim.size() // victim: PackedGuid
    }
}

