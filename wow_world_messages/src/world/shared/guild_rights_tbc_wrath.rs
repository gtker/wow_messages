use std::io::{Read, Write};

use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::shared::guild_bank_rights_tbc_wrath::GuildBankRights;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:75`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L75):
/// ```text
/// struct GuildRights {
///     u32 rights;
///     Gold money_per_day;
///     GuildBankRights[6] bank_tab_rights;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct GuildRights {
    pub rights: u32,
    pub money_per_day: Gold,
    pub bank_tab_rights: [GuildBankRights; 6],
}

impl GuildRights {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // money_per_day: Gold
        w.write_all((self.money_per_day.as_int()).to_le_bytes().as_slice())?;

        // bank_tab_rights: GuildBankRights[6]
        for i in self.bank_tab_rights.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl GuildRights {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // rights: u32
        let rights = crate::util::read_u32_le(&mut r)?;

        // money_per_day: Gold
        let money_per_day = Gold::new(crate::util::read_u32_le(&mut r)?);

        // bank_tab_rights: GuildBankRights[6]
        let bank_tab_rights = {
            let mut bank_tab_rights = [GuildBankRights::default(); 6];
            for i in bank_tab_rights.iter_mut() {
                *i = GuildBankRights::read(&mut r)?;
            }
            bank_tab_rights
        };

        Ok(Self {
            rights,
            money_per_day,
            bank_tab_rights,
        })
    }

}

