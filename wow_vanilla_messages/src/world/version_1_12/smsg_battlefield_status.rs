use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::map::{Map, map_try_from};
use crate::world::version_1_12::StatusId;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
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

impl ServerMessage for SMSG_BATTLEFIELD_STATUS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // queue_slot: u32
        w.write_all(&self.queue_slot.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        match &self.map {
            SMSG_BATTLEFIELD_STATUS_Map::EASTERN_KINGDOMS => {}
            SMSG_BATTLEFIELD_STATUS_Map::KALIMDOR {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::TESTING {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::SCOTT_TEST {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::CASH_TEST {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ALTERAC_VALLEY {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::SHADOWFANG_KEEP {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::STORMWIND_STOCKADE {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::STORMWIND_PRISON {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::DEADMINES {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::AZSHARA_CRATER {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::COLLINS_TEST {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::WAILING_CAVERNS {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::MONASTERY {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RAZORFEN_KRAUL {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BLACKFATHOM_DEEPS {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ULDAMAN {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::GNOMERAGON {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::SUNKEN_TEMPLE {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RAZORFEN_DOWNS {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::EMERALD_DREAM {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::SCARLET_MONASTERY {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ZUL_FARRAK {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BLACKROCK_SPIRE {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BLACKROCK_DEPTHS {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ONYXIAS_LAIR {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::CAVERNS_OF_TIME {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::SCHOLOMANCE {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ZUL_GURUB {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::STRATHOLME {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::MAURADON {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::DEEPRUN_TRAM {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RAGEFIRE_CHASM {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::MOLTEN_CORE {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::DIRE_MAUL {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ALLIANCE_PVP_BARRACKS {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::HORDE_PVP_BARRACKS {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::DEVELOPMENT_LAND {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::BLACKWING_LAIR {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::WARSONG_GULCH {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::RUINS_OF_AHN_QIRAJ {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::ARATHI_BASIN {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::AHN_QIRAJ_TEMPLE {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUS_Map::NAXXRAMAS {
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
                    SMSG_BATTLEFIELD_STATUS_StatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE => {}
                }

            }
        }

        Ok(())
    }
    const OPCODE: u16 = 0x02d4;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // queue_slot: u32
        let queue_slot = crate::util::read_u32_le(r)?;

        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

        let map_if = match map {
            Map::EASTERN_KINGDOMS => SMSG_BATTLEFIELD_STATUS_Map::EASTERN_KINGDOMS,
            Map::KALIMDOR => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::KALIMDOR {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::TESTING => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::TESTING {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::SCOTT_TEST => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::SCOTT_TEST {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::CASH_TEST => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::CASH_TEST {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ALTERAC_VALLEY => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::ALTERAC_VALLEY {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::SHADOWFANG_KEEP => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::SHADOWFANG_KEEP {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::STORMWIND_STOCKADE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::STORMWIND_STOCKADE {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::STORMWIND_PRISON => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::STORMWIND_PRISON {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::DEADMINES => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::DEADMINES {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::AZSHARA_CRATER => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::AZSHARA_CRATER {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::COLLINS_TEST => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::COLLINS_TEST {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::WAILING_CAVERNS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::WAILING_CAVERNS {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::MONASTERY => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::MONASTERY {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RAZORFEN_KRAUL => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::RAZORFEN_KRAUL {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BLACKFATHOM_DEEPS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::BLACKFATHOM_DEEPS {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ULDAMAN => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::ULDAMAN {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::GNOMERAGON => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::GNOMERAGON {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::SUNKEN_TEMPLE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::SUNKEN_TEMPLE {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RAZORFEN_DOWNS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::RAZORFEN_DOWNS {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::EMERALD_DREAM => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::EMERALD_DREAM {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::SCARLET_MONASTERY => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::SCARLET_MONASTERY {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ZUL_FARRAK => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::ZUL_FARRAK {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BLACKROCK_SPIRE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::BLACKROCK_SPIRE {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BLACKROCK_DEPTHS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::BLACKROCK_DEPTHS {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ONYXIAS_LAIR => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::ONYXIAS_LAIR {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::CAVERNS_OF_TIME => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::CAVERNS_OF_TIME {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::SCHOLOMANCE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::SCHOLOMANCE {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ZUL_GURUB => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::ZUL_GURUB {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::STRATHOLME => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::STRATHOLME {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::MAURADON => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::MAURADON {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::DEEPRUN_TRAM => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::DEEPRUN_TRAM {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RAGEFIRE_CHASM => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::RAGEFIRE_CHASM {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::MOLTEN_CORE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::MOLTEN_CORE {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::DIRE_MAUL => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::DIRE_MAUL {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ALLIANCE_PVP_BARRACKS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::ALLIANCE_PVP_BARRACKS {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::HORDE_PVP_BARRACKS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::HORDE_PVP_BARRACKS {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::DEVELOPMENT_LAND => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::DEVELOPMENT_LAND {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::BLACKWING_LAIR => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::BLACKWING_LAIR {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::WARSONG_GULCH => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::WARSONG_GULCH {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::RUINS_OF_AHN_QIRAJ => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::RUINS_OF_AHN_QIRAJ {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::ARATHI_BASIN => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::ARATHI_BASIN {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::AHN_QIRAJ_TEMPLE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::AHN_QIRAJ_TEMPLE {
                    client_instance_id,
                    status_id: status_id_if,
                    unknown0,
                }
            }
            Map::NAXXRAMAS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUS_StatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUS_StatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUS_StatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUS_Map::NAXXRAMAS {
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

impl SMSG_BATTLEFIELD_STATUS {
    pub(crate) fn size(&self) -> usize {
        4 // queue_slot: u32
        + self.map.size() // map: SMSG_BATTLEFIELD_STATUS_Map
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_BATTLEFIELD_STATUS_StatusId {
    NONE,
    WAIT_QUEUE {
        average_wait_time_in_ms: u32,
        time_in_queue_in_ms: u32,
    },
    WAIT_JOIN {
        time_to_remove_in_queue_in_ms: u32,
    },
    IN_PROGRESS {
        time_to_bg_autoleave_in_ms: u32,
        time_to_bg_start_in_ms: u32,
    },
    WAIT_LEAVE,
}

impl Default for SMSG_BATTLEFIELD_STATUS_StatusId {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl SMSG_BATTLEFIELD_STATUS_StatusId {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0,
            Self::WAIT_QUEUE { .. } => 1,
            Self::WAIT_JOIN { .. } => 2,
            Self::IN_PROGRESS { .. } => 3,
            Self::WAIT_LEAVE => 4,
        }
    }

}

impl SMSG_BATTLEFIELD_STATUS_StatusId {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NONE => {
                1
            }
            Self::WAIT_QUEUE {
                average_wait_time_in_ms,
                time_in_queue_in_ms,
            } => {
                1
                + 4 // average_wait_time_in_ms: u32
                + 4 // time_in_queue_in_ms: u32
            }
            Self::WAIT_JOIN {
                time_to_remove_in_queue_in_ms,
            } => {
                1
                + 4 // time_to_remove_in_queue_in_ms: u32
            }
            Self::IN_PROGRESS {
                time_to_bg_autoleave_in_ms,
                time_to_bg_start_in_ms,
            } => {
                1
                + 4 // time_to_bg_autoleave_in_ms: u32
                + 4 // time_to_bg_start_in_ms: u32
            }
            Self::WAIT_LEAVE => {
                1
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_BATTLEFIELD_STATUS_Map {
    EASTERN_KINGDOMS,
    KALIMDOR {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    TESTING {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    SCOTT_TEST {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    CASH_TEST {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ALTERAC_VALLEY {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    SHADOWFANG_KEEP {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    STORMWIND_STOCKADE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    STORMWIND_PRISON {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    DEADMINES {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    AZSHARA_CRATER {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    COLLINS_TEST {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    WAILING_CAVERNS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    MONASTERY {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RAZORFEN_KRAUL {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BLACKFATHOM_DEEPS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ULDAMAN {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    GNOMERAGON {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    SUNKEN_TEMPLE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RAZORFEN_DOWNS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    EMERALD_DREAM {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    SCARLET_MONASTERY {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ZUL_FARRAK {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BLACKROCK_SPIRE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BLACKROCK_DEPTHS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ONYXIAS_LAIR {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    CAVERNS_OF_TIME {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    SCHOLOMANCE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ZUL_GURUB {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    STRATHOLME {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    MAURADON {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    DEEPRUN_TRAM {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RAGEFIRE_CHASM {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    MOLTEN_CORE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    DIRE_MAUL {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ALLIANCE_PVP_BARRACKS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    HORDE_PVP_BARRACKS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    DEVELOPMENT_LAND {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    BLACKWING_LAIR {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    WARSONG_GULCH {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    RUINS_OF_AHN_QIRAJ {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    ARATHI_BASIN {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    AHN_QIRAJ_TEMPLE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
    NAXXRAMAS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUS_StatusId,
        unknown0: u8,
    },
}

impl Default for SMSG_BATTLEFIELD_STATUS_Map {
    fn default() -> Self {
        // First enumerator without any fields
        Self::EASTERN_KINGDOMS
    }
}

impl SMSG_BATTLEFIELD_STATUS_Map {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::EASTERN_KINGDOMS => 0,
            Self::KALIMDOR { .. } => 1,
            Self::TESTING { .. } => 13,
            Self::SCOTT_TEST { .. } => 25,
            Self::CASH_TEST { .. } => 29,
            Self::ALTERAC_VALLEY { .. } => 30,
            Self::SHADOWFANG_KEEP { .. } => 33,
            Self::STORMWIND_STOCKADE { .. } => 34,
            Self::STORMWIND_PRISON { .. } => 35,
            Self::DEADMINES { .. } => 36,
            Self::AZSHARA_CRATER { .. } => 37,
            Self::COLLINS_TEST { .. } => 42,
            Self::WAILING_CAVERNS { .. } => 43,
            Self::MONASTERY { .. } => 44,
            Self::RAZORFEN_KRAUL { .. } => 47,
            Self::BLACKFATHOM_DEEPS { .. } => 48,
            Self::ULDAMAN { .. } => 70,
            Self::GNOMERAGON { .. } => 90,
            Self::SUNKEN_TEMPLE { .. } => 109,
            Self::RAZORFEN_DOWNS { .. } => 129,
            Self::EMERALD_DREAM { .. } => 169,
            Self::SCARLET_MONASTERY { .. } => 189,
            Self::ZUL_FARRAK { .. } => 209,
            Self::BLACKROCK_SPIRE { .. } => 229,
            Self::BLACKROCK_DEPTHS { .. } => 230,
            Self::ONYXIAS_LAIR { .. } => 249,
            Self::CAVERNS_OF_TIME { .. } => 269,
            Self::SCHOLOMANCE { .. } => 289,
            Self::ZUL_GURUB { .. } => 309,
            Self::STRATHOLME { .. } => 329,
            Self::MAURADON { .. } => 349,
            Self::DEEPRUN_TRAM { .. } => 369,
            Self::RAGEFIRE_CHASM { .. } => 389,
            Self::MOLTEN_CORE { .. } => 409,
            Self::DIRE_MAUL { .. } => 429,
            Self::ALLIANCE_PVP_BARRACKS { .. } => 449,
            Self::HORDE_PVP_BARRACKS { .. } => 450,
            Self::DEVELOPMENT_LAND { .. } => 451,
            Self::BLACKWING_LAIR { .. } => 469,
            Self::WARSONG_GULCH { .. } => 489,
            Self::RUINS_OF_AHN_QIRAJ { .. } => 509,
            Self::ARATHI_BASIN { .. } => 529,
            Self::AHN_QIRAJ_TEMPLE { .. } => 531,
            Self::NAXXRAMAS { .. } => 533,
        }
    }

}

impl SMSG_BATTLEFIELD_STATUS_Map {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::EASTERN_KINGDOMS => {
                4
            }
            Self::KALIMDOR {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::TESTING {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::SCOTT_TEST {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::CASH_TEST {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ALTERAC_VALLEY {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::SHADOWFANG_KEEP {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::STORMWIND_STOCKADE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::STORMWIND_PRISON {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::DEADMINES {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::AZSHARA_CRATER {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::COLLINS_TEST {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::WAILING_CAVERNS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::MONASTERY {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RAZORFEN_KRAUL {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BLACKFATHOM_DEEPS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ULDAMAN {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::GNOMERAGON {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::SUNKEN_TEMPLE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RAZORFEN_DOWNS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::EMERALD_DREAM {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::SCARLET_MONASTERY {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ZUL_FARRAK {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BLACKROCK_SPIRE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BLACKROCK_DEPTHS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ONYXIAS_LAIR {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::CAVERNS_OF_TIME {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::SCHOLOMANCE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ZUL_GURUB {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::STRATHOLME {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::MAURADON {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::DEEPRUN_TRAM {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RAGEFIRE_CHASM {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::MOLTEN_CORE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::DIRE_MAUL {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ALLIANCE_PVP_BARRACKS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::HORDE_PVP_BARRACKS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::DEVELOPMENT_LAND {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::BLACKWING_LAIR {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::WARSONG_GULCH {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::RUINS_OF_AHN_QIRAJ {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::ARATHI_BASIN {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::AHN_QIRAJ_TEMPLE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUS_StatusId
                + 1 // unknown0: u8
            }
            Self::NAXXRAMAS {
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

