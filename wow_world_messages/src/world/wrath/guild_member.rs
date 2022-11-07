use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::Area;
use crate::world::wrath::Class;
use crate::world::wrath::Gender;
use crate::world::wrath::GuildMemberStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:56`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L56):
/// ```text
/// struct GuildMember {
///     Guid guid;
///     u32 unknown;
///     GuildMemberStatus status;
///     CString name;
///     u32 rank;
///     u8 level;
///     Class class;
///     Gender gender;
///     Area area;
///     if (status == OFFLINE) {
///         f32 time_offline;
///     }
///     CString public_note;
///     CString officer_note;
/// }
/// ```
pub struct GuildMember {
    pub guid: Guid,
    /// arcemu: high guid
    ///
    pub unknown: u32,
    pub status: GuildMember_GuildMemberStatus,
    pub name: String,
    pub rank: u32,
    pub level: u8,
    pub class: Class,
    pub gender: Gender,
    pub area: Area,
    pub public_note: String,
    pub officer_note: String,
}

impl GuildMember {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // status: GuildMemberStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // rank: u32
        w.write_all(&self.rank.to_le_bytes())?;

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        match &self.status {
            GuildMember_GuildMemberStatus::Offline {
                time_offline,
            } => {
                // time_offline: f32
                w.write_all(&time_offline.to_le_bytes())?;

            }
            GuildMember_GuildMemberStatus::Online => {}
        }

        // public_note: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.public_note.as_bytes().iter().rev().next(), Some(&0_u8), "String `public_note` must not be null-terminated.");
        w.write_all(self.public_note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // officer_note: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.officer_note.as_bytes().iter().rev().next(), Some(&0_u8), "String `officer_note` must not be null-terminated.");
        w.write_all(self.officer_note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl GuildMember {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        // status: GuildMemberStatus
        let status: GuildMemberStatus = crate::util::read_u8_le(r)?.try_into()?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // rank: u32
        let rank = crate::util::read_u32_le(r)?;

        // level: u8
        let level = crate::util::read_u8_le(r)?;

        // class: Class
        let class: Class = crate::util::read_u8_le(r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        let status_if = match status {
            GuildMemberStatus::Offline => {
                // time_offline: f32
                let time_offline = crate::util::read_f32_le(r)?;
                GuildMember_GuildMemberStatus::Offline {
                    time_offline,
                }
            }
            GuildMemberStatus::Online => GuildMember_GuildMemberStatus::Online,
        };

        // public_note: CString
        let public_note = crate::util::read_c_string_to_vec(r)?;
        let public_note = String::from_utf8(public_note)?;

        // officer_note: CString
        let officer_note = crate::util::read_c_string_to_vec(r)?;
        let officer_note = String::from_utf8(officer_note)?;

        Ok(Self {
            guid,
            unknown,
            status: status_if,
            name,
            rank,
            level,
            class,
            gender,
            area,
            public_note,
            officer_note,
        })
    }

}

impl GuildMember {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // unknown: u32
        + self.status.size() // status: GuildMember_GuildMemberStatus
        + self.name.len() + 1 // name: CString
        + 4 // rank: u32
        + 1 // level: u8
        + 1 // class: Class
        + 1 // gender: Gender
        + 4 // area: Area
        + self.public_note.len() + 1 // public_note: CString
        + self.officer_note.len() + 1 // officer_note: CString
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuildMember_GuildMemberStatus {
    Offline {
        time_offline: f32,
    },
    Online,
}

impl Default for GuildMember_GuildMemberStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Online
    }
}

impl GuildMember_GuildMemberStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Offline { .. } => 0,
            Self::Online => 1,
        }
    }

}

impl GuildMember_GuildMemberStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Offline {
                time_offline,
            } => {
                1
                + 4 // time_offline: f32
            }
            Self::Online => {
                1
            }
        }
    }
}

