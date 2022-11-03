use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::RaidTargetUpdate;
use crate::world::vanilla::RaidTargetUpdateType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L26):
/// ```text
/// smsg MSG_RAID_TARGET_UPDATE_Server = 0x0321 {
///     RaidTargetUpdateType update_type;
///     if (update_type == FULL) {
///         RaidTargetUpdate[8] raid_targets;
///     }
///     else if (update_type == PARTIAL) {
///         RaidTargetUpdate raid_target;
///     }
/// }
/// ```
pub struct MSG_RAID_TARGET_UPDATE_Server {
    pub update_type: MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType,
}

impl crate::Message for MSG_RAID_TARGET_UPDATE_Server {
    const OPCODE: u32 = 0x0321;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // update_type: RaidTargetUpdateType
        w.write_all(&(self.update_type.as_int() as u8).to_le_bytes())?;

        match &self.update_type {
            MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Partial {
                raid_target,
            } => {
                // raid_target: RaidTargetUpdate
                raid_target.write_into_vec(w)?;

            }
            MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Full {
                raid_targets,
            } => {
                // raid_targets: RaidTargetUpdate[8]
                for i in raid_targets.iter() {
                    i.write_into_vec(w)?;
                }

            }
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 1 || body_size > 73 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0321, size: body_size as u32 });
        }

        // update_type: RaidTargetUpdateType
        let update_type: RaidTargetUpdateType = crate::util::read_u8_le(r)?.try_into()?;

        let update_type_if = match update_type {
            RaidTargetUpdateType::Partial => {
                // raid_target: RaidTargetUpdate
                let raid_target = RaidTargetUpdate::read(r)?;

                MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Partial {
                    raid_target,
                }
            }
            RaidTargetUpdateType::Full => {
                // raid_targets: RaidTargetUpdate[8]
                let mut raid_targets = [RaidTargetUpdate::default(); 8];
                for i in raid_targets.iter_mut() {
                    *i = RaidTargetUpdate::read(r)?;
                }

                MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Full {
                    raid_targets,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_RAID_TARGET_UPDATE_Server {}

impl MSG_RAID_TARGET_UPDATE_Server {
    pub(crate) fn size(&self) -> usize {
        self.update_type.size() // update_type: MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    Partial {
        raid_target: RaidTargetUpdate,
    },
    Full {
        raid_targets: [RaidTargetUpdate; 8],
    },
}

impl Default for MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Partial {
            raid_target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Partial { .. } => 0,
            Self::Full { .. } => 1,
        }
    }

}

impl MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Partial {
                raid_target,
            } => {
                1
                + 9 // raid_target: RaidTargetUpdate
            }
            Self::Full {
                raid_targets,
            } => {
                1
                + 8 * 9 // raid_targets: RaidTargetUpdate[8]
            }
        }
    }
}

