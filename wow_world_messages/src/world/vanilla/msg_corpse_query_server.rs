use std::io::{Read, Write};

use crate::vanilla::{
    CorpseQueryResult, Map, Vector3d,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
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

impl crate::private::Sealed for MSG_CORPSE_QUERY_Server {}
impl MSG_CORPSE_QUERY_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=21).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // result: CorpseQueryResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            CorpseQueryResult::NotFound => MSG_CORPSE_QUERY_Server_CorpseQueryResult::NotFound,
            CorpseQueryResult::Found => {
                // map: Map
                let map = crate::util::read_u32_le(&mut r)?.try_into()?;

                // position: Vector3d
                let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

                // corpse_map: Map
                let corpse_map = crate::util::read_u32_le(&mut r)?.try_into()?;

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

impl crate::Message for MSG_CORPSE_QUERY_Server {
    const OPCODE: u32 = 0x0216;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_CORPSE_QUERY_Server {{").unwrap();
        // Members
        writeln!(s, "    result = {};", CorpseQueryResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::vanilla::MSG_CORPSE_QUERY_Server_CorpseQueryResult::Found {
                corpse_map,
                map,
                position,
            } => {
                writeln!(s, "    map = {};", map.as_test_case_value()).unwrap();
                // position: Vector3d
                writeln!(s, "    position = {{").unwrap();
                // Members
                writeln!(s, "    {}", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                writeln!(s, "    {}", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                writeln!(s, "    {}", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                writeln!(s, "    }};").unwrap();
                writeln!(s, "    corpse_map = {};", corpse_map.as_test_case_value()).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 534_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self.result {
            crate::vanilla::MSG_CORPSE_QUERY_Server_CorpseQueryResult::Found {
                corpse_map,
                map,
                position,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
                writeln!(s, "    /* position: Vector3d start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
                writeln!(s, "    /* position: Vector3d end */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "corpse_map", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&position, &mut w)?;

                // corpse_map: Map
                w.write_all(&(corpse_map.as_int().to_le_bytes()))?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(534, "MSG_CORPSE_QUERY_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_CORPSE_QUERY_Server {}

impl MSG_CORPSE_QUERY_Server {
    pub(crate) const fn size(&self) -> usize {
        self.result.size() // result: MSG_CORPSE_QUERY_Server_CorpseQueryResult
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

