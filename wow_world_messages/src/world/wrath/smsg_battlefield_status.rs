use std::io::{Read, Write};

use crate::wrath::{
    ArenaFaction, ArenaType, BattlegroundType, Map, StatusId,
};

/// mangosone treats `arena_type`, `unknown1`, `battleground_type_id`, and `unknown2` as one big u64 and does not send any fields after these if all fields are 0.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:88`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L88):
/// ```text
/// smsg SMSG_BATTLEFIELD_STATUS = 0x02D4 {
///     u32 queue_slot;
///     ArenaType arena_type;
///     u8 is_arena;
///     BattlegroundType battleground_type;
///     u16 unknown1;
///     u8 minimum_level;
///     u8 maximum_level;
///     u32 client_instance_id;
///     Bool rated;
///     StatusId status_id;
///     if (status_id == WAIT_QUEUE) {
///         u32 average_wait_time_in_ms;
///         u32 time_in_queue_in_ms;
///     }
///     else if (status_id == WAIT_JOIN) {
///         Map map1;
///         u64 unknown2;
///         u32 time_to_remove_in_queue_in_ms;
///     }
///     else if (status_id == IN_PROGRESS) {
///         Map map2;
///         u64 unknown3;
///         u32 time_to_bg_autoleave_in_ms;
///         u32 time_to_bg_start_in_ms;
///         ArenaFaction faction;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_BATTLEFIELD_STATUS {
    /// vmangos: players can be in 3 queues at the same time (0..2)
    pub queue_slot: u32,
    pub arena_type: ArenaType,
    /// azerothcore sets to 0x0E if it is arena, 0 otherwise.
    pub is_arena: u8,
    pub battleground_type: BattlegroundType,
    /// azerothcore sets to 0x1F90
    pub unknown1: u16,
    pub minimum_level: u8,
    pub maximum_level: u8,
    pub client_instance_id: u32,
    pub rated: bool,
    pub status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_STATUS {}
