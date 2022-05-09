use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{GroupListMember, GroupListMemberError};
use crate::world::v1::v12::{GroupLootSetting, GroupLootSettingError};
use crate::world::v1::v12::{GroupType, GroupTypeError};
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GROUP_LIST {
    pub group_type: GroupType,
    pub own_flags: u8,
    pub members: Vec<GroupListMember>,
    pub leader: Guid,
    pub group_not_empty: Option<SMSG_GROUP_LIST_group_not_empty>,
}

impl ServerMessageWrite for SMSG_GROUP_LIST {}

impl MessageBody for SMSG_GROUP_LIST {
    const OPCODE: u16 = 0x007d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GROUP_LISTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // group_type: GroupType
        let group_type = GroupType::read(r)?;

        // own_flags: u8
        let own_flags = crate::util::read_u8_le(r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(r)?;

        // members: GroupListMember[amount_of_members]
        let mut members = Vec::with_capacity(amount_of_members as usize);
        for i in 0..amount_of_members {
            members.push(GroupListMember::read(r)?);
        }

        // leader: Guid
        let leader = Guid::read(r)?;

        // optional group_not_empty
        let current_size = {
            0
            + 1 // group_type: GroupType
            + 1 // own_flags: u8
            + 4 // amount_of_members: u32
            + members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
            + 8 // leader: Guid
        };
        let group_not_empty = if current_size < body_size as usize {
            // loot_setting: GroupLootSetting
            let loot_setting = GroupLootSetting::read(r)?;

            // master_loot: Guid
            let master_loot = Guid::read(r)?;

            // loot_threshold: ItemQuality
            let loot_threshold = ItemQuality::read(r)?;

            Some(SMSG_GROUP_LISTgroup_not_empty {
                loot_setting,
                master_loot,
                loot_threshold,
            })
        } else {
            None
        };

        Ok(Self {
            group_type,
            own_flags,
            members,
            leader,
            group_not_empty,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // group_type: GroupType
        self.group_type.write(w)?;

        // own_flags: u8
        w.write_all(&self.own_flags.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: GroupListMember[amount_of_members]
        for i in self.members.iter() {
            i.write(w)?;
        }

        // leader: Guid
        self.leader.write(w)?;

        // optional group_not_empty
        if let Some(v) = &self.group_not_empty {
            // loot_setting: GroupLootSetting
            v.loot_setting.write(w)?;

            // master_loot: Guid
            v.master_loot.write(w)?;

            // loot_threshold: ItemQuality
            v.loot_threshold.write(w)?;

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
            // group_type: GroupType
            let group_type = GroupType::tokio_read(r).await?;

            // own_flags: u8
            let own_flags = crate::util::tokio_read_u8_le(r).await?;

            // amount_of_members: u32
            let amount_of_members = crate::util::tokio_read_u32_le(r).await?;

            // members: GroupListMember[amount_of_members]
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(GroupListMember::tokio_read(r).await?);
            }

            // leader: Guid
            let leader = Guid::tokio_read(r).await?;

            // optional group_not_empty
            let current_size = {
                0
                + 1 // group_type: GroupType
                + 1 // own_flags: u8
                + 4 // amount_of_members: u32
                + members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
                + 8 // leader: Guid
            };
            let group_not_empty = if current_size < body_size as usize {
                // loot_setting: GroupLootSetting
                let loot_setting = GroupLootSetting::tokio_read(r).await?;

                // master_loot: Guid
                let master_loot = Guid::tokio_read(r).await?;

                // loot_threshold: ItemQuality
                let loot_threshold = ItemQuality::tokio_read(r).await?;

                Some(SMSG_GROUP_LISTgroup_not_empty {
                    loot_setting,
                    master_loot,
                    loot_threshold,
                })
            } else {
                None
            };

            Ok(Self {
                group_type,
                own_flags,
                members,
                leader,
                group_not_empty,
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
            // group_type: GroupType
            self.group_type.tokio_write(w).await?;

            // own_flags: u8
            w.write_all(&self.own_flags.to_le_bytes()).await?;

            // amount_of_members: u32
            w.write_all(&(self.members.len() as u32).to_le_bytes()).await?;

            // members: GroupListMember[amount_of_members]
            for i in self.members.iter() {
                i.tokio_write(w).await?;
            }

            // leader: Guid
            self.leader.tokio_write(w).await?;

            // optional group_not_empty
            if let Some(v) = &self.group_not_empty {
                // loot_setting: GroupLootSetting
                v.loot_setting.tokio_write(w).await?;

                // master_loot: Guid
                v.master_loot.tokio_write(w).await?;

                // loot_threshold: ItemQuality
                v.loot_threshold.tokio_write(w).await?;

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
            // group_type: GroupType
            let group_type = GroupType::astd_read(r).await?;

            // own_flags: u8
            let own_flags = crate::util::astd_read_u8_le(r).await?;

            // amount_of_members: u32
            let amount_of_members = crate::util::astd_read_u32_le(r).await?;

            // members: GroupListMember[amount_of_members]
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(GroupListMember::astd_read(r).await?);
            }

            // leader: Guid
            let leader = Guid::astd_read(r).await?;

            // optional group_not_empty
            let current_size = {
                0
                + 1 // group_type: GroupType
                + 1 // own_flags: u8
                + 4 // amount_of_members: u32
                + members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
                + 8 // leader: Guid
            };
            let group_not_empty = if current_size < body_size as usize {
                // loot_setting: GroupLootSetting
                let loot_setting = GroupLootSetting::astd_read(r).await?;

                // master_loot: Guid
                let master_loot = Guid::astd_read(r).await?;

                // loot_threshold: ItemQuality
                let loot_threshold = ItemQuality::astd_read(r).await?;

                Some(SMSG_GROUP_LISTgroup_not_empty {
                    loot_setting,
                    master_loot,
                    loot_threshold,
                })
            } else {
                None
            };

            Ok(Self {
                group_type,
                own_flags,
                members,
                leader,
                group_not_empty,
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
            // group_type: GroupType
            self.group_type.astd_write(w).await?;

            // own_flags: u8
            w.write_all(&self.own_flags.to_le_bytes()).await?;

            // amount_of_members: u32
            w.write_all(&(self.members.len() as u32).to_le_bytes()).await?;

            // members: GroupListMember[amount_of_members]
            for i in self.members.iter() {
                i.astd_write(w).await?;
            }

            // leader: Guid
            self.leader.astd_write(w).await?;

            // optional group_not_empty
            if let Some(v) = &self.group_not_empty {
                // loot_setting: GroupLootSetting
                v.loot_setting.astd_write(w).await?;

                // master_loot: Guid
                v.master_loot.astd_write(w).await?;

                // loot_threshold: ItemQuality
                v.loot_threshold.astd_write(w).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_GROUP_LIST {
    fn size(&self) -> usize {
        0
        + 1 // group_type: GroupType
        + 1 // own_flags: u8
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
        + 8 // leader: Guid
        + if let Some(group_not_empty) = &self.group_not_empty {
            0
            + 1 // loot_setting: GroupLootSetting
            + 8 // master_loot: Guid
            + 1 // loot_threshold: ItemQuality
        } else {
            0
        }
    }
}

impl MaximumPossibleSized for SMSG_GROUP_LIST {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum SMSG_GROUP_LISTError {
    Io(std::io::Error),
    GroupListMember(GroupListMemberError),
    GroupLootSetting(GroupLootSettingError),
    GroupType(GroupTypeError),
    ItemQuality(ItemQualityError),
}

impl std::error::Error for SMSG_GROUP_LISTError {}
impl std::fmt::Display for SMSG_GROUP_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::GroupListMember(i) => i.fmt(f),
            Self::GroupLootSetting(i) => i.fmt(f),
            Self::GroupType(i) => i.fmt(f),
            Self::ItemQuality(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GROUP_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<GroupListMemberError> for SMSG_GROUP_LISTError {
    fn from(e: GroupListMemberError) -> Self {
        Self::GroupListMember(e)
    }
}

impl From<GroupLootSettingError> for SMSG_GROUP_LISTError {
    fn from(e: GroupLootSettingError) -> Self {
        Self::GroupLootSetting(e)
    }
}

impl From<GroupTypeError> for SMSG_GROUP_LISTError {
    fn from(e: GroupTypeError) -> Self {
        Self::GroupType(e)
    }
}

impl From<ItemQualityError> for SMSG_GROUP_LISTError {
    fn from(e: ItemQualityError) -> Self {
        Self::ItemQuality(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_GROUP_LIST_group_not_empty {
    pub loot_setting: GroupLootSetting,
    pub master_loot: Guid,
    pub loot_threshold: ItemQuality,
}

impl SMSG_GROUP_LIST_group_not_empty {
    pub(crate) fn size(&self) -> usize {
        1 // loot_setting: GroupLootSetting
        + 8 // master_loot: Guid
        + 1 // loot_threshold: ItemQuality
    }

}

