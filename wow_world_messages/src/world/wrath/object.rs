use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MovementBlock, MovementFlags, ObjectType, SplineFlag, TransportInfo, UpdateFlag, 
    UpdateMask, UpdateType, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:187`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L187):
/// ```text
/// struct Object {
///     UpdateType update_type;
///     if (update_type == VALUES) {
///         PackedGuid guid1;
///         UpdateMask mask1;
///     }
///     else if (update_type == MOVEMENT) {
///         PackedGuid guid2;
///         MovementBlock movement1;
///     }
///     else if (update_type == CREATE_OBJECT
///         || update_type == CREATE_OBJECT2) {
///         PackedGuid guid3;
///         ObjectType object_type;
///         MovementBlock movement2;
///         UpdateMask mask2;
///     }
///     else if (update_type == OUT_OF_RANGE_OBJECTS
///         || update_type == NEAR_OBJECTS) {
///         u32 count;
///         PackedGuid[count] guids;
///     }
/// }
/// ```
pub struct Object {
    pub update_type: Object_UpdateType,
}

impl Object {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // update_type: UpdateType
        w.write_all(&(self.update_type.as_int().to_le_bytes()))?;

        match &self.update_type {
            Object_UpdateType::Values {
                guid1,
                mask1,
            } => {
                // guid1: PackedGuid
                crate::util::write_packed_guid(&guid1, &mut w)?;

                // mask1: UpdateMask
                mask1.write_into_vec(&mut w)?;

            }
            Object_UpdateType::Movement {
                guid2,
                movement1,
            } => {
                // guid2: PackedGuid
                crate::util::write_packed_guid(&guid2, &mut w)?;

                // movement1: MovementBlock
                movement1.write_into_vec(&mut w)?;

            }
            Object_UpdateType::CreateObject {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                // guid3: PackedGuid
                crate::util::write_packed_guid(&guid3, &mut w)?;

                // object_type: ObjectType
                w.write_all(&(object_type.as_int().to_le_bytes()))?;

                // movement2: MovementBlock
                movement2.write_into_vec(&mut w)?;

                // mask2: UpdateMask
                mask2.write_into_vec(&mut w)?;

            }
            Object_UpdateType::CreateObject2 {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                // guid3: PackedGuid
                crate::util::write_packed_guid(&guid3, &mut w)?;

                // object_type: ObjectType
                w.write_all(&(object_type.as_int().to_le_bytes()))?;

                // movement2: MovementBlock
                movement2.write_into_vec(&mut w)?;

                // mask2: UpdateMask
                mask2.write_into_vec(&mut w)?;

            }
            Object_UpdateType::OutOfRangeObjects {
                guids,
            } => {
                // count: u32
                w.write_all(&(guids.len() as u32).to_le_bytes())?;

                // guids: PackedGuid[count]
                for i in guids.iter() {
                    crate::util::write_packed_guid(i, &mut w)?;
                }

            }
            Object_UpdateType::NearObjects {
                guids,
            } => {
                // count: u32
                w.write_all(&(guids.len() as u32).to_le_bytes())?;

                // guids: PackedGuid[count]
                for i in guids.iter() {
                    crate::util::write_packed_guid(i, &mut w)?;
                }

            }
        }

        Ok(())
    }
}

