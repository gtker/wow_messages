use crate:: {
};
use crate::wrath:: {
    Gold,
};
use crate::wrath::BankTab;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_guild_permissions.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_guild_permissions.wowm#L26):
/// ```text
/// smsg MSG_GUILD_PERMISSIONS_Server = 0x03FD {
///     u32 id;
///     u32 rights;
///     Gold gold_limit_per_day;
///     u8 purchased_bank_tabs;
///     BankTab[6] bank_tabs;
/// }
/// ```
pub struct MSG_GUILD_PERMISSIONS_Server {
    pub id: u32,
    pub rights: u32,
    pub gold_limit_per_day: Gold,
    pub purchased_bank_tabs: u8,
    pub bank_tabs: [BankTab; 6],
}

impl crate::Message for MSG_GUILD_PERMISSIONS_Server {
    const OPCODE: u32 = 0x03fd;

    fn size_without_header(&self) -> u32 {
        61
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // gold_limit_per_day: Gold
        w.write_all(u32::from(self.gold_limit_per_day.as_int()).to_le_bytes().as_slice())?;

        // purchased_bank_tabs: u8
        w.write_all(&self.purchased_bank_tabs.to_le_bytes())?;

        // bank_tabs: BankTab[6]
        for i in self.bank_tabs.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 61 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FD, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(&mut r)?;

        // gold_limit_per_day: Gold
        let gold_limit_per_day = Gold::new(crate::util::read_u32_le(&mut r)?);

        // purchased_bank_tabs: u8
        let purchased_bank_tabs = crate::util::read_u8_le(&mut r)?;

        // bank_tabs: BankTab[6]
        let bank_tabs = {
            let mut bank_tabs = [BankTab::default(); 6];
            for i in bank_tabs.iter_mut() {
                *i = BankTab::read(&mut r)?;
            }
            bank_tabs
        };

        Ok(Self {
            id,
            rights,
            gold_limit_per_day,
            purchased_bank_tabs,
            bank_tabs,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_GUILD_PERMISSIONS_Server {}

