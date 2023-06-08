use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    BankSwapSource, BankSwapStoreMode,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_swap_items.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_swap_items.wowm#L15):
/// ```text
/// cmsg CMSG_GUILD_BANK_SWAP_ITEMS = 0x03E8 {
///     Guid bank;
///     BankSwapSource source;
///     if (source == BANK) {
///         u8 bank_destination_tab;
///         u8 bank_destination_slot;
///         u32 unknown1;
///         u8 bank_source_tab;
///         u8 bank_source_slot;
///         u32 item1;
///         u8 unknown2;
///         u8 amount;
///     }
///     else {
///         u8 bank_tab;
///         u8 bank_slot;
///         u32 item2;
///         BankSwapStoreMode mode;
///         if (mode == AUTOMATIC) {
///             u32 auto_count;
///             u8 unknown3;
///             u8 unknown4;
///         }
///         else {
///             u8 player_bag;
///             u8 player_bag_slot;
///             Bool bank_to_character_transfer;
///             u8 split_amount;
///         }
///     }
///     u8[-] unknown5;
/// }
/// ```
pub struct CMSG_GUILD_BANK_SWAP_ITEMS {
    pub bank: Guid,
    pub source: CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource,
    /// cmangos-tbc/mangosone has extra
    pub unknown5: Vec<u8>,
}

impl crate::private::Sealed for CMSG_GUILD_BANK_SWAP_ITEMS {}
impl crate::Message for CMSG_GUILD_BANK_SWAP_ITEMS {
    const OPCODE: u32 = 0x03e8;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_BANK_SWAP_ITEMS {{").unwrap();
        // Members
        writeln!(s, "    bank = {};", self.bank.guid()).unwrap();
        writeln!(s, "    source = {};", crate::tbc::BankSwapSource::try_from(self.source.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.source {
            crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Inventory {
                bank_slot,
                bank_tab,
                item2,
                mode,
            } => {
                writeln!(s, "    bank_tab = {};", bank_tab).unwrap();
                writeln!(s, "    bank_slot = {};", bank_slot).unwrap();
                writeln!(s, "    item2 = {};", item2).unwrap();
                writeln!(s, "    mode = {};", crate::tbc::BankSwapStoreMode::try_from(mode.as_int()).unwrap().as_test_case_value()).unwrap();
                match &mode {
                    crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Manual {
                        bank_to_character_transfer,
                        player_bag,
                        player_bag_slot,
                        split_amount,
                    } => {
                        writeln!(s, "    player_bag = {};", player_bag).unwrap();
                        writeln!(s, "    player_bag_slot = {};", player_bag_slot).unwrap();
                        writeln!(s, "    bank_to_character_transfer = {};", if *bank_to_character_transfer { "TRUE" } else { "FALSE" }).unwrap();
                        writeln!(s, "    split_amount = {};", split_amount).unwrap();
                    }
                    crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Automatic {
                        auto_count,
                        unknown3,
                        unknown4,
                    } => {
                        writeln!(s, "    auto_count = {};", auto_count).unwrap();
                        writeln!(s, "    unknown3 = {};", unknown3).unwrap();
                        writeln!(s, "    unknown4 = {};", unknown4).unwrap();
                    }
                }

            }
            crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Bank {
                amount,
                bank_destination_slot,
                bank_destination_tab,
                bank_source_slot,
                bank_source_tab,
                item1,
                unknown1,
                unknown2,
            } => {
                writeln!(s, "    bank_destination_tab = {};", bank_destination_tab).unwrap();
                writeln!(s, "    bank_destination_slot = {};", bank_destination_slot).unwrap();
                writeln!(s, "    unknown1 = {};", unknown1).unwrap();
                writeln!(s, "    bank_source_tab = {};", bank_source_tab).unwrap();
                writeln!(s, "    bank_source_slot = {};", bank_source_slot).unwrap();
                writeln!(s, "    item1 = {};", item1).unwrap();
                writeln!(s, "    unknown2 = {};", unknown2).unwrap();
                writeln!(s, "    amount = {};", amount).unwrap();
            }
        }

        write!(s, "    unknown5 = [").unwrap();
        for v in self.unknown5.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1000_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "bank", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "source", "    ");
        match &self.source {
            crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Inventory {
                bank_slot,
                bank_tab,
                item2,
                mode,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bank_tab", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bank_slot", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "mode", "    ");
                match &mode {
                    crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Manual {
                        bank_to_character_transfer,
                        player_bag,
                        player_bag_slot,
                        split_amount,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "player_bag", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "player_bag_slot", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "bank_to_character_transfer", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "split_amount", "    ");
                    }
                    crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Automatic {
                        auto_count,
                        unknown3,
                        unknown4,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "auto_count", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown3", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown4", "    ");
                    }
                }

            }
            crate::tbc::CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Bank {
                amount,
                bank_destination_slot,
                bank_destination_tab,
                bank_source_slot,
                bank_source_tab,
                item1,
                unknown1,
                unknown2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bank_destination_tab", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bank_destination_slot", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bank_source_tab", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bank_source_slot", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount", "    ");
            }
        }

