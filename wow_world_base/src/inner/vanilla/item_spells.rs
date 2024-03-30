use std::io::{Read, Write};

use crate::vanilla::SpellTriggerType;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L40):
/// ```text
/// struct ItemSpells {
///     Spell spell;
///     (u32)SpellTriggerType spell_trigger;
///     i32 spell_charges;
///     i32 spell_cooldown;
///     u32 spell_category;
///     i32 spell_category_cooldown;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ItemSpells {
    pub spell: u32,
    pub spell_trigger: SpellTriggerType,
    /// let the database control the sign here. negative means that the item should be consumed once the charges are consumed.
    pub spell_charges: i32,
    pub spell_cooldown: i32,
    pub spell_category: u32,
    pub spell_category_cooldown: i32,
}

