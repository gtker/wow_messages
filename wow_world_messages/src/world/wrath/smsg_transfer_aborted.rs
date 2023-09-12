use std::io::{Read, Write};

use crate::wrath::{
    DungeonDifficulty, Map, TransferAbortReason,
};

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

impl crate::private::Sealed for SMSG_TRANSFER_ABORTED {}
impl SMSG_TRANSFER_ABORTED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=6).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // reason: TransferAbortReason
        let reason = crate::util::read_u8_le(&mut r)?.try_into()?;

        let reason_if = match reason {
            TransferAbortReason::None => SMSG_TRANSFER_ABORTED_TransferAbortReason::None,
            TransferAbortReason::ErrorX => SMSG_TRANSFER_ABORTED_TransferAbortReason::ErrorX,
            TransferAbortReason::MaxPlayers => SMSG_TRANSFER_ABORTED_TransferAbortReason::MaxPlayers,
            TransferAbortReason::NotFound => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound,
            TransferAbortReason::TooManyInstances => SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyInstances,
            TransferAbortReason::ZoneInCombat => SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneInCombat,
            TransferAbortReason::InsufficientExpansionLevel => {
                // difficulty: DungeonDifficulty
                let difficulty = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                    difficulty,
                }
            }
            TransferAbortReason::DifficultyNotAvailable => {
                // difficulty: DungeonDifficulty
                let difficulty = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                    difficulty,
                }
            }
            TransferAbortReason::UniqueMessage => {
                // difficulty: DungeonDifficulty
                let difficulty = crate::util::read_u8_le(&mut r)?.try_into()?;

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

impl crate::Message for SMSG_TRANSFER_ABORTED {
    const OPCODE: u32 = 0x0040;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_TRANSFER_ABORTED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRANSFER_ABORTED {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    reason = {};", TransferAbortReason::try_from(self.reason.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.reason {
            crate::wrath::SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                difficulty,
            } => {
                writeln!(s, "    difficulty = {};", difficulty.as_test_case_value()).unwrap();
            }
            crate::wrath::SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                difficulty,
            } => {
                writeln!(s, "    difficulty = {};", difficulty.as_test_case_value()).unwrap();
            }
            crate::wrath::SMSG_TRANSFER_ABORTED_TransferAbortReason::UniqueMessage {
                difficulty,
            } => {
                writeln!(s, "    difficulty = {};", difficulty.as_test_case_value()).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 64_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "reason", "    ");
        match &self.reason {
            crate::wrath::SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                difficulty,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "difficulty", "    ");
            }
            crate::wrath::SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                difficulty,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "difficulty", "    ");
            }
            crate::wrath::SMSG_TRANSFER_ABORTED_TransferAbortReason::UniqueMessage {
                difficulty,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "difficulty", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int().to_le_bytes()))?;

        match &self.reason {
            SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int().to_le_bytes()))?;

            }
            SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int().to_le_bytes()))?;

            }
            SMSG_TRANSFER_ABORTED_TransferAbortReason::UniqueMessage {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int().to_le_bytes()))?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(64, "SMSG_TRANSFER_ABORTED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TRANSFER_ABORTED {}

impl SMSG_TRANSFER_ABORTED {
    pub(crate) const fn size(&self) -> usize {
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

impl std::fmt::Display for SMSG_TRANSFER_ABORTED_TransferAbortReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::ErrorX => f.write_str("ErrorX"),
            Self::MaxPlayers => f.write_str("MaxPlayers"),
            Self::NotFound => f.write_str("NotFound"),
            Self::TooManyInstances => f.write_str("TooManyInstances"),
            Self::ZoneInCombat => f.write_str("ZoneInCombat"),
            Self::InsufficientExpansionLevel{ .. } => f.write_str("InsufficientExpansionLevel"),
            Self::DifficultyNotAvailable{ .. } => f.write_str("DifficultyNotAvailable"),
            Self::UniqueMessage{ .. } => f.write_str("UniqueMessage"),
            Self::TooManyRealmInstances => f.write_str("TooManyRealmInstances"),
            Self::NeedGroup => f.write_str("NeedGroup"),
            Self::NotFound1 => f.write_str("NotFound1"),
            Self::NotFound2 => f.write_str("NotFound2"),
            Self::NotFound3 => f.write_str("NotFound3"),
            Self::RealmOnly => f.write_str("RealmOnly"),
            Self::MapNotAllowed => f.write_str("MapNotAllowed"),
        }
    }
}

impl SMSG_TRANSFER_ABORTED_TransferAbortReason {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::InsufficientExpansionLevel {
                ..
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            Self::DifficultyNotAvailable {
                ..
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            Self::UniqueMessage {
                ..
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            _ => 1,
        }
    }
}

