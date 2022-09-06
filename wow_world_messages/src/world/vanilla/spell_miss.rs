use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellMissInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L27):
/// ```text
/// struct SpellMiss {
///     Guid target_guid;
///     SpellMissInfo miss_info;
/// }
/// ```
pub struct SpellMiss {
    pub target_guid: Guid,
    pub miss_info: SpellMissInfo,
}

impl SpellMiss {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
}

impl SpellMiss {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

}

