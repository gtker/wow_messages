use std::convert::{TryFrom, TryInto};
use crate::world::tbc::Vector3d;
use crate::world::tbc::CorpseQueryResult;
use crate::world::tbc::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

impl crate::Message for MSG_CORPSE_QUERY_Server {
    const OPCODE: u32 = 0x0216;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // result: CorpseQueryResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            MSG_CORPSE_QUERY_Server_CorpseQueryResult::NotFound => {}
            MSG_CORPSE_QUERY_Server_CorpseQueryResult::Found {
                corpse_map,
                map,
                position,
            } => {
                // map: Map
                w.write_all(&(map.as_int() as u32).to_le_bytes())?;

                // position: Vector3d
                position.write_into_vec(w)?;

                // corpse_map: Map
                w.write_all(&(corpse_map.as_int() as u32).to_le_bytes())?;

            }
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=21).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0216, size: body_size as u32 });
        }

        // result: CorpseQueryResult
        let result: CorpseQueryResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            CorpseQueryResult::NotFound => MSG_CORPSE_QUERY_Server_CorpseQueryResult::NotFound,
            CorpseQueryResult::Found => {
                // map: Map
                let map: Map = crate::util::read_u32_le(r)?.try_into()?;

                // position: Vector3d
                let position = Vector3d::read(r)?;

                // corpse_map: Map
                let corpse_map: Map = crate::util::read_u32_le(r)?.try_into()?;

                MSG_CORPSE_QUERY_Server_CorpseQueryResult::Found {
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
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_CORPSE_QUERY_Server {}

impl MSG_CORPSE_QUERY_Server {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: MSG_CORPSE_QUERY_Server_CorpseQueryResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    NotFound,
    Found {
        corpse_map: Map,
        map: Map,
        position: Vector3d,
    },
}

impl Default for MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotFound
    }
}

impl MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotFound => 0,
            Self::Found { .. } => 1,
        }
    }

}

impl MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotFound => {
                1
            }
            Self::Found {
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

