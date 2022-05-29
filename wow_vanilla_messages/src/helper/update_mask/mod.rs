mod impls;

pub use impls::*;
use std::collections::{BTreeMap, HashMap};

use crate::Guid;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io;
use std::io::Read;
use std::io::Write;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

macro_rules! update_item {
    ($name:ident) => {
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name {
            header: Vec<u32>,
            values: BTreeMap<u16, u32>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    header: vec![],
                    values: Default::default(),
                }
            }

            fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
                Self { header, values }
            }

            pub(crate) fn header_set(&mut self, bit: u16) {
                header_set(&mut self.header, &mut self.values, bit);
            }

            pub(crate) fn as_bytes(&self) -> Vec<u8> {
                as_bytes(&self.header, &self.values)
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

fn as_bytes(header: &[u32], values: &BTreeMap<u16, u32>) -> Vec<u8> {
    let mut v = Vec::new();

    v.write_all(&[header.len() as u8]).unwrap();

    for h in header {
        v.write_all(h.to_le_bytes().as_slice()).unwrap();
    }

    for (_, value) in values {
        v.write_all(&value.to_le_bytes()).unwrap();
    }

    v
}

update_item!(UpdateItem);
update_item!(UpdateContainer);
update_item!(UpdateUnit);
update_item!(UpdatePlayer);
update_item!(UpdateGameObject);
update_item!(UpdateDynamicObject);
update_item!(UpdateCorpse);

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
    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        let amount_of_blocks = crate::util::read_u8_le(r)?;

        let mut header = Vec::new();
        for _ in 0..amount_of_blocks {
            header.push(crate::util::read_u32_le(r)?);
        }

        let mut values = BTreeMap::new();
        let mut index = 0;
        for block in &header {
            for bit in 0..32 {
                if (block & 1 << bit) != 0 {
                    values.insert(index * 32 + bit, crate::util::read_u32_le(r)?);
                }
            }

            index += 1;
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

        const OBJECT: u32 = 0x0001;
        const ITEM: u32 = 0x0002;
        const CONTAINER: u32 = 0x0004;
        const UNIT: u32 = 0x0008;
        const PLAYER: u32 = 0x0010;
        const GAMEOBJECT: u32 = 0x0020;
        const DYNAMICOBJECT: u32 = 0x0040;
        const CORPSE: u32 = 0x0080;

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

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            UpdateMask::Item(i) => i.as_bytes(),
            UpdateMask::Container(i) => i.as_bytes(),
            UpdateMask::Unit(i) => i.as_bytes(),
            UpdateMask::Player(i) => i.as_bytes(),
            UpdateMask::GameObject(i) => i.as_bytes(),
            UpdateMask::DynamicObject(i) => i.as_bytes(),
            UpdateMask::Corpse(i) => i.as_bytes(),
        }
    }

    pub fn size(&self) -> usize {
        self.as_bytes().len()
    }

    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod test {
    use crate::helper::update_mask::UpdatePlayer;
    use crate::v1::v12::{Class, Gender, Power, Race};
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

        let mut update_mask = UpdatePlayer::new();

        update_mask.set_object_GUID(Guid::new(4));
        update_mask.set_object_TYPE(25);
        update_mask.set_unit_BYTES_0(1 << 24 | 1 << 16 | 1 << 8 | 1);
        update_mask.set_unit_HEALTH(100);

        let mut v = update_mask.as_bytes();
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

        let mut update_mask = UpdatePlayer::new();

        update_mask.set_object_GUID(Guid::new(4));
        update_mask.set_object_TYPE(25);
        update_mask.set_unit_BYTES_0(1 << 24 | 1 << 16 | 1 << 8 | 1);
        update_mask.set_object_SCALE_X(1.0);
        update_mask.set_unit_HEALTH(100);
        update_mask.set_unit_MAXHEALTH(100);
        update_mask.set_unit_LEVEL(1);
        update_mask.set_unit_FACTIONTEMPLATE(1);
        update_mask.set_unit_DISPLAYID(50);
        update_mask.set_unit_NATIVEDISPLAYID(50);

        let mut v = update_mask.as_bytes();
        assert_eq!(b.as_slice(), v.as_slice());
    }
}
