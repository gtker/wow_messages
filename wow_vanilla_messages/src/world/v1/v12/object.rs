use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::UpdateMask;
use crate::world::v1::v12::MovementBlock;
use crate::world::v1::v12::{ObjectType, ObjectTypeError};
use crate::world::v1::v12::{UpdateType, UpdateTypeError};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Object {
    pub update_type: ObjectUpdateType,
}

impl Object {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // update_type: UpdateType
        w.write_all(&(self.update_type.as_int() as u8).to_le_bytes())?;

        match &self.update_type {
            ObjectUpdateType::VALUES {
                guid1,
                mask1,
            } => {
                // guid1: PackedGuid
                w.write_all(&guid1.packed_guid())?;

                // mask1: UpdateMask
                w.write_all(&mask1.as_bytes())?;

            }
            ObjectUpdateType::MOVEMENT {
                guid2,
                movement1,
            } => {
                // guid2: PackedGuid
                w.write_all(&guid2.packed_guid())?;

                // movement1: MovementBlock
                w.write_all(&movement1.as_bytes()?)?;

            }
            ObjectUpdateType::CREATE_OBJECT {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                // guid3: PackedGuid
                w.write_all(&guid3.packed_guid())?;

                // object_type: ObjectType
                w.write_all(&(object_type.as_int() as u8).to_le_bytes())?;

                // movement2: MovementBlock
                w.write_all(&movement2.as_bytes()?)?;

                // mask2: UpdateMask
                w.write_all(&mask2.as_bytes())?;

            }
            ObjectUpdateType::CREATE_OBJECT2 {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                // guid3: PackedGuid
                w.write_all(&guid3.packed_guid())?;

                // object_type: ObjectType
                w.write_all(&(object_type.as_int() as u8).to_le_bytes())?;

                // movement2: MovementBlock
                w.write_all(&movement2.as_bytes()?)?;

                // mask2: UpdateMask
                w.write_all(&mask2.as_bytes())?;

            }
            ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                count,
                guids,
            } => {
                // count: u32
                w.write_all(&count.to_le_bytes())?;

                // guids: PackedGuid[count]
                for i in guids.iter() {
                    w.write_all(&i.packed_guid())?;
                }

            }
            ObjectUpdateType::NEAR_OBJECTS {
                count,
                guids,
            } => {
                // count: u32
                w.write_all(&count.to_le_bytes())?;

                // guids: PackedGuid[count]
                for i in guids.iter() {
                    w.write_all(&i.packed_guid())?;
                }

            }
        }

        Ok(w)
    }
}

impl Object {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ObjectError> {
        // update_type: UpdateType
        let update_type: UpdateType = crate::util::read_u8_le(r)?.try_into()?;

        let update_type_if = match update_type {
            UpdateType::VALUES => {
                // guid1: PackedGuid
                let guid1 = Guid::read_packed(r)?;

                // mask1: UpdateMask
                let mask1 = UpdateMask::read(r)?;

                ObjectUpdateType::VALUES {
                    guid1,
                    mask1,
                }
            }
            UpdateType::MOVEMENT => {
                // guid2: PackedGuid
                let guid2 = Guid::read_packed(r)?;

                // movement1: MovementBlock
                let movement1 = MovementBlock::read(r)?;

                ObjectUpdateType::MOVEMENT {
                    guid2,
                    movement1,
                }
            }
            UpdateType::CREATE_OBJECT => {
                // guid3: PackedGuid
                let guid3 = Guid::read_packed(r)?;

                // object_type: ObjectType
                let object_type: ObjectType = crate::util::read_u8_le(r)?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::read(r)?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::read(r)?;

                ObjectUpdateType::CREATE_OBJECT {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::CREATE_OBJECT2 => {
                // guid3: PackedGuid
                let guid3 = Guid::read_packed(r)?;

                // object_type: ObjectType
                let object_type: ObjectType = crate::util::read_u8_le(r)?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::read(r)?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::read(r)?;

                ObjectUpdateType::CREATE_OBJECT2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::OUT_OF_RANGE_OBJECTS => {
                // count: u32
                let count = crate::util::read_u32_le(r)?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::read_packed(r)?);
                }

                ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                    count,
                    guids,
                }
            }
            UpdateType::NEAR_OBJECTS => {
                // count: u32
                let count = crate::util::read_u32_le(r)?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::read_packed(r)?);
                }

