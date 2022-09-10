use std::convert::{TryFrom, TryInto};
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
///         u8 unknown0;
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
    pub queue_slot: u32,
    pub map: SMSG_BATTLEFIELD_STATUS_Map,
}

impl crate::Message for SMSG_BATTLEFIELD_STATUS {
    const OPCODE: u32 = 0x02d4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // queue_slot: u32
        w.write_all(&self.queue_slot.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        match &self.map {
            SMSG_BATTLEFIELD_STATUS_Map::EasternKingdoms => {}
            SMSG_BATTLEFIELD_STATUS_Map::Kalimdor {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
            SMSG_BATTLEFIELD_STATUS_Map::Monastery {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
            SMSG_BATTLEFIELD_STATUS_Map::Gnomeragon {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
            SMSG_BATTLEFIELD_STATUS_Map::CavernsOfTime {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
            SMSG_BATTLEFIELD_STATUS_Map::Mauradon {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

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

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // queue_slot: u32
        let queue_slot = crate::util::read_u32_le(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        let map_if = match map {
            Map::EasternKingdoms => SMSG_BATTLEFIELD_STATUS_Map::EasternKingdoms,
            Map::Kalimdor => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Testing => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ScottTest => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::CashTest => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::AlteracValley => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ShadowfangKeep => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::StormwindStockade => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::StormwindPrison => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Deadmines => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::AzsharaCrater => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::CollinsTest => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::WailingCaverns => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Monastery => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Monastery {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RazorfenKraul => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BlackfathomDeeps => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Uldaman => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Gnomeragon => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Gnomeragon {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::SunkenTemple => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RazorfenDowns => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::EmeraldDream => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ScarletMonastery => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ZulFarrak => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BlackrockSpire => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BlackrockDepths => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::OnyxiasLair => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::CavernsOfTime => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::CavernsOfTime {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Scholomance => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ZulGurub => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Stratholme => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Mauradon => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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

                SMSG_BATTLEFIELD_STATUS_Map::Mauradon {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::DeeprunTram => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RagefireChasm => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::MoltenCore => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::DireMaul => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::AlliancePvpBarracks => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::HordePvpBarracks => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::DevelopmentLand => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BlackwingLair => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::WarsongGulch => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RuinsOfAhnQiraj => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ArathiBasin => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::AhnQirajTemple => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::Naxxramas => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

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
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
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
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Testing {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ScottTest {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    CashTest {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    AlteracValley {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ShadowfangKeep {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    StormwindStockade {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    StormwindPrison {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Deadmines {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    AzsharaCrater {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    CollinsTest {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    WailingCaverns {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Monastery {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RazorfenKraul {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BlackfathomDeeps {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Uldaman {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Gnomeragon {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    SunkenTemple {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RazorfenDowns {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    EmeraldDream {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ScarletMonastery {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ZulFarrak {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BlackrockSpire {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BlackrockDepths {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    OnyxiasLair {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    CavernsOfTime {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Scholomance {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ZulGurub {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Stratholme {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Mauradon {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    DeeprunTram {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RagefireChasm {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    MoltenCore {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    DireMaul {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    AlliancePvpBarracks {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    HordePvpBarracks {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    DevelopmentLand {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BlackwingLair {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    WarsongGulch {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RuinsOfAhnQiraj {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ArathiBasin {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    AhnQirajTemple {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    Naxxramas {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
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
            Self::Monastery { .. } => 44,
            Self::RazorfenKraul { .. } => 47,
            Self::BlackfathomDeeps { .. } => 48,
            Self::Uldaman { .. } => 70,
            Self::Gnomeragon { .. } => 90,
            Self::SunkenTemple { .. } => 109,
            Self::RazorfenDowns { .. } => 129,
            Self::EmeraldDream { .. } => 169,
            Self::ScarletMonastery { .. } => 189,
            Self::ZulFarrak { .. } => 209,
            Self::BlackrockSpire { .. } => 229,
            Self::BlackrockDepths { .. } => 230,
            Self::OnyxiasLair { .. } => 249,
            Self::CavernsOfTime { .. } => 269,
            Self::Scholomance { .. } => 289,
            Self::ZulGurub { .. } => 309,
            Self::Stratholme { .. } => 329,
            Self::Mauradon { .. } => 349,
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
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Testing {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ScottTest {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::CashTest {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::AlteracValley {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ShadowfangKeep {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::StormwindStockade {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::StormwindPrison {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Deadmines {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::AzsharaCrater {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::CollinsTest {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::WailingCaverns {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Monastery {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RazorfenKraul {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BlackfathomDeeps {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Uldaman {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Gnomeragon {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::SunkenTemple {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RazorfenDowns {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::EmeraldDream {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ScarletMonastery {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ZulFarrak {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BlackrockSpire {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BlackrockDepths {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::OnyxiasLair {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::CavernsOfTime {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Scholomance {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ZulGurub {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Stratholme {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Mauradon {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::DeeprunTram {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RagefireChasm {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::MoltenCore {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::DireMaul {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::AlliancePvpBarracks {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::HordePvpBarracks {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::DevelopmentLand {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BlackwingLair {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::WarsongGulch {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RuinsOfAhnQiraj {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ArathiBasin {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::AhnQirajTemple {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::Naxxramas {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
        }
    }
}

