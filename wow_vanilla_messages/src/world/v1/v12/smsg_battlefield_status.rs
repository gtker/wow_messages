use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{StatusId, StatusIdError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_BATTLEFIELD_STATUS {
    pub queue_slot: u32,
    pub map: SMSG_BATTLEFIELD_STATUSMap,
}

impl ServerMessageWrite for SMSG_BATTLEFIELD_STATUS {}

impl MessageBody for SMSG_BATTLEFIELD_STATUS {
    const OPCODE: u16 = 0x02d4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_BATTLEFIELD_STATUSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // queue_slot: u32
        let queue_slot = crate::util::read_u32_le(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        let map_if = match map {
            Map::EASTERN_KINGDOMS => SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS,
            Map::KALIMDOR => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id: StatusId = crate::util::read_u8_le(r)?.try_into()?;

                let status_id_if = match status_id {
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::TESTING {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::SCOTT_TEST {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::CASH_TEST {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::ALTERAC_VALLEY {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::SHADOWFANG_KEEP {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::STORMWIND_STOCKADE {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::STORMWIND_PRISON {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::DEADMINES {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::AZSHARA_CRATER {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::COLLINS_TEST {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::WAILING_CAVERNS {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::MONASTERY {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_KRAUL {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::BLACKFATHOM_DEEPS {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::ULDAMAN {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::GNOMERAGON {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::SUNKEN_TEMPLE {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_DOWNS {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::EMERALD_DREAM {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::SCARLET_MONASTERY {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::ZUL_FARRAK {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_SPIRE {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_DEPTHS {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::ONYXIAS_LAIR {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::CAVERNS_OF_TIME {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::SCHOLOMANCE {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::ZUL_GURUB {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::STRATHOLME {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::MAURADON {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::DEEPRUN_TRAM {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::RAGEFIRE_CHASM {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::MOLTEN_CORE {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::DIRE_MAUL {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::ALLIANCE_PVP_BARRACKS {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::HORDE_PVP_BARRACKS {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::DEVELOPMENT_LAND {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::BLACKWING_LAIR {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::WARSONG_GULCH {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::RUINS_OF_AHN_QIRAJ {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::ARATHI_BASIN {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::AHN_QIRAJ_TEMPLE {
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
                    StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                    StatusId::WAIT_QUEUE => {
                        // average_wait_time_in_ms: u32
                        let average_wait_time_in_ms = crate::util::read_u32_le(r)?;

                        // time_in_queue_in_ms: u32
                        let time_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        }
                    }
                    StatusId::WAIT_JOIN => {
                        // time_to_remove_in_queue_in_ms: u32
                        let time_to_remove_in_queue_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        }
                    }
                    StatusId::IN_PROGRESS => {
                        // time_to_bg_autoleave_in_ms: u32
                        let time_to_bg_autoleave_in_ms = crate::util::read_u32_le(r)?;

                        // time_to_bg_start_in_ms: u32
                        let time_to_bg_start_in_ms = crate::util::read_u32_le(r)?;

                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        }
                    }
                    StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                };

                SMSG_BATTLEFIELD_STATUSMap::NAXXRAMAS {
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // queue_slot: u32
        w.write_all(&self.queue_slot.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        match &self.map {
            SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS => {}
            SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::TESTING {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::SCOTT_TEST {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::CASH_TEST {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::ALTERAC_VALLEY {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::SHADOWFANG_KEEP {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::STORMWIND_STOCKADE {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::STORMWIND_PRISON {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::DEADMINES {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::AZSHARA_CRATER {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::COLLINS_TEST {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::WAILING_CAVERNS {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::MONASTERY {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_KRAUL {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::BLACKFATHOM_DEEPS {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::ULDAMAN {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::GNOMERAGON {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::SUNKEN_TEMPLE {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_DOWNS {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::EMERALD_DREAM {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::SCARLET_MONASTERY {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::ZUL_FARRAK {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_SPIRE {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_DEPTHS {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::ONYXIAS_LAIR {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::CAVERNS_OF_TIME {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::SCHOLOMANCE {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::ZUL_GURUB {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::STRATHOLME {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::MAURADON {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::DEEPRUN_TRAM {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::RAGEFIRE_CHASM {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::MOLTEN_CORE {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::DIRE_MAUL {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::ALLIANCE_PVP_BARRACKS {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::HORDE_PVP_BARRACKS {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::DEVELOPMENT_LAND {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::BLACKWING_LAIR {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::WARSONG_GULCH {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::RUINS_OF_AHN_QIRAJ {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::ARATHI_BASIN {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::AHN_QIRAJ_TEMPLE {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
            SMSG_BATTLEFIELD_STATUSMap::NAXXRAMAS {
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
                    SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                        average_wait_time_in_ms,
                        time_in_queue_in_ms,
                    } => {
                        // average_wait_time_in_ms: u32
                        w.write_all(&average_wait_time_in_ms.to_le_bytes())?;

                        // time_in_queue_in_ms: u32
                        w.write_all(&time_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                        time_to_remove_in_queue_in_ms,
                    } => {
                        // time_to_remove_in_queue_in_ms: u32
                        w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                        time_to_bg_autoleave_in_ms,
                        time_to_bg_start_in_ms,
                    } => {
                        // time_to_bg_autoleave_in_ms: u32
                        w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes())?;

                        // time_to_bg_start_in_ms: u32
                        w.write_all(&time_to_bg_start_in_ms.to_le_bytes())?;

                    }
                    SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                }

            }
        }

        Ok(())
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // queue_slot: u32
            let queue_slot = crate::util::tokio_read_u32_le(r).await?;

            // map: Map
            let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            let map_if = match map {
                Map::EASTERN_KINGDOMS => SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS,
                Map::KALIMDOR => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::TESTING => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::TESTING {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SCOTT_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SCOTT_TEST {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::CASH_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::CASH_TEST {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ALTERAC_VALLEY => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ALTERAC_VALLEY {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SHADOWFANG_KEEP => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SHADOWFANG_KEEP {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::STORMWIND_STOCKADE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::STORMWIND_STOCKADE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::STORMWIND_PRISON => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::STORMWIND_PRISON {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DEADMINES => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DEADMINES {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::AZSHARA_CRATER => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::AZSHARA_CRATER {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::COLLINS_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::COLLINS_TEST {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::WAILING_CAVERNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::WAILING_CAVERNS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::MONASTERY {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RAZORFEN_KRAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_KRAUL {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKFATHOM_DEEPS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKFATHOM_DEEPS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ULDAMAN => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ULDAMAN {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::GNOMERAGON => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::GNOMERAGON {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SUNKEN_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SUNKEN_TEMPLE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RAZORFEN_DOWNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_DOWNS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::EMERALD_DREAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::EMERALD_DREAM {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SCARLET_MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SCARLET_MONASTERY {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ZUL_FARRAK => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ZUL_FARRAK {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKROCK_SPIRE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_SPIRE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKROCK_DEPTHS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_DEPTHS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ONYXIAS_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ONYXIAS_LAIR {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::CAVERNS_OF_TIME => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::CAVERNS_OF_TIME {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SCHOLOMANCE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SCHOLOMANCE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ZUL_GURUB => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ZUL_GURUB {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::STRATHOLME => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::STRATHOLME {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::MAURADON => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::MAURADON {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DEEPRUN_TRAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DEEPRUN_TRAM {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RAGEFIRE_CHASM => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RAGEFIRE_CHASM {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::MOLTEN_CORE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::MOLTEN_CORE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DIRE_MAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DIRE_MAUL {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ALLIANCE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ALLIANCE_PVP_BARRACKS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::HORDE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::HORDE_PVP_BARRACKS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DEVELOPMENT_LAND => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DEVELOPMENT_LAND {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKWING_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKWING_LAIR {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::WARSONG_GULCH => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::WARSONG_GULCH {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RUINS_OF_AHN_QIRAJ => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RUINS_OF_AHN_QIRAJ {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ARATHI_BASIN => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ARATHI_BASIN {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::AHN_QIRAJ_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::AHN_QIRAJ_TEMPLE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::NAXXRAMAS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::tokio_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::NAXXRAMAS {
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
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // queue_slot: u32
            w.write_all(&self.queue_slot.to_le_bytes()).await?;

            // map: Map
            w.write_all(&(self.map.as_int() as u32).to_le_bytes()).await?;

            match &self.map {
                SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS => {}
                SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::TESTING {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SCOTT_TEST {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::CASH_TEST {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ALTERAC_VALLEY {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SHADOWFANG_KEEP {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::STORMWIND_STOCKADE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::STORMWIND_PRISON {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DEADMINES {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::AZSHARA_CRATER {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::COLLINS_TEST {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::WAILING_CAVERNS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::MONASTERY {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_KRAUL {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKFATHOM_DEEPS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ULDAMAN {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::GNOMERAGON {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SUNKEN_TEMPLE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_DOWNS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::EMERALD_DREAM {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SCARLET_MONASTERY {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ZUL_FARRAK {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_SPIRE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_DEPTHS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ONYXIAS_LAIR {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::CAVERNS_OF_TIME {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SCHOLOMANCE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ZUL_GURUB {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::STRATHOLME {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::MAURADON {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DEEPRUN_TRAM {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RAGEFIRE_CHASM {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::MOLTEN_CORE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DIRE_MAUL {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ALLIANCE_PVP_BARRACKS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::HORDE_PVP_BARRACKS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DEVELOPMENT_LAND {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKWING_LAIR {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::WARSONG_GULCH {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RUINS_OF_AHN_QIRAJ {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ARATHI_BASIN {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::AHN_QIRAJ_TEMPLE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::NAXXRAMAS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
            }

            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // queue_slot: u32
            let queue_slot = crate::util::astd_read_u32_le(r).await?;

            // map: Map
            let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

            let map_if = match map {
                Map::EASTERN_KINGDOMS => SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS,
                Map::KALIMDOR => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::TESTING => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::TESTING {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SCOTT_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SCOTT_TEST {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::CASH_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::CASH_TEST {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ALTERAC_VALLEY => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ALTERAC_VALLEY {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SHADOWFANG_KEEP => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SHADOWFANG_KEEP {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::STORMWIND_STOCKADE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::STORMWIND_STOCKADE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::STORMWIND_PRISON => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::STORMWIND_PRISON {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DEADMINES => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DEADMINES {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::AZSHARA_CRATER => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::AZSHARA_CRATER {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::COLLINS_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::COLLINS_TEST {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::WAILING_CAVERNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::WAILING_CAVERNS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::MONASTERY {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RAZORFEN_KRAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_KRAUL {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKFATHOM_DEEPS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKFATHOM_DEEPS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ULDAMAN => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ULDAMAN {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::GNOMERAGON => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::GNOMERAGON {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SUNKEN_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SUNKEN_TEMPLE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RAZORFEN_DOWNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_DOWNS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::EMERALD_DREAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::EMERALD_DREAM {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SCARLET_MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SCARLET_MONASTERY {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ZUL_FARRAK => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ZUL_FARRAK {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKROCK_SPIRE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_SPIRE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKROCK_DEPTHS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_DEPTHS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ONYXIAS_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ONYXIAS_LAIR {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::CAVERNS_OF_TIME => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::CAVERNS_OF_TIME {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::SCHOLOMANCE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::SCHOLOMANCE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ZUL_GURUB => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ZUL_GURUB {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::STRATHOLME => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::STRATHOLME {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::MAURADON => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::MAURADON {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DEEPRUN_TRAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DEEPRUN_TRAM {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RAGEFIRE_CHASM => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RAGEFIRE_CHASM {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::MOLTEN_CORE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::MOLTEN_CORE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DIRE_MAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DIRE_MAUL {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ALLIANCE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ALLIANCE_PVP_BARRACKS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::HORDE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::HORDE_PVP_BARRACKS {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::DEVELOPMENT_LAND => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::DEVELOPMENT_LAND {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::BLACKWING_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::BLACKWING_LAIR {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::WARSONG_GULCH => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::WARSONG_GULCH {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::RUINS_OF_AHN_QIRAJ => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::RUINS_OF_AHN_QIRAJ {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::ARATHI_BASIN => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::ARATHI_BASIN {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::AHN_QIRAJ_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::AHN_QIRAJ_TEMPLE {
                        client_instance_id,
                        status_id: status_id_if,
                        unknown0,
                    }
                }
                Map::NAXXRAMAS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id: StatusId = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let status_id_if = match status_id {
                        StatusId::NONE => SMSG_BATTLEFIELD_STATUSStatusId::NONE,
                        StatusId::WAIT_QUEUE => {
                            // average_wait_time_in_ms: u32
                            let average_wait_time_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_in_queue_in_ms: u32
                            let time_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                                average_wait_time_in_ms,
                                time_in_queue_in_ms,
                            }
                        }
                        StatusId::WAIT_JOIN => {
                            // time_to_remove_in_queue_in_ms: u32
                            let time_to_remove_in_queue_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                                time_to_remove_in_queue_in_ms,
                            }
                        }
                        StatusId::IN_PROGRESS => {
                            // time_to_bg_autoleave_in_ms: u32
                            let time_to_bg_autoleave_in_ms = crate::util::astd_read_u32_le(r).await?;

                            // time_to_bg_start_in_ms: u32
                            let time_to_bg_start_in_ms = crate::util::astd_read_u32_le(r).await?;

                            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                                time_to_bg_autoleave_in_ms,
                                time_to_bg_start_in_ms,
                            }
                        }
                        StatusId::WAIT_LEAVE => SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE,
                    };

                    SMSG_BATTLEFIELD_STATUSMap::NAXXRAMAS {
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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // queue_slot: u32
            w.write_all(&self.queue_slot.to_le_bytes()).await?;

            // map: Map
            w.write_all(&(self.map.as_int() as u32).to_le_bytes()).await?;

            match &self.map {
                SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS => {}
                SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::TESTING {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SCOTT_TEST {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::CASH_TEST {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ALTERAC_VALLEY {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SHADOWFANG_KEEP {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::STORMWIND_STOCKADE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::STORMWIND_PRISON {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DEADMINES {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::AZSHARA_CRATER {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::COLLINS_TEST {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::WAILING_CAVERNS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::MONASTERY {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_KRAUL {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKFATHOM_DEEPS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ULDAMAN {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::GNOMERAGON {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SUNKEN_TEMPLE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_DOWNS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::EMERALD_DREAM {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SCARLET_MONASTERY {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ZUL_FARRAK {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_SPIRE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_DEPTHS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ONYXIAS_LAIR {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::CAVERNS_OF_TIME {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::SCHOLOMANCE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ZUL_GURUB {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::STRATHOLME {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::MAURADON {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DEEPRUN_TRAM {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RAGEFIRE_CHASM {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::MOLTEN_CORE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DIRE_MAUL {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ALLIANCE_PVP_BARRACKS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::HORDE_PVP_BARRACKS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::DEVELOPMENT_LAND {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::BLACKWING_LAIR {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::WARSONG_GULCH {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::RUINS_OF_AHN_QIRAJ {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::ARATHI_BASIN {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::AHN_QIRAJ_TEMPLE {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
                SMSG_BATTLEFIELD_STATUSMap::NAXXRAMAS {
                    client_instance_id,
                    status_id,
                    unknown0,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    w.write_all(&(status_id.as_int() as u8).to_le_bytes()).await?;

                    match &status_id {
                        SMSG_BATTLEFIELD_STATUSStatusId::NONE => {}
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE {
                            average_wait_time_in_ms,
                            time_in_queue_in_ms,
                        } => {
                            // average_wait_time_in_ms: u32
                            w.write_all(&average_wait_time_in_ms.to_le_bytes()).await?;

                            // time_in_queue_in_ms: u32
                            w.write_all(&time_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN {
                            time_to_remove_in_queue_in_ms,
                        } => {
                            // time_to_remove_in_queue_in_ms: u32
                            w.write_all(&time_to_remove_in_queue_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS {
                            time_to_bg_autoleave_in_ms,
                            time_to_bg_start_in_ms,
                        } => {
                            // time_to_bg_autoleave_in_ms: u32
                            w.write_all(&time_to_bg_autoleave_in_ms.to_le_bytes()).await?;

                            // time_to_bg_start_in_ms: u32
                            w.write_all(&time_to_bg_start_in_ms.to_le_bytes()).await?;

                        }
                        SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => {}
                    }

                }
            }

            Ok(())
        })
    }

}

impl SMSG_BATTLEFIELD_STATUS {
    pub fn size(&self) -> usize {
        0
        + 4 // queue_slot: u32
        + self.map.size() // map: SMSG_BATTLEFIELD_STATUSMap
    }
}

#[derive(Debug)]
pub enum SMSG_BATTLEFIELD_STATUSError {
    Io(std::io::Error),
    Map(MapError),
    StatusId(StatusIdError),
}

impl std::error::Error for SMSG_BATTLEFIELD_STATUSError {}
impl std::fmt::Display for SMSG_BATTLEFIELD_STATUSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
            Self::StatusId(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_BATTLEFIELD_STATUSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_BATTLEFIELD_STATUSError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

impl From<StatusIdError> for SMSG_BATTLEFIELD_STATUSError {
    fn from(e: StatusIdError) -> Self {
        Self::StatusId(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_BATTLEFIELD_STATUSStatusId {
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

impl Default for SMSG_BATTLEFIELD_STATUSStatusId {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl SMSG_BATTLEFIELD_STATUSStatusId {
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

impl SMSG_BATTLEFIELD_STATUSStatusId {
    pub fn size(&self) -> usize {
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
pub enum SMSG_BATTLEFIELD_STATUSMap {
    EASTERN_KINGDOMS,
    KALIMDOR {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    TESTING {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    SCOTT_TEST {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    CASH_TEST {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    ALTERAC_VALLEY {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    SHADOWFANG_KEEP {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    STORMWIND_STOCKADE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    STORMWIND_PRISON {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    DEADMINES {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    AZSHARA_CRATER {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    COLLINS_TEST {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    WAILING_CAVERNS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    MONASTERY {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    RAZORFEN_KRAUL {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    BLACKFATHOM_DEEPS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    ULDAMAN {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    GNOMERAGON {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    SUNKEN_TEMPLE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    RAZORFEN_DOWNS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    EMERALD_DREAM {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    SCARLET_MONASTERY {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    ZUL_FARRAK {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    BLACKROCK_SPIRE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    BLACKROCK_DEPTHS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    ONYXIAS_LAIR {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    CAVERNS_OF_TIME {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    SCHOLOMANCE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    ZUL_GURUB {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    STRATHOLME {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    MAURADON {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    DEEPRUN_TRAM {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    RAGEFIRE_CHASM {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    MOLTEN_CORE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    DIRE_MAUL {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    ALLIANCE_PVP_BARRACKS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    HORDE_PVP_BARRACKS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    DEVELOPMENT_LAND {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    BLACKWING_LAIR {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    WARSONG_GULCH {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    RUINS_OF_AHN_QIRAJ {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    ARATHI_BASIN {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    AHN_QIRAJ_TEMPLE {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
    NAXXRAMAS {
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
        unknown0: u8,
    },
}

impl Default for SMSG_BATTLEFIELD_STATUSMap {
    fn default() -> Self {
        // First enumerator without any fields
        Self::EASTERN_KINGDOMS
    }
}

impl SMSG_BATTLEFIELD_STATUSMap {
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

impl SMSG_BATTLEFIELD_STATUSMap {
    pub fn size(&self) -> usize {
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
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::TESTING {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::SCOTT_TEST {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::CASH_TEST {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::ALTERAC_VALLEY {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::SHADOWFANG_KEEP {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::STORMWIND_STOCKADE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::STORMWIND_PRISON {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::DEADMINES {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::AZSHARA_CRATER {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::COLLINS_TEST {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::WAILING_CAVERNS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::MONASTERY {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::RAZORFEN_KRAUL {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::BLACKFATHOM_DEEPS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::ULDAMAN {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::GNOMERAGON {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::SUNKEN_TEMPLE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::RAZORFEN_DOWNS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::EMERALD_DREAM {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::SCARLET_MONASTERY {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::ZUL_FARRAK {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::BLACKROCK_SPIRE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::BLACKROCK_DEPTHS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::ONYXIAS_LAIR {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::CAVERNS_OF_TIME {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::SCHOLOMANCE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::ZUL_GURUB {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::STRATHOLME {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::MAURADON {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::DEEPRUN_TRAM {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::RAGEFIRE_CHASM {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::MOLTEN_CORE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::DIRE_MAUL {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::ALLIANCE_PVP_BARRACKS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::HORDE_PVP_BARRACKS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::DEVELOPMENT_LAND {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::BLACKWING_LAIR {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::WARSONG_GULCH {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::RUINS_OF_AHN_QIRAJ {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::ARATHI_BASIN {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::AHN_QIRAJ_TEMPLE {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
            Self::NAXXRAMAS {
                client_instance_id,
                status_id,
                unknown0,
            } => {
                4
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId
                + 1 // unknown0: u8
            }
        }
    }
}

