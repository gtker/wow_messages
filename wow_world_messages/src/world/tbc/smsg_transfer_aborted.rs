use std::io::{Read, Write};

use crate::tbc::{
    DungeonDifficulty, Map, TransferAbortReason,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:45`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L45):
/// ```text
/// smsg SMSG_TRANSFER_ABORTED = 0x0040 {
///     Map map;
///     TransferAbortReason reason;
///     if (reason == INSUFFICIENT_EXPANSION_LEVEL
///         || reason == DIFFICULTY_NOT_AVAILABLE) {
///         DungeonDifficulty difficulty;
///     }
/// }
/// ```
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: SMSG_TRANSFER_ABORTED_TransferAbortReason,
}

#[cfg(feature = "print-testcase")]
impl SMSG_TRANSFER_ABORTED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRANSFER_ABORTED {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    reason = {};", crate::tbc::TransferAbortReason::try_from(self.reason.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.reason {
            crate::tbc::SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                difficulty,
            } => {
                writeln!(s, "    difficulty = {};", difficulty.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                difficulty,
            } => {
                writeln!(s, "    difficulty = {};", difficulty.as_test_case_value()).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 64_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_TRANSFER_ABORTED {}
impl crate::Message for SMSG_TRANSFER_ABORTED {
    const OPCODE: u32 = 0x0040;

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
            _ => {}
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=6).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0040, size: body_size });
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // reason: TransferAbortReason
        let reason = crate::util::read_u8_le(&mut r)?.try_into()?;

        let reason_if = match reason {
            TransferAbortReason::None => SMSG_TRANSFER_ABORTED_TransferAbortReason::None,
            TransferAbortReason::IsFull => SMSG_TRANSFER_ABORTED_TransferAbortReason::IsFull,
            TransferAbortReason::NotFound => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound,
            TransferAbortReason::TooManyInstances => SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyInstances,
            TransferAbortReason::ZoneIsInCombat => SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneIsInCombat,
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
            TransferAbortReason::MissingDifficulty => SMSG_TRANSFER_ABORTED_TransferAbortReason::MissingDifficulty,
            TransferAbortReason::ZoneInCombat => SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneInCombat,
            TransferAbortReason::InstanceIsFull => SMSG_TRANSFER_ABORTED_TransferAbortReason::InstanceIsFull,
            TransferAbortReason::NotAllowed => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotAllowed,
            TransferAbortReason::HasBind => SMSG_TRANSFER_ABORTED_TransferAbortReason::HasBind,
        };

        Ok(Self {
            map,
            reason: reason_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TRANSFER_ABORTED {}

impl SMSG_TRANSFER_ABORTED {
    pub(crate) const fn size(&self) -> usize {
        4 // map: Map
        + self.reason.size() // reason: SMSG_TRANSFER_ABORTED_TransferAbortReason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_TRANSFER_ABORTED_TransferAbortReason {
    None,
    IsFull,
    NotFound,
    TooManyInstances,
    ZoneIsInCombat,
    InsufficientExpansionLevel {
        difficulty: DungeonDifficulty,
    },
    DifficultyNotAvailable {
        difficulty: DungeonDifficulty,
    },
    MissingDifficulty,
    ZoneInCombat,
    InstanceIsFull,
    NotAllowed,
    HasBind,
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
            Self::IsFull => 1,
            Self::NotFound => 2,
            Self::TooManyInstances => 3,
            Self::ZoneIsInCombat => 5,
            Self::InsufficientExpansionLevel { .. } => 6,
            Self::DifficultyNotAvailable { .. } => 7,
            Self::MissingDifficulty => 8,
            Self::ZoneInCombat => 9,
            Self::InstanceIsFull => 10,
            Self::NotAllowed => 11,
            Self::HasBind => 12,
        }
    }

}

impl std::fmt::Display for SMSG_TRANSFER_ABORTED_TransferAbortReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::IsFull => f.write_str("IsFull"),
            Self::NotFound => f.write_str("NotFound"),
            Self::TooManyInstances => f.write_str("TooManyInstances"),
            Self::ZoneIsInCombat => f.write_str("ZoneIsInCombat"),
            Self::InsufficientExpansionLevel{ .. } => f.write_str("InsufficientExpansionLevel"),
            Self::DifficultyNotAvailable{ .. } => f.write_str("DifficultyNotAvailable"),
            Self::MissingDifficulty => f.write_str("MissingDifficulty"),
            Self::ZoneInCombat => f.write_str("ZoneInCombat"),
            Self::InstanceIsFull => f.write_str("InstanceIsFull"),
            Self::NotAllowed => f.write_str("NotAllowed"),
            Self::HasBind => f.write_str("HasBind"),
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
            _ => 1,
        }
    }
}

