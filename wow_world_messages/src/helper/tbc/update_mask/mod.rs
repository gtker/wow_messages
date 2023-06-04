#[allow(clippy::missing_panics_doc)]
mod impls;
mod indices;

pub use impls::*;
pub use indices::*;

use crate::helper::update_mask_common;
use crate::helper::update_mask_common::{
    skill_info, update_item, update_mask, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM,
    PLAYER, UNIT,
};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io;
use std::io::Read;

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
