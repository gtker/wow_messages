use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PET_DISMISS_SOUND {
    pub sound_id: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
}

impl ServerMessageWrite for SMSG_PET_DISMISS_SOUND {}

impl MessageBody for SMSG_PET_DISMISS_SOUND {
    const OPCODE: u16 = 0x0325;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        Ok(Self {
            sound_id,
            position_x,
            position_y,
            position_z,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PET_DISMISS_SOUND {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PET_DISMISS_SOUND {
    fn maximum_possible_size() -> usize {
        4 // sound_id: u32
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
    }
}

