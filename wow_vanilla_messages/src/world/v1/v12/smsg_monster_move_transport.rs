use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{MonsterMoveType, MonsterMoveTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_MONSTER_MOVE_TRANSPORT {
    pub transport: Guid,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub spline_id: u32,
    pub move_type: MonsterMoveType,
}

impl ServerMessageWrite for SMSG_MONSTER_MOVE_TRANSPORT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_MONSTER_MOVE_TRANSPORT {
    const OPCODE: u16 = 0x02ae;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_MONSTER_MOVE_TRANSPORTError;

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
        let move_type = MonsterMoveType::read(r)?;

        Ok(Self {
            transport,
            position_x,
            position_y,
            position_z,
            spline_id,
            move_type,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // transport: PackedGuid
        self.transport.write_packed(w)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        self.move_type.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
        let move_type = MonsterMoveType::tokio_read(r).await?;

        Ok(Self {
            transport,
            position_x,
            position_y,
            position_z,
            spline_id,
            move_type,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // transport: PackedGuid
        self.transport.tokio_write_packed(w).await?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes()).await?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes()).await?;

        // move_type: MonsterMoveType
        self.move_type.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
        let move_type = MonsterMoveType::astd_read(r).await?;

        Ok(Self {
            transport,
            position_x,
            position_y,
            position_z,
            spline_id,
            move_type,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // transport: PackedGuid
        self.transport.astd_write_packed(w).await?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes()).await?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes()).await?;

        // move_type: MonsterMoveType
        self.move_type.astd_write(w).await?;

        Ok(())
    }
}

impl VariableSized for SMSG_MONSTER_MOVE_TRANSPORT {
    fn size(&self) -> usize {
        self.transport.size() // transport: PackedGuid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // spline_id: u32
        + MonsterMoveType::size() // move_type: MonsterMoveType
    }
}

impl MaximumPossibleSized for SMSG_MONSTER_MOVE_TRANSPORT {
    fn maximum_possible_size() -> usize {
        9 // transport: PackedGuid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // spline_id: u32
        + MonsterMoveType::maximum_possible_size() // move_type: MonsterMoveType
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

