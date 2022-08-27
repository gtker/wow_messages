mod impls;

pub use impls::*;

use crate::Guid;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::collections::BTreeMap;
use std::io;
use std::io::Read;
use std::io::Write;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

const OBJECT: u32 = 0x0001;
const ITEM: u32 = 0x0002;
const CONTAINER: u32 = 0x0004;
const UNIT: u32 = 0x0008;
const PLAYER: u32 = 0x0010;
const GAMEOBJECT: u32 = 0x0020;
const DYNAMICOBJECT: u32 = 0x0040;
const CORPSE: u32 = 0x0080;

macro_rules! update_item {
    ($name:ident, $type_value:expr) => {
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name {
            header: Vec<u32>,
            values: BTreeMap<u16, u32>,
        }

        impl $name {
            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut header = vec![];
                let mut values = BTreeMap::new();

                header_set(&mut header, &mut values, OBJECT_FIELD_TYPE);
                values.insert(OBJECT_FIELD_TYPE, OBJECT | $type_value);

                Self { header, values }
            }

            fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
                Self { header, values }
            }

            pub(crate) fn header_set(&mut self, bit: u16) {
                header_set(&mut self.header, &mut self.values, bit);
            }

            pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
                write_into_vec(v, &self.header, &self.values)
            }
        }
    };
}

fn header_set(header: &mut Vec<u32>, values: &mut BTreeMap<u16, u32>, bit: u16) {
    let index = bit / 32;
    let offset = bit % 32;

    if index >= header.len() as u16 {
        let extras = index - header.len() as u16;
        for _ in 0..=extras {
            header.push(0);
        }
    }

    header[index as usize] |= 1 << offset;
}

fn write_into_vec(
    v: &mut Vec<u8>,
    header: &[u32],
    values: &BTreeMap<u16, u32>,
) -> Result<(), std::io::Error> {
    v.write_all(&[header.len() as u8])?;

    for h in header {
        v.write_all(h.to_le_bytes().as_slice())?;
    }

    for value in values.values() {
        v.write_all(&value.to_le_bytes())?;
    }

    Ok(())
}

update_item!(UpdateItem, ITEM);
update_item!(UpdateContainer, ITEM | CONTAINER);
update_item!(UpdateUnit, UNIT);
update_item!(UpdatePlayer, UNIT | PLAYER);
update_item!(UpdateGameObject, GAMEOBJECT);
update_item!(UpdateDynamicObject, DYNAMICOBJECT);
update_item!(UpdateCorpse, CORPSE);

#[derive(Debug, Clone, PartialEq)]
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
        let amount_of_blocks = crate::util::read_u8_le(r)?;

        let mut header = Vec::new();
        for _ in 0..amount_of_blocks {
            header.push(crate::util::read_u32_le(r)?);
        }

        let mut values = BTreeMap::new();
        for (index, block) in header.iter().enumerate() {
            for bit in 0..32 {
                if (block & 1 << bit) != 0 {
                    values.insert(index as u16 * 32 + bit, crate::util::read_u32_le(r)?);
                }
            }
        }

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
    use crate::helper::update_mask::UpdatePlayer;
    use crate::vanilla::{Class, Gender, Power, Race};
    use crate::{Guid, UpdateMask};

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

        let mut update_mask = UpdatePlayer::new()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_unit_HEALTH(100);
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

        let mut update_mask = UpdatePlayer::new()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50);
        let update_mask = UpdateMask::Player(update_mask);

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }
}
