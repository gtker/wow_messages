use std::convert::{TryFrom, TryInto};
use crate::world::shared::guild_bank_rights_tbc_wrath::GuildBankRights;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm#L16):
/// ```text
/// cmsg CMSG_GUILD_RANK = 0x0231 {
///     u32 rank_id;
///     u32 rights;
///     CString rank_name;
///     u32 money_per_day;
///     GuildBankRights[6] bank_tab_rights;
/// }
/// ```
pub struct CMSG_GUILD_RANK {
    pub rank_id: u32,
    pub rights: u32,
    pub rank_name: String,
    pub money_per_day: u32,
    pub bank_tab_rights: [GuildBankRights; 6],
}

impl crate::Message for CMSG_GUILD_RANK {
    const OPCODE: u32 = 0x0231;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // rank_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.rank_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `rank_name` must not be null-terminated.");
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // money_per_day: u32
        w.write_all(&self.money_per_day.to_le_bytes())?;

        // bank_tab_rights: GuildBankRights[6]
        for i in self.bank_tab_rights.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(61..=316).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0231, size: body_size as u32 });
        }

        // rank_id: u32
        let rank_id = crate::util::read_u32_le(r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(r)?;

        // rank_name: CString
        let rank_name = crate::util::read_c_string_to_vec(r)?;
        let rank_name = String::from_utf8(rank_name)?;

        // money_per_day: u32
        let money_per_day = crate::util::read_u32_le(r)?;

        // bank_tab_rights: GuildBankRights[6]
        let mut bank_tab_rights = [GuildBankRights::default(); 6];
        for i in bank_tab_rights.iter_mut() {
            *i = GuildBankRights::read(r)?;
        }

        Ok(Self {
            rank_id,
            rights,
            rank_name,
            money_per_day,
            bank_tab_rights,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GUILD_RANK {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GUILD_RANK {}

impl CMSG_GUILD_RANK {
    pub(crate) fn size(&self) -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + self.rank_name.len() + 1 // rank_name: CString
        + 4 // money_per_day: u32
        + 6 * 8 // bank_tab_rights: GuildBankRights[6]
    }
}

