use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    GroupListMember, GroupLootSetting, GroupType, ItemQuality,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L19):
/// ```text
/// smsg SMSG_GROUP_LIST = 0x007D {
///     GroupType group_type;
///     u8 flags;
///     u32 amount_of_members;
///     GroupListMember[amount_of_members] members;
///     Guid leader;
///     optional group_not_empty {
///         GroupLootSetting loot_setting;
///         Guid master_loot;
///         ItemQuality loot_threshold;
///     }
/// }
/// ```
pub struct SMSG_GROUP_LIST {
    pub group_type: GroupType,
    /// mangoszero/cmangos/vmangos: own flags (groupid | (assistant?0x80:0))
    ///
    pub flags: u8,
    pub members: Vec<GroupListMember>,
    pub leader: Guid,
    pub group_not_empty: Option<SMSG_GROUP_LIST_group_not_empty>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_GROUP_LIST {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GROUP_LIST {{").unwrap();
        // Members
        writeln!(s, "    group_type = {};", self.group_type.as_test_case_value()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    amount_of_members = {};", self.members.len()).unwrap();
        write!(s, "    members = [").unwrap();
        for v in self.members.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        name = \"{}\";", v.name).unwrap();
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "        is_online = {};", if v.is_online { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "        flags = {};", v.flags).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    leader = {};", self.leader.guid()).unwrap();
        if let Some(group_not_empty) = &self.group_not_empty {
            writeln!(s, "    loot_setting = {};", group_not_empty.loot_setting.as_test_case_value()).unwrap();
            writeln!(s, "    master_loot = {};", group_not_empty.master_loot.guid()).unwrap();
            writeln!(s, "    loot_threshold = {};", group_not_empty.loot_threshold.as_test_case_value()).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 125_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "group_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_members", "    ");
        if !self.members.is_empty() {
            writeln!(s, "    /* members: GroupListMember[amount_of_members] start */").unwrap();
            for (i, v) in self.members.iter().enumerate() {
                writeln!(s, "    /* members: GroupListMember[amount_of_members] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, v.name.len() + 1, "name", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "is_online", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "        ");
                writeln!(s, "    /* members: GroupListMember[amount_of_members] {i} end */").unwrap();
            }
            writeln!(s, "    /* members: GroupListMember[amount_of_members] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 8, "leader", "    ");
        if let Some(group_not_empty) = &self.group_not_empty {
            crate::util::write_bytes(&mut s, &mut bytes, 1, "loot_setting", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 8, "master_loot", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "loot_threshold", "    ");
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_GROUP_LIST {}
impl crate::Message for SMSG_GROUP_LIST {
    const OPCODE: u32 = 0x007d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_GROUP_LIST::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // group_type: GroupType
        w.write_all(&(self.group_type.as_int().to_le_bytes()))?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

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
            w.write_all(&(v.loot_setting.as_int().to_le_bytes()))?;

            // master_loot: Guid
            w.write_all(&v.master_loot.guid().to_le_bytes())?;

            // loot_threshold: ItemQuality
            w.write_all(&(v.loot_threshold.as_int().to_le_bytes()))?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(14..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x007D, size: body_size });
        }

        // group_type: GroupType
        let group_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // flags: u8
        let flags = crate::util::read_u8_le(&mut r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // members: GroupListMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for _ in 0..amount_of_members {
                members.push(GroupListMember::read(&mut r)?);
            }
            members
        };

        // leader: Guid
        let leader = crate::util::read_guid(&mut r)?;

        // optional group_not_empty
        let current_size = {
            1 // group_type: GroupType
            + 1 // flags: u8
            + 4 // amount_of_members: u32
            + members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
            + 8 // leader: Guid
        };
        let group_not_empty = if current_size < body_size as usize {
            // loot_setting: GroupLootSetting
            let loot_setting = crate::util::read_u8_le(&mut r)?.try_into()?;

            // master_loot: Guid
            let master_loot = crate::util::read_guid(&mut r)?;

            // loot_threshold: ItemQuality
            let loot_threshold = crate::util::read_u8_le(&mut r)?.try_into()?;

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
            flags,
            members,
            leader,
            group_not_empty,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GROUP_LIST {}

impl SMSG_GROUP_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // group_type: GroupType
        + 1 // flags: u8
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: GroupListMember[amount_of_members]
        + 8 // leader: Guid
        + if let Some(group_not_empty) = &self.group_not_empty {
            1 // loot_setting: GroupLootSetting
            + 8 // master_loot: Guid
            + 1 // loot_threshold: ItemQuality
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
}

