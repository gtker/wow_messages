use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L29):
/// ```text
/// struct VisibleItem {
///     Guid creator;
///     u32 item;
///     u32[6] enchants;
///     u32 random_property_id;
///     u32 item_suffix_factor;
/// }
/// ```
pub struct VisibleItem {
    pub creator: Guid,
    pub item: u32,
    pub enchants: [u32; 6],
    pub random_property_id: u32,
    pub item_suffix_factor: u32,
}

impl VisibleItem {
    pub const fn new(creator: Guid, item: u32, enchants: [u32; 6], random_property_id: u32, item_suffix_factor: u32, ) -> Self {
        Self {
            creator,
            item,
            enchants,
            random_property_id,
            item_suffix_factor,
        }
    }

}
