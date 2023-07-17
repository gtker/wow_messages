#[allow(clippy::missing_panics_doc)]
mod impls;
mod indices;

pub use impls::*;
pub use indices::*;

use crate::helper::update_mask_common::{
    skill_info, update_item, update_mask, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM,
    PLAYER, UNIT,
};
use std::convert::TryFrom;

update_item!(UpdateItem, UpdateItemBuilder, ITEM);
update_item!(UpdateContainer, UpdateContainerBuilder, ITEM | CONTAINER);
update_item!(UpdateUnit, UpdateUnitBuilder, UNIT);
update_item!(UpdatePlayer, UpdatePlayerBuilder, UNIT | PLAYER);
update_item!(UpdateGameObject, UpdateGameObjectBuilder, GAMEOBJECT);
update_item!(
    UpdateDynamicObject,
    UpdateDynamicObjectBuilder,
    DYNAMICOBJECT
);
update_item!(UpdateCorpse, UpdateCorpseBuilder, CORPSE);

update_mask!();

skill_info!(wow_world_base::tbc::Skill, indices::SkillInfoIndex);

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct VisibleItem {
    creator: crate::Guid,
    entry: u32,
    enchants: [u32; 6],
    random_property_id: u32,
    item_suffix_factor: u32,
}

impl VisibleItem {
    pub const fn new(
        creator: crate::Guid,
        entry: u32,
        enchants: [u32; 6],
        random_property_id: u32,
        item_suffix_factor: u32,
    ) -> Self {
        Self {
            creator,
            entry,
            enchants,
            random_property_id,
            item_suffix_factor,
        }
    }

    pub(crate) const fn mask_values(&self, index: VisibleItemIndex) -> [(u16, u32); 12] {
        let offset = index.offset();

        let (guid_lower, guid_upper) = self.creator.to_u32s();

        [
            (offset, guid_lower),
            (offset + 1, guid_upper),
            (offset + 2, self.entry),
            (offset + 3, self.enchants[0]),
            (offset + 4, self.enchants[1]),
            (offset + 5, self.enchants[2]),
            (offset + 6, self.enchants[3]),
            (offset + 7, self.enchants[4]),
            (offset + 8, self.enchants[5]),
            (offset + 9, self.enchants[6]),
            (offset + 10, self.random_property_id),
            (offset + 11, self.item_suffix_factor),
        ]
    }

    pub(crate) fn from_range<'a>(
        mut range: impl Iterator<Item = (&'a u16, &'a u32)>,
    ) -> Option<Self> {
        let (_, guid_lower) = range.next()?;
        let (_, guid_upper) = range.next()?;
        let (_, entry) = range.next()?;
        let (_, first_enchants) = range.next()?;
        let (_, second_enchants) = range.next()?;
        let (_, third_enchants) = range.next()?;
        let (_, fourth_enchants) = range.next()?;
        let (_, fifth_enchants) = range.next()?;
        let (_, sixth_enchants) = range.next()?;
        let (_, random_property_id) = range.next()?;
        let (_, item_suffix_factor) = range.next()?;

        let creator = crate::Guid::from_u32s(*guid_lower, *guid_upper);

        Some(Self {
            creator,
            entry: *entry,
            enchants: [
                *first_enchants,
                *second_enchants,
                *third_enchants,
                *fourth_enchants,
                *fifth_enchants,
                *sixth_enchants,
            ],
            random_property_id: *random_property_id,
            item_suffix_factor: *item_suffix_factor,
        })
    }
}
