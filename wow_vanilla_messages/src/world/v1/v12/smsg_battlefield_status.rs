use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{StatusId, StatusIdError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
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
        let map = Map::read(r)?;

        let map_if = match map {
            Map::EASTERN_KINGDOMS => SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS,
            Map::KALIMDOR => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::TESTING => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::SCOTT_TEST => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::CASH_TEST => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ALTERAC_VALLEY => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::SHADOWFANG_KEEP => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::STORMWIND_STOCKADE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::STORMWIND_PRISON => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::DEADMINES => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::AZSHARA_CRATER => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::COLLINS_TEST => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::WAILING_CAVERNS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::MONASTERY => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RAZORFEN_KRAUL => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BLACKFATHOM_DEEPS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ULDAMAN => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::GNOMERAGON => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::SUNKEN_TEMPLE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RAZORFEN_DOWNS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::EMERALD_DREAM => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::SCARLET_MONASTERY => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ZUL_FARRAK => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BLACKROCK_SPIRE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BLACKROCK_DEPTHS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ONYXIAS_LAIR => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::CAVERNS_OF_TIME => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::SCHOLOMANCE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ZUL_GURUB => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::STRATHOLME => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::MAURADON => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::DEEPRUN_TRAM => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RAGEFIRE_CHASM => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::MOLTEN_CORE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::DIRE_MAUL => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ALLIANCE_PVP_BARRACKS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::HORDE_PVP_BARRACKS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::DEVELOPMENT_LAND => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::BLACKWING_LAIR => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::WARSONG_GULCH => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::RUINS_OF_AHN_QIRAJ => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::ARATHI_BASIN => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::AHN_QIRAJ_TEMPLE => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
                    client_instance_id,
                    status_id: status_id_if,
                }
            }
            Map::NAXXRAMAS => {
                // unknown0: u8
                let unknown0 = crate::util::read_u8_le(r)?;

                // client_instance_id: u32
                let client_instance_id = crate::util::read_u32_le(r)?;

                // status_id: StatusId
                let status_id = StatusId::read(r)?;

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
                    unknown0,
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // queue_slot: u32
        w.write_all(&self.queue_slot.to_le_bytes())?;

        // map: Map
        self.map.write(w)?;

        match &self.map {
            SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS => {}
            SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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
                unknown0,
                client_instance_id,
                status_id,
            } => {
                // unknown0: u8
                w.write_all(&unknown0.to_le_bytes())?;

                // client_instance_id: u32
                w.write_all(&client_instance_id.to_le_bytes())?;

                // status_id: StatusId
                status_id.write(w)?;

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

    #[cfg(feature = "async_tokio")]
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
            let map = Map::tokio_read(r).await?;

            let map_if = match map {
                Map::EASTERN_KINGDOMS => SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS,
                Map::KALIMDOR => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::TESTING => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SCOTT_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::CASH_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ALTERAC_VALLEY => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SHADOWFANG_KEEP => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::STORMWIND_STOCKADE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::STORMWIND_PRISON => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DEADMINES => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::AZSHARA_CRATER => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::COLLINS_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::WAILING_CAVERNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RAZORFEN_KRAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKFATHOM_DEEPS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ULDAMAN => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::GNOMERAGON => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SUNKEN_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RAZORFEN_DOWNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::EMERALD_DREAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SCARLET_MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ZUL_FARRAK => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKROCK_SPIRE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKROCK_DEPTHS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ONYXIAS_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::CAVERNS_OF_TIME => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SCHOLOMANCE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ZUL_GURUB => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::STRATHOLME => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::MAURADON => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DEEPRUN_TRAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RAGEFIRE_CHASM => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::MOLTEN_CORE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DIRE_MAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ALLIANCE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::HORDE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DEVELOPMENT_LAND => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKWING_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::WARSONG_GULCH => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RUINS_OF_AHN_QIRAJ => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ARATHI_BASIN => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::AHN_QIRAJ_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::NAXXRAMAS => {
                    // unknown0: u8
                    let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::tokio_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::tokio_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
            };

            Ok(Self {
                queue_slot,
                map: map_if,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            self.map.tokio_write(w).await?;

            match &self.map {
                SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS => {}
                SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.tokio_write(w).await?;

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

    #[cfg(feature = "async_std")]
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
            let map = Map::astd_read(r).await?;

            let map_if = match map {
                Map::EASTERN_KINGDOMS => SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS,
                Map::KALIMDOR => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::TESTING => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SCOTT_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::CASH_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ALTERAC_VALLEY => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SHADOWFANG_KEEP => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::STORMWIND_STOCKADE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::STORMWIND_PRISON => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DEADMINES => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::AZSHARA_CRATER => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::COLLINS_TEST => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::WAILING_CAVERNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RAZORFEN_KRAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKFATHOM_DEEPS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ULDAMAN => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::GNOMERAGON => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SUNKEN_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RAZORFEN_DOWNS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::EMERALD_DREAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SCARLET_MONASTERY => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ZUL_FARRAK => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKROCK_SPIRE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKROCK_DEPTHS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ONYXIAS_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::CAVERNS_OF_TIME => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::SCHOLOMANCE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ZUL_GURUB => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::STRATHOLME => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::MAURADON => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DEEPRUN_TRAM => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RAGEFIRE_CHASM => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::MOLTEN_CORE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DIRE_MAUL => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ALLIANCE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::HORDE_PVP_BARRACKS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::DEVELOPMENT_LAND => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::BLACKWING_LAIR => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::WARSONG_GULCH => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::RUINS_OF_AHN_QIRAJ => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::ARATHI_BASIN => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::AHN_QIRAJ_TEMPLE => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
                Map::NAXXRAMAS => {
                    // unknown0: u8
                    let unknown0 = crate::util::astd_read_u8_le(r).await?;

                    // client_instance_id: u32
                    let client_instance_id = crate::util::astd_read_u32_le(r).await?;

                    // status_id: StatusId
                    let status_id = StatusId::astd_read(r).await?;

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
                        unknown0,
                        client_instance_id,
                        status_id: status_id_if,
                    }
                }
            };

            Ok(Self {
                queue_slot,
                map: map_if,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            self.map.astd_write(w).await?;

            match &self.map {
                SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS => {}
                SMSG_BATTLEFIELD_STATUSMap::KALIMDOR {
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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
                    unknown0,
                    client_instance_id,
                    status_id,
                } => {
                    // unknown0: u8
                    w.write_all(&unknown0.to_le_bytes()).await?;

                    // client_instance_id: u32
                    w.write_all(&client_instance_id.to_le_bytes()).await?;

                    // status_id: StatusId
                    status_id.astd_write(w).await?;

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

impl VariableSized for SMSG_BATTLEFIELD_STATUS {
    fn size(&self) -> usize {
        0
        + 4 // queue_slot: u32
        + self.map.size() // map: SMSG_BATTLEFIELD_STATUSMap
    }
}

impl MaximumPossibleSized for SMSG_BATTLEFIELD_STATUS {
    fn maximum_possible_size() -> usize {
        0
        + 4 // queue_slot: u32
        + 18 // map: SMSG_BATTLEFIELD_STATUSMap
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

impl From<&StatusId> for SMSG_BATTLEFIELD_STATUSStatusId {
    fn from(e: &StatusId) -> Self {
        match &e {
            StatusId::NONE => Self::NONE,
            StatusId::WAIT_QUEUE => Self::WAIT_QUEUE {
                average_wait_time_in_ms: Default::default(),
                time_in_queue_in_ms: Default::default(),
            },
            StatusId::WAIT_JOIN => Self::WAIT_JOIN {
                time_to_remove_in_queue_in_ms: Default::default(),
            },
            StatusId::IN_PROGRESS => Self::IN_PROGRESS {
                time_to_bg_autoleave_in_ms: Default::default(),
                time_to_bg_start_in_ms: Default::default(),
            },
            StatusId::WAIT_LEAVE => Self::WAIT_LEAVE,
        }
    }
}

impl From<&SMSG_BATTLEFIELD_STATUSStatusId> for StatusId {
    fn from(v: &SMSG_BATTLEFIELD_STATUSStatusId) -> Self {
        match &v {
            SMSG_BATTLEFIELD_STATUSStatusId::NONE => Self::NONE,
            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_QUEUE { .. } => Self::WAIT_QUEUE,
            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_JOIN { .. } => Self::WAIT_JOIN,
            SMSG_BATTLEFIELD_STATUSStatusId::IN_PROGRESS { .. } => Self::IN_PROGRESS,
            SMSG_BATTLEFIELD_STATUSStatusId::WAIT_LEAVE => Self::WAIT_LEAVE,
        }
    }
}

impl Default for SMSG_BATTLEFIELD_STATUSStatusId {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl SMSG_BATTLEFIELD_STATUSStatusId {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.astd_write_u16_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.astd_write_u16_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.astd_write_u32_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.astd_write_u32_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.astd_write_u64_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_BATTLEFIELD_STATUSStatusId {
    fn size(&self) -> usize {
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

impl MaximumPossibleSized for SMSG_BATTLEFIELD_STATUSStatusId {
    fn maximum_possible_size() -> usize {
        1
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

impl From<&Map> for SMSG_BATTLEFIELD_STATUSMap {
    fn from(e: &Map) -> Self {
        match &e {
            Map::EASTERN_KINGDOMS => Self::EASTERN_KINGDOMS,
            Map::KALIMDOR => Self::KALIMDOR {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::TESTING => Self::TESTING {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::SCOTT_TEST => Self::SCOTT_TEST {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::CASH_TEST => Self::CASH_TEST {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::ALTERAC_VALLEY => Self::ALTERAC_VALLEY {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::SHADOWFANG_KEEP => Self::SHADOWFANG_KEEP {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::STORMWIND_STOCKADE => Self::STORMWIND_STOCKADE {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::STORMWIND_PRISON => Self::STORMWIND_PRISON {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::DEADMINES => Self::DEADMINES {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::AZSHARA_CRATER => Self::AZSHARA_CRATER {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::COLLINS_TEST => Self::COLLINS_TEST {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::WAILING_CAVERNS => Self::WAILING_CAVERNS {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::MONASTERY => Self::MONASTERY {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::RAZORFEN_KRAUL => Self::RAZORFEN_KRAUL {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::BLACKFATHOM_DEEPS => Self::BLACKFATHOM_DEEPS {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::ULDAMAN => Self::ULDAMAN {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::GNOMERAGON => Self::GNOMERAGON {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::SUNKEN_TEMPLE => Self::SUNKEN_TEMPLE {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::RAZORFEN_DOWNS => Self::RAZORFEN_DOWNS {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::EMERALD_DREAM => Self::EMERALD_DREAM {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::SCARLET_MONASTERY => Self::SCARLET_MONASTERY {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::ZUL_FARRAK => Self::ZUL_FARRAK {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::BLACKROCK_SPIRE => Self::BLACKROCK_SPIRE {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::BLACKROCK_DEPTHS => Self::BLACKROCK_DEPTHS {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::ONYXIAS_LAIR => Self::ONYXIAS_LAIR {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::CAVERNS_OF_TIME => Self::CAVERNS_OF_TIME {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::SCHOLOMANCE => Self::SCHOLOMANCE {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::ZUL_GURUB => Self::ZUL_GURUB {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::STRATHOLME => Self::STRATHOLME {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::MAURADON => Self::MAURADON {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::DEEPRUN_TRAM => Self::DEEPRUN_TRAM {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::RAGEFIRE_CHASM => Self::RAGEFIRE_CHASM {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::MOLTEN_CORE => Self::MOLTEN_CORE {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::DIRE_MAUL => Self::DIRE_MAUL {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::ALLIANCE_PVP_BARRACKS => Self::ALLIANCE_PVP_BARRACKS {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::HORDE_PVP_BARRACKS => Self::HORDE_PVP_BARRACKS {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::DEVELOPMENT_LAND => Self::DEVELOPMENT_LAND {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::BLACKWING_LAIR => Self::BLACKWING_LAIR {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::WARSONG_GULCH => Self::WARSONG_GULCH {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::RUINS_OF_AHN_QIRAJ => Self::RUINS_OF_AHN_QIRAJ {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::ARATHI_BASIN => Self::ARATHI_BASIN {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::AHN_QIRAJ_TEMPLE => Self::AHN_QIRAJ_TEMPLE {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
            Map::NAXXRAMAS => Self::NAXXRAMAS {
                client_instance_id: Default::default(),
                status_id: Default::default(),
                unknown0: Default::default(),
            },
        }
    }
}

impl From<&SMSG_BATTLEFIELD_STATUSMap> for Map {
    fn from(v: &SMSG_BATTLEFIELD_STATUSMap) -> Self {
        match &v {
            SMSG_BATTLEFIELD_STATUSMap::EASTERN_KINGDOMS => Self::EASTERN_KINGDOMS,
            SMSG_BATTLEFIELD_STATUSMap::KALIMDOR { .. } => Self::KALIMDOR,
            SMSG_BATTLEFIELD_STATUSMap::TESTING { .. } => Self::TESTING,
            SMSG_BATTLEFIELD_STATUSMap::SCOTT_TEST { .. } => Self::SCOTT_TEST,
            SMSG_BATTLEFIELD_STATUSMap::CASH_TEST { .. } => Self::CASH_TEST,
            SMSG_BATTLEFIELD_STATUSMap::ALTERAC_VALLEY { .. } => Self::ALTERAC_VALLEY,
            SMSG_BATTLEFIELD_STATUSMap::SHADOWFANG_KEEP { .. } => Self::SHADOWFANG_KEEP,
            SMSG_BATTLEFIELD_STATUSMap::STORMWIND_STOCKADE { .. } => Self::STORMWIND_STOCKADE,
            SMSG_BATTLEFIELD_STATUSMap::STORMWIND_PRISON { .. } => Self::STORMWIND_PRISON,
            SMSG_BATTLEFIELD_STATUSMap::DEADMINES { .. } => Self::DEADMINES,
            SMSG_BATTLEFIELD_STATUSMap::AZSHARA_CRATER { .. } => Self::AZSHARA_CRATER,
            SMSG_BATTLEFIELD_STATUSMap::COLLINS_TEST { .. } => Self::COLLINS_TEST,
            SMSG_BATTLEFIELD_STATUSMap::WAILING_CAVERNS { .. } => Self::WAILING_CAVERNS,
            SMSG_BATTLEFIELD_STATUSMap::MONASTERY { .. } => Self::MONASTERY,
            SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_KRAUL { .. } => Self::RAZORFEN_KRAUL,
            SMSG_BATTLEFIELD_STATUSMap::BLACKFATHOM_DEEPS { .. } => Self::BLACKFATHOM_DEEPS,
            SMSG_BATTLEFIELD_STATUSMap::ULDAMAN { .. } => Self::ULDAMAN,
            SMSG_BATTLEFIELD_STATUSMap::GNOMERAGON { .. } => Self::GNOMERAGON,
            SMSG_BATTLEFIELD_STATUSMap::SUNKEN_TEMPLE { .. } => Self::SUNKEN_TEMPLE,
            SMSG_BATTLEFIELD_STATUSMap::RAZORFEN_DOWNS { .. } => Self::RAZORFEN_DOWNS,
            SMSG_BATTLEFIELD_STATUSMap::EMERALD_DREAM { .. } => Self::EMERALD_DREAM,
            SMSG_BATTLEFIELD_STATUSMap::SCARLET_MONASTERY { .. } => Self::SCARLET_MONASTERY,
            SMSG_BATTLEFIELD_STATUSMap::ZUL_FARRAK { .. } => Self::ZUL_FARRAK,
            SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_SPIRE { .. } => Self::BLACKROCK_SPIRE,
            SMSG_BATTLEFIELD_STATUSMap::BLACKROCK_DEPTHS { .. } => Self::BLACKROCK_DEPTHS,
            SMSG_BATTLEFIELD_STATUSMap::ONYXIAS_LAIR { .. } => Self::ONYXIAS_LAIR,
            SMSG_BATTLEFIELD_STATUSMap::CAVERNS_OF_TIME { .. } => Self::CAVERNS_OF_TIME,
            SMSG_BATTLEFIELD_STATUSMap::SCHOLOMANCE { .. } => Self::SCHOLOMANCE,
            SMSG_BATTLEFIELD_STATUSMap::ZUL_GURUB { .. } => Self::ZUL_GURUB,
            SMSG_BATTLEFIELD_STATUSMap::STRATHOLME { .. } => Self::STRATHOLME,
            SMSG_BATTLEFIELD_STATUSMap::MAURADON { .. } => Self::MAURADON,
            SMSG_BATTLEFIELD_STATUSMap::DEEPRUN_TRAM { .. } => Self::DEEPRUN_TRAM,
            SMSG_BATTLEFIELD_STATUSMap::RAGEFIRE_CHASM { .. } => Self::RAGEFIRE_CHASM,
            SMSG_BATTLEFIELD_STATUSMap::MOLTEN_CORE { .. } => Self::MOLTEN_CORE,
            SMSG_BATTLEFIELD_STATUSMap::DIRE_MAUL { .. } => Self::DIRE_MAUL,
            SMSG_BATTLEFIELD_STATUSMap::ALLIANCE_PVP_BARRACKS { .. } => Self::ALLIANCE_PVP_BARRACKS,
            SMSG_BATTLEFIELD_STATUSMap::HORDE_PVP_BARRACKS { .. } => Self::HORDE_PVP_BARRACKS,
            SMSG_BATTLEFIELD_STATUSMap::DEVELOPMENT_LAND { .. } => Self::DEVELOPMENT_LAND,
            SMSG_BATTLEFIELD_STATUSMap::BLACKWING_LAIR { .. } => Self::BLACKWING_LAIR,
            SMSG_BATTLEFIELD_STATUSMap::WARSONG_GULCH { .. } => Self::WARSONG_GULCH,
            SMSG_BATTLEFIELD_STATUSMap::RUINS_OF_AHN_QIRAJ { .. } => Self::RUINS_OF_AHN_QIRAJ,
            SMSG_BATTLEFIELD_STATUSMap::ARATHI_BASIN { .. } => Self::ARATHI_BASIN,
            SMSG_BATTLEFIELD_STATUSMap::AHN_QIRAJ_TEMPLE { .. } => Self::AHN_QIRAJ_TEMPLE,
            SMSG_BATTLEFIELD_STATUSMap::NAXXRAMAS { .. } => Self::NAXXRAMAS,
        }
    }
}

impl Default for SMSG_BATTLEFIELD_STATUSMap {
    fn default() -> Self {
        // First enumerator without any fields
        Self::EASTERN_KINGDOMS
    }
}

impl SMSG_BATTLEFIELD_STATUSMap {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.astd_write_u32_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.astd_write_u32_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.astd_write_u64_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_BATTLEFIELD_STATUSMap {
    fn size(&self) -> usize {
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

impl MaximumPossibleSized for SMSG_BATTLEFIELD_STATUSMap {
    fn maximum_possible_size() -> usize {
        18
    }
}

