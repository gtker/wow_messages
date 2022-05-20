mod impls;

pub use impls::*;
use std::collections::{BTreeMap, HashMap};

use crate::Guid;
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use std::io;
#[cfg(feature = "sync")]
use std::io::{Read, Write};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateMask {
    header: Vec<u8>,
    values: BTreeMap<u16, u64>,
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

    #[cfg(feature = "sync")]
    pub fn write(&self, w: &mut impl Write) -> Result<(), io::Error> {
        todo!()
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), io::Error> {
        todo!()
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), io::Error> {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    fn header_set(&mut self, value: u16) {
        let index = value / 8;
        let offset = value % 8;
        if index >= self.header.len() as u16 {
            let extras = index - self.header.len() as u16;
            for _ in 0..=extras {
                self.header.push(0);
            }
        }
        self.header[index as usize] |= 1 << offset;
    }

    pub fn new() -> Self {
        Self {
            header: vec![0],
            values: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
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

        let mut update_mask = UpdateMask::new();
        update_mask.set_object_GUID(Guid::new(4));
        update_mask.set_object_TYPE(25);
        update_mask.set_unit_BYTES_0(1 << 24 | 1 << 16 | 1 << 8 | 1);
        update_mask.set_unit_HEALTH(100);

        //let mut v = Vec::new();
        //update_mask.write(&mut v);
        //assert_eq!(b.as_slice(), v.as_slice());
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

        /*
        let mut update_mask = UpdateMask::new();
        update_mask.set_object_guid(Guid::new(4));
        update_mask.set_object_scale(1.0);
        update_mask.set_unit_health(100);
        update_mask.set_unit_race(Race::Human);
        update_mask.set_unit_class(Class::Warrior);
        update_mask.set_unit_gender(Gender::Female);
        update_mask.set_unit_power(Power::Rage);
         */
    }
}
