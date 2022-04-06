use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{GroupListMember, GroupListMemberError};
use crate::world::v1::v12::{GroupLootSetting, GroupLootSettingError};
use crate::world::v1::v12::{GroupType, GroupTypeError};
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L14):
/// ```text
/// smsg SMSG_GROUP_LIST = 0x7D {
///     GroupType group_type;
///     u8 own_flags;
///     u32 amount_of_members;
///     GroupListMember[amount_of_members] members;
///     Guid leader;
///     OPTIONAL-STATEMENT-DOCC: unimplemented
/// }
/// ```
pub struct SMSG_GROUP_LIST {
    pub group_type: GroupType,
    pub own_flags: u8,
    pub amount_of_members: u32,
    pub members: Vec<GroupListMember>,
    pub leader: Guid,
    pub group_not_empty: Option<SMSG_GROUP_LIST_group_not_empty>,
}

impl WorldServerMessageWrite for SMSG_GROUP_LIST {
    const OPCODE: u16 = 0x7d;

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
impl WorldMessageBody for SMSG_GROUP_LIST {
    type Error = SMSG_GROUP_LISTError;

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
            0 // If no fields are present, TODO remove when not needed
            + GroupType::size() // group_type: GroupType
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

            Some(SMSG_GROUP_LIST_group_not_empty {
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
            amount_of_members,
            members,
            leader,
            group_not_empty,
        })
    }

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
}

impl VariableSized for SMSG_GROUP_LIST {
    fn size(&self) -> usize {
        GroupType::size() // group_type: GroupType
        + 1 // own_flags: u8
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
        + 8 // leader: Guid
        + {
            if let Some(v) = &self.group_not_empty {
                v.size()
            } else {
                0
            }
        } // optional group_not_empty
    }
}

impl MaximumPossibleSized for SMSG_GROUP_LIST {
    fn maximum_possible_size() -> usize {
        GroupType::maximum_possible_size() // group_type: GroupType
        + 1 // own_flags: u8
        + 4 // amount_of_members: u32
        + 4294967295 * GroupListMember::maximum_possible_size() // members: GroupListMember[amount_of_members]
        + 8 // leader: Guid
        + 65536 // optional group_not_empty
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
    pub fn size(&self) -> usize {
        GroupLootSetting::size() // loot_setting: GroupLootSetting
        + 8 // master_loot: Guid
        + ItemQuality::size() // loot_threshold: ItemQuality
    }
}

