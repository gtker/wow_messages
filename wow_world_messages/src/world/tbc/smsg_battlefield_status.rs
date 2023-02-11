use crate::tbc::ArenaType;
use crate::tbc::BattlegroundType;
use crate::tbc::StatusId;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// mangosone treats `arena_type`, `unknown1`, `battleground_type_id`, and `unknown2` as one big u64 and does not send any fields after these if all fields are 0.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L49):
/// ```text
/// smsg SMSG_BATTLEFIELD_STATUS = 0x02D4 {
///     u32 queue_slot;
///     ArenaType arena_type;
///     u8 unknown1;
///     BattlegroundType battleground_type;
///     u16 unknown2;
///     u32 client_instance_id;
///     Bool rated;
///     StatusId status_id;
///     if (status_id == WAIT_QUEUE) {
///         u32 average_wait_time_in_ms;
///         u32 time_in_queue_in_ms;
///     }
///     else if (status_id == WAIT_JOIN) {
///         u32 time_to_remove_in_queue_in_ms;
///     }
///     else if (status_id == IN_PROGRESS) {
///         u32 time_to_bg_autoleave_in_ms;
///         u32 time_to_bg_start_in_ms;
///     }
/// }
/// ```
pub struct SMSG_BATTLEFIELD_STATUS {
    /// vmangos: players can be in 3 queues at the same time (0..2)
    ///
    pub queue_slot: u32,
    pub arena_type: ArenaType,
    /// mangosone sets to 0x0D.
    ///
    pub unknown1: u8,
    pub battleground_type: BattlegroundType,
    /// mangosone sets to 0x1F90
    ///
    pub unknown2: u16,
    pub client_instance_id: u32,
    pub rated: bool,
    pub status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
}

impl crate::Message for SMSG_BATTLEFIELD_STATUS {
    const OPCODE: u32 = 0x02d4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // queue_slot: u32
        w.write_all(&self.queue_slot.to_le_bytes())?;

        // arena_type: ArenaType
        w.write_all(&(self.arena_type.as_int() as u8).to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // battleground_type: BattlegroundType
        w.write_all(&(self.battleground_type.as_int() as u32).to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        // client_instance_id: u32
        w.write_all(&self.client_instance_id.to_le_bytes())?;

        // rated: Bool
        w.write_all(u8::from(self.rated).to_le_bytes().as_slice())?;

        // status_id: StatusId
        w.write_all(&(self.status_id.as_int() as u8).to_le_bytes())?;

        match &self.status_id {
            SMSG_BATTLEFIELD_STATUS_StatusId::None => {}
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
                time_to_remove_in_queue_in_ms,
            } => {
                // time_to_remove_in_queue_in_ms: u32
                w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

            }
            SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                time_to_bg_autoleave_in_ms,
                time_to_bg_start_in_ms,
            } => {
                // time_to_bg_autoleave_in_ms: u32
                w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                // time_to_bg_start_in_ms: u32
                w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

            }
            SMSG_BATTLEFIELD_STATUS_StatusId::WaitLeave => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(18..=26).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D4, size: body_size as u32 });
        }

        // queue_slot: u32
        let queue_slot = crate::util::read_u32_le(r)?;

        // arena_type: ArenaType
        let arena_type: ArenaType = crate::util::read_u8_le(r)?.try_into()?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // battleground_type: BattlegroundType
        let battleground_type: BattlegroundType = crate::util::read_u32_le(r)?.try_into()?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        // client_instance_id: u32
        let client_instance_id = crate::util::read_u32_le(r)?;

        // rated: Bool
        let rated = crate::util::read_u8_le(r)? != 0;
        // status_id: StatusId
        let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

        let status_id_if = match status_id {
            StatusId::None => SMSG_BATTLEFIELD_STATUS_StatusId::None,
            StatusId::WaitQueue => {
                // average_wait_time_in_ms: u32
                let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                // time_in_queue_in_ms: u32
                let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                    average_wait_time_in_ms,
                    time_in_queue_in_ms,
                }
            }
            StatusId::WaitJoin => {
                // time_to_remove_in_queue_in_ms: u32
                let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                    time_to_remove_in_queue_in_ms,
                }
            }
            StatusId::InProgress => {
                // time_to_bg_autoleave_in_ms: u32
                let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                // time_to_bg_start_in_ms: u32
                let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                    time_to_bg_autoleave_in_ms,
                    time_to_bg_start_in_ms,
                }
            }
            StatusId::WaitLeave => SMSG_BATTLEFIELD_STATUS_StatusId::WaitLeave,
        };

        Ok(Self {
            queue_slot,
            arena_type,
            unknown1,
            battleground_type,
            unknown2,
            client_instance_id,
            rated,
            status_id: status_id_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BATTLEFIELD_STATUS {}

impl SMSG_BATTLEFIELD_STATUS {
    pub(crate) fn size(&self) -> usize {
        4 // queue_slot: u32
        + 1 // arena_type: ArenaType
        + 1 // unknown1: u8
        + 4 // battleground_type: BattlegroundType
        + 2 // unknown2: u16
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
        time_to_remove_in_queue_in_ms: u32,
    },
    InProgress {
        time_to_bg_autoleave_in_ms: u32,
        time_to_bg_start_in_ms: u32,
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

impl SMSG_BATTLEFIELD_STATUS_StatusId {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                1
            }
            Self::WaitQueue {
                average_wait_time_in_ms,
                time_in_queue_in_ms,
            } => {
                1
                + 4 // average_wait_time_in_ms: u32
                + 4 // time_in_queue_in_ms: u32
            }
            Self::WaitJoin {
                time_to_remove_in_queue_in_ms,
            } => {
                1
                + 4 // time_to_remove_in_queue_in_ms: u32
            }
            Self::InProgress {
                time_to_bg_autoleave_in_ms,
                time_to_bg_start_in_ms,
            } => {
                1
                + 4 // time_to_bg_autoleave_in_ms: u32
                + 4 // time_to_bg_start_in_ms: u32
            }
            Self::WaitLeave => {
                1
            }
        }
    }
}

