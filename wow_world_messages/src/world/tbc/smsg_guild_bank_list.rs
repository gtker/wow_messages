use std::io::{Read, Write};

use crate::tbc::{
    GuildBankSlot, GuildBankSocket, GuildBankTab, GuildBankTabResult, VariableItemRandomProperty,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm:42`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm#L42):
/// ```text
/// smsg SMSG_GUILD_BANK_LIST = 0x03E7 {
///     u64 bank_balance;
///     u8 tab_id;
///     u32 amount_of_allowed_item_withdraws;
///     GuildBankTabResult tab_result;
///     if (tab_result == PRESENT) {
///         u8 amount_of_bank_tabs;
///         GuildBankTab[amount_of_bank_tabs] tabs;
///     }
///     u8 amount_of_slot_updates;
///     GuildBankSlot[amount_of_slot_updates] slot_updates;
/// }
/// ```
pub struct SMSG_GUILD_BANK_LIST {
    pub bank_balance: u64,
    pub tab_id: u8,
    pub amount_of_allowed_item_withdraws: u32,
    pub tab_result: SMSG_GUILD_BANK_LIST_GuildBankTabResult,
    pub slot_updates: Vec<GuildBankSlot>,
}

impl crate::private::Sealed for SMSG_GUILD_BANK_LIST {}
impl SMSG_GUILD_BANK_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(15..=463888).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x03E7, size: body_size });
        }

        // bank_balance: u64
        let bank_balance = crate::util::read_u64_le(&mut r)?;

        // tab_id: u8
        let tab_id = crate::util::read_u8_le(&mut r)?;

        // amount_of_allowed_item_withdraws: u32
        let amount_of_allowed_item_withdraws = crate::util::read_u32_le(&mut r)?;

        // tab_result: GuildBankTabResult
        let tab_result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let tab_result_if = match tab_result {
            GuildBankTabResult::NotPresent => SMSG_GUILD_BANK_LIST_GuildBankTabResult::NotPresent,
            GuildBankTabResult::Present => {
                // amount_of_bank_tabs: u8
                let amount_of_bank_tabs = crate::util::read_u8_le(&mut r)?;

                // tabs: GuildBankTab[amount_of_bank_tabs]
                let tabs = {
                    let mut tabs = Vec::with_capacity(amount_of_bank_tabs as usize);
                    for _ in 0..amount_of_bank_tabs {
                        tabs.push(GuildBankTab::read(&mut r)?);
                    }
                    tabs
                };

                SMSG_GUILD_BANK_LIST_GuildBankTabResult::Present {
                    tabs,
                }
            }
        };

        // amount_of_slot_updates: u8
        let amount_of_slot_updates = crate::util::read_u8_le(&mut r)?;

        // slot_updates: GuildBankSlot[amount_of_slot_updates]
        let slot_updates = {
            let mut slot_updates = Vec::with_capacity(amount_of_slot_updates as usize);
            for _ in 0..amount_of_slot_updates {
                slot_updates.push(GuildBankSlot::read(&mut r)?);
            }
            slot_updates
        };

        Ok(Self {
            bank_balance,
            tab_id,
            amount_of_allowed_item_withdraws,
            tab_result: tab_result_if,
            slot_updates,
        })
    }

}

