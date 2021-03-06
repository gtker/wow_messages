use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::RaidTargetIndex;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L35):
/// ```text
/// cmsg MSG_RAID_TARGET_UPDATE_Client = 0x0321 {
///     RaidTargetIndex index;
///     if (index != REQUEST_ICONS) {
///         Guid target;
///     }
/// }
/// ```
pub struct MSG_RAID_TARGET_UPDATE_Client {
    pub index: MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex,
}

impl ClientMessage for MSG_RAID_TARGET_UPDATE_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes())?;

        match &self.index {
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN0 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN1 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN2 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN3 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN4 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN5 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN6 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN7 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN8 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::REQUEST_ICONS => {}
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0321;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // index: RaidTargetIndex
        let index: RaidTargetIndex = crate::util::read_u8_le(r)?.try_into()?;

        let index_if = match index {
            RaidTargetIndex::UNKNOWN0 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN0 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN1 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN1 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN2 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN2 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN3 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN3 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN4 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN4 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN5 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN5 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN6 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN6 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN7 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN7 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN8 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::UNKNOWN8 {
                    target,
                }
            }
            RaidTargetIndex::REQUEST_ICONS => MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::REQUEST_ICONS,
        };

        Ok(Self {
            index: index_if,
        })
    }

}

impl MSG_RAID_TARGET_UPDATE_Client {
    pub(crate) fn size(&self) -> usize {
        self.index.size() // index: MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    UNKNOWN0 {
        target: Guid,
    },
    UNKNOWN1 {
        target: Guid,
    },
    UNKNOWN2 {
        target: Guid,
    },
    UNKNOWN3 {
        target: Guid,
    },
    UNKNOWN4 {
        target: Guid,
    },
    UNKNOWN5 {
        target: Guid,
    },
    UNKNOWN6 {
        target: Guid,
    },
    UNKNOWN7 {
        target: Guid,
    },
    UNKNOWN8 {
        target: Guid,
    },
    REQUEST_ICONS,
}

impl Default for MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    fn default() -> Self {
        // First enumerator without any fields
        Self::UNKNOWN0 {
            target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::UNKNOWN0 { .. } => 0,
            Self::UNKNOWN1 { .. } => 1,
            Self::UNKNOWN2 { .. } => 2,
            Self::UNKNOWN3 { .. } => 3,
            Self::UNKNOWN4 { .. } => 4,
            Self::UNKNOWN5 { .. } => 5,
            Self::UNKNOWN6 { .. } => 6,
            Self::UNKNOWN7 { .. } => 7,
            Self::UNKNOWN8 { .. } => 8,
            Self::REQUEST_ICONS => 255,
        }
    }

}

impl MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::UNKNOWN0 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN1 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN2 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN3 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN4 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN5 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN6 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN7 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN8 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::REQUEST_ICONS => {
                1
            }
        }
    }
}

