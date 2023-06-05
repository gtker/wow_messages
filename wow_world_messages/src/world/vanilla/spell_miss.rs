use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::SpellMissInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L31):
/// ```text
/// struct SpellMiss {
///     Guid target;
///     SpellMissInfo miss_info;
/// }
/// ```
pub struct SpellMiss {
    pub target: Guid,
    pub miss_info: SpellMissInfo,
}

impl SpellMiss {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int().to_le_bytes()))?;

        Ok(())
    }
}

impl SpellMiss {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            target,
            miss_info,
        })
    }

}

