use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::wrath::LfgRole;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_role_check_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_role_check_update.wowm#L1):
/// ```text
/// smsg SMSG_LFG_ROLE_CHECK_UPDATE = 0x0363 {
///     u32 rolecheck_state;
///     u8 rolecheck_initializing;
///     u8 amount_of_dungeon_entries;
///     u32[amount_of_dungeon_entries] dungeon_entries;
///     u8 amount_of_roles;
///     LfgRole[amount_of_roles] roles;
/// }
/// ```
pub struct SMSG_LFG_ROLE_CHECK_UPDATE {
    pub rolecheck_state: u32,
    pub rolecheck_initializing: u8,
    pub dungeon_entries: Vec<u32>,
    /// azerothcore: Leader info MUST be sent first.
    pub roles: Vec<LfgRole>,
}

impl crate::private::Sealed for SMSG_LFG_ROLE_CHECK_UPDATE {}
impl SMSG_LFG_ROLE_CHECK_UPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(7..=4615).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // rolecheck_state: u32
        let rolecheck_state = crate::util::read_u32_le(&mut r)?;

        // rolecheck_initializing: u8
        let rolecheck_initializing = crate::util::read_u8_le(&mut r)?;

        // amount_of_dungeon_entries: u8
        let amount_of_dungeon_entries = crate::util::read_u8_le(&mut r)?;

        // dungeon_entries: u32[amount_of_dungeon_entries]
        let dungeon_entries = {
            let mut dungeon_entries = Vec::with_capacity(amount_of_dungeon_entries as usize);
            for _ in 0..amount_of_dungeon_entries {
                dungeon_entries.push(crate::util::read_u32_le(&mut r)?);
            }
            dungeon_entries
        };

        // amount_of_roles: u8
        let amount_of_roles = crate::util::read_u8_le(&mut r)?;

        // roles: LfgRole[amount_of_roles]
        let roles = {
            let mut roles = Vec::with_capacity(amount_of_roles as usize);
            for _ in 0..amount_of_roles {
                roles.push(LfgRole::read(&mut r)?);
            }
            roles
        };

        Ok(Self {
            rolecheck_state,
            rolecheck_initializing,
            dungeon_entries,
            roles,
        })
    }

}

impl crate::Message for SMSG_LFG_ROLE_CHECK_UPDATE {
    const OPCODE: u32 = 0x0363;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LFG_ROLE_CHECK_UPDATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_ROLE_CHECK_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    rolecheck_state = {};", self.rolecheck_state).unwrap();
        writeln!(s, "    rolecheck_initializing = {};", self.rolecheck_initializing).unwrap();
        writeln!(s, "    amount_of_dungeon_entries = {};", self.dungeon_entries.len()).unwrap();
        write!(s, "    dungeon_entries = [").unwrap();
        for v in self.dungeon_entries.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_roles = {};", self.roles.len()).unwrap();
        write!(s, "    roles = [").unwrap();
        for v in self.roles.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "        ready = {};", if v.ready { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "        roles = {};", v.roles).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 867_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "rolecheck_state", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "rolecheck_initializing", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_dungeon_entries", "    ");
        if !self.dungeon_entries.is_empty() {
            writeln!(s, "    /* dungeon_entries: u32[amount_of_dungeon_entries] start */").unwrap();
            for (i, v) in self.dungeon_entries.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("dungeon_entries {i}"), "    ");
            }
            writeln!(s, "    /* dungeon_entries: u32[amount_of_dungeon_entries] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_roles", "    ");
        if !self.roles.is_empty() {
            writeln!(s, "    /* roles: LfgRole[amount_of_roles] start */").unwrap();
            for (i, v) in self.roles.iter().enumerate() {
                writeln!(s, "    /* roles: LfgRole[amount_of_roles] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "ready", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "roles", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
                writeln!(s, "    /* roles: LfgRole[amount_of_roles] {i} end */").unwrap();
            }
            writeln!(s, "    /* roles: LfgRole[amount_of_roles] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // rolecheck_state: u32
        w.write_all(&self.rolecheck_state.to_le_bytes())?;

        // rolecheck_initializing: u8
        w.write_all(&self.rolecheck_initializing.to_le_bytes())?;

        // amount_of_dungeon_entries: u8
        w.write_all(&(self.dungeon_entries.len() as u8).to_le_bytes())?;

        // dungeon_entries: u32[amount_of_dungeon_entries]
        for i in self.dungeon_entries.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_roles: u8
        w.write_all(&(self.roles.len() as u8).to_le_bytes())?;

        // roles: LfgRole[amount_of_roles]
        for i in self.roles.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(867, "SMSG_LFG_ROLE_CHECK_UPDATE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_ROLE_CHECK_UPDATE {}

impl SMSG_LFG_ROLE_CHECK_UPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // rolecheck_state: u32
        + 1 // rolecheck_initializing: u8
        + 1 // amount_of_dungeon_entries: u8
        + self.dungeon_entries.len() * core::mem::size_of::<u32>() // dungeon_entries: u32[amount_of_dungeon_entries]
        + 1 // amount_of_roles: u8
        + self.roles.len() * 14 // roles: LfgRole[amount_of_roles]
    }
}

