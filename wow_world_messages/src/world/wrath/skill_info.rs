use std::io::{Read, Write};

use crate::wrath::Skill;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

}