        crate::util::write_bytes(&mut s, &mut bytes, self.unknown5.len(), "unknown5", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // source: BankSwapSource
        w.write_all(&(self.source.as_int().to_le_bytes()))?;

        match &self.source {
            CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Inventory {
                bank_slot,
                bank_tab,
                item2,
                mode,
            } => {
                // bank_tab: u8
                w.write_all(&bank_tab.to_le_bytes())?;

                // bank_slot: u8
                w.write_all(&bank_slot.to_le_bytes())?;

                // item2: u32
                w.write_all(&item2.to_le_bytes())?;

                // mode: BankSwapStoreMode
                w.write_all(&(mode.as_int().to_le_bytes()))?;

                match &mode {
                    CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Manual {
                        bank_to_character_transfer,
                        player_bag,
                        player_bag_slot,
                        split_amount,
                    } => {
                        // player_bag: u8
                        w.write_all(&player_bag.to_le_bytes())?;

                        // player_bag_slot: u8
                        w.write_all(&player_bag_slot.to_le_bytes())?;

                        // bank_to_character_transfer: Bool
                        w.write_all(u8::from(*bank_to_character_transfer).to_le_bytes().as_slice())?;

                        // split_amount: u8
                        w.write_all(&split_amount.to_le_bytes())?;

                    }
                    CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Automatic {
                        auto_count,
                        unknown3,
                        unknown4,
                    } => {
                        // auto_count: u32
                        w.write_all(&auto_count.to_le_bytes())?;

                        // unknown3: u8
                        w.write_all(&unknown3.to_le_bytes())?;

                        // unknown4: u8
                        w.write_all(&unknown4.to_le_bytes())?;

                    }
                }

            }
            CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Bank {
                amount,
                bank_destination_slot,
                bank_destination_tab,
                bank_source_slot,
                bank_source_tab,
                item1,
                unknown1,
                unknown2,
            } => {
                // bank_destination_tab: u8
                w.write_all(&bank_destination_tab.to_le_bytes())?;

                // bank_destination_slot: u8
                w.write_all(&bank_destination_slot.to_le_bytes())?;

                // unknown1: u32
                w.write_all(&unknown1.to_le_bytes())?;

                // bank_source_tab: u8
                w.write_all(&bank_source_tab.to_le_bytes())?;

                // bank_source_slot: u8
                w.write_all(&bank_source_slot.to_le_bytes())?;

                // item1: u32
                w.write_all(&item1.to_le_bytes())?;

                // unknown2: u8
                w.write_all(&unknown2.to_le_bytes())?;

                // amount: u8
                w.write_all(&amount.to_le_bytes())?;

            }
        }

        // unknown5: u8[-]
        for i in self.unknown5.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(20..=65558).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E8, size: body_size });
        }

        // bank: Guid
        let bank = crate::util::read_guid(&mut r)?;

        // source: BankSwapSource
        let source = crate::util::read_u8_le(&mut r)?.try_into()?;

        let source_if = match source {
            BankSwapSource::Inventory => {
                // bank_tab: u8
                let bank_tab = crate::util::read_u8_le(&mut r)?;

                // bank_slot: u8
                let bank_slot = crate::util::read_u8_le(&mut r)?;

                // item2: u32
                let item2 = crate::util::read_u32_le(&mut r)?;

                // mode: BankSwapStoreMode
                let mode = crate::util::read_u8_le(&mut r)?.try_into()?;

                let mode_if = match mode {
                    BankSwapStoreMode::Manual => {
                        // player_bag: u8
                        let player_bag = crate::util::read_u8_le(&mut r)?;

                        // player_bag_slot: u8
                        let player_bag_slot = crate::util::read_u8_le(&mut r)?;

                        // bank_to_character_transfer: Bool
                        let bank_to_character_transfer = crate::util::read_u8_le(&mut r)? != 0;

                        // split_amount: u8
                        let split_amount = crate::util::read_u8_le(&mut r)?;

                        CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Manual {
                            bank_to_character_transfer,
                            player_bag,
                            player_bag_slot,
                            split_amount,
                        }
                    }
                    BankSwapStoreMode::Automatic => {
                        // auto_count: u32
                        let auto_count = crate::util::read_u32_le(&mut r)?;

                        // unknown3: u8
                        let unknown3 = crate::util::read_u8_le(&mut r)?;

                        // unknown4: u8
                        let unknown4 = crate::util::read_u8_le(&mut r)?;

                        CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode::Automatic {
                            auto_count,
                            unknown3,
                            unknown4,
                        }
                    }
                };

                CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Inventory {
                    bank_slot,
                    bank_tab,
                    item2,
                    mode: mode_if,
                }
            }
            BankSwapSource::Bank => {
                // bank_destination_tab: u8
                let bank_destination_tab = crate::util::read_u8_le(&mut r)?;

                // bank_destination_slot: u8
                let bank_destination_slot = crate::util::read_u8_le(&mut r)?;

                // unknown1: u32
                let unknown1 = crate::util::read_u32_le(&mut r)?;

                // bank_source_tab: u8
                let bank_source_tab = crate::util::read_u8_le(&mut r)?;

                // bank_source_slot: u8
                let bank_source_slot = crate::util::read_u8_le(&mut r)?;

                // item1: u32
                let item1 = crate::util::read_u32_le(&mut r)?;

                // unknown2: u8
                let unknown2 = crate::util::read_u8_le(&mut r)?;

                // amount: u8
                let amount = crate::util::read_u8_le(&mut r)?;

                CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource::Bank {
                    amount,
                    bank_destination_slot,
                    bank_destination_tab,
                    bank_source_slot,
                    bank_source_tab,
                    item1,
                    unknown1,
                    unknown2,
                }
            }
        };

        // unknown5: u8[-]
        let unknown5 = {
            let mut current_size = {
                8 // bank: Guid
                + 1 // source: CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource
            };
            let mut unknown5 = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                unknown5.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            unknown5
        };

        Ok(Self {
            bank,
            source: source_if,
            unknown5,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_BANK_SWAP_ITEMS {}

impl CMSG_GUILD_BANK_SWAP_ITEMS {
    pub(crate) fn size(&self) -> usize {
        8 // bank: Guid
        + self.source.size() // source: CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource
        + self.unknown5.len() * core::mem::size_of::<u8>() // unknown5: u8[-]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode {
    Manual {
        bank_to_character_transfer: bool,
        player_bag: u8,
        player_bag_slot: u8,
        split_amount: u8,
    },
    Automatic {
        auto_count: u32,
        unknown3: u8,
        unknown4: u8,
    },
}

impl Default for CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Manual {
            bank_to_character_transfer: Default::default(),
            player_bag: Default::default(),
            player_bag_slot: Default::default(),
            split_amount: Default::default(),
        }
    }
}

impl CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Manual { .. } => 0,
            Self::Automatic { .. } => 1,
        }
    }

}

