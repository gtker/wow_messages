use crate::Guid;
use wow_world_base::shared::raid_target_index_vanilla_tbc_wrath::RaidTargetIndex;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L35):
/// ```text
/// cmsg MSG_RAID_TARGET_UPDATE_Client = 0x0321 {
///     RaidTargetIndex target_index;
///     if (target_index != REQUEST_ICONS) {
///         Guid target;
///     }
/// }
/// ```
pub struct MSG_RAID_TARGET_UPDATE_Client {
    pub target_index: MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex,
}

impl crate::Message for MSG_RAID_TARGET_UPDATE_Client {
    const OPCODE: u32 = 0x0321;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target_index: RaidTargetIndex
        w.write_all(&u8::from(self.target_index.as_int()).to_le_bytes())?;

        match &self.target_index {
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
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0321, size: body_size as u32 });
        }

        // target_index: RaidTargetIndex
        let target_index: RaidTargetIndex = crate::util::read_u8_le(&mut r)?.try_into()?;

        let target_index_if = match target_index {
            RaidTargetIndex::Unknown0 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown0 {
                    target,
                }
            }
            RaidTargetIndex::Unknown1 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown1 {
                    target,
                }
            }
            RaidTargetIndex::Unknown2 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown2 {
                    target,
                }
            }
            RaidTargetIndex::Unknown3 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown3 {
                    target,
                }
            }
            RaidTargetIndex::Unknown4 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown4 {
                    target,
                }
            }
            RaidTargetIndex::Unknown5 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown5 {
                    target,
                }
            }
            RaidTargetIndex::Unknown6 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown6 {
                    target,
                }
            }
            RaidTargetIndex::Unknown7 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown7 {
                    target,
                }
            }
            RaidTargetIndex::Unknown8 => {
                // target: Guid
                let target = Guid::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown8 {
                    target,
                }
            }
            RaidTargetIndex::RequestIcons => MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::RequestIcons,
        };

        Ok(Self {
            target_index: target_index_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_RAID_TARGET_UPDATE_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_RAID_TARGET_UPDATE_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_RAID_TARGET_UPDATE_Client {}

impl MSG_RAID_TARGET_UPDATE_Client {
    pub(crate) fn size(&self) -> usize {
        self.target_index.size() // target_index: MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
        Self::RequestIcons
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

