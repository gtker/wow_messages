use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L29):
/// ```text
/// struct LfgPlayerMember {
///     PackedGuid guid;
///     Level32 level;
/// }
/// ```
pub struct LfgPlayerMember {
    pub guid: Guid,
    pub level: Level,
}

impl LfgPlayerMember {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // level: Level32
        w.write_all(&u32::from(self.level.as_int()).to_le_bytes())?;

        Ok(())
    }
}

impl LfgPlayerMember {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // level: Level32
        let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        Ok(Self {
            guid,
            level,
        })
    }

}

impl LfgPlayerMember {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // level: Level32
    }
}

