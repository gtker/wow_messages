use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_list.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_LIST = 0x03F8 {
///     Guid creature;
///     PackedGuid master_looter;
///     PackedGuid group_looter;
/// }
/// ```
pub struct SMSG_LOOT_LIST {
    pub creature: Guid,
    pub master_looter: Guid,
    pub group_looter: Guid,
}

impl crate::private::Sealed for SMSG_LOOT_LIST {}
impl crate::Message for SMSG_LOOT_LIST {
    const OPCODE: u32 = 0x03f8;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // master_looter: PackedGuid
        self.master_looter.write_packed_guid_into_vec(&mut w)?;

        // group_looter: PackedGuid
        self.group_looter.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=26).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F8, size: body_size });
        }

        // creature: Guid
        let creature = Guid::read(&mut r)?;

        // master_looter: PackedGuid
        let master_looter = Guid::read_packed(&mut r)?;

        // group_looter: PackedGuid
        let group_looter = Guid::read_packed(&mut r)?;

        Ok(Self {
            creature,
            master_looter,
            group_looter,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_LIST {}

impl SMSG_LOOT_LIST {
    pub(crate) const fn size(&self) -> usize {
        8 // creature: Guid
        + self.master_looter.size() // master_looter: PackedGuid
        + self.group_looter.size() // group_looter: PackedGuid
    }
}

