use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::GuildMember;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L24):
/// ```text
/// smsg SMSG_GUILD_ROSTER = 0x008A {
///     u32 amount_of_members;
///     CString motd;
///     CString guild_info;
///     u32 amount_of_rights;
///     u32[amount_of_rights] rights;
///     GuildMember[amount_of_members] members;
/// }
/// ```
pub struct SMSG_GUILD_ROSTER {
    pub motd: String,
    pub guild_info: String,
    pub rights: Vec<u32>,
    pub members: Vec<GuildMember>,
}

impl crate::Message for SMSG_GUILD_ROSTER {
    const OPCODE: u32 = 0x008a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
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

        // rights: u32[amount_of_rights]
        for i in self.rights.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // members: GuildMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(r)?;

        // motd: CString
        let motd = crate::util::read_c_string_to_vec(r)?;
        let motd = String::from_utf8(motd)?;

        // guild_info: CString
        let guild_info = crate::util::read_c_string_to_vec(r)?;
        let guild_info = String::from_utf8(guild_info)?;

        // amount_of_rights: u32
        let amount_of_rights = crate::util::read_u32_le(r)?;

        // rights: u32[amount_of_rights]
        let mut rights = Vec::with_capacity(amount_of_rights as usize);
        for i in 0..amount_of_rights {
            rights.push(crate::util::read_u32_le(r)?);
        }

        // members: GuildMember[amount_of_members]
        let mut members = Vec::with_capacity(amount_of_members as usize);
        for i in 0..amount_of_members {
            members.push(GuildMember::read(r)?);
        }

        Ok(Self {
            motd,
            guild_info,
            rights,
            members,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GUILD_ROSTER {}

impl SMSG_GUILD_ROSTER {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_members: u32
        + self.motd.len() + 1 // motd: CString
        + self.guild_info.len() + 1 // guild_info: CString
        + 4 // amount_of_rights: u32
        + self.rights.len() * core::mem::size_of::<u32>() // rights: u32[amount_of_rights]
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: GuildMember[amount_of_members]
    }
}

