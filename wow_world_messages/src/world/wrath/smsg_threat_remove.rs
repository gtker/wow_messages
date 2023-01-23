use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_threat_remove.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_threat_remove.wowm#L1):
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

impl crate::Message for SMSG_THREAT_REMOVE {
    const OPCODE: u32 = 0x0484;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w);

        // victim: PackedGuid
        self.victim.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=18).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0484, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        // victim: PackedGuid
        let victim = Guid::read_packed(r)?;

        Ok(Self {
            unit,
            victim,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_THREAT_REMOVE {}

impl SMSG_THREAT_REMOVE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + self.victim.size() // victim: Guid
    }
}

