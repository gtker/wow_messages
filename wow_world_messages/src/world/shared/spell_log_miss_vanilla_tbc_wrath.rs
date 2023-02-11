use crate::Guid;
use crate::shared::spell_miss_info_vanilla_vanilla_tbc_wrath::SpellMissInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L1):
/// ```text
/// struct SpellLogMiss {
///     Guid target;
///     SpellMissInfo miss_info;
/// }
/// ```
pub struct SpellLogMiss {
    pub target: Guid,
    pub miss_info: SpellMissInfo,
}

impl SpellLogMiss {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
}

impl SpellLogMiss {
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

