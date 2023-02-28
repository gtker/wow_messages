use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L30):
/// ```text
/// struct LfgPlayerMember {
///     PackedGuid guid;
///     u32 level;
/// }
/// ```
pub struct LfgPlayerMember {
    pub guid: Guid,
    pub level: u32,
}

impl LfgPlayerMember {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        Ok(())
    }
}

impl LfgPlayerMember {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // level: u32
        let level = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            level,
        })
    }

}

impl LfgPlayerMember {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // level: u32
    }
}

