use std::convert::{TryFrom, TryInto};
use crate::world::shared::guild_bank_rights_tbc_wrath::GuildBankRights;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:77`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L77):
/// ```text
/// struct GuildRights {
///     u32 rights;
///     u32 money_per_day;
///     GuildBankRights[6] bank_tab_rights;
/// }
/// ```
pub struct GuildRights {
    pub rights: u32,
    pub money_per_day: u32,
    pub bank_tab_rights: [GuildBankRights; 6],
}

impl GuildRights {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // money_per_day: u32
        w.write_all(&self.money_per_day.to_le_bytes())?;

        // bank_tab_rights: GuildBankRights[6]
        for i in self.bank_tab_rights.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl GuildRights {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // rights: u32
        let rights = crate::util::read_u32_le(r)?;

        // money_per_day: u32
        let money_per_day = crate::util::read_u32_le(r)?;

        // bank_tab_rights: GuildBankRights[6]
        let mut bank_tab_rights = [GuildBankRights::default(); 6];
        for i in bank_tab_rights.iter_mut() {
            *i = GuildBankRights::read(r)?;
        }

        Ok(Self {
            rights,
            money_per_day,
            bank_tab_rights,
        })
    }

}

