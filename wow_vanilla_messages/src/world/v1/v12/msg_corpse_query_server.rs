use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::CorpseQueryResult;
use crate::world::v1::v12::Map;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_CORPSE_QUERY_Server {
    pub result: MSG_CORPSE_QUERY_ServerCorpseQueryResult,
}

impl ServerMessage for MSG_CORPSE_QUERY_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: CorpseQueryResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND => {}
            MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                corpse_map,
                map,
                position_x,
                position_y,
                position_z,
            } => {
                // map: Map
                w.write_all(&(map.as_int() as u32).to_le_bytes())?;

                // position_x: f32
                w.write_all(&position_x.to_le_bytes())?;

                // position_y: f32
                w.write_all(&position_y.to_le_bytes())?;

                // position_z: f32
                w.write_all(&position_z.to_le_bytes())?;

                // corpse_map: Map
                w.write_all(&(corpse_map.as_int() as u32).to_le_bytes())?;

            }
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0216;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: CorpseQueryResult
        let result: CorpseQueryResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            CorpseQueryResult::NOT_FOUND => MSG_CORPSE_QUERY_ServerCorpseQueryResult::NOT_FOUND,
            CorpseQueryResult::FOUND => {
                // map: Map
                let map: Map = crate::util::read_u32_le(r)?.try_into()?;

                // position_x: f32
                let position_x = crate::util::read_f32_le(r)?;
                // position_y: f32
                let position_y = crate::util::read_f32_le(r)?;
                // position_z: f32
                let position_z = crate::util::read_f32_le(r)?;
                // corpse_map: Map
                let corpse_map: Map = crate::util::read_u32_le(r)?.try_into()?;

                MSG_CORPSE_QUERY_ServerCorpseQueryResult::FOUND {
                    corpse_map,
                    map,
                    position_x,
                    position_y,
                    position_z,
                }
            }
        };

        Ok(Self {
            result: result_if,
        })
    }

}

impl MSG_CORPSE_QUERY_Server {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: MSG_CORPSE_QUERY_ServerCorpseQueryResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    NOT_FOUND,
    FOUND {
        corpse_map: Map,
        map: Map,
        position_x: f32,
        position_y: f32,
        position_z: f32,
    },
}

impl Default for MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NOT_FOUND
    }
}

impl MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_FOUND => 0,
            Self::FOUND { .. } => 1,
        }
    }

}

impl MSG_CORPSE_QUERY_ServerCorpseQueryResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NOT_FOUND => {
                1
            }
            Self::FOUND {
                corpse_map,
                map,
                position_x,
                position_y,
                position_z,
            } => {
                1
                + 4 // corpse_map: Map
                + 4 // map: Map
                + 4 // position_x: f32
                + 4 // position_y: f32
                + 4 // position_z: f32
            }
        }
    }
}

