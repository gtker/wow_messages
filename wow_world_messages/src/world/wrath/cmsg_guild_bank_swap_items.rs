use crate:: {
    Guid,
};
use crate::wrath::BankSwapSource;
use crate::wrath::BankSwapStoreMode;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_swap_items.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_swap_items.wowm#L50):
/// ```text
/// cmsg CMSG_GUILD_BANK_SWAP_ITEMS = 0x03E9 {
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
///         u32 amount;
///     }
///     else {
///         u8 bank_tab;
///         u8 bank_slot;
///         u32 item2;
///         BankSwapStoreMode mode;
///         if (mode == AUTOMATIC) {
///             u32 auto_count;
///             u8 unknown3;
///             u32 unknown4;
///         }
///         else {
///             u8 player_bag;
///             u8 player_bag_slot;
///             Bool bank_to_character_transfer;
///             u32 split_amount;
///         }
///     }
///     u8[-] unknown5;
/// }
/// ```
pub struct CMSG_GUILD_BANK_SWAP_ITEMS {
    pub bank: Guid,
    pub source: CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource,
    /// cmangos-tbc/mangosone has extra
    ///
    pub unknown5: Vec<u8>,
}

impl crate::Message for CMSG_GUILD_BANK_SWAP_ITEMS {
    const OPCODE: u32 = 0x03e9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // source: BankSwapSource
        w.write_all(&u8::from(self.source.as_int()).to_le_bytes())?;

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
                w.write_all(&u8::from(mode.as_int()).to_le_bytes())?;

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

                        // split_amount: u32
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

                        // unknown4: u32
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

                // amount: u32
                w.write_all(&amount.to_le_bytes())?;

            }
        }

        // unknown5: u8[-]
        for i in self.unknown5.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(23..=65561).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E9, size: body_size as u32 });
        }

        // bank: Guid
        let bank = Guid::read(&mut r)?;

        // source: BankSwapSource
        let source: BankSwapSource = crate::util::read_u8_le(&mut r)?.try_into()?;

        let source_if = match source {
            BankSwapSource::Inventory => {
                // bank_tab: u8
                let bank_tab = crate::util::read_u8_le(&mut r)?;

                // bank_slot: u8
                let bank_slot = crate::util::read_u8_le(&mut r)?;

                // item2: u32
                let item2 = crate::util::read_u32_le(&mut r)?;

                // mode: BankSwapStoreMode
                let mode: BankSwapStoreMode = crate::util::read_u8_le(&mut r)?.try_into()?;

                let mode_if = match mode {
                    BankSwapStoreMode::Manual => {
                        // player_bag: u8
                        let player_bag = crate::util::read_u8_le(&mut r)?;

                        // player_bag_slot: u8
                        let player_bag_slot = crate::util::read_u8_le(&mut r)?;

                        // bank_to_character_transfer: Bool
                        let bank_to_character_transfer = crate::util::read_u8_le(&mut r)? != 0;

                        // split_amount: u32
                        let split_amount = crate::util::read_u32_le(&mut r)?;

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

                        // unknown4: u32
                        let unknown4 = crate::util::read_u32_le(&mut r)?;

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

                // amount: u32
                let amount = crate::util::read_u32_le(&mut r)?;

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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANK_SWAP_ITEMS {}

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
        split_amount: u32,
    },
    Automatic {
        auto_count: u32,
        unknown3: u8,
        unknown4: u32,
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

impl CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Manual {
                bank_to_character_transfer,
                player_bag,
                player_bag_slot,
                split_amount,
            } => {
                1
                + 1 // bank_to_character_transfer: Bool
                + 1 // player_bag: u8
                + 1 // player_bag_slot: u8
                + 4 // split_amount: u32
            }
            Self::Automatic {
                auto_count,
                unknown3,
                unknown4,
            } => {
                1
                + 4 // auto_count: u32
                + 1 // unknown3: u8
                + 4 // unknown4: u32
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
        amount: u32,
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

impl CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapSource {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Inventory {
                bank_slot,
                bank_tab,
                item2,
                mode,
            } => {
                1
                + 1 // bank_slot: u8
                + 1 // bank_tab: u8
                + 4 // item2: u32
                + mode.size() // mode: CMSG_GUILD_BANK_SWAP_ITEMS_BankSwapStoreMode
            }
            Self::Bank {
                amount,
                bank_destination_slot,
                bank_destination_tab,
                bank_source_slot,
                bank_source_tab,
                item1,
                unknown1,
                unknown2,
            } => {
                1
                + 4 // amount: u32
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

