use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_list.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_list.wowm#L9):
/// ```text
/// smsg SMSG_LOOT_LIST = 0x03F9 {
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

impl crate::Message for SMSG_LOOT_LIST {
    const OPCODE: u32 = 0x03f9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // master_looter: PackedGuid
        self.master_looter.write_packed_guid_into_vec(w)?;

        // group_looter: PackedGuid
        self.group_looter.write_packed_guid_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(12..=26).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F9, size: body_size as u32 });
        }

        // creature: Guid
        let creature = Guid::read(r)?;

        // master_looter: PackedGuid
        let master_looter = Guid::read_packed(r)?;

        // group_looter: PackedGuid
        let group_looter = Guid::read_packed(r)?;

        Ok(Self {
            creature,
            master_looter,
            group_looter,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_LIST {}

impl SMSG_LOOT_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // creature: Guid
        + self.master_looter.size() // master_looter: Guid
        + self.group_looter.size() // group_looter: Guid
    }
}

