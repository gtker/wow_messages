use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::SpellMissInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common_3_3_5.wowm#L49):
/// ```text
/// struct SpellMiss {
///     Guid target;
///     SpellMissInfo miss_info;
///     if (miss_info == REFLECT) {
///         u8 reflect_result;
///     }
/// }
/// ```
pub struct SpellMiss {
    pub target: Guid,
    pub miss_info: SpellMiss_SpellMissInfo,
}

impl SpellMiss {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int().to_le_bytes()))?;

        match &self.miss_info {
            SpellMiss_SpellMissInfo::Reflect {
                reflect_result,
            } => {
                // reflect_result: u8
                w.write_all(&reflect_result.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl SpellMiss {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::read_u32_le(&mut r)?.try_into()?;

        let miss_info_if = match miss_info {
            SpellMissInfo::None => SpellMiss_SpellMissInfo::None,
            SpellMissInfo::Miss => SpellMiss_SpellMissInfo::Miss,
            SpellMissInfo::Resist => SpellMiss_SpellMissInfo::Resist,
            SpellMissInfo::Dodge => SpellMiss_SpellMissInfo::Dodge,
            SpellMissInfo::Parry => SpellMiss_SpellMissInfo::Parry,
            SpellMissInfo::Block => SpellMiss_SpellMissInfo::Block,
            SpellMissInfo::Evade => SpellMiss_SpellMissInfo::Evade,
            SpellMissInfo::Immune => SpellMiss_SpellMissInfo::Immune,
            SpellMissInfo::Immune2 => SpellMiss_SpellMissInfo::Immune2,
            SpellMissInfo::Deflect => SpellMiss_SpellMissInfo::Deflect,
            SpellMissInfo::Absorb => SpellMiss_SpellMissInfo::Absorb,
            SpellMissInfo::Reflect => {
                // reflect_result: u8
                let reflect_result = crate::util::read_u8_le(&mut r)?;

                SpellMiss_SpellMissInfo::Reflect {
                    reflect_result,
                }
            }
        };

        Ok(Self {
            target,
            miss_info: miss_info_if,
        })
    }

}

impl SpellMiss {
    pub(crate) const fn size(&self) -> usize {
        8 // target: Guid
        + self.miss_info.size() // miss_info: SpellMiss_SpellMissInfo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellMiss_SpellMissInfo {
    None,
    Miss,
    Resist,
    Dodge,
    Parry,
    Block,
    Evade,
    Immune,
    Immune2,
    Deflect,
    Absorb,
    Reflect {
        reflect_result: u8,
    },
}

impl Default for SpellMiss_SpellMissInfo {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SpellMiss_SpellMissInfo {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Miss => 1,
            Self::Resist => 2,
            Self::Dodge => 3,
            Self::Parry => 4,
            Self::Block => 5,
            Self::Evade => 6,
            Self::Immune => 7,
            Self::Immune2 => 8,
            Self::Deflect => 9,
            Self::Absorb => 10,
            Self::Reflect { .. } => 11,
        }
    }

}

impl SpellMiss_SpellMissInfo {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Reflect {
                ..
            } => {
                4
                + 1 // reflect_result: u8
            }
            _ => 4,
        }
    }
}

