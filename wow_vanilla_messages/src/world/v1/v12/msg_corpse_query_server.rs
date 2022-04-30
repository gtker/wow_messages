use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{CorpseQueryResult, CorpseQueryResultError};
use crate::world::v1::v12::{Map, MapError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_CORPSE_QUERY_Server {
    pub result: MSG_CORPSE_QUERY_ServerCorpseQueryResult,
}

impl WorldServerMessageWrite for MSG_CORPSE_QUERY_Server {
    const OPCODE: u16 = 0x216;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_CORPSE_QUERY_Server {
    type Error = MSG_CORPSE_QUERY_ServerError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: CorpseQueryResult
        let result = CorpseQueryResult::read(r)?;

        let result_if = match result {
            CorpseQueryResult::NOT_FOUND => MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND,
            CorpseQueryResult::FOUND => {
                // map: Map
                let map = Map::read(r)?;

                // position_x: f32
                let position_x = crate::util::read_f32_le(r)?;
                // position_y: f32
                let position_y = crate::util::read_f32_le(r)?;
                // position_z: f32
                let position_z = crate::util::read_f32_le(r)?;
                // corpse_map: Map
                let corpse_map = Map::read(r)?;

                MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                    map,
                    position_x,
                    position_y,
                    position_z,
                    corpse_map,
                }
            }
        };

        Ok(Self {
            result: result_if,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: CorpseQueryResult
        self.result.write(w)?;

        match &self.result {
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND => {}
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                map,
                position_x,
                position_y,
                position_z,
                corpse_map,
            } => {
                // map: Map
                map.write(w)?;

                // position_x: f32
                w.write_all(&position_x.to_le_bytes())?;

                // position_y: f32
                w.write_all(&position_y.to_le_bytes())?;

                // position_z: f32
                w.write_all(&position_z.to_le_bytes())?;

                // corpse_map: Map
                corpse_map.write(w)?;

            }
        }

        Ok(())
    }
}

impl VariableSized for MSG_CORPSE_QUERY_Server {
    fn size(&self) -> usize {
        self.result.size() // result: CorpseQueryResult and subfields
    }
}

impl MaximumPossibleSized for MSG_CORPSE_QUERY_Server {
    fn maximum_possible_size() -> usize {
        CorpseQueryResult::maximum_possible_size() // result: CorpseQueryResult
    }
}

#[derive(Debug)]
pub enum MSG_CORPSE_QUERY_ServerError {
    Io(std::io::Error),
    CorpseQueryResult(CorpseQueryResultError),
    Map(MapError),
}

impl std::error::Error for MSG_CORPSE_QUERY_ServerError {}
impl std::fmt::Display for MSG_CORPSE_QUERY_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::CorpseQueryResult(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_CORPSE_QUERY_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CorpseQueryResultError> for MSG_CORPSE_QUERY_ServerError {
    fn from(e: CorpseQueryResultError) -> Self {
        Self::CorpseQueryResult(e)
    }
}

impl From<MapError> for MSG_CORPSE_QUERY_ServerError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    NOT_FOUND,
    FOUND {
        map: Map,
        position_x: f32,
        position_y: f32,
        position_z: f32,
        corpse_map: Map,
    },
}

impl From<&CorpseQueryResult> for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn from(e: &CorpseQueryResult) -> Self {
        match &e {
            CorpseQueryResult::NOT_FOUND => Self::NOT_FOUND,
            CorpseQueryResult::FOUND => Self::FOUND {
                map: Default::default(),
                position_x: Default::default(),
                position_y: Default::default(),
                position_z: Default::default(),
                corpse_map: Default::default(),
            },
        }
    }
}

impl From<&MSG_CORPSE_QUERY_ServerCorpseQueryResult> for CorpseQueryResult {
    fn from(v: &MSG_CORPSE_QUERY_ServerCorpseQueryResult) -> Self {
        match &v {
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND => Self::NOT_FOUND,
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND { .. } => Self::FOUND,
        }
    }
}

impl Default for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NOT_FOUND
    }
}

impl MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CorpseQueryResult = self.into();
        a.tokio_write_u64_be(w).await
    }

}

impl VariableSized for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn size(&self) -> usize {
        match self {
            Self::NOT_FOUND =>  {
                1
            }
            Self::FOUND  {
                map,
                position_x,
                position_y,
                position_z,
                corpse_map,
            } => {
                1
                + Map::size() // map: Map
                + 4 // position_x: f32
                + 4 // position_y: f32
                + 4 // position_z: f32
                + Map::size() // corpse_map: Map
            }
        }
    }
}

impl MaximumPossibleSized for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

