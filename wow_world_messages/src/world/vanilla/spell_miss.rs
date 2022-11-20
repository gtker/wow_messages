use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellMissInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
}

impl SpellMiss {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // target: Guid
        let target = Guid::read(r)?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            target,
            miss_info,
        })
    }

}