impl std::fmt::Display for CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual{ .. } => f.write_str("Manual"),
            Self::Automatic{ .. } => f.write_str("Automatic"),
        }
    }
}

impl CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Manual {
                ..
            } => {
                1
                + 1 // bank_to_character_transfer: Bool
                + 1 // player_bag: u8
                + 1 // player_bag_slot: u8
                + 1 // split_amount: u8
            }
            Self::Automatic {
                ..
            } => {
                1
                + 4 // auto_count: u32
                + 1 // unknown3: u8
                + 1 // unknown4: u8
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource {
    Inventory {
        bank_slot: u8,
        bank_tab: u8,
        item2: u32,
        mode: CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode,
    },
    Bank {
        amount: u8,
        bank_destination_slot: u8,
        bank_destination_tab: u8,
        bank_source_slot: u8,
        bank_source_tab: u8,
        item1: u32,
        unknown1: u32,
        unknown2: u8,
    },
}

impl Default for CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Inventory {
            bank_slot: Default::default(),
            bank_tab: Default::default(),
            item2: Default::default(),
            mode: Default::default(),
        }
    }
}

impl CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Inventory { .. } => 0,
            Self::Bank { .. } => 1,
        }
    }

}

impl std::fmt::Display for CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inventory{ .. } => f.write_str("Inventory"),
            Self::Bank{ .. } => f.write_str("Bank"),
        }
    }
}

impl CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Inventory {
                mode,
                ..
            } => {
                1
                + 1 // bank_slot: u8
                + 1 // bank_tab: u8
                + 4 // item2: u32
                + mode.size() // mode: CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode
            }
            Self::Bank {
                ..
            } => {
                1
                + 1 // amount: u8
                + 1 // bank_destination_slot: u8
                + 1 // bank_destination_tab: u8
                + 1 // bank_source_slot: u8
                + 1 // bank_source_tab: u8
                + 4 // item1: u32
                + 4 // unknown1: u32
                + 1 // unknown2: u8
            }
        }
    }
}

