use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/threat/smsg_threat_clear.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/threat/smsg_threat_clear.wowm#L1):
/// ```text
/// smsg SMSG_THREAT_CLEAR = 0x0485 {
///     PackedGuid unit;
/// }
/// ```
pub struct SMSG_THREAT_CLEAR {
    pub unit: Guid,
}

impl crate::private::Sealed for SMSG_THREAT_CLEAR {}
impl crate::Message for SMSG_THREAT_CLEAR {
    const OPCODE: u32 = 0x0485;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0485, size: body_size });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        Ok(Self {
            unit,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_THREAT_CLEAR {}

impl SMSG_THREAT_CLEAR {
    pub(crate) const fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
    }
}