impl SMSG_BATTLEFIELD_STATUS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(20..=41).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // queue_slot: u32
        let queue_slot = crate::util::read_u32_le(&mut r)?;

        // arena_type: ArenaType
        let arena_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // is_arena: u8
        let is_arena = crate::util::read_u8_le(&mut r)?;

        // battleground_type: BattlegroundType
        let battleground_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(&mut r)?;

        // minimum_level: u8
        let minimum_level = crate::util::read_u8_le(&mut r)?;

        // maximum_level: u8
        let maximum_level = crate::util::read_u8_le(&mut r)?;

        // client_instance_id: u32
        let client_instance_id = crate::util::read_u32_le(&mut r)?;

        // rated: Bool
        let rated = crate::util::read_bool_u8(&mut r)?;

        // status_id: StatusId
        let status_id = crate::util::read_u8_le(&mut r)?.try_into()?;

        let status_id_if = match status_id {
            StatusId::None => SMSG_BATTLEFIELD_STATUS_StatusId::None,
            StatusId::WaitQueue => {
                // average_wait_time_in_ms: u32
                let average_wait_time_in_ms = crate::util::read_u32_le(&mut r)?;

                // time_in_queue_in_ms: u32
                let time_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                    average_wait_time_in_ms,
                    time_in_queue_in_ms,
                }
            }
            StatusId::WaitJoin => {
                // map1: Map
                let map1 = crate::util::read_u32_le(&mut r)?.try_into()?;

                // unknown2: u64
                let unknown2 = crate::util::read_u64_le(&mut r)?;

                // time_to_remove_in_queue_in_ms: u32
                let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                    map1,
                    time_to_remove_in_queue_in_ms,
                    unknown2,
                }
            }
            StatusId::InProgress => {
                // map2: Map
                let map2 = crate::util::read_u32_le(&mut r)?.try_into()?;

                // unknown3: u64
                let unknown3 = crate::util::read_u64_le(&mut r)?;

                // time_to_bg_autoleave_in_ms: u32
                let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                // time_to_bg_start_in_ms: u32
                let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

                // faction: ArenaFaction
                let faction = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                    faction,
                    map2,
                    time_to_bg_autoleave_in_ms,
                    time_to_bg_start_in_ms,
                    unknown3,
                }
            }
            StatusId::WaitLeave => SMSG_BATTLEFIELD_STATUS_StatusId::WaitLeave,
        };

        Ok(Self {
            queue_slot,
            arena_type,
            is_arena,
            battleground_type,
            unknown1,
            minimum_level,
            maximum_level,
            client_instance_id,
            rated,
            status_id: status_id_if,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_STATUS {
    const OPCODE: u32 = 0x02d4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEFIELD_STATUS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_STATUS {{").unwrap();
        // Members
        writeln!(s, "    queue_slot = {};", self.queue_slot).unwrap();
        writeln!(s, "    arena_type = {};", self.arena_type.as_test_case_value()).unwrap();
        writeln!(s, "    is_arena = {};", self.is_arena).unwrap();
        writeln!(s, "    battleground_type = {};", self.battleground_type.as_test_case_value()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    minimum_level = {};", self.minimum_level).unwrap();
        writeln!(s, "    maximum_level = {};", self.maximum_level).unwrap();
        writeln!(s, "    client_instance_id = {};", self.client_instance_id).unwrap();
        writeln!(s, "    rated = {};", if self.rated { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    status_id = {};", StatusId::try_from(self.status_id.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.status_id {
            crate::wrath::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                average_wait_time_in_ms,
                time_in_queue_in_ms,
            } => {
                writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
            }
            crate::wrath::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                map1,
                time_to_remove_in_queue_in_ms,
                unknown2,
            } => {
                writeln!(s, "    map1 = {};", map1.as_test_case_value()).unwrap();
                writeln!(s, "    unknown2 = {};", unknown2).unwrap();
                writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
            }
            crate::wrath::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                faction,
                map2,
                time_to_bg_autoleave_in_ms,
                time_to_bg_start_in_ms,
                unknown3,
            } => {
                writeln!(s, "    map2 = {};", map2.as_test_case_value()).unwrap();
                writeln!(s, "    unknown3 = {};", unknown3).unwrap();
                writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                writeln!(s, "    faction = {};", faction.as_test_case_value()).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 724_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "queue_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "arena_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "is_arena", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "battleground_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "minimum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "maximum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "rated", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
        match &self.status_id {
            crate::wrath::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                average_wait_time_in_ms,
                time_in_queue_in_ms,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
            }
            crate::wrath::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                map1,
                time_to_remove_in_queue_in_ms,
                unknown2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "map1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "unknown2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
            }
            crate::wrath::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                faction,
                map2,
                time_to_bg_autoleave_in_ms,
                time_to_bg_start_in_ms,
                unknown3,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "map2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "unknown3", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "faction", "    ");
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
        // queue_slot: u32
        w.write_all(&self.queue_slot.to_le_bytes())?;

        // arena_type: ArenaType
        w.write_all(&(self.arena_type.as_int().to_le_bytes()))?;

        // is_arena: u8
        w.write_all(&self.is_arena.to_le_bytes())?;

        // battleground_type: BattlegroundType
        w.write_all(&(self.battleground_type.as_int().to_le_bytes()))?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        // minimum_level: u8
        w.write_all(&self.minimum_level.to_le_bytes())?;

        // maximum_level: u8
        w.write_all(&self.maximum_level.to_le_bytes())?;

        // client_instance_id: u32
        w.write_all(&self.client_instance_id.to_le_bytes())?;

        // rated: Bool
        w.write_all(u8::from(self.rated).to_le_bytes().as_slice())?;

        // status_id: StatusId
        w.write_all(&(self.status_id.as_int().to_le_bytes()))?;

        match &self.status_id {
            SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                average_wait_time_in_ms,
                time_in_queue_in_ms,
            } => {
                // average_wait_time_in_ms: u32
                w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                // time_in_queue_in_ms: u32
                w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

            }
            SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                map1,
                time_to_remove_in_queue_in_ms,
                unknown2,
            } => {
                // map1: Map
                w.write_all(&(map1.as_int().to_le_bytes()))?;

                // unknown2: u64
                w.write_all(&unknown2.to_le_bytes())?;

                // time_to_remove_in_queue_in_ms: u32
                w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

            }
            SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                faction,
                map2,
                time_to_bg_autoleave_in_ms,
                time_to_bg_start_in_ms,
                unknown3,
            } => {
                // map2: Map
                w.write_all(&(map2.as_int().to_le_bytes()))?;

                // unknown3: u64
                w.write_all(&unknown3.to_le_bytes())?;

                // time_to_bg_autoleave_in_ms: u32
                w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                // time_to_bg_start_in_ms: u32
                w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                // faction: ArenaFaction
                w.write_all(&(faction.as_int().to_le_bytes()))?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(724, "SMSG_BATTLEFIELD_STATUS", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_STATUS {}

impl SMSG_BATTLEFIELD_STATUS {
    pub(crate) const fn size(&self) -> usize {
        4 // queue_slot: u32
        + 1 // arena_type: ArenaType
        + 1 // is_arena: u8
        + 4 // battleground_type: BattlegroundType
        + 2 // unknown1: u16
        + 1 // minimum_level: u8
        + 1 // maximum_level: u8
        + 4 // client_instance_id: u32
        + 1 // rated: Bool
        + self.status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_BATTLEFIELD_STATUS_StatusId {
    None,
    WaitQueue {
        average_wait_time_in_ms: u32,
        time_in_queue_in_ms: u32,
    },
    WaitJoin {
        map1: Map,
        time_to_remove_in_queue_in_ms: u32,
        unknown2: u64,
    },
    InProgress {
        faction: ArenaFaction,
        map2: Map,
        time_to_bg_autoleave_in_ms: u32,
        time_to_bg_start_in_ms: u32,
        unknown3: u64,
    },
    WaitLeave,
}

impl Default for SMSG_BATTLEFIELD_STATUS_StatusId {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SMSG_BATTLEFIELD_STATUS_StatusId {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::WaitQueue { .. } => 1,
            Self::WaitJoin { .. } => 2,
            Self::InProgress { .. } => 3,
            Self::WaitLeave => 4,
        }
    }

}

impl std::fmt::Display for SMSG_BATTLEFIELD_STATUS_StatusId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::WaitQueue{ .. } => f.write_str("WaitQueue"),
            Self::WaitJoin{ .. } => f.write_str("WaitJoin"),
            Self::InProgress{ .. } => f.write_str("InProgress"),
            Self::WaitLeave => f.write_str("WaitLeave"),
        }
    }
}

impl SMSG_BATTLEFIELD_STATUS_StatusId {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::WaitQueue {
                ..
            } => {
                1
                + 4 // average_wait_time_in_ms: u32
                + 4 // time_in_queue_in_ms: u32
            }
            Self::WaitJoin {
                ..
            } => {
                1
                + 4 // map1: Map
                + 4 // time_to_remove_in_queue_in_ms: u32
                + 8 // unknown2: u64
            }
            Self::InProgress {
                ..
            } => {
                1
                + 1 // faction: ArenaFaction
                + 4 // map2: Map
                + 4 // time_to_bg_autoleave_in_ms: u32
                + 4 // time_to_bg_start_in_ms: u32
                + 8 // unknown3: u64
            }
            _ => 1,
        }
    }
}

