/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_corpse_query_server.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_corpse_query_server.wowm#L1):
/// ```text
/// enum CorpseQueryResult : u8 {
///     NOT_FOUND = 0;
///     FOUND = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CorpseQueryResult {
    NotFound,
    Found,
}

impl CorpseQueryResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NotFound => 0x0,
            Self::Found => 0x1,
        }
    }

}

impl Default for CorpseQueryResult {
    fn default() -> Self {
        Self::NotFound
    }
}

impl std::fmt::Display for CorpseQueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => f.write_str("NotFound"),
            Self::Found => f.write_str("Found"),
        }
    }
}

impl TryFrom<u8> for CorpseQueryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotFound),
            1 => Ok(Self::Found),
            v => Err(crate::errors::EnumError::new("CorpseQueryResult", v as u64),)
        }
    }
}

