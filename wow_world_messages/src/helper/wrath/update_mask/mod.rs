mod impls;
mod indices;

pub use impls::*;
pub use indices::*;

use crate::helper::update_mask_common::{
    skill_info, update_item, update_mask, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM,
    PLAYER, UNIT,
};
use crate::util::{u16s_to_u32, u32_to_u16s};
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

skill_info!(wow_world_base::wrath::Skill, indices::SkillInfoIndex);

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct VisibleItem {
    entry: u32,
    enchants: [u16; 2],
}

impl VisibleItem {
    pub const fn new(entry: u32, enchants: [u16; 2]) -> Self {
        Self { entry, enchants }
    }

    pub(crate) const fn mask_values(&self, index: VisibleItemIndex) -> [(u16, u32); 2] {
        let offset = index.offset();

        let enchants = u16s_to_u32(self.enchants[0], self.enchants[1]);

        [(offset, self.entry), (offset + 1, enchants)]
    }

    pub(crate) fn from_range<'a>(
        mut range: impl Iterator<Item = (&'a u16, &'a u32)>,
    ) -> Option<Self> {
        let (_, entry) = range.next()?;
        let (_, enchants) = range.next()?;
        let (first_enchants, second_enchants) = u32_to_u16s(*enchants);

        Some(Self {
            entry: *entry,
            enchants: [first_enchants, second_enchants],
        })
    }
}
