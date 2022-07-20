use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::CorpseQueryResult;
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::world::version_1_12::Vector3d;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_corpse_query_server.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_corpse_query_server.wowm#L8):
/// ```text
/// smsg MSG_CORPSE_QUERY_Server = 0x0216 {
///     CorpseQueryResult result;
///     if (result == FOUND) {
///         Map map;
///         Vector3d position;
///         Map corpse_map;
///     }
/// }
/// ```
pub struct MSG_CORPSE_QUERY_Server {
    pub result: MSG_CORPSE_QUERY_Server_CorpseQueryResult,
}

impl ServerMessage for MSG_CORPSE_QUERY_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: CorpseQueryResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            MSG_CORPSE_QUERY_Server_CorpseQueryResult::NOT_FOUND => {}
            MSG_CORPSE_QUERY_Server_CorpseQueryResult::FOUND {
                corpse_map,
                map,
                position,
            } => {
                // map: Map
                w.write_all(&(map_as_int(map) as u32).to_le_bytes())?;

                // position: Vector3d
                position.write_into_vec(w)?;

                // corpse_map: Map
                w.write_all(&(map_as_int(corpse_map) as u32).to_le_bytes())?;

            }
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0216;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: CorpseQueryResult
        let result: CorpseQueryResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            CorpseQueryResult::NOT_FOUND => MSG_CORPSE_QUERY_Server_CorpseQueryResult::NOT_FOUND,
            CorpseQueryResult::FOUND => {
                // map: Map
                let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

                // position: Vector3d
                let position = Vector3d::read(r)?;

                // corpse_map: Map
                let corpse_map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

                MSG_CORPSE_QUERY_Server_CorpseQueryResult::FOUND {
                    corpse_map,
                    map,
                    position,
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
        self.result.size() // result: MSG_CORPSE_QUERY_Server_CorpseQueryResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    NOT_FOUND,
    FOUND {
        corpse_map: Map,
        map: Map,
        position: Vector3d,
    },
}

impl Default for MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NOT_FOUND
    }
}

impl MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_FOUND => 0,
            Self::FOUND { .. } => 1,
        }
    }

}

impl MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NOT_FOUND => {
                1
            }
            Self::FOUND {
                corpse_map,
                map,
                position,
            } => {
                1
                + 4 // corpse_map: Map
                + 4 // map: Map
                + 12 // position: Vector3d
            }
        }
    }
}

