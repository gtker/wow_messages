#[allow(clippy::missing_panics_doc)]
mod impls;

pub use impls::*;

use crate::helper::update_mask_common;
use crate::helper::update_mask_common::{
    update_item, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM, PLAYER, UNIT,
};
use std::collections::BTreeMap;
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
                ));
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

    /*
       Object
            Item
                Container
            Unit
                Player
            GameObject
            DynamicObject
            Corpse
    */

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

#[cfg(test)]
mod test {
    use crate::vanilla::update_mask::UpdatePlayer;
    use crate::vanilla::UpdateMask;
    use crate::vanilla::{Class, Gender, Power, Race};
    use crate::Guid;

    #[test]
    fn most_minimal_example() {
        let b = [
            2_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            7, 0, 64, 0, 16, 0, 0, 0, // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
        ];

        let update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_unit_HEALTH(100)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_header_add_one() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50)
            .finalize();
        update_mask.header_reset();

        update_mask.set_object_GUID(4.into());

        let update_mask = UpdateMask::Player(update_mask);

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_header() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
            // End of mask blocks
            // No value blocks
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50)
            .finalize();

        update_mask.header_reset();
        let update_mask = UpdateMask::Player(update_mask);

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn small_example() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            23, 0, 64, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            0, 0, 128, 63, // Scale (1.0)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            100, 0, 0, 0, // UNIT_FIELD_MAXHEALTH (100)
            1, 0, 0, 0, // UNIT_FIELD_LEVEL (1)
            1, 0, 0, 0, // UNIT_FIELD_FACTIONTEMPLATE (1)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            50, 0, 0, 0, // UNIT_FIELD_DISPLAYD (50, Human Female)
            50, 0, 0, 0, // UNIT_FIELD_NATIVEDISPLAYID (50, Human Female)
        ];

        let update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }
}
