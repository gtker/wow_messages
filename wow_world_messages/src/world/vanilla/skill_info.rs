use std::io::{Read, Write};

use crate::vanilla::Skill;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L3):
/// ```text
/// struct SkillInfo {
///     Skill skill;
///     u16 skill_step;
///     u16 minimum;
///     u16 maximum;
///     u16 permanent_bonus;
///     u16 temporary_bonus;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SkillInfo {
    pub skill: Skill,
    pub skill_step: u16,
    pub minimum: u16,
    pub maximum: u16,
    pub permanent_bonus: u16,
    pub temporary_bonus: u16,
}

impl SkillInfo {
    pub const fn new(skill: Skill, skill_step: u16, minimum: u16, maximum: u16, permanent_bonus: u16, temporary_bonus: u16, ) -> Self {
        Self {
            skill,
            skill_step,
            minimum,
            maximum,
            permanent_bonus,
            temporary_bonus,
        }
    }

    pub(crate) fn from_range<'a>(mut range: impl Iterator<Item = (&'a u16, &'a u32)>) -> Option<Self> {
        // index 0: skill and skill_step
        let (_, &skill) = range.next()?;
        let (skill_step, skill) = crate::util::u32_to_u16s(skill);
        let skill = skill.try_into().ok()?;

        // index 1: minimum and maximum
        let (_, &minimum) = range.next()?;
        let (maximum, minimum) = crate::util::u32_to_u16s(minimum);

        // index 2: permanent_bonus and temporary_bonus
        let (_, &permanent_bonus) = range.next()?;
        let (temporary_bonus, permanent_bonus) = crate::util::u32_to_u16s(permanent_bonus);

        Some(Self {
            skill,
            skill_step,
            minimum,
            maximum,
            permanent_bonus,
            temporary_bonus,
        })
    }

    pub(crate) const fn mask_values(&self, index: crate::vanilla::SkillInfoIndex) -> [(u16, u32); 3] {
        let offset = index.offset();
        [
            (offset, crate::util::u16s_to_u32(self.skill_step, self.skill.as_int())),

            (offset + 1, crate::util::u16s_to_u32(self.maximum, self.minimum)),

            (offset + 2, crate::util::u16s_to_u32(self.temporary_bonus, self.permanent_bonus)),

        ]
    }
}
