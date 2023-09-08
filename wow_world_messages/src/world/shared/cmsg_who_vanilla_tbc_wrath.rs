use std::io::{Read, Write};

use crate::shared::level_vanilla_tbc_wrath::Level;

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
impl CMSG_WHO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(26..=10240).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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
            for _ in 0..amount_of_zones {
                zones.push(crate::util::read_u32_le(&mut r)?);
            }
            zones
        };

        // amount_of_strings: u32
        let amount_of_strings = crate::util::read_u32_le(&mut r)?;

        // search_strings: CString[amount_of_strings]
        let search_strings = {
            let mut search_strings = Vec::with_capacity(amount_of_strings as usize);
            for _ in 0..amount_of_strings {
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

impl crate::Message for CMSG_WHO {
    const OPCODE: u32 = 0x0062;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_WHO {{").unwrap();
        // Members
        writeln!(s, "    minimum_level = {};", self.minimum_level.as_int()).unwrap();
        writeln!(s, "    maximum_level = {};", self.maximum_level.as_int()).unwrap();
        writeln!(s, "    player_name = \"{}\";", self.player_name).unwrap();
        writeln!(s, "    guild_name = \"{}\";", self.guild_name).unwrap();
        writeln!(s, "    race_mask = {};", self.race_mask).unwrap();
        writeln!(s, "    class_mask = {};", self.class_mask).unwrap();
        writeln!(s, "    amount_of_zones = {};", self.zones.len()).unwrap();
        write!(s, "    zones = [").unwrap();
        for v in self.zones.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_strings = {};", self.search_strings.len()).unwrap();
        write!(s, "    search_strings = [").unwrap();
        for v in self.search_strings.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 98_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "minimum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "maximum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.player_name.len() + 1, "player_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.guild_name.len() + 1, "guild_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "race_mask", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "class_mask", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_zones", "    ");
        if !self.zones.is_empty() {
            writeln!(s, "    /* zones: u32[amount_of_zones] start */").unwrap();
            for (i, v) in self.zones.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("zones {i}"), "    ");
            }
            writeln!(s, "    /* zones: u32[amount_of_zones] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_strings", "    ");
        if !self.search_strings.is_empty() {
            writeln!(s, "    /* search_strings: CString[amount_of_strings] start */").unwrap();
            for (i, v) in self.search_strings.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("search_strings {i}"), "    ");
            }
            writeln!(s, "    /* search_strings: CString[amount_of_strings] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // minimum_level: Level32
        w.write_all(&u32::from(self.minimum_level.as_int()).to_le_bytes())?;

        // maximum_level: Level32
        w.write_all(&u32::from(self.maximum_level.as_int()).to_le_bytes())?;

        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().next_back(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_name.as_bytes().iter().next_back(), Some(&0_u8), "String `guild_name` must not be null-terminated.");
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(98, "CMSG_WHO", body_size, a))
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

