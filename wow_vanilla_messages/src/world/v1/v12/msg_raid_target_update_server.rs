use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RaidTargetUpdate, RaidTargetUpdateError};
use crate::world::v1::v12::{RaidTargetUpdateType, RaidTargetUpdateTypeError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L26):
/// ```text
/// smsg MSG_RAID_TARGET_UPDATE_Server = 0x321 {
///     RaidTargetUpdateType update_type;
///     if (update_type == FULL) {
///         RaidTargetUpdate[8] raid_targets;
///         ELSE-IF-STATEMENT-DOCC: unimplemented
///     }
/// }
/// ```
pub struct MSG_RAID_TARGET_UPDATE_Server {
    pub update_type: MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType,
}

impl WorldServerMessageWrite for MSG_RAID_TARGET_UPDATE_Server {
    const OPCODE: u16 = 0x321;

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
impl WorldMessageBody for MSG_RAID_TARGET_UPDATE_Server {
    type Error = MSG_RAID_TARGET_UPDATE_ServerError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // update_type: RaidTargetUpdateType
        let update_type = RaidTargetUpdateType::read(r)?;

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
                let mut raid_targets = Vec::with_capacity(8 as usize);
                for i in 0..8 {
                    raid_targets.push(RaidTargetUpdate::read(r)?);
                }
                let raid_targets = raid_targets.try_into().unwrap();

                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                    raid_targets,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // update_type: RaidTargetUpdateType
        self.update_type.write(w)?;

        match &self.update_type {
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                raid_target,
            } => {
                // raid_target: RaidTargetUpdate
                raid_target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                raid_targets,
            } => {
                // raid_targets: RaidTargetUpdate[8]
                for i in raid_targets.iter() {
                    i.write(w)?;
                }

            }
        }

        Ok(())
    }
}

impl VariableSized for MSG_RAID_TARGET_UPDATE_Server {
    fn size(&self) -> usize {
        self.update_type.size() // update_type: RaidTargetUpdateType and subfields
    }
}

impl MaximumPossibleSized for MSG_RAID_TARGET_UPDATE_Server {
    fn maximum_possible_size() -> usize {
        RaidTargetUpdateType::maximum_possible_size() // update_type: RaidTargetUpdateType
    }
}

#[derive(Debug)]
pub enum MSG_RAID_TARGET_UPDATE_ServerError {
    Io(std::io::Error),
    RaidTargetUpdate(RaidTargetUpdateError),
    RaidTargetUpdateType(RaidTargetUpdateTypeError),
}

impl std::error::Error for MSG_RAID_TARGET_UPDATE_ServerError {}
impl std::fmt::Display for MSG_RAID_TARGET_UPDATE_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidTargetUpdate(i) => i.fmt(f),
            Self::RaidTargetUpdateType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_RAID_TARGET_UPDATE_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidTargetUpdateError> for MSG_RAID_TARGET_UPDATE_ServerError {
    fn from(e: RaidTargetUpdateError) -> Self {
        Self::RaidTargetUpdate(e)
    }
}

impl From<RaidTargetUpdateTypeError> for MSG_RAID_TARGET_UPDATE_ServerError {
    fn from(e: RaidTargetUpdateTypeError) -> Self {
        Self::RaidTargetUpdateType(e)
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

impl From<&RaidTargetUpdateType> for MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    fn from(e: &RaidTargetUpdateType) -> Self {
        match &e {
            RaidTargetUpdateType::PARTIAL => Self::PARTIAL {
                raid_target: Default::default(),
            },
            RaidTargetUpdateType::FULL => Self::FULL {
                raid_targets: Default::default(),
            },
        }
    }
}

impl From<&MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType> for RaidTargetUpdateType {
    fn from(v: &MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType) -> Self {
        match &v {
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL { .. } => Self::PARTIAL,
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL { .. } => Self::FULL,
        }
    }
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
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetUpdateType = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetUpdateType = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetUpdateType = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetUpdateType = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetUpdateType = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetUpdateType = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetUpdateType = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    fn size(&self) -> usize {
        match self {
            Self::PARTIAL  {
                raid_target,
            } => {
                1
                + RaidTargetUpdate::size() // raid_target: RaidTargetUpdate
            }
            Self::FULL  {
                raid_targets,
            } => {
                1
                + 8 * RaidTargetUpdate::size() // raid_targets: RaidTargetUpdate[8]
            }
        }
    }
}

impl MaximumPossibleSized for MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

