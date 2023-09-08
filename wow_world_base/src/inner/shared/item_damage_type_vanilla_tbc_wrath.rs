use std::io::{Read, Write};

use crate::shared::spell_school_vanilla_tbc_wrath::SpellSchool;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:85`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L85):
/// ```text
/// struct ItemDamageType {
///     f32 damage_minimum;
///     f32 damage_maximum;
///     (u32)SpellSchool school;
/// }
/// ```
pub struct ItemDamageType {
    pub damage_minimum: f32,
    pub damage_maximum: f32,
    pub school: SpellSchool,
}

