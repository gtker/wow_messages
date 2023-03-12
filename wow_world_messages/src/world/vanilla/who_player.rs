use std::io::{Read, Write};
use crate::vanilla::Area;
use crate::vanilla::Class;
use crate::vanilla::Race;
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_who.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_who.wowm#L1):
/// ```text
/// struct WhoPlayer {
///     CString name;
///     CString guild;
///     Level32 level;
///     Class class;
///     Race race;
///     Area area;
///     u32 party_status;
/// }
/// ```
pub struct WhoPlayer {
    pub name: String,
    pub guild: String,
    pub level: Level,
    pub class: Class,
    pub race: Race,
    pub area: Area,
    pub party_status: u32,
}

impl WhoPlayer {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild` must not be null-terminated.");
        w.write_all(self.guild.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // level: Level32
        w.write_all(&u32::from(self.level.as_int()).to_le_bytes())?;

        // class: Class
        w.write_all(&u8::from(self.class.as_int()).to_le_bytes())?;

        // race: Race
        w.write_all(&u8::from(self.race.as_int()).to_le_bytes())?;

        // area: Area
        w.write_all(&u32::from(self.area.as_int()).to_le_bytes())?;

        // party_status: u32
        w.write_all(&self.party_status.to_le_bytes())?;

        Ok(())
    }
}

impl WhoPlayer {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // guild: CString
        let guild = {
            let guild = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild)?
        };

        // level: Level32
        let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // class: Class
        let class: Class = crate::util::read_u8_le(&mut r)?.try_into()?;

        // race: Race
        let race: Race = crate::util::read_u8_le(&mut r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // party_status: u32
        let party_status = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            name,
            guild,
            level,
            class,
            race,
            area,
            party_status,
        })
    }

}

impl WhoPlayer {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + self.guild.len() + 1 // guild: CString
        + 4 // level: Level32
        + 1 // class: Class
        + 1 // race: Race
        + 4 // area: Area
        + 4 // party_status: u32
    }
}

