use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{MonsterMoveType, MonsterMoveTypeError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_MONSTER_MOVE_TRANSPORT {
    pub transport: Guid,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub spline_id: u32,
    pub move_type: MonsterMoveType,
}

impl SMSG_MONSTER_MOVE_TRANSPORT {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // transport: PackedGuid
        w.write_all(&self.transport.packed_guid())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int() as u8).to_le_bytes())?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_MONSTER_MOVE_TRANSPORT {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // transport: PackedGuid
        w.write_all(&self.transport.packed_guid())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int() as u8).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x02ae;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_MONSTER_MOVE_TRANSPORTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // transport: PackedGuid
        let transport = Guid::read_packed(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // spline_id: u32
        let spline_id = crate::util::read_u32_le(r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            transport,
            position_x,
            position_y,
            position_z,
            spline_id,
            move_type,
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // transport: PackedGuid
            let transport = Guid::tokio_read_packed(r).await?;

            // position_x: f32
            let position_x = crate::util::tokio_read_f32_le(r).await?;
            // position_y: f32
            let position_y = crate::util::tokio_read_f32_le(r).await?;
            // position_z: f32
            let position_z = crate::util::tokio_read_f32_le(r).await?;
            // spline_id: u32
            let spline_id = crate::util::tokio_read_u32_le(r).await?;

            // move_type: MonsterMoveType
            let move_type: MonsterMoveType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                transport,
                position_x,
                position_y,
                position_z,
                spline_id,
                move_type,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // transport: PackedGuid
            let transport = Guid::astd_read_packed(r).await?;

            // position_x: f32
            let position_x = crate::util::astd_read_f32_le(r).await?;
            // position_y: f32
            let position_y = crate::util::astd_read_f32_le(r).await?;
            // position_z: f32
            let position_z = crate::util::astd_read_f32_le(r).await?;
            // spline_id: u32
            let spline_id = crate::util::astd_read_u32_le(r).await?;

            // move_type: MonsterMoveType
            let move_type: MonsterMoveType = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                transport,
                position_x,
                position_y,
                position_z,
                spline_id,
                move_type,
            })
        })
    }

}

impl SMSG_MONSTER_MOVE_TRANSPORT {
    pub fn size(&self) -> usize {
        0
        + self.transport.size() // transport: Guid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // spline_id: u32
        + 1 // move_type: MonsterMoveType
    }
}

#[derive(Debug)]
pub enum SMSG_MONSTER_MOVE_TRANSPORTError {
    Io(std::io::Error),
    MonsterMoveType(MonsterMoveTypeError),
}

impl std::error::Error for SMSG_MONSTER_MOVE_TRANSPORTError {}
impl std::fmt::Display for SMSG_MONSTER_MOVE_TRANSPORTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::MonsterMoveType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MONSTER_MOVE_TRANSPORTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MonsterMoveTypeError> for SMSG_MONSTER_MOVE_TRANSPORTError {
    fn from(e: MonsterMoveTypeError) -> Self {
        Self::MonsterMoveType(e)
    }
}

