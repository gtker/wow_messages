use std::io::{Read, Write};

use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_who.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_who.wowm#L3):
/// ```text
/// cmsg CMSG_WHO = 0x0062 {
///     Level32 minimum_level;
///     Level32 maximum_level;
///     CString player_name;
///     CString guild_name;
///     u32 race_mask;
///     u32 class_mask;
///     u32 amount_of_zones;
///     u32[amount_of_zones] zones;
///     u32 amount_of_strings;
///     CString[amount_of_strings] search_strings;
/// }
/// ```
pub struct CMSG_WHO {
    pub minimum_level: Level,
    pub maximum_level: Level,
    pub player_name: String,
    pub guild_name: String,
    pub race_mask: u32,
    pub class_mask: u32,
    pub zones: Vec<u32>,
    pub search_strings: Vec<String>,
}

impl crate::private::Sealed for CMSG_WHO {}
impl crate::Message for CMSG_WHO {
    const OPCODE: u32 = 0x0062;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // minimum_level: Level32
        w.write_all(&u32::from(self.minimum_level.as_int()).to_le_bytes())?;

        // maximum_level: Level32
        w.write_all(&u32::from(self.maximum_level.as_int()).to_le_bytes())?;

        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_name` must not be null-terminated.");
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race_mask: u32
        w.write_all(&self.race_mask.to_le_bytes())?;

        // class_mask: u32
        w.write_all(&self.class_mask.to_le_bytes())?;

        // amount_of_zones: u32
        w.write_all(&(self.zones.len() as u32).to_le_bytes())?;

        // zones: u32[amount_of_zones]
        for i in self.zones.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_strings: u32
        w.write_all(&(self.search_strings.len() as u32).to_le_bytes())?;

        // search_strings: CString[amount_of_strings]
        for i in self.search_strings.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(26..=10240).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0062, size: body_size as u32 });
        }

        // minimum_level: Level32
        let minimum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // maximum_level: Level32
        let maximum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // player_name: CString
        let player_name = {
            let player_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player_name)?
        };

        // guild_name: CString
        let guild_name = {
            let guild_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_name)?
        };

        // race_mask: u32
        let race_mask = crate::util::read_u32_le(&mut r)?;

        // class_mask: u32
        let class_mask = crate::util::read_u32_le(&mut r)?;

        // amount_of_zones: u32
        let amount_of_zones = crate::util::read_u32_le(&mut r)?;

        // zones: u32[amount_of_zones]
        let zones = {
            let mut zones = Vec::with_capacity(amount_of_zones as usize);
            for i in 0..amount_of_zones {
                zones.push(crate::util::read_u32_le(&mut r)?);
            }
            zones
        };

        // amount_of_strings: u32
        let amount_of_strings = crate::util::read_u32_le(&mut r)?;

        // search_strings: CString[amount_of_strings]
        let search_strings = {
            let mut search_strings = Vec::with_capacity(amount_of_strings as usize);
            for i in 0..amount_of_strings {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                search_strings.push(String::from_utf8(s)?);
            }
            search_strings
        };

        Ok(Self {
            minimum_level,
            maximum_level,
            player_name,
            guild_name,
            race_mask,
            class_mask,
            zones,
            search_strings,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_WHO {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_WHO {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_WHO {}

impl CMSG_WHO {
    pub(crate) fn size(&self) -> usize {
        4 // minimum_level: Level32
        + 4 // maximum_level: Level32
        + self.player_name.len() + 1 // player_name: CString
        + self.guild_name.len() + 1 // guild_name: CString
        + 4 // race_mask: u32
        + 4 // class_mask: u32
        + 4 // amount_of_zones: u32
        + self.zones.len() * core::mem::size_of::<u32>() // zones: u32[amount_of_zones]
        + 4 // amount_of_strings: u32
        + self.search_strings.iter().fold(0, |acc, x| acc + x.len() + 1) // search_strings: CString[amount_of_strings]
    }
}

