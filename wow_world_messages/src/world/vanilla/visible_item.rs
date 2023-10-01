use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L14):
/// ```text
/// struct VisibleItem {
///     Guid creator;
///     Item item;
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

    pub(crate) fn from_range<'a>(mut range: impl Iterator<Item = (&'a u16, &'a u32)>) -> Option<Self> {
        // index 0: creator
        let (_, &creator_lower) = range.next()?;
        let (_, &creator_upper) = range.next()?;
        let creator = crate::Guid::from_u32s(creator_lower, creator_upper);

        // index 2: item
        let (_, &item) = range.next()?;

        // index 3: enchants
        let mut enchants = [0; 2];
        let (_, &enchants_temp) = range.next()?;
        enchants[0] = enchants_temp;
        let (_, &enchants_temp) = range.next()?;
        enchants[1] = enchants_temp;

        // index 10: random_property_id
        let (_, &random_property_id) = range.next()?;

        // index 11: item_suffix_factor
        let (_, &item_suffix_factor) = range.next()?;

        Some(Self {
            creator,
            item,
            enchants,
            random_property_id,
            item_suffix_factor,
        })
    }

    pub(crate) const fn mask_values(&self, index: crate::vanilla::VisibleItemIndex) -> [(u16, u32); 7] {
        let offset = index.offset();
        let (creator_lower, creator_upper) = self.creator.to_u32s();
        [
            (offset, creator_lower),
            (offset + 1, creator_upper),

            (offset + 2, self.item),

            (offset + 3, self.enchants[0]),
            (offset + 4, self.enchants[1]),

            (offset + 10, self.random_property_id),

            (offset + 11, self.item_suffix_factor),

        ]
    }
}
