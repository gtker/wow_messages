use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::RaidTargetUpdate;
use crate::world::v1::v12::RaidTargetUpdateType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_RAID_TARGET_UPDATE_Server {
    pub update_type: MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType,
}

impl ServerMessage for MSG_RAID_TARGET_UPDATE_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // update_type: RaidTargetUpdateType
        w.write_all(&(self.update_type.as_int() as u8).to_le_bytes())?;

        match &self.update_type {
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                raid_target,
            } => {
                // raid_target: RaidTargetUpdate
                raid_target.write_into_vec(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                raid_targets,
            } => {
                // raid_targets: RaidTargetUpdate[8]
                for i in raid_targets.iter() {
                    i.write_into_vec(w)?;
                }

            }
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0321;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // update_type: RaidTargetUpdateType
        let update_type: RaidTargetUpdateType = crate::util::read_u8_le(r)?.try_into()?;

        let update_type_if = match update_type {
            RaidTargetUpdateType::PARTIAL => {
                // raid_target: RaidTargetUpdate
                let raid_target = RaidTargetUpdate::read(r)?;

                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                    raid_target,
                }
            }
            RaidTargetUpdateType::FULL => {
                // raid_targets: RaidTargetUpdate[8]
                let mut raid_targets = [RaidTargetUpdate::default(); 8];
                for i in raid_targets.iter_mut() {
                    *i = RaidTargetUpdate::read(r)?;
                }

                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                    raid_targets,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

}

impl MSG_RAID_TARGET_UPDATE_Server {
    pub(crate) fn size(&self) -> usize {
        self.update_type.size() // update_type: MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    PARTIAL {
        raid_target: RaidTargetUpdate,
    },
    FULL {
        raid_targets: [RaidTargetUpdate; 8],
    },
}

impl Default for MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::PARTIAL {
            raid_target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PARTIAL { .. } => 0,
            Self::FULL { .. } => 1,
        }
    }

}

impl MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::PARTIAL {
                raid_target,
            } => {
                1
                + 9 // raid_target: RaidTargetUpdate
            }
            Self::FULL {
                raid_targets,
            } => {
                1
                + 8 * 9 // raid_targets: RaidTargetUpdate[8]
            }
        }
    }
}

