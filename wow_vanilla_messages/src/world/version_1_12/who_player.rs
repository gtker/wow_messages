use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::class::{Class, class_try_from};
use crate::world::version_1_12::race::{Race, race_try_from};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_who.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_who.wowm#L3):
/// ```text
/// struct WhoPlayer {
///     CString name;
///     CString guild;
///     u32 level;
///     Class class;
///     Race race;
///     u32 zone_id;
///     u32 party_status;
/// }
/// ```
pub struct WhoPlayer {
    pub name: String,
    pub guild: String,
    pub level: u32,
    pub class: Class,
    pub race: Race,
    pub zone_id: u32,
    pub party_status: u32,
}

impl WhoPlayer {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild: CString
        w.write_all(self.guild.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        // party_status: u32
        w.write_all(&self.party_status.to_le_bytes())?;

        Ok(())
    }
}

impl WhoPlayer {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // guild: CString
        let guild = crate::util::read_c_string_to_vec(r)?;
        let guild = String::from_utf8(guild)?;

        // level: u32
        let level = crate::util::read_u32_le(r)?;

        // class: Class
        let class: Class = class_try_from(crate::util::read_u8_le(r)?)?;

        // race: Race
        let race: Race = race_try_from(crate::util::read_u8_le(r)?)?;

        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        // party_status: u32
        let party_status = crate::util::read_u32_le(r)?;

        Ok(Self {
            name,
            guild,
            level,
            class,
            race,
            zone_id,
            party_status,
        })
    }

}

impl WhoPlayer {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + self.guild.len() + 1 // guild: CString
        + 4 // level: u32
        + 1 // class: Class
        + 1 // race: Race
        + 4 // zone_id: u32
        + 4 // party_status: u32
    }
}

