use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L29):
/// ```text
/// struct VisibleItem {
///     Guid creator;
///     Item item;
///     u32[6] enchants;
///     u32 random_property_id;
///     u32 item_suffix_factor;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    pub(crate) fn from_range<'a>(mut range: impl Iterator<Item = (&'a u16, &'a u32)>) -> Option<Self> {
        // index 0: creator
        let (_, &creator_lower) = range.next()?;
        let (_, &creator_upper) = range.next()?;
        let creator = crate::Guid::from_u32s(creator_lower, creator_upper);

        // index 2: item
        let (_, &item) = range.next()?;

        // index 3: enchants
        let mut enchants = [0; 6];
        let (_, &enchants_temp) = range.next()?;
        enchants[0] = enchants_temp;
        let (_, &enchants_temp) = range.next()?;
        enchants[1] = enchants_temp;
        let (_, &enchants_temp) = range.next()?;
        enchants[2] = enchants_temp;
        let (_, &enchants_temp) = range.next()?;
        enchants[3] = enchants_temp;
        let (_, &enchants_temp) = range.next()?;
        enchants[4] = enchants_temp;
        let (_, &enchants_temp) = range.next()?;
        enchants[5] = enchants_temp;

        // index 9: random_property_id
        let (_, &random_property_id) = range.next()?;

        // index 10: item_suffix_factor
        let (_, &item_suffix_factor) = range.next()?;

        Some(Self {
            creator,
            item,
            enchants,
            random_property_id,
            item_suffix_factor,
        })
    }

    pub(crate) const fn mask_values(&self, index: crate::tbc::VisibleItemIndex) -> [(u16, u32); 11] {
        let offset = index.offset();
        let (creator_lower, creator_upper) = self.creator.to_u32s();
        [
            (offset, creator_lower),
            (offset + 1, creator_upper),

            (offset + 2, self.item),

            (offset + 3, self.enchants[0]),
            (offset + 4, self.enchants[1]),
            (offset + 5, self.enchants[2]),
            (offset + 6, self.enchants[3]),
            (offset + 7, self.enchants[4]),
            (offset + 8, self.enchants[5]),

            (offset + 9, self.random_property_id),

            (offset + 10, self.item_suffix_factor),

        ]
    }
}
