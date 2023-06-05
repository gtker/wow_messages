use std::io::{Read, Write};

use crate::wrath::{
    CorpseQueryResult, Map, Vector3d,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_corpse_query_server.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_corpse_query_server.wowm#L19):
/// ```text
/// smsg MSG_CORPSE_QUERY_Server = 0x0216 {
///     CorpseQueryResult result;
///     if (result == FOUND) {
///         Map map;
///         Vector3d position;
///         Map corpse_map;
///     }
///     u32 unknown;
/// }
/// ```
pub struct MSG_CORPSE_QUERY_Server {
    pub result: MSG_CORPSE_QUERY_Server_CorpseQueryResult,
    pub unknown: u32,
}

impl crate::private::Sealed for MSG_CORPSE_QUERY_Server {}
impl crate::Message for MSG_CORPSE_QUERY_Server {
    const OPCODE: u32 = 0x0216;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: CorpseQueryResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        match &self.result {
            MSG_CORPSE_QUERY_Server_CorpseQueryResult::Found {
                corpse_map,
                map,
                position,
            } => {
                // map: Map
                w.write_all(&(map.as_int().to_le_bytes()))?;

                // position: Vector3d
                position.write_into_vec(&mut w)?;

                // corpse_map: Map
                w.write_all(&(corpse_map.as_int().to_le_bytes()))?;

            }
            _ => {}
        }

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=25).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0216, size: body_size });
        }

        // result: CorpseQueryResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            CorpseQueryResult::NotFound => MSG_CORPSE_QUERY_Server_CorpseQueryResult::NotFound,
            CorpseQueryResult::Found => {
                // map: Map
                let map = crate::util::read_u32_le(&mut r)?.try_into()?;

                // position: Vector3d
                let position = Vector3d::read(&mut r)?;

                // corpse_map: Map
                let corpse_map = crate::util::read_u32_le(&mut r)?.try_into()?;

                MSG_CORPSE_QUERY_Server_CorpseQueryResult::Found {
                    corpse_map,
                    map,
                    position,
                }
            }
        };

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            result: result_if,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_CORPSE_QUERY_Server {}

impl MSG_CORPSE_QUERY_Server {
    pub(crate) const fn size(&self) -> usize {
        self.result.size() // result: MSG_CORPSE_QUERY_Server_CorpseQueryResult
        + 4 // unknown: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

impl std::fmt::Display for MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => f.write_str("NotFound"),
            Self::Found{ .. } => f.write_str("Found"),
        }
    }
}

impl MSG_CORPSE_QUERY_Server_CorpseQueryResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Found {
                ..
            } => {
                1
                + 4 // corpse_map: Map
                + 4 // map: Map
                + 12 // position: Vector3d
            }
            _ => 1,
        }
    }
}

