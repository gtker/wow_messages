#[rustfmt::skip]
#[allow(clippy::missing_panics_doc)]
mod impls;
#[rustfmt::skip]
mod indices;

pub use indices::*;

use crate::helper::update_mask_common::{
    update_item, update_mask, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM, PLAYER, UNIT,
};

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