impl crate::Message for SMSG_GUILD_BANK_LIST {
    const OPCODE: u32 = 0x03e7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_BANK_LIST {{").unwrap();
        // Members
        writeln!(s, "    bank_balance = {};", self.bank_balance).unwrap();
        writeln!(s, "    tab_id = {};", self.tab_id).unwrap();
        writeln!(s, "    amount_of_allowed_item_withdraws = {};", self.amount_of_allowed_item_withdraws).unwrap();
        writeln!(s, "    tab_result = {};", GuildBankTabResult::try_from(self.tab_result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.tab_result {
            crate::tbc::SMSG_GUILD_BANK_LIST_GuildBankTabResult::Present {
                tabs,
            } => {
                writeln!(s, "    amount_of_bank_tabs = {};", tabs.len()).unwrap();
                write!(s, "    tabs = [").unwrap();
                for v in tabs.as_slice() {
                    writeln!(s, "{{").unwrap();
                    // Members
                    writeln!(s, "        tab_name = \"{}\";", v.tab_name).unwrap();
                    writeln!(s, "        tab_icon = \"{}\";", v.tab_icon).unwrap();

                    writeln!(s, "    }},").unwrap();
                }
                writeln!(s, "];").unwrap();
            }
            _ => {}
        }

        writeln!(s, "    amount_of_slot_updates = {};", self.slot_updates.len()).unwrap();
        write!(s, "    slot_updates = [").unwrap();
        for v in self.slot_updates.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        slot = {};", v.slot).unwrap();
            writeln!(s, "        item = {};", v.item).unwrap();
            panic!("unsupported type for test case printing: 'VariableItemRandomProperty' for variable 'item_random_property_id'");
            writeln!(s, "        amount_of_items = {};", v.amount_of_items).unwrap();
            writeln!(s, "        enchant = {};", v.enchant).unwrap();
            writeln!(s, "        charges = {};", v.charges).unwrap();
            writeln!(s, "        amount_of_sockets = {};", v.sockets.len()).unwrap();
            write!(s, "        sockets = [").unwrap();
            for v in v.sockets.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "            socket_index = {};", v.socket_index).unwrap();
                writeln!(s, "            gem = {};", v.gem).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 999_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "bank_balance", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_allowed_item_withdraws", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab_result", "    ");
        match &self.tab_result {
            crate::tbc::SMSG_GUILD_BANK_LIST_GuildBankTabResult::Present {
                tabs,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_bank_tabs", "    ");
                if !tabs.is_empty() {
                    writeln!(s, "    /* tabs: GuildBankTab[amount_of_bank_tabs] start */").unwrap();
                    for (i, v) in tabs.iter().enumerate() {
                        writeln!(s, "    /* tabs: GuildBankTab[amount_of_bank_tabs] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, v.tab_name.len() + 1, "tab_name", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, v.tab_icon.len() + 1, "tab_icon", "        ");
                        writeln!(s, "    /* tabs: GuildBankTab[amount_of_bank_tabs] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* tabs: GuildBankTab[amount_of_bank_tabs] end */").unwrap();
                }
            }
            _ => {}
        }

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_slot_updates", "    ");
        if !self.slot_updates.is_empty() {
            writeln!(s, "    /* slot_updates: GuildBankSlot[amount_of_slot_updates] start */").unwrap();
            for (i, v) in self.slot_updates.iter().enumerate() {
                writeln!(s, "    /* slot_updates: GuildBankSlot[amount_of_slot_updates] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                panic!("unsupported type VariableItemRandomProperty for variable 'item_random_property_id'");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_items", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "enchant", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "charges", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_sockets", "        ");
                if !v.sockets.is_empty() {
                    writeln!(s, "    /* sockets: GuildBankSocket[amount_of_sockets] start */").unwrap();
                    for (i, v) in v.sockets.iter().enumerate() {
                        writeln!(s, "    /* sockets: GuildBankSocket[amount_of_sockets] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "socket_index", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "gem", "            ");
                        writeln!(s, "    /* sockets: GuildBankSocket[amount_of_sockets] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* sockets: GuildBankSocket[amount_of_sockets] end */").unwrap();
                }
                writeln!(s, "    /* slot_updates: GuildBankSlot[amount_of_slot_updates] {i} end */").unwrap();
            }
            writeln!(s, "    /* slot_updates: GuildBankSlot[amount_of_slot_updates] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bank_balance: u64
        w.write_all(&self.bank_balance.to_le_bytes())?;

        // tab_id: u8
        w.write_all(&self.tab_id.to_le_bytes())?;

        // amount_of_allowed_item_withdraws: u32
        w.write_all(&self.amount_of_allowed_item_withdraws.to_le_bytes())?;

        // tab_result: GuildBankTabResult
        w.write_all(&(self.tab_result.as_int().to_le_bytes()))?;

        match &self.tab_result {
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
            _ => {}
        }

        // amount_of_slot_updates: u8
        w.write_all(&(self.slot_updates.len() as u8).to_le_bytes())?;

        // slot_updates: GuildBankSlot[amount_of_slot_updates]
        for i in self.slot_updates.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GUILD_BANK_LIST {}

impl SMSG_GUILD_BANK_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // bank_balance: u64
        + 1 // tab_id: u8
        + 4 // amount_of_allowed_item_withdraws: u32
        + self.tab_result.size() // tab_result: SMSG_GUILD_BANK_LIST_GuildBankTabResult
        + 1 // amount_of_slot_updates: u8
        + self.slot_updates.iter().fold(0, |acc, x| acc + x.size()) // slot_updates: GuildBankSlot[amount_of_slot_updates]
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

impl std::fmt::Display for SMSG_GUILD_BANK_LIST_GuildBankTabResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPresent => f.write_str("NotPresent"),
            Self::Present{ .. } => f.write_str("Present"),
        }
    }
}

impl SMSG_GUILD_BANK_LIST_GuildBankTabResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Present {
                tabs,
            } => {
                1
                + 1 // amount_of_bank_tabs: u8
                + tabs.iter().fold(0, |acc, x| acc + x.size()) // tabs: GuildBankTab[amount_of_bank_tabs]
            }
            _ => 1,
        }
    }
}

