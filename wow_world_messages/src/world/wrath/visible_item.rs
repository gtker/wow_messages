use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L39):
/// ```text
/// struct VisibleItem {
///     u32 item;
///     u16[2] enchants;
/// }
/// ```
pub struct VisibleItem {
    pub item: u32,
    pub enchants: [u16; 2],
}

impl VisibleItem {
    pub const fn new(item: u32, enchants: [u16; 2], ) -> Self {
        Self {
            item,
            enchants,
        }
    }

}
