use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestCompletable {
    NOT_COMPLETABLE,
    COMPLETEABLE,
}

impl QuestCompletable {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NOT_COMPLETABLE => 0x0,
            Self::COMPLETEABLE => 0x3,
        }
    }

}

impl Default for QuestCompletable {
    fn default() -> Self {
        Self::NOT_COMPLETABLE
    }
}

impl std::fmt::Display for QuestCompletable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_COMPLETABLE => f.write_str("NOT_COMPLETABLE"),
            Self::COMPLETEABLE => f.write_str("COMPLETEABLE"),
        }
    }
}

impl TryFrom<u32> for QuestCompletable {
    type Error = QuestCompletableError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_COMPLETABLE),
            3 => Ok(Self::COMPLETEABLE),
            _ => Err(QuestCompletableError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct QuestCompletableError {
    pub value: u32,
}

impl QuestCompletableError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for QuestCompletableError {}
impl std::fmt::Display for QuestCompletableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'QuestCompletable': '{}'", self.value))
    }
}

