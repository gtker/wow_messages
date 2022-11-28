use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::BattlegroundBracket;
use crate::world::vanilla::Map;
use crate::world::vanilla::StatusId;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L21):
/// ```text
/// smsg SMSG_BATTLEFIELD_STATUS = 0x02D4 {
///     u32 queue_slot;
///     Map map;
///     if (map != EASTERN_KINGDOMS) {
///         BattlegroundBracket bracket;
///         u32 client_instance_id;
///         StatusId status_id;
///         if (status_id == WAIT_QUEUE) {
///             u32 average_wait_time_in_ms;
///             u32 time_in_queue_in_ms;
///         }
///         else if (status_id == WAIT_JOIN) {
///             u32 time_to_remove_in_queue_in_ms;
///         }
///         else if (status_id == IN_PROGRESS) {
///             u32 time_to_bg_autoleave_in_ms;
///             u32 time_to_bg_start_in_ms;
///         }
///     }
/// }
/// ```
pub struct SMSG_BATTLEFIELD_STATUS {
    /// vmangos: players can be in 3 queues at the same time (0..2)
    ///
    pub queue_slot: u32,
    pub map: SMSG_BATTLEFIELD_STATUS_Map,
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

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        match &self.map {
            SMSG_BATTLEFIELD_STATUS_Map::EasternKingdoms => {}
            SMSG_BATTLEFIELD_STATUS_Map::Kalimdor {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Testing {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::ScottTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::CashTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::AlteracValley {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::ShadowfangKeep {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::StormwindStockade {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::StormwindPrison {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Deadmines {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::AzsharaCrater {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::CollinsTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::WailingCaverns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::MonasteryUnused {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::RazorfenKraul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackfathomDeeps {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Uldaman {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Gnomeregan {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::SunkenTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::RazorfenDowns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::EmeraldDream {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::ScarletMonastery {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::ZulFarrak {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackrockSpire {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackrockDepths {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::OnyxiasLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::OpeningOfTheDarkPortal {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Scholomance {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::ZulGurub {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Stratholme {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Maraudon {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::DeeprunTram {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::RagefireChasm {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::MoltenCore {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::DireMaul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::AlliancePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::HordePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::DevelopmentLand {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackwingLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::WarsongGulch {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::RuinsOfAhnQiraj {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::ArathiBasin {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::AhnQirajTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
            SMSG_BATTLEFIELD_STATUS_Map::Naxxramas {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int() as u8).to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int() as u8).to_le_bytes())?;

                match &status_id {
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

            }
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=22).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D4, size: body_size as u32 });
        }

        // queue_slot: u32
        let queue_slot = crate::util::read_u32_le(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        let map_if = match map {
            Map::EasternKingdoms => SMSG_BATTLEFIELD_STATUS_Map::EasternKingdoms,
            Map::Kalimdor => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Kalimdor {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Testing => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Testing {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ScottTest => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::ScottTest {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::CashTest => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::CashTest {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::AlteracValley => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::AlteracValley {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ShadowfangKeep => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::ShadowfangKeep {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::StormwindStockade => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::StormwindStockade {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::StormwindPrison => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::StormwindPrison {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Deadmines => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Deadmines {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::AzsharaCrater => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::AzsharaCrater {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::CollinsTest => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::CollinsTest {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::WailingCaverns => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::WailingCaverns {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::MonasteryUnused => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::MonasteryUnused {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RazorfenKraul => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::RazorfenKraul {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BlackfathomDeeps => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::BlackfathomDeeps {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Uldaman => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Uldaman {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Gnomeregan => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Gnomeregan {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::SunkenTemple => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::SunkenTemple {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RazorfenDowns => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::RazorfenDowns {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::EmeraldDream => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::EmeraldDream {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ScarletMonastery => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::ScarletMonastery {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ZulFarrak => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::ZulFarrak {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BlackrockSpire => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::BlackrockSpire {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BlackrockDepths => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::BlackrockDepths {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::OnyxiasLair => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::OnyxiasLair {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::OpeningOfTheDarkPortal => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::OpeningOfTheDarkPortal {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Scholomance => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Scholomance {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ZulGurub => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::ZulGurub {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Stratholme => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Stratholme {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Maraudon => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Maraudon {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::DeeprunTram => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::DeeprunTram {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RagefireChasm => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::RagefireChasm {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::MoltenCore => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::MoltenCore {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::DireMaul => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::DireMaul {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::AlliancePvpBarracks => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::AlliancePvpBarracks {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::HordePvpBarracks => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::HordePvpBarracks {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::DevelopmentLand => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::DevelopmentLand {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BlackwingLair => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::BlackwingLair {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::WarsongGulch => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::WarsongGulch {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RuinsOfAhnQiraj => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::RuinsOfAhnQiraj {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ArathiBasin => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::ArathiBasin {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::AhnQirajTemple => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::AhnQirajTemple {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::Naxxramas => {
                // bracket: BattlegroundBracket
                let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Naxxramas {
                    bracket,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
        };

        Ok(Self {
            queue_slot,
            map: map_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_BATTLEFIELD_STATUS {}

impl SMSG_BATTLEFIELD_STATUS {
    pub(crate) fn size(&self) -> usize {
        4 // queue_slot: u32
        + self.map.size() // map: SMSG_BATTLEFIELD_STATUS_Map
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SMSG_BATTLEFIELD_STATUS_Map {
    EasternKingdoms,
    Kalimdor {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Testing {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    ScottTest {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    CashTest {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    AlteracValley {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    ShadowfangKeep {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    StormwindStockade {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    StormwindPrison {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Deadmines {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    AzsharaCrater {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    CollinsTest {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    WailingCaverns {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    MonasteryUnused {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    RazorfenKraul {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    BlackfathomDeeps {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Uldaman {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Gnomeregan {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    SunkenTemple {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    RazorfenDowns {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    EmeraldDream {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    ScarletMonastery {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    ZulFarrak {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    BlackrockSpire {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    BlackrockDepths {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    OnyxiasLair {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    OpeningOfTheDarkPortal {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Scholomance {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    ZulGurub {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Stratholme {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Maraudon {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    DeeprunTram {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    RagefireChasm {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    MoltenCore {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    DireMaul {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    AlliancePvpBarracks {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    HordePvpBarracks {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    DevelopmentLand {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    BlackwingLair {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    WarsongGulch {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    RuinsOfAhnQiraj {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    ArathiBasin {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    AhnQirajTemple {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
    Naxxramas {
        bracket: BattlegroundBracket,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
    },
}

impl Default for SMSG_BATTLEFIELD_STATUS_Map {
    fn default() -> Self {
        // First enumerator without any fields
        Self::EasternKingdoms
    }
}

impl SMSG_BATTLEFIELD_STATUS_Map {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::EasternKingdoms => 0,
            Self::Kalimdor { .. } => 1,
            Self::Testing { .. } => 13,
            Self::ScottTest { .. } => 25,
            Self::CashTest { .. } => 29,
            Self::AlteracValley { .. } => 30,
            Self::ShadowfangKeep { .. } => 33,
            Self::StormwindStockade { .. } => 34,
            Self::StormwindPrison { .. } => 35,
            Self::Deadmines { .. } => 36,
            Self::AzsharaCrater { .. } => 37,
            Self::CollinsTest { .. } => 42,
            Self::WailingCaverns { .. } => 43,
            Self::MonasteryUnused { .. } => 44,
            Self::RazorfenKraul { .. } => 47,
            Self::BlackfathomDeeps { .. } => 48,
            Self::Uldaman { .. } => 70,
            Self::Gnomeregan { .. } => 90,
            Self::SunkenTemple { .. } => 109,
            Self::RazorfenDowns { .. } => 129,
            Self::EmeraldDream { .. } => 169,
            Self::ScarletMonastery { .. } => 189,
            Self::ZulFarrak { .. } => 209,
            Self::BlackrockSpire { .. } => 229,
            Self::BlackrockDepths { .. } => 230,
            Self::OnyxiasLair { .. } => 249,
            Self::OpeningOfTheDarkPortal { .. } => 269,
            Self::Scholomance { .. } => 289,
            Self::ZulGurub { .. } => 309,
            Self::Stratholme { .. } => 329,
            Self::Maraudon { .. } => 349,
            Self::DeeprunTram { .. } => 369,
            Self::RagefireChasm { .. } => 389,
            Self::MoltenCore { .. } => 409,
            Self::DireMaul { .. } => 429,
            Self::AlliancePvpBarracks { .. } => 449,
            Self::HordePvpBarracks { .. } => 450,
            Self::DevelopmentLand { .. } => 451,
            Self::BlackwingLair { .. } => 469,
            Self::WarsongGulch { .. } => 489,
            Self::RuinsOfAhnQiraj { .. } => 509,
            Self::ArathiBasin { .. } => 529,
            Self::AhnQirajTemple { .. } => 531,
            Self::Naxxramas { .. } => 533,
        }
    }

}

impl SMSG_BATTLEFIELD_STATUS_Map {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::EasternKingdoms => {
                4
            }
            Self::Kalimdor {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Testing {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ScottTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::CashTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AlteracValley {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ShadowfangKeep {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::StormwindStockade {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::StormwindPrison {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Deadmines {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AzsharaCrater {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::CollinsTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::WailingCaverns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::MonasteryUnused {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RazorfenKraul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackfathomDeeps {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Uldaman {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Gnomeregan {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::SunkenTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RazorfenDowns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::EmeraldDream {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ScarletMonastery {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ZulFarrak {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackrockSpire {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackrockDepths {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::OnyxiasLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::OpeningOfTheDarkPortal {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Scholomance {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ZulGurub {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Stratholme {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Maraudon {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::DeeprunTram {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RagefireChasm {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::MoltenCore {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::DireMaul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AlliancePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::HordePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::DevelopmentLand {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackwingLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::WarsongGulch {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RuinsOfAhnQiraj {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ArathiBasin {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AhnQirajTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Naxxramas {
                bracket,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
        }
    }
}

