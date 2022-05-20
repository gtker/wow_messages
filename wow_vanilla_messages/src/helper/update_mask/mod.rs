mod impls;

pub use impls::*;
use std::collections::{BTreeMap, HashMap};

use crate::Guid;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io;
#[cfg(feature = "sync")]
use std::io::{Read, Write};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

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

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateValue {
    Guid(u64),
    U32(u32),
    I32(i32),
    F32(f32),
}

impl Default for UpdateValue {
    fn default() -> Self {
        Self::Guid(0)
    }
}

macro_rules! update_item {
    ($name:ident) => {
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name {
            header: Vec<u32>,
            values: BTreeMap<u16, UpdateValue>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    header: vec![],
                    values: Default::default(),
                }
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

fn header_set(header: &mut Vec<u32>, values: &mut BTreeMap<u16, UpdateValue>, bit: u16) {
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

fn as_bytes(header: &[u32], values: &BTreeMap<u16, UpdateValue>) -> Vec<u8> {
    let mut v = Vec::new();

    v.write_all(&[header.len() as u8]).unwrap();

    for h in header {
        v.write_all(h.to_le_bytes().as_slice()).unwrap();
    }

    for (_, value) in values {
        match value {
            UpdateValue::Guid(g) => {
                v.write_all(&g.to_le_bytes()).unwrap();
            }
            UpdateValue::U32(u) => {
                v.write_all(&u.to_le_bytes()).unwrap();
            }
            UpdateValue::I32(i) => {
                v.write_all(&i.to_le_bytes()).unwrap();
            }
            UpdateValue::F32(f) => {
                v.write_all(&f.to_le_bytes()).unwrap();
            }
        }
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
    #[cfg(feature = "sync")]
    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        todo!()
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<Self, io::Error> {
        todo!()
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, io::Error> {
        todo!()
    }

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
