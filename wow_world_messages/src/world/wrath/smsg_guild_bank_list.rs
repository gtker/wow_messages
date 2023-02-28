use crate::wrath::GuildBankSlot;
use crate::wrath::GuildBankTab;
use crate::wrath::GuildBankContentResult;
use crate::wrath::GuildBankTabResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm:79`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm#L79):
/// ```text
/// smsg SMSG_GUILD_BANK_LIST = 0x03E8 {
///     u64 bank_balance;
///     u8 tab_id;
///     u32 amount_of_allowed_item_withdraws;
///     GuildBankTabResult tab_result;
///     if (tab_result == PRESENT) {
///         u8 amount_of_bank_tabs;
///         GuildBankTab[amount_of_bank_tabs] tabs;
///     }
///     GuildBankContentResult content_result;
///     if (content_result == PRESENT) {
///         u8 amount_of_slot_updates;
///         GuildBankSlot[amount_of_slot_updates] slot_updates;
///     }
/// }
/// ```
pub struct SMSG_GUILD_BANK_LIST {
    pub bank_balance: u64,
    pub tab_id: u8,
    pub amount_of_allowed_item_withdraws: u32,
    pub tab_result: SMSG_GUILD_BANK_LIST_GuildBankTabResult,
    pub content_result: SMSG_GUILD_BANK_LIST_GuildBankContentResult,
}

impl crate::Message for SMSG_GUILD_BANK_LIST {
    const OPCODE: u32 = 0x03e8;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // bank_balance: u64
        w.write_all(&self.bank_balance.to_le_bytes())?;

        // tab_id: u8
        w.write_all(&self.tab_id.to_le_bytes())?;

        // amount_of_allowed_item_withdraws: u32
        w.write_all(&self.amount_of_allowed_item_withdraws.to_le_bytes())?;

        // tab_result: GuildBankTabResult
        w.write_all(&u8::from(self.tab_result.as_int()).to_le_bytes())?;

        match &self.tab_result {
            SMSG_GUILD_BANK_LIST_GuildBankTabResult::NotPresent => {}
            SMSG_GUILD_BANK_LIST_GuildBankTabResult::Present {
                tabs,
            } => {
                // amount_of_bank_tabs: u8
                w.write_all(&(tabs.len() as u8).to_le_bytes())?;

                // tabs: GuildBankTab[amount_of_bank_tabs]
                for i in tabs.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
        }

        // content_result: GuildBankContentResult
        w.write_all(&u8::from(self.content_result.as_int()).to_le_bytes())?;

        match &self.content_result {
            SMSG_GUILD_BANK_LIST_GuildBankContentResult::NotPresent => {}
            SMSG_GUILD_BANK_LIST_GuildBankContentResult::Present {
                slot_updates,
            } => {
                // amount_of_slot_updates: u8
                w.write_all(&(slot_updates.len() as u8).to_le_bytes())?;

                // slot_updates: GuildBankSlot[amount_of_slot_updates]
                for i in slot_updates.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(15..=464657).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E8, size: body_size as u32 });
        }

        // bank_balance: u64
        let bank_balance = crate::util::read_u64_le(&mut r)?;

        // tab_id: u8
        let tab_id = crate::util::read_u8_le(&mut r)?;

        // amount_of_allowed_item_withdraws: u32
        let amount_of_allowed_item_withdraws = crate::util::read_u32_le(&mut r)?;

        // tab_result: GuildBankTabResult
        let tab_result: GuildBankTabResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        let tab_result_if = match tab_result {
            GuildBankTabResult::NotPresent => SMSG_GUILD_BANK_LIST_GuildBankTabResult::NotPresent,
            GuildBankTabResult::Present => {
                // amount_of_bank_tabs: u8
                let amount_of_bank_tabs = crate::util::read_u8_le(&mut r)?;

                // tabs: GuildBankTab[amount_of_bank_tabs]
                let tabs = {
                    let mut tabs = Vec::with_capacity(amount_of_bank_tabs as usize);
                    for i in 0..amount_of_bank_tabs {
                        tabs.push(GuildBankTab::read(&mut r)?);
                    }
                    tabs
                };

                SMSG_GUILD_BANK_LIST_GuildBankTabResult::Present {
                    tabs,
                }
            }
        };

        // content_result: GuildBankContentResult
        let content_result: GuildBankContentResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        let content_result_if = match content_result {
            GuildBankContentResult::NotPresent => SMSG_GUILD_BANK_LIST_GuildBankContentResult::NotPresent,
            GuildBankContentResult::Present => {
                // amount_of_slot_updates: u8
                let amount_of_slot_updates = crate::util::read_u8_le(&mut r)?;

                // slot_updates: GuildBankSlot[amount_of_slot_updates]
                let slot_updates = {
                    let mut slot_updates = Vec::with_capacity(amount_of_slot_updates as usize);
                    for i in 0..amount_of_slot_updates {
                        slot_updates.push(GuildBankSlot::read(&mut r)?);
                    }
                    slot_updates
                };

                SMSG_GUILD_BANK_LIST_GuildBankContentResult::Present {
                    slot_updates,
                }
            }
        };

        Ok(Self {
            bank_balance,
            tab_id,
            amount_of_allowed_item_withdraws,
            tab_result: tab_result_if,
            content_result: content_result_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GUILD_BANK_LIST {}

impl SMSG_GUILD_BANK_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // bank_balance: u64
        + 1 // tab_id: u8
        + 4 // amount_of_allowed_item_withdraws: u32
        + self.tab_result.size() // tab_result: SMSG_GUILD_BANK_LIST_GuildBankTabResult
        + self.content_result.size() // content_result: SMSG_GUILD_BANK_LIST_GuildBankContentResult
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_GUILD_BANK_LIST_GuildBankTabResult {
    NotPresent,
    Present {
        tabs: Vec<GuildBankTab>,
    },
}

impl Default for SMSG_GUILD_BANK_LIST_GuildBankTabResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl SMSG_GUILD_BANK_LIST_GuildBankTabResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl SMSG_GUILD_BANK_LIST_GuildBankTabResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotPresent => {
                1
            }
            Self::Present {
                tabs,
            } => {
                1
                + 1 // amount_of_bank_tabs: u8
                + tabs.iter().fold(0, |acc, x| acc + x.size()) // tabs: GuildBankTab[amount_of_bank_tabs]
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_GUILD_BANK_LIST_GuildBankContentResult {
    NotPresent,
    Present {
        slot_updates: Vec<GuildBankSlot>,
    },
}

impl Default for SMSG_GUILD_BANK_LIST_GuildBankContentResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl SMSG_GUILD_BANK_LIST_GuildBankContentResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl SMSG_GUILD_BANK_LIST_GuildBankContentResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotPresent => {
                1
            }
            Self::Present {
                slot_updates,
            } => {
                1
                + 1 // amount_of_slot_updates: u8
                + slot_updates.iter().fold(0, |acc, x| acc + x.size()) // slot_updates: GuildBankSlot[amount_of_slot_updates]
            }
        }
    }
}

