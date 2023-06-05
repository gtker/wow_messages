use std::io::{Read, Write};

use crate::wrath::{
    GuildMember, GuildRights,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:84`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L84):
/// ```text
/// smsg SMSG_GUILD_ROSTER = 0x008A {
///     u32 amount_of_members;
///     CString motd;
///     CString guild_info;
///     u32 amount_of_rights;
///     GuildRights[amount_of_rights] rights;
///     GuildMember[amount_of_members] members;
/// }
/// ```
pub struct SMSG_GUILD_ROSTER {
    pub motd: String,
    pub guild_info: String,
    pub rights: Vec<GuildRights>,
    pub members: Vec<GuildMember>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_GUILD_ROSTER {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_ROSTER {{").unwrap();
        // Members
        writeln!(s, "    amount_of_members = {};", self.members.len()).unwrap();
        writeln!(s, "    motd = \"{}\";", self.motd).unwrap();
        writeln!(s, "    guild_info = \"{}\";", self.guild_info).unwrap();
        writeln!(s, "    amount_of_rights = {};", self.rights.len()).unwrap();
        write!(s, "    rights = [").unwrap();
        for v in self.rights.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        rights = {};", v.rights).unwrap();
            writeln!(s, "        money_per_day = {};", v.money_per_day.as_int()).unwrap();
            write!(s, "        bank_tab_rights = [").unwrap();
            for v in v.bank_tab_rights.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "            rights = {};", v.rights).unwrap();
                writeln!(s, "            slots_per_day = {};", v.slots_per_day).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    members = [").unwrap();
        for v in self.members.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "        unknown = {};", v.unknown).unwrap();
            writeln!(s, "        status = {};", crate::vanilla::GuildMemberStatus::try_from(v.status.as_int()).unwrap().as_test_case_value()).unwrap();
            writeln!(s, "        name = \"{}\";", v.name).unwrap();
            writeln!(s, "        rank = {};", v.rank).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();
            writeln!(s, "        class = {};", v.class.as_test_case_value()).unwrap();
            writeln!(s, "        gender = {};", v.gender.as_test_case_value()).unwrap();
            writeln!(s, "        area = {};", v.area.as_test_case_value()).unwrap();
            match &v.status {
                crate::wrath::GuildMember_GuildMemberStatus::Offline {
                    time_offline,
                } => {
                    writeln!(s, "    {}", if time_offline.to_string().contains(".") { time_offline.to_string() } else { format!("{}.0", time_offline) }).unwrap();
                }
                _ => {}
            }

            writeln!(s, "        public_note = \"{}\";", v.public_note).unwrap();
            writeln!(s, "        officer_note = \"{}\";", v.officer_note).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 138_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_members", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.motd.len() + 1, "motd", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.guild_info.len() + 1, "guild_info", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_rights", "    ");
        if !self.rights.is_empty() {
            writeln!(s, "    /* rights: GuildRights[amount_of_rights] start */").unwrap();
            for (i, v) in self.rights.iter().enumerate() {
                writeln!(s, "    /* rights: GuildRights[amount_of_rights] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "rights", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "money_per_day", "        ");
                writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] start */").unwrap();
                for (i, v) in v.bank_tab_rights.iter().enumerate() {
                    writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] {i} start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "rights", "            ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "slots_per_day", "            ");
                    writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] {i} end */").unwrap();
                }
                writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] end */").unwrap();
                writeln!(s, "    /* rights: GuildRights[amount_of_rights] {i} end */").unwrap();
            }
            writeln!(s, "    /* rights: GuildRights[amount_of_rights] end */").unwrap();
        }
        if !self.members.is_empty() {
            writeln!(s, "    /* members: GuildMember[amount_of_members] start */").unwrap();
            for (i, v) in self.members.iter().enumerate() {
                writeln!(s, "    /* members: GuildMember[amount_of_members] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.name.len() + 1, "name", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "rank", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "class", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "gender", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                match &v.status {
                    crate::wrath::GuildMember_GuildMemberStatus::Offline {
                        time_offline,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_offline", "        ");
                    }
                    _ => {}
                }

                crate::util::write_bytes(&mut s, &mut bytes, v.public_note.len() + 1, "public_note", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.officer_note.len() + 1, "officer_note", "        ");
                writeln!(s, "    /* members: GuildMember[amount_of_members] {i} end */").unwrap();
            }
            writeln!(s, "    /* members: GuildMember[amount_of_members] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_GUILD_ROSTER {}
impl crate::Message for SMSG_GUILD_ROSTER {
    const OPCODE: u32 = 0x008a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_GUILD_ROSTER::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // motd: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.motd.as_bytes().iter().rev().next(), Some(&0_u8), "String `motd` must not be null-terminated.");
        w.write_all(self.motd.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild_info: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_info.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_info` must not be null-terminated.");
        w.write_all(self.guild_info.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // amount_of_rights: u32
        w.write_all(&(self.rights.len() as u32).to_le_bytes())?;

        // rights: GuildRights[amount_of_rights]
        for i in self.rights.iter() {
            i.write_into_vec(&mut w)?;
        }

        // members: GuildMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(10..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x008A, size: body_size });
        }

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // motd: CString
        let motd = {
            let motd = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(motd)?
        };

        // guild_info: CString
        let guild_info = {
            let guild_info = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_info)?
        };

        // amount_of_rights: u32
        let amount_of_rights = crate::util::read_u32_le(&mut r)?;

        // rights: GuildRights[amount_of_rights]
        let rights = {
            let mut rights = Vec::with_capacity(amount_of_rights as usize);
            for _ in 0..amount_of_rights {
                rights.push(GuildRights::read(&mut r)?);
            }
            rights
        };

        // members: GuildMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for _ in 0..amount_of_members {
                members.push(GuildMember::read(&mut r)?);
            }
            members
        };

        Ok(Self {
            motd,
            guild_info,
            rights,
            members,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GUILD_ROSTER {}

impl SMSG_GUILD_ROSTER {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_members: u32
        + self.motd.len() + 1 // motd: CString
        + self.guild_info.len() + 1 // guild_info: CString
        + 4 // amount_of_rights: u32
        + self.rights.len() * 56 // rights: GuildRights[amount_of_rights]
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: GuildMember[amount_of_members]
    }
}

