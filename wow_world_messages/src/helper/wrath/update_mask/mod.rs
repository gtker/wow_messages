mod impls;

pub use impls::*;

use crate::helper::update_mask_common;
use crate::helper::update_mask_common::{
    update_item, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM, PLAYER, UNIT,
};
use crate::Guid;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::collections::BTreeMap;
use std::io;
use std::io::Read;
use std::io::Write;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

update_item!(UpdateItem, ITEM);
update_item!(UpdateContainer, ITEM | CONTAINER);
update_item!(UpdateUnit, UNIT);
update_item!(UpdatePlayer, UNIT | PLAYER);
update_item!(UpdateGameObject, GAMEOBJECT);
update_item!(UpdateDynamicObject, DYNAMICOBJECT);
update_item!(UpdateCorpse, CORPSE);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum UpdateMask {
    Item(UpdateItem),
    Container(UpdateContainer),
    Unit(UpdateUnit),
    Player(UpdatePlayer),
    GameObject(UpdateGameObject),
    DynamicObject(UpdateDynamicObject),
    Corpse(UpdateCorpse),
}

impl Default for UpdateMask {
    fn default() -> Self {
        Self::Item(Default::default())
    }
}

impl UpdateMask {
    pub(crate) fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        let (header, values) = update_mask_common::read_inner(r)?;

        let ty = match values.get(&2) {
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Missing object TYPE",
                ))
            }
            Some(ty) => *ty,
        };

        Ok(if (ty & CONTAINER) != 0 {
            Self::Container(UpdateContainer::from_inners(header, values))
        } else if (ty & ITEM) != 0 {
            Self::Item(UpdateItem::from_inners(header, values))
        } else if (ty & PLAYER) != 0 {
            Self::Player(UpdatePlayer::from_inners(header, values))
        } else if (ty & UNIT) != 0 {
            Self::Unit(UpdateUnit::from_inners(header, values))
        } else if (ty & GAMEOBJECT) != 0 {
            Self::GameObject(UpdateGameObject::from_inners(header, values))
        } else if (ty & DYNAMICOBJECT) != 0 {
            Self::DynamicObject(UpdateDynamicObject::from_inners(header, values))
        } else if (ty & CORPSE) != 0 {
            Self::Corpse(UpdateCorpse::from_inners(header, values))
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Object type not valid",
            ));
        })
    }

    pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            UpdateMask::Item(i) => i.write_into_vec(v),
            UpdateMask::Container(i) => i.write_into_vec(v),
            UpdateMask::Unit(i) => i.write_into_vec(v),
            UpdateMask::Player(i) => i.write_into_vec(v),
            UpdateMask::GameObject(i) => i.write_into_vec(v),
            UpdateMask::DynamicObject(i) => i.write_into_vec(v),
            UpdateMask::Corpse(i) => i.write_into_vec(v),
        }
    }

    pub(crate) fn size(&self) -> usize {
        let (header_len, values_len) = match self {
            UpdateMask::Item(i) => (i.header.len(), i.values.len()),
            UpdateMask::Container(i) => (i.header.len(), i.values.len()),
            UpdateMask::Unit(i) => (i.header.len(), i.values.len()),
            UpdateMask::Player(i) => (i.header.len(), i.values.len()),
            UpdateMask::GameObject(i) => (i.header.len(), i.values.len()),
            UpdateMask::DynamicObject(i) => (i.header.len(), i.values.len()),
            UpdateMask::Corpse(i) => (i.header.len(), i.values.len()),
        };

        1 // amount_of_blocks
            + header_len * 4
            + values_len * 4
    }
}
