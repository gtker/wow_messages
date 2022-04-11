use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{StatusId, StatusIdError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_BATTLEFIELD_STATUS {
    pub queue_slot: u32,
    pub map: SMSG_BATTLEFIELD_STATUSMap,
}

impl WorldServerMessageWrite for SMSG_BATTLEFIELD_STATUS {
    const OPCODE: u16 = 0x2d4;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_BATTLEFIELD_STATUS {
    type Error = SMSG_BATTLEFIELD_STATUSError;

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
}

impl VariableSized for SMSG_BATTLEFIELD_STATUS {
    fn size(&self) -> usize {
        4 // queue_slot: u32
        + self.map.size() // map: Map and subfields
    }
}

impl MaximumPossibleSized for SMSG_BATTLEFIELD_STATUS {
    fn maximum_possible_size() -> usize {
        4 // queue_slot: u32
        + Map::maximum_possible_size() // map: Map
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
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: StatusId = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for SMSG_BATTLEFIELD_STATUSStatusId {
    fn size(&self) -> usize {
        match self {
            Self::NONE =>  {
                1
            }
            Self::WAIT_QUEUE  {
                average_wait_time_in_ms,
                time_in_queue_in_ms,
            } => {
                1
                + 4 // average_wait_time_in_ms: u32
                + 4 // time_in_queue_in_ms: u32
            }
            Self::WAIT_JOIN  {
                time_to_remove_in_queue_in_ms,
            } => {
                1
                + 4 // time_to_remove_in_queue_in_ms: u32
            }
            Self::IN_PROGRESS  {
                time_to_bg_autoleave_in_ms,
                time_to_bg_start_in_ms,
            } => {
                1
                + 4 // time_to_bg_autoleave_in_ms: u32
                + 4 // time_to_bg_start_in_ms: u32
            }
            Self::WAIT_LEAVE =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_BATTLEFIELD_STATUSStatusId {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_BATTLEFIELD_STATUSMap {
    EASTERN_KINGDOMS,
    KALIMDOR {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    TESTING {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    SCOTT_TEST {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    CASH_TEST {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    ALTERAC_VALLEY {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    SHADOWFANG_KEEP {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    STORMWIND_STOCKADE {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    STORMWIND_PRISON {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    DEADMINES {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    AZSHARA_CRATER {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    COLLINS_TEST {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    WAILING_CAVERNS {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    MONASTERY {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    RAZORFEN_KRAUL {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    BLACKFATHOM_DEEPS {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    ULDAMAN {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    GNOMERAGON {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    SUNKEN_TEMPLE {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    RAZORFEN_DOWNS {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    EMERALD_DREAM {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    SCARLET_MONASTERY {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    ZUL_FARRAK {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    BLACKROCK_SPIRE {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    BLACKROCK_DEPTHS {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    ONYXIAS_LAIR {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    CAVERNS_OF_TIME {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    SCHOLOMANCE {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    ZUL_GURUB {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    STRATHOLME {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    MAURADON {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    DEEPRUN_TRAM {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    RAGEFIRE_CHASM {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    MOLTEN_CORE {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    DIRE_MAUL {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    ALLIANCE_PVP_BARRACKS {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    HORDE_PVP_BARRACKS {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    DEVELOPMENT_LAND {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    BLACKWING_LAIR {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    WARSONG_GULCH {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    RUINS_OF_AHN_QIRAJ {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    ARATHI_BASIN {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    AHN_QIRAJ_TEMPLE {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
    NAXXRAMAS {
        unknown0: u8,
        client_instance_id: u32,
        status_id: SMSG_BATTLEFIELD_STATUSStatusId,
    },
}

impl From<&Map> for SMSG_BATTLEFIELD_STATUSMap {
    fn from(e: &Map) -> Self {
        match &e {
            Map::EASTERN_KINGDOMS => Self::EASTERN_KINGDOMS,
            Map::KALIMDOR => Self::KALIMDOR {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::TESTING => Self::TESTING {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::SCOTT_TEST => Self::SCOTT_TEST {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::CASH_TEST => Self::CASH_TEST {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::ALTERAC_VALLEY => Self::ALTERAC_VALLEY {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::SHADOWFANG_KEEP => Self::SHADOWFANG_KEEP {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::STORMWIND_STOCKADE => Self::STORMWIND_STOCKADE {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::STORMWIND_PRISON => Self::STORMWIND_PRISON {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::DEADMINES => Self::DEADMINES {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::AZSHARA_CRATER => Self::AZSHARA_CRATER {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::COLLINS_TEST => Self::COLLINS_TEST {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::WAILING_CAVERNS => Self::WAILING_CAVERNS {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::MONASTERY => Self::MONASTERY {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::RAZORFEN_KRAUL => Self::RAZORFEN_KRAUL {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::BLACKFATHOM_DEEPS => Self::BLACKFATHOM_DEEPS {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::ULDAMAN => Self::ULDAMAN {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::GNOMERAGON => Self::GNOMERAGON {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::SUNKEN_TEMPLE => Self::SUNKEN_TEMPLE {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::RAZORFEN_DOWNS => Self::RAZORFEN_DOWNS {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::EMERALD_DREAM => Self::EMERALD_DREAM {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::SCARLET_MONASTERY => Self::SCARLET_MONASTERY {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::ZUL_FARRAK => Self::ZUL_FARRAK {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::BLACKROCK_SPIRE => Self::BLACKROCK_SPIRE {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::BLACKROCK_DEPTHS => Self::BLACKROCK_DEPTHS {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::ONYXIAS_LAIR => Self::ONYXIAS_LAIR {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::CAVERNS_OF_TIME => Self::CAVERNS_OF_TIME {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::SCHOLOMANCE => Self::SCHOLOMANCE {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::ZUL_GURUB => Self::ZUL_GURUB {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::STRATHOLME => Self::STRATHOLME {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::MAURADON => Self::MAURADON {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::DEEPRUN_TRAM => Self::DEEPRUN_TRAM {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::RAGEFIRE_CHASM => Self::RAGEFIRE_CHASM {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::MOLTEN_CORE => Self::MOLTEN_CORE {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::DIRE_MAUL => Self::DIRE_MAUL {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::ALLIANCE_PVP_BARRACKS => Self::ALLIANCE_PVP_BARRACKS {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::HORDE_PVP_BARRACKS => Self::HORDE_PVP_BARRACKS {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::DEVELOPMENT_LAND => Self::DEVELOPMENT_LAND {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::BLACKWING_LAIR => Self::BLACKWING_LAIR {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::WARSONG_GULCH => Self::WARSONG_GULCH {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::RUINS_OF_AHN_QIRAJ => Self::RUINS_OF_AHN_QIRAJ {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::ARATHI_BASIN => Self::ARATHI_BASIN {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::AHN_QIRAJ_TEMPLE => Self::AHN_QIRAJ_TEMPLE {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
            },
            Map::NAXXRAMAS => Self::NAXXRAMAS {
                unknown0: Default::default(),
                client_instance_id: Default::default(),
                status_id: Default::default(),
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
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: Map = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for SMSG_BATTLEFIELD_STATUSMap {
    fn size(&self) -> usize {
        match self {
            Self::EASTERN_KINGDOMS =>  {
                4
            }
            Self::KALIMDOR  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::TESTING  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::SCOTT_TEST  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::CASH_TEST  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::ALTERAC_VALLEY  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::SHADOWFANG_KEEP  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::STORMWIND_STOCKADE  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::STORMWIND_PRISON  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::DEADMINES  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::AZSHARA_CRATER  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::COLLINS_TEST  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::WAILING_CAVERNS  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::MONASTERY  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::RAZORFEN_KRAUL  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::BLACKFATHOM_DEEPS  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::ULDAMAN  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::GNOMERAGON  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::SUNKEN_TEMPLE  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::RAZORFEN_DOWNS  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::EMERALD_DREAM  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::SCARLET_MONASTERY  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::ZUL_FARRAK  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::BLACKROCK_SPIRE  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::BLACKROCK_DEPTHS  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::ONYXIAS_LAIR  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::CAVERNS_OF_TIME  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::SCHOLOMANCE  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::ZUL_GURUB  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::STRATHOLME  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::MAURADON  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::DEEPRUN_TRAM  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::RAGEFIRE_CHASM  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::MOLTEN_CORE  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::DIRE_MAUL  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::ALLIANCE_PVP_BARRACKS  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::HORDE_PVP_BARRACKS  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::DEVELOPMENT_LAND  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::BLACKWING_LAIR  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::WARSONG_GULCH  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::RUINS_OF_AHN_QIRAJ  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::ARATHI_BASIN  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::AHN_QIRAJ_TEMPLE  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
            Self::NAXXRAMAS  {
                unknown0,
                client_instance_id,
                status_id,
            } => {
                4
                + 1 // unknown0: u8
                + 4 // client_instance_id: u32
                + status_id.size() // status_id: SMSG_BATTLEFIELD_STATUSStatusId and subfields
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_BATTLEFIELD_STATUSMap {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

