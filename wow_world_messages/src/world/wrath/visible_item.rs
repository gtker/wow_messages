use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L39):
/// ```text
/// struct VisibleItem {
///     Item item;
///     u16[2] enchants;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    pub(crate) fn from_range<'a>(mut range: impl Iterator<Item = (&'a u16, &'a u32)>) -> Option<Self> {
        // index 0: item
        let (_, &item) = range.next()?;

        // index 1: enchants
        let mut enchants = [0; 2];
        let (_, &enchants_temp) = range.next()?;
        let (enchants_temp_a, enchants_temp_b) = crate::util::u32_to_u16s(enchants_temp);
        enchants[0] = enchants_temp_a;
        enchants[1] = enchants_temp_b;

        Some(Self {
            item,
            enchants,
        })
    }

    pub(crate) const fn mask_values(&self, index: crate::wrath::VisibleItemIndex) -> [(u16, u32); 2] {
        let offset = index.offset();
        [
            (offset, self.item),

            (offset + 1, crate::util::u16s_to_u32(self.enchants[0], self.enchants[1])),

        ]
    }
}
