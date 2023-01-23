use std::convert::{TryFrom, TryInto};
use crate::world::wrath::LfgRole;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_role_check_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_role_check_update.wowm#L1):
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

impl crate::Message for SMSG_LFG_ROLE_CHECK_UPDATE {
    const OPCODE: u32 = 0x0363;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
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
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(7..=4615).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0363, size: body_size as u32 });
        }

        // rolecheck_state: u32
        let rolecheck_state = crate::util::read_u32_le(r)?;

        // rolecheck_initializing: u8
        let rolecheck_initializing = crate::util::read_u8_le(r)?;

        // amount_of_dungeon_entries: u8
        let amount_of_dungeon_entries = crate::util::read_u8_le(r)?;

        // dungeon_entries: u32[amount_of_dungeon_entries]
        let mut dungeon_entries = Vec::with_capacity(amount_of_dungeon_entries as usize);
        for i in 0..amount_of_dungeon_entries {
            dungeon_entries.push(crate::util::read_u32_le(r)?);
        }

        // amount_of_roles: u8
        let amount_of_roles = crate::util::read_u8_le(r)?;

        // roles: LfgRole[amount_of_roles]
        let mut roles = Vec::with_capacity(amount_of_roles as usize);
        for i in 0..amount_of_roles {
            roles.push(LfgRole::read(r)?);
        }

        Ok(Self {
            rolecheck_state,
            rolecheck_initializing,
            dungeon_entries,
            roles,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_LFG_ROLE_CHECK_UPDATE {}

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

