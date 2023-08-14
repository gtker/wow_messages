use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L14):
/// ```text
/// struct VisibleItem {
///     Guid creator;
///     u32 item;
///     u32[2] enchants;
///     u32 padding5 = 0;
///     u32 padding6 = 0;
///     u32 padding7 = 0;
///     u32 padding8 = 0;
///     u32 padding9 = 0;
///     u32 random_property_id;
///     u32 item_suffix_factor;
/// }
/// ```
pub struct VisibleItem {
    pub creator: Guid,
    pub item: u32,
    pub enchants: [u32; 2],
    pub random_property_id: u32,
    pub item_suffix_factor: u32,
}

impl VisibleItem {
    pub const fn new(creator: Guid, item: u32, enchants: [u32; 2], random_property_id: u32, item_suffix_factor: u32, ) -> Self {
        Self {
            creator,
            item,
            enchants,
            random_property_id,
            item_suffix_factor,
        }
    }

}
