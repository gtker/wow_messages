use std::convert::{TryFrom, TryInto};
use crate::world::wrath::DungeonDifficulty;
use crate::world::wrath::Map;
use crate::world::wrath::TransferAbortReason;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:106`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L106):
/// ```text
/// smsg SMSG_TRANSFER_ABORTED = 0x0040 {
///     Map map;
///     TransferAbortReason reason;
///     if (reason == INSUFFICIENT_EXPANSION_LEVEL
///         || reason == DIFFICULTY_NOT_AVAILABLE
///         || reason == UNIQUE_MESSAGE) {
///         DungeonDifficulty difficulty;
///     }
/// }
/// ```
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: SMSG_TRANSFER_ABORTED_TransferAbortReason,
}

impl crate::Message for SMSG_TRANSFER_ABORTED {
    const OPCODE: u32 = 0x0040;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        match &self.reason {
            SMSG_TRANSFER_ABORTED_TransferAbortReason::None => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::ErrorX => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::MaxPlayers => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyInstances => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneInCombat => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int() as u8).to_le_bytes())?;

            }
            SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int() as u8).to_le_bytes())?;

            }
            SMSG_TRANSFER_ABORTED_TransferAbortReason::UniqueMessage {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int() as u8).to_le_bytes())?;

            }
            SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyRealmInstances => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::NeedGroup => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound1 => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound2 => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound3 => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::RealmOnly => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::MapNotAllowed => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=6).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0040, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // reason: TransferAbortReason
        let reason: TransferAbortReason = crate::util::read_u8_le(r)?.try_into()?;

        let reason_if = match reason {
            TransferAbortReason::None => SMSG_TRANSFER_ABORTED_TransferAbortReason::None,
            TransferAbortReason::ErrorX => SMSG_TRANSFER_ABORTED_TransferAbortReason::ErrorX,
            TransferAbortReason::MaxPlayers => SMSG_TRANSFER_ABORTED_TransferAbortReason::MaxPlayers,
            TransferAbortReason::NotFound => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound,
            TransferAbortReason::TooManyInstances => SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyInstances,
            TransferAbortReason::ZoneInCombat => SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneInCombat,
            TransferAbortReason::InsufficientExpansionLevel => {
                // difficulty: DungeonDifficulty
                let difficulty: DungeonDifficulty = crate::util::read_u8_le(r)?.try_into()?;

                SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                    difficulty,
                }
            }
            TransferAbortReason::DifficultyNotAvailable => {
                // difficulty: DungeonDifficulty
                let difficulty: DungeonDifficulty = crate::util::read_u8_le(r)?.try_into()?;

                SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                    difficulty,
                }
            }
            TransferAbortReason::UniqueMessage => {
                // difficulty: DungeonDifficulty
                let difficulty: DungeonDifficulty = crate::util::read_u8_le(r)?.try_into()?;

                SMSG_TRANSFER_ABORTED_TransferAbortReason::UniqueMessage {
                    difficulty,
                }
            }
            TransferAbortReason::TooManyRealmInstances => SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyRealmInstances,
            TransferAbortReason::NeedGroup => SMSG_TRANSFER_ABORTED_TransferAbortReason::NeedGroup,
            TransferAbortReason::NotFound1 => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound1,
            TransferAbortReason::NotFound2 => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound2,
            TransferAbortReason::NotFound3 => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound3,
            TransferAbortReason::RealmOnly => SMSG_TRANSFER_ABORTED_TransferAbortReason::RealmOnly,
            TransferAbortReason::MapNotAllowed => SMSG_TRANSFER_ABORTED_TransferAbortReason::MapNotAllowed,
        };

        Ok(Self {
            map,
            reason: reason_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_TRANSFER_ABORTED {}

impl SMSG_TRANSFER_ABORTED {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + self.reason.size() // reason: SMSG_TRANSFER_ABORTED_TransferAbortReason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_TRANSFER_ABORTED_TransferAbortReason {
    None,
    ErrorX,
    MaxPlayers,
    NotFound,
    TooManyInstances,
    ZoneInCombat,
    InsufficientExpansionLevel {
        difficulty: DungeonDifficulty,
    },
    DifficultyNotAvailable {
        difficulty: DungeonDifficulty,
    },
    UniqueMessage {
        difficulty: DungeonDifficulty,
    },
    TooManyRealmInstances,
    NeedGroup,
    NotFound1,
    NotFound2,
    NotFound3,
    RealmOnly,
    MapNotAllowed,
}

impl Default for SMSG_TRANSFER_ABORTED_TransferAbortReason {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SMSG_TRANSFER_ABORTED_TransferAbortReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::ErrorX => 1,
            Self::MaxPlayers => 2,
            Self::NotFound => 3,
            Self::TooManyInstances => 4,
            Self::ZoneInCombat => 6,
            Self::InsufficientExpansionLevel { .. } => 7,
            Self::DifficultyNotAvailable { .. } => 8,
            Self::UniqueMessage { .. } => 9,
            Self::TooManyRealmInstances => 10,
            Self::NeedGroup => 11,
            Self::NotFound1 => 12,
            Self::NotFound2 => 13,
            Self::NotFound3 => 14,
            Self::RealmOnly => 15,
            Self::MapNotAllowed => 16,
        }
    }

}

impl SMSG_TRANSFER_ABORTED_TransferAbortReason {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                1
            }
            Self::ErrorX => {
                1
            }
            Self::MaxPlayers => {
                1
            }
            Self::NotFound => {
                1
            }
            Self::TooManyInstances => {
                1
            }
            Self::ZoneInCombat => {
                1
            }
            Self::InsufficientExpansionLevel {
                difficulty,
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            Self::DifficultyNotAvailable {
                difficulty,
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            Self::UniqueMessage {
                difficulty,
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            Self::TooManyRealmInstances => {
                1
            }
            Self::NeedGroup => {
                1
            }
            Self::NotFound1 => {
                1
            }
            Self::NotFound2 => {
                1
            }
            Self::NotFound3 => {
                1
            }
            Self::RealmOnly => {
                1
            }
            Self::MapNotAllowed => {
                1
            }
        }
    }
}

