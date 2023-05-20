use std::io::{Read, Write};

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
    ///
    pub roles: Vec<LfgRole>,
}

impl crate::private::Sealed for SMSG_LFG_ROLE_CHECK_UPDATE {}
impl crate::Message for SMSG_LFG_ROLE_CHECK_UPDATE {
    const OPCODE: u32 = 0x0363;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(7..=4615).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0363, size: body_size });
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