                ObjectUpdateType::NEAR_OBJECTS {
                    count,
                    guids,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ObjectError> {
        // update_type: UpdateType
        let update_type: UpdateType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

        let update_type_if = match update_type {
            UpdateType::VALUES => {
                // guid1: PackedGuid
                let guid1 = Guid::tokio_read_packed(r).await?;

                // mask1: UpdateMask
                let mask1 = UpdateMask::tokio_read(r).await?;

                ObjectUpdateType::VALUES {
                    guid1,
                    mask1,
                }
            }
            UpdateType::MOVEMENT => {
                // guid2: PackedGuid
                let guid2 = Guid::tokio_read_packed(r).await?;

                // movement1: MovementBlock
                let movement1 = MovementBlock::tokio_read(r).await?;

                ObjectUpdateType::MOVEMENT {
                    guid2,
                    movement1,
                }
            }
            UpdateType::CREATE_OBJECT => {
                // guid3: PackedGuid
                let guid3 = Guid::tokio_read_packed(r).await?;

                // object_type: ObjectType
                let object_type: ObjectType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::tokio_read(r).await?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::tokio_read(r).await?;

                ObjectUpdateType::CREATE_OBJECT {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::CREATE_OBJECT2 => {
                // guid3: PackedGuid
                let guid3 = Guid::tokio_read_packed(r).await?;

                // object_type: ObjectType
                let object_type: ObjectType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::tokio_read(r).await?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::tokio_read(r).await?;

                ObjectUpdateType::CREATE_OBJECT2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::OUT_OF_RANGE_OBJECTS => {
                // count: u32
                let count = crate::util::tokio_read_u32_le(r).await?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::tokio_read_packed(r).await?);
                }

                ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                    count,
                    guids,
                }
            }
            UpdateType::NEAR_OBJECTS => {
                // count: u32
                let count = crate::util::tokio_read_u32_le(r).await?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::tokio_read_packed(r).await?);
                }

                ObjectUpdateType::NEAR_OBJECTS {
                    count,
                    guids,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ObjectError> {
        // update_type: UpdateType
        let update_type: UpdateType = crate::util::astd_read_u8_le(r).await?.try_into()?;

        let update_type_if = match update_type {
            UpdateType::VALUES => {
                // guid1: PackedGuid
                let guid1 = Guid::astd_read_packed(r).await?;

                // mask1: UpdateMask
                let mask1 = UpdateMask::astd_read(r).await?;

                ObjectUpdateType::VALUES {
                    guid1,
                    mask1,
                }
            }
            UpdateType::MOVEMENT => {
                // guid2: PackedGuid
                let guid2 = Guid::astd_read_packed(r).await?;

                // movement1: MovementBlock
                let movement1 = MovementBlock::astd_read(r).await?;

                ObjectUpdateType::MOVEMENT {
                    guid2,
                    movement1,
                }
            }
            UpdateType::CREATE_OBJECT => {
                // guid3: PackedGuid
                let guid3 = Guid::astd_read_packed(r).await?;

                // object_type: ObjectType
                let object_type: ObjectType = crate::util::astd_read_u8_le(r).await?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::astd_read(r).await?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::astd_read(r).await?;

                ObjectUpdateType::CREATE_OBJECT {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::CREATE_OBJECT2 => {
                // guid3: PackedGuid
                let guid3 = Guid::astd_read_packed(r).await?;

                // object_type: ObjectType
                let object_type: ObjectType = crate::util::astd_read_u8_le(r).await?.try_into()?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::astd_read(r).await?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::astd_read(r).await?;

                ObjectUpdateType::CREATE_OBJECT2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::OUT_OF_RANGE_OBJECTS => {
                // count: u32
                let count = crate::util::astd_read_u32_le(r).await?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::astd_read_packed(r).await?);
                }

                ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                    count,
                    guids,
                }
            }
            UpdateType::NEAR_OBJECTS => {
                // count: u32
                let count = crate::util::astd_read_u32_le(r).await?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::astd_read_packed(r).await?);
                }

                ObjectUpdateType::NEAR_OBJECTS {
                    count,
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
    pub fn size(&self) -> usize {
        0
        + self.update_type.size() // update_type: ObjectUpdateType
    }
}

#[derive(Debug)]
pub enum ObjectError {
    Io(std::io::Error),
    ObjectType(ObjectTypeError),
    UpdateType(UpdateTypeError),
}

impl std::error::Error for ObjectError {}
impl std::fmt::Display for ObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::ObjectType(i) => i.fmt(f),
            Self::UpdateType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for ObjectError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ObjectTypeError> for ObjectError {
    fn from(e: ObjectTypeError) -> Self {
        Self::ObjectType(e)
    }
}

impl From<UpdateTypeError> for ObjectError {
    fn from(e: UpdateTypeError) -> Self {
        Self::UpdateType(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectUpdateType {
    VALUES {
        guid1: Guid,
        mask1: UpdateMask,
    },
    MOVEMENT {
        guid2: Guid,
        movement1: MovementBlock,
    },
    CREATE_OBJECT {
        guid3: Guid,
        mask2: UpdateMask,
        movement2: MovementBlock,
        object_type: ObjectType,
    },
    CREATE_OBJECT2 {
        guid3: Guid,
        mask2: UpdateMask,
        movement2: MovementBlock,
        object_type: ObjectType,
    },
    OUT_OF_RANGE_OBJECTS {
        count: u32,
        guids: Vec<Guid>,
    },
    NEAR_OBJECTS {
        count: u32,
        guids: Vec<Guid>,
    },
}

impl Default for ObjectUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::VALUES {
            guid1: Default::default(),
            mask1: Default::default(),
        }
    }
}

impl ObjectUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::VALUES { .. } => 0,
            Self::MOVEMENT { .. } => 1,
            Self::CREATE_OBJECT { .. } => 2,
            Self::CREATE_OBJECT2 { .. } => 3,
            Self::OUT_OF_RANGE_OBJECTS { .. } => 4,
            Self::NEAR_OBJECTS { .. } => 5,
        }
    }

}

impl ObjectUpdateType {
    pub fn size(&self) -> usize {
        match self {
            Self::VALUES {
                guid1,
                mask1,
            } => {
                1
                + guid1.size() // guid1: Guid
                + mask1.size() // mask1: UpdateMask
            }
            Self::MOVEMENT {
                guid2,
                movement1,
            } => {
                1
                + guid2.size() // guid2: Guid
                + movement1.size() // movement1: MovementBlock
            }
            Self::CREATE_OBJECT {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                1
                + guid3.size() // guid3: Guid
                + mask2.size() // mask2: UpdateMask
                + movement2.size() // movement2: MovementBlock
                + 1 // object_type: ObjectType
            }
            Self::CREATE_OBJECT2 {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                1
                + guid3.size() // guid3: Guid
                + mask2.size() // mask2: UpdateMask
                + movement2.size() // movement2: MovementBlock
                + 1 // object_type: ObjectType
            }
            Self::OUT_OF_RANGE_OBJECTS {
                count,
                guids,
            } => {
                1
                + 4 // count: u32
                + guids.iter().fold(0, |acc, x| acc + x.size()) // guids: PackedGuid[count]
            }
            Self::NEAR_OBJECTS {
                count,
                guids,
            } => {
                1
                + 4 // count: u32
                + guids.iter().fold(0, |acc, x| acc + x.size()) // guids: PackedGuid[count]
            }
        }
    }
}