impl Object {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // update_type: UpdateType
        let update_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        let update_type_if = match update_type {
            UpdateType::Values => {
                // guid1: PackedGuid
                let guid1 = crate::util::read_packed_guid(&mut r)?;

                // mask1: UpdateMask
                let mask1 = UpdateMask::read(&mut r)?;

                Object_UpdateType::Values {
                    guid1,
                    mask1,
                }
            }
            UpdateType::Movement => {
                // guid2: PackedGuid
                let guid2 = crate::util::read_packed_guid(&mut r)?;

                // movement1: MovementBlock
                let movement1 = MovementBlock::read(&mut r)?;

                Object_UpdateType::Movement {
                    guid2,
                    movement1,
                }
            }
            UpdateType::CreateObject => {
                // guid3: PackedGuid
                let guid3 = crate::util::read_packed_guid(&mut r)?;

                // object_type: ObjectType
                let object_type = crate::util::read_u8_le(&mut r)?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::read(&mut r)?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::read(&mut r)?;

                Object_UpdateType::CreateObject {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::CreateObject2 => {
                // guid3: PackedGuid
                let guid3 = crate::util::read_packed_guid(&mut r)?;

                // object_type: ObjectType
                let object_type = crate::util::read_u8_le(&mut r)?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::read(&mut r)?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::read(&mut r)?;

                Object_UpdateType::CreateObject2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::OutOfRangeObjects => {
                // count: u32
                let count = crate::util::read_u32_le(&mut r)?;

                // guids: PackedGuid[count]
                let guids = {
                    let mut guids = Vec::with_capacity(count as usize);
                    for _ in 0..count {
                        guids.push(crate::util::read_packed_guid(&mut r)?);
                    }
                    guids
                };

                Object_UpdateType::OutOfRangeObjects {
                    guids,
                }
            }
            UpdateType::NearObjects => {
                // count: u32
                let count = crate::util::read_u32_le(&mut r)?;

                // guids: PackedGuid[count]
                let guids = {
                    let mut guids = Vec::with_capacity(count as usize);
                    for _ in 0..count {
                        guids.push(crate::util::read_packed_guid(&mut r)?);
                    }
                    guids
                };

                Object_UpdateType::NearObjects {
                    guids,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

}

impl Object {
    pub(crate) fn size(&self) -> usize {
        self.update_type.size() // update_type: Object_UpdateType
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object_UpdateType {
    Values {
        guid1: Guid,
        mask1: UpdateMask,
    },
    Movement {
        guid2: Guid,
        movement1: MovementBlock,
    },
    CreateObject {
        guid3: Guid,
        mask2: UpdateMask,
        movement2: MovementBlock,
        object_type: ObjectType,
    },
    CreateObject2 {
        guid3: Guid,
        mask2: UpdateMask,
        movement2: MovementBlock,
        object_type: ObjectType,
    },
    OutOfRangeObjects {
        guids: Vec<Guid>,
    },
    NearObjects {
        guids: Vec<Guid>,
    },
}

impl Default for Object_UpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Values {
            guid1: Default::default(),
            mask1: Default::default(),
        }
    }
}

impl Object_UpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Values { .. } => 0,
            Self::Movement { .. } => 1,
            Self::CreateObject { .. } => 2,
            Self::CreateObject2 { .. } => 3,
            Self::OutOfRangeObjects { .. } => 4,
            Self::NearObjects { .. } => 5,
        }
    }

}

impl std::fmt::Display for Object_UpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Values{ .. } => f.write_str("Values"),
            Self::Movement{ .. } => f.write_str("Movement"),
            Self::CreateObject{ .. } => f.write_str("CreateObject"),
            Self::CreateObject2{ .. } => f.write_str("CreateObject2"),
            Self::OutOfRangeObjects{ .. } => f.write_str("OutOfRangeObjects"),
            Self::NearObjects{ .. } => f.write_str("NearObjects"),
        }
    }
}

impl Object_UpdateType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Values {
                guid1,
                mask1,
            } => {
                1
                + crate::util::packed_guid_size(&guid1) // guid1: PackedGuid
                + mask1.size() // mask1: UpdateMask
            }
            Self::Movement {
                guid2,
                movement1,
            } => {
                1
                + crate::util::packed_guid_size(&guid2) // guid2: PackedGuid
                + movement1.size() // movement1: MovementBlock
            }
            Self::CreateObject {
                guid3,
                mask2,
                movement2,
                ..
            } => {
                1
                + crate::util::packed_guid_size(&guid3) // guid3: PackedGuid
                + mask2.size() // mask2: UpdateMask
                + movement2.size() // movement2: MovementBlock
                + 1 // object_type: ObjectType
            }
            Self::CreateObject2 {
                guid3,
                mask2,
                movement2,
                ..
            } => {
                1
                + crate::util::packed_guid_size(&guid3) // guid3: PackedGuid
                + mask2.size() // mask2: UpdateMask
                + movement2.size() // movement2: MovementBlock
                + 1 // object_type: ObjectType
            }
            Self::OutOfRangeObjects {
                guids,
            } => {
                1
                + 4 // count: u32
                + guids.iter().fold(0, |acc, x| acc + crate::util::packed_guid_size(x)) // guids: PackedGuid[count]
            }
            Self::NearObjects {
                guids,
            } => {
                1
                + 4 // count: u32
                + guids.iter().fold(0, |acc, x| acc + crate::util::packed_guid_size(x)) // guids: PackedGuid[count]
            }
        }
    }
}

