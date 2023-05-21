use std::io::{Read, Write};

use crate::vanilla::{
    BattlegroundBracket, Map, StatusId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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
    pub queue_slot: u32,
    pub map: SMSG_BATTLEFIELD_STATUS_Map,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_STATUS {}
impl SMSG_BATTLEFIELD_STATUS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(8..=22).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D4, size: body_size });
        }

        // queue_slot: u32
        let queue_slot = crate::util::read_u32_le(&mut r)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        let map_if = match map {
            Map::EasternKingdoms => SMSG_BATTLEFIELD_STATUS_Map::EasternKingdoms,
            Map::Kalimdor => {
                // bracket: BattlegroundBracket
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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
                let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(&mut r)?;

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
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(&mut r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::InProgress => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(&mut r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(&mut r)?;

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

impl crate::Message for SMSG_BATTLEFIELD_STATUS {
    const OPCODE: u32 = 0x02d4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_STATUS {{").unwrap();
        // Members
        writeln!(s, "    queue_slot = {};", self.queue_slot).unwrap();
        writeln!(s, "    map = {};", Map::try_from(self.map.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.map {
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Kalimdor {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Testing {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ScottTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::CashTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AlteracValley {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ShadowfangKeep {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::StormwindStockade {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::StormwindPrison {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Deadmines {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AzsharaCrater {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::CollinsTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::WailingCaverns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::MonasteryUnused {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RazorfenKraul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackfathomDeeps {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Uldaman {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Gnomeregan {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::SunkenTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RazorfenDowns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::EmeraldDream {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ScarletMonastery {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ZulFarrak {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackrockSpire {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackrockDepths {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::OnyxiasLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::OpeningOfTheDarkPortal {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Scholomance {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ZulGurub {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Stratholme {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Maraudon {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::DeeprunTram {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RagefireChasm {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::MoltenCore {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::DireMaul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AlliancePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::HordePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::DevelopmentLand {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackwingLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::WarsongGulch {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RuinsOfAhnQiraj {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ArathiBasin {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AhnQirajTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Naxxramas {
                bracket,
                client_instance_id,
                status_id,
            } => {
                writeln!(s, "    bracket = {};", bracket.as_test_case_value()).unwrap();
                writeln!(s, "    client_instance_id = {};", client_instance_id).unwrap();
                writeln!(s, "    status_id = {};", StatusId::try_from(status_id.as_int()).unwrap().as_test_case_value()).unwrap();
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        writeln!(s, "    average_wait_time_in_ms = {};", average_wait_time_in_ms).unwrap();
                        writeln!(s, "    time_in_queue_in_ms = {};", time_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        writeln!(s, "    time_to_remove_in_queue_in_ms = {};", time_to_remove_in_queue_in_ms).unwrap();
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        writeln!(s, "    time_to_bg_autoleave_in_ms = {};", time_to_bg_autoleave_in_ms).unwrap();
                        writeln!(s, "    time_to_bg_start_in_ms = {};", time_to_bg_start_in_ms).unwrap();
                    }
                    _ => {}
                }

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
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        match &self.map {
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Kalimdor {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Testing {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ScottTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::CashTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AlteracValley {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ShadowfangKeep {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::StormwindStockade {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::StormwindPrison {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Deadmines {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AzsharaCrater {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::CollinsTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::WailingCaverns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::MonasteryUnused {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RazorfenKraul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackfathomDeeps {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Uldaman {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Gnomeregan {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::SunkenTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RazorfenDowns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::EmeraldDream {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ScarletMonastery {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ZulFarrak {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackrockSpire {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackrockDepths {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::OnyxiasLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::OpeningOfTheDarkPortal {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Scholomance {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ZulGurub {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Stratholme {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Maraudon {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::DeeprunTram {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RagefireChasm {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::MoltenCore {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::DireMaul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AlliancePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::HordePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::DevelopmentLand {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::BlackwingLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::WarsongGulch {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::RuinsOfAhnQiraj {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::ArathiBasin {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::AhnQirajTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

            }
            crate::vanilla::SMSG_BATTLEFIELD_STATUS_Map::Naxxramas {
                bracket,
                client_instance_id,
                status_id,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "client_instance_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status_id", "    ");
                match &status_id {
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitQueue {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::WaitJoin {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_remove_in_queue_in_ms", "    ");
                    }
                    crate::vanilla::SMSG_BATTLEFIELD_STATUS_StatusId::InProgress {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_autoleave_in_ms", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_to_bg_start_in_ms", "    ");
                    }
                    _ => {}
                }

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
        // queue_slot: u32
        w.write_all(&self.queue_slot.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        match &self.map {
            SMSG_BATTLEFIELD_STATUS_Map::Kalimdor {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Testing {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ScottTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::CashTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::AlteracValley {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ShadowfangKeep {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::StormwindStockade {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::StormwindPrison {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Deadmines {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::AzsharaCrater {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::CollinsTest {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::WailingCaverns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::MonasteryUnused {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RazorfenKraul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackfathomDeeps {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Uldaman {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Gnomeregan {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::SunkenTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RazorfenDowns {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::EmeraldDream {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ScarletMonastery {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ZulFarrak {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackrockSpire {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackrockDepths {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::OnyxiasLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::OpeningOfTheDarkPortal {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Scholomance {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ZulGurub {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Stratholme {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Maraudon {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::DeeprunTram {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RagefireChasm {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::MoltenCore {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::DireMaul {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::AlliancePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::HordePvpBarracks {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::DevelopmentLand {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BlackwingLair {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::WarsongGulch {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RuinsOfAhnQiraj {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ArathiBasin {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::AhnQirajTemple {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::Naxxramas {
                bracket,
                client_instance_id,
                status_id,
            } => {
                // bracket: BattlegroundBracket
                w.write_all(&(bracket.as_int().to_le_bytes()))?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                w.write_all(&(status_id.as_int().to_le_bytes()))?;

                match &status_id {
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
                    _ => {}
                }

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BATTLEFIELD_STATUS {}

impl SMSG_BATTLEFIELD_STATUS {
    pub(crate) const fn size(&self) -> usize {
        4 // queue_slot: u32
        + self.map.size() // map: SMSG_BATTLEFIELD_STATUS_Map
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
                + 4 // time_to_remove_in_queue_in_ms: u32
            }
            Self::InProgress {
                ..
            } => {
                1
                + 4 // time_to_bg_autoleave_in_ms: u32
                + 4 // time_to_bg_start_in_ms: u32
            }
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl std::fmt::Display for SMSG_BATTLEFIELD_STATUS_Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EasternKingdoms => f.write_str("EasternKingdoms"),
            Self::Kalimdor{ .. } => f.write_str("Kalimdor"),
            Self::Testing{ .. } => f.write_str("Testing"),
            Self::ScottTest{ .. } => f.write_str("ScottTest"),
            Self::CashTest{ .. } => f.write_str("CashTest"),
            Self::AlteracValley{ .. } => f.write_str("AlteracValley"),
            Self::ShadowfangKeep{ .. } => f.write_str("ShadowfangKeep"),
            Self::StormwindStockade{ .. } => f.write_str("StormwindStockade"),
            Self::StormwindPrison{ .. } => f.write_str("StormwindPrison"),
            Self::Deadmines{ .. } => f.write_str("Deadmines"),
            Self::AzsharaCrater{ .. } => f.write_str("AzsharaCrater"),
            Self::CollinsTest{ .. } => f.write_str("CollinsTest"),
            Self::WailingCaverns{ .. } => f.write_str("WailingCaverns"),
            Self::MonasteryUnused{ .. } => f.write_str("MonasteryUnused"),
            Self::RazorfenKraul{ .. } => f.write_str("RazorfenKraul"),
            Self::BlackfathomDeeps{ .. } => f.write_str("BlackfathomDeeps"),
            Self::Uldaman{ .. } => f.write_str("Uldaman"),
            Self::Gnomeregan{ .. } => f.write_str("Gnomeregan"),
            Self::SunkenTemple{ .. } => f.write_str("SunkenTemple"),
            Self::RazorfenDowns{ .. } => f.write_str("RazorfenDowns"),
            Self::EmeraldDream{ .. } => f.write_str("EmeraldDream"),
            Self::ScarletMonastery{ .. } => f.write_str("ScarletMonastery"),
            Self::ZulFarrak{ .. } => f.write_str("ZulFarrak"),
            Self::BlackrockSpire{ .. } => f.write_str("BlackrockSpire"),
            Self::BlackrockDepths{ .. } => f.write_str("BlackrockDepths"),
            Self::OnyxiasLair{ .. } => f.write_str("OnyxiasLair"),
            Self::OpeningOfTheDarkPortal{ .. } => f.write_str("OpeningOfTheDarkPortal"),
            Self::Scholomance{ .. } => f.write_str("Scholomance"),
            Self::ZulGurub{ .. } => f.write_str("ZulGurub"),
            Self::Stratholme{ .. } => f.write_str("Stratholme"),
            Self::Maraudon{ .. } => f.write_str("Maraudon"),
            Self::DeeprunTram{ .. } => f.write_str("DeeprunTram"),
            Self::RagefireChasm{ .. } => f.write_str("RagefireChasm"),
            Self::MoltenCore{ .. } => f.write_str("MoltenCore"),
            Self::DireMaul{ .. } => f.write_str("DireMaul"),
            Self::AlliancePvpBarracks{ .. } => f.write_str("AlliancePvpBarracks"),
            Self::HordePvpBarracks{ .. } => f.write_str("HordePvpBarracks"),
            Self::DevelopmentLand{ .. } => f.write_str("DevelopmentLand"),
            Self::BlackwingLair{ .. } => f.write_str("BlackwingLair"),
            Self::WarsongGulch{ .. } => f.write_str("WarsongGulch"),
            Self::RuinsOfAhnQiraj{ .. } => f.write_str("RuinsOfAhnQiraj"),
            Self::ArathiBasin{ .. } => f.write_str("ArathiBasin"),
            Self::AhnQirajTemple{ .. } => f.write_str("AhnQirajTemple"),
            Self::Naxxramas{ .. } => f.write_str("Naxxramas"),
        }
    }
}

impl SMSG_BATTLEFIELD_STATUS_Map {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Kalimdor {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Testing {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ScottTest {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::CashTest {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AlteracValley {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ShadowfangKeep {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::StormwindStockade {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::StormwindPrison {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Deadmines {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AzsharaCrater {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::CollinsTest {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::WailingCaverns {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::MonasteryUnused {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RazorfenKraul {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackfathomDeeps {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Uldaman {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Gnomeregan {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::SunkenTemple {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RazorfenDowns {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::EmeraldDream {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ScarletMonastery {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ZulFarrak {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackrockSpire {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackrockDepths {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::OnyxiasLair {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::OpeningOfTheDarkPortal {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Scholomance {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ZulGurub {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Stratholme {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Maraudon {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::DeeprunTram {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RagefireChasm {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::MoltenCore {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::DireMaul {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AlliancePvpBarracks {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::HordePvpBarracks {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::DevelopmentLand {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::BlackwingLair {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::WarsongGulch {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::RuinsOfAhnQiraj {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::ArathiBasin {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::AhnQirajTemple {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            Self::Naxxramas {
                status_id,
                ..
            } => {
                4
                + 1 // bracket: BattlegroundBracket
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
            }
            _ => 4,
        }
    }
}

