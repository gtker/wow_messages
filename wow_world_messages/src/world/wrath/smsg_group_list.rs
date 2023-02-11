use crate::Guid;
use crate::wrath::GroupListMember;
use crate::wrath::DungeonDifficulty;
use crate::wrath::GroupLootSetting;
use crate::wrath::ItemQuality;
use crate::wrath::RaidDifficulty;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:95`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L95):
/// ```text
/// smsg SMSG_GROUP_LIST = 0x007D {
///     u8 group_type;
///     u8 group_id;
///     u8 flags;
///     u8 roles;
///     Guid group;
///     u32 counter;
///     u32 amount_of_members;
///     GroupListMember[amount_of_members] members;
///     Guid leader;
///     optional group_not_empty {
///         GroupLootSetting loot_setting;
///         Guid master_loot;
///         ItemQuality loot_threshold;
///         DungeonDifficulty difficulty;
///         RaidDifficulty raid_difficulty;
///         Bool heroic;
///     }
/// }
/// ```
pub struct SMSG_GROUP_LIST {
    pub group_type: u8,
    pub group_id: u8,
    /// mangoszero/cmangos/vmangos: own flags (groupid | (assistant?0x80:0))
    ///
    pub flags: u8,
    pub roles: u8,
    pub group: Guid,
    /// azerothcore: 3.3, value increases every time this packet gets sent
    ///
    pub counter: u32,
    pub members: Vec<GroupListMember>,
    pub leader: Guid,
    pub group_not_empty: Option<SMSG_GROUP_LIST_group_not_empty>,
}

impl crate::Message for SMSG_GROUP_LIST {
    const OPCODE: u32 = 0x007d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // group_type: u8
        w.write_all(&self.group_type.to_le_bytes())?;

        // group_id: u8
        w.write_all(&self.group_id.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // roles: u8
        w.write_all(&self.roles.to_le_bytes())?;

        // group: Guid
        w.write_all(&self.group.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: GroupListMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(w)?;
        }

        // leader: Guid
        w.write_all(&self.leader.guid().to_le_bytes())?;

        // optional group_not_empty
        if let Some(v) = &self.group_not_empty {
            // loot_setting: GroupLootSetting
            w.write_all(&(v.loot_setting.as_int() as u8).to_le_bytes())?;

            // master_loot: Guid
            w.write_all(&v.master_loot.guid().to_le_bytes())?;

            // loot_threshold: ItemQuality
            w.write_all(&(v.loot_threshold.as_int() as u8).to_le_bytes())?;

            // difficulty: DungeonDifficulty
            w.write_all(&(v.difficulty.as_int() as u8).to_le_bytes())?;

            // raid_difficulty: RaidDifficulty
            w.write_all(&(v.raid_difficulty.as_int() as u8).to_le_bytes())?;

            // heroic: Bool
            w.write_all(u8::from(v.heroic).to_le_bytes().as_slice())?;

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(28..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x007D, size: body_size as u32 });
        }

        // group_type: u8
        let group_type = crate::util::read_u8_le(r)?;

        // group_id: u8
        let group_id = crate::util::read_u8_le(r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        // roles: u8
        let roles = crate::util::read_u8_le(r)?;

        // group: Guid
        let group = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

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
            1 // group_type: u8
            + 1 // group_id: u8
            + 1 // flags: u8
            + 1 // roles: u8
            + 8 // group: Guid
            + 4 // counter: u32
            + 4 // amount_of_members: u32
            + members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
            + 8 // leader: Guid
        };
        let group_not_empty = if current_size < body_size as usize {
            // loot_setting: GroupLootSetting
            let loot_setting: GroupLootSetting = crate::util::read_u8_le(r)?.try_into()?;

            // master_loot: Guid
            let master_loot = Guid::read(r)?;

            // loot_threshold: ItemQuality
            let loot_threshold: ItemQuality = crate::util::read_u8_le(r)?.try_into()?;

            // difficulty: DungeonDifficulty
            let difficulty: DungeonDifficulty = crate::util::read_u8_le(r)?.try_into()?;

            // raid_difficulty: RaidDifficulty
            let raid_difficulty: RaidDifficulty = crate::util::read_u8_le(r)?.try_into()?;

            // heroic: Bool
            let heroic = crate::util::read_u8_le(r)? != 0;
            Some(SMSG_GROUP_LIST_group_not_empty {
                loot_setting,
                master_loot,
                loot_threshold,
                difficulty,
                raid_difficulty,
                heroic,
            })
        } else {
            None
        };

        Ok(Self {
            group_type,
            group_id,
            flags,
            roles,
            group,
            counter,
            members,
            leader,
            group_not_empty,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GROUP_LIST {}

impl SMSG_GROUP_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // group_type: u8
        + 1 // group_id: u8
        + 1 // flags: u8
        + 1 // roles: u8
        + 8 // group: Guid
        + 4 // counter: u32
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
        + 8 // leader: Guid
        + if let Some(group_not_empty) = &self.group_not_empty {
            1 // loot_setting: GroupLootSetting
            + 8 // master_loot: Guid
            + 1 // loot_threshold: ItemQuality
            + 1 // difficulty: DungeonDifficulty
            + 1 // raid_difficulty: RaidDifficulty
            + 1 // heroic: Bool
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
    pub raid_difficulty: RaidDifficulty,
    pub heroic: bool,
}

impl SMSG_GROUP_LIST_group_not_empty {
    pub(crate) fn size(&self) -> usize {
        1 // loot_setting: GroupLootSetting
        + 8 // master_loot: Guid
        + 1 // loot_threshold: ItemQuality
        + 1 // difficulty: DungeonDifficulty
        + 1 // raid_difficulty: RaidDifficulty
        + 1 // heroic: Bool
    }

}

