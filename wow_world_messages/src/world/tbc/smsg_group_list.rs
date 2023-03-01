use crate:: {
    Guid,
};
use crate::tbc::GroupListMember;
use crate::tbc::DungeonDifficulty;
use crate::tbc::GroupLootSetting;
use crate::tbc::GroupType;
use crate::tbc::ItemQuality;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L50):
/// ```text
/// smsg SMSG_GROUP_LIST = 0x007D {
///     GroupType group_type;
///     Bool battleground_group;
///     u8 group_id;
///     u8 flags;
///     Guid group;
///     u32 amount_of_members;
///     GroupListMember[amount_of_members] members;
///     Guid leader;
///     optional group_not_empty {
///         GroupLootSetting loot_setting;
///         Guid master_loot;
///         ItemQuality loot_threshold;
///         DungeonDifficulty difficulty;
///     }
/// }
/// ```
pub struct SMSG_GROUP_LIST {
    pub group_type: GroupType,
    pub battleground_group: bool,
    pub group_id: u8,
    /// mangoszero/cmangos/vmangos: own flags (groupid | (assistant?0x80:0))
    ///
    pub flags: u8,
    pub group: Guid,
    pub members: Vec<GroupListMember>,
    pub leader: Guid,
    pub group_not_empty: Option<SMSG_GROUP_LIST_group_not_empty>,
}

impl crate::Message for SMSG_GROUP_LIST {
    const OPCODE: u32 = 0x007d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // group_type: GroupType
        w.write_all(&u8::from(self.group_type.as_int()).to_le_bytes())?;

        // battleground_group: Bool
        w.write_all(u8::from(self.battleground_group).to_le_bytes().as_slice())?;

        // group_id: u8
        w.write_all(&self.group_id.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // group: Guid
        w.write_all(&self.group.guid().to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: GroupListMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        // leader: Guid
        w.write_all(&self.leader.guid().to_le_bytes())?;

        // optional group_not_empty
        if let Some(v) = &self.group_not_empty {
            // loot_setting: GroupLootSetting
            w.write_all(&u8::from(v.loot_setting.as_int()).to_le_bytes())?;

            // master_loot: Guid
            w.write_all(&v.master_loot.guid().to_le_bytes())?;

            // loot_threshold: ItemQuality
            w.write_all(&u8::from(v.loot_threshold.as_int()).to_le_bytes())?;

            // difficulty: DungeonDifficulty
            w.write_all(&u8::from(v.difficulty.as_int()).to_le_bytes())?;

        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(24..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x007D, size: body_size as u32 });
        }

        // group_type: GroupType
        let group_type: GroupType = crate::util::read_u8_le(&mut r)?.try_into()?;

        // battleground_group: Bool
        let battleground_group = crate::util::read_u8_le(&mut r)? != 0;

        // group_id: u8
        let group_id = crate::util::read_u8_le(&mut r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(&mut r)?;

        // group: Guid
        let group = Guid::read(&mut r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // members: GroupListMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(GroupListMember::read(&mut r)?);
            }
            members
        };

        // leader: Guid
        let leader = Guid::read(&mut r)?;

        // optional group_not_empty
        let current_size = {
            1 // group_type: GroupType
            + 1 // battleground_group: Bool
            + 1 // group_id: u8
            + 1 // flags: u8
            + 8 // group: Guid
            + 4 // amount_of_members: u32
            + members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
            + 8 // leader: Guid
        };
        let group_not_empty = if current_size < body_size as usize {
            // loot_setting: GroupLootSetting
            let loot_setting: GroupLootSetting = crate::util::read_u8_le(&mut r)?.try_into()?;

            // master_loot: Guid
            let master_loot = Guid::read(&mut r)?;

            // loot_threshold: ItemQuality
            let loot_threshold: ItemQuality = crate::util::read_u8_le(&mut r)?.try_into()?;

            // difficulty: DungeonDifficulty
            let difficulty: DungeonDifficulty = crate::util::read_u8_le(&mut r)?.try_into()?;

            Some(SMSG_GROUP_LIST_group_not_empty {
                loot_setting,
                master_loot,
                loot_threshold,
                difficulty,
            })
        } else {
            None
        };

        Ok(Self {
            group_type,
            battleground_group,
            group_id,
            flags,
            group,
            members,
            leader,
            group_not_empty,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GROUP_LIST {}

impl SMSG_GROUP_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // group_type: GroupType
        + 1 // battleground_group: Bool
        + 1 // group_id: u8
        + 1 // flags: u8
        + 8 // group: Guid
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
        + 8 // leader: Guid
        + if let Some(group_not_empty) = &self.group_not_empty {
            1 // loot_setting: GroupLootSetting
            + 8 // master_loot: Guid
            + 1 // loot_threshold: ItemQuality
            + 1 // difficulty: DungeonDifficulty
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_GROUP_LIST_group_not_empty {
    pub loot_setting: GroupLootSetting,
    pub master_loot: Guid,
    pub loot_threshold: ItemQuality,
    pub difficulty: DungeonDifficulty,
}

impl SMSG_GROUP_LIST_group_not_empty {
    pub(crate) fn size(&self) -> usize {
        1 // loot_setting: GroupLootSetting
        + 8 // master_loot: Guid
        + 1 // loot_threshold: ItemQuality
        + 1 // difficulty: DungeonDifficulty
    }

}

