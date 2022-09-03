use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::RaidTargetIndex;
use crate::world::vanilla::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

impl crate::Message for MSG_RAID_TARGET_UPDATE_Client {
    const OPCODE: u32 = 0x0321;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes())?;

        match &self.index {
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown0 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown1 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown2 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown3 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown4 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown5 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown6 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown7 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown8 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::RequestIcons => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // index: RaidTargetIndex
        let index: RaidTargetIndex = crate::util::read_u8_le(r)?.try_into()?;

        let index_if = match index {
            RaidTargetIndex::Unknown0 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown0 {
                    target,
                }
            }
            RaidTargetIndex::Unknown1 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown1 {
                    target,
                }
            }
            RaidTargetIndex::Unknown2 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown2 {
                    target,
                }
            }
            RaidTargetIndex::Unknown3 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown3 {
                    target,
                }
            }
            RaidTargetIndex::Unknown4 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown4 {
                    target,
                }
            }
            RaidTargetIndex::Unknown5 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown5 {
                    target,
                }
            }
            RaidTargetIndex::Unknown6 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown6 {
                    target,
                }
            }
            RaidTargetIndex::Unknown7 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown7 {
                    target,
                }
            }
            RaidTargetIndex::Unknown8 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown8 {
                    target,
                }
            }
            RaidTargetIndex::RequestIcons => MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::RequestIcons,
        };

        Ok(Self {
            index: index_if,
        })
    }

}
impl ClientMessage for MSG_RAID_TARGET_UPDATE_Client {}

impl MSG_RAID_TARGET_UPDATE_Client {
    pub(crate) fn size(&self) -> usize {
        self.index.size() // index: MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex
    }
}

#[derive(Debug, PartialEq, Clone)]
#[derive(Copy)]
pub enum MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    Unknown0 {
        target: Guid,
    },
    Unknown1 {
        target: Guid,
    },
    Unknown2 {
        target: Guid,
    },
    Unknown3 {
        target: Guid,
    },
    Unknown4 {
        target: Guid,
    },
    Unknown5 {
        target: Guid,
    },
    Unknown6 {
        target: Guid,
    },
    Unknown7 {
        target: Guid,
    },
    Unknown8 {
        target: Guid,
    },
    RequestIcons,
}

impl Default for MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Unknown0 {
            target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Unknown0 { .. } => 0,
            Self::Unknown1 { .. } => 1,
            Self::Unknown2 { .. } => 2,
            Self::Unknown3 { .. } => 3,
            Self::Unknown4 { .. } => 4,
            Self::Unknown5 { .. } => 5,
            Self::Unknown6 { .. } => 6,
            Self::Unknown7 { .. } => 7,
            Self::Unknown8 { .. } => 8,
            Self::RequestIcons => 255,
        }
    }

}

impl MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Unknown0 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown1 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown2 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown3 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown4 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown5 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown6 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown7 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown8 {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::RequestIcons => {
                1
            }
        }
    }
}

