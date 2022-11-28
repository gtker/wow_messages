use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_event.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_event.wowm#L20):
/// ```text
/// enum GuildEvent : u8 {
///     PROMOTION = 0x00;
///     DEMOTION = 0x01;
///     MOTD = 0x02;
///     JOINED = 0x03;
///     LEFT = 0x04;
///     REMOVED = 0x05;
///     LEADER_IS = 0x06;
///     LEADER_CHANGED = 0x07;
///     DISBANDED = 0x08;
///     TABARD_CHANGED = 0x09;
///     UNK1 = 0x0A;
///     UNK2 = 0x0B;
///     SIGNED_ON = 0x0C;
///     SIGNED_OFF = 0x0D;
///     GUILD_BANK_BAG_SLOTS_CHANGED = 0x0E;
///     BANKTAB_PURCHASED = 0x0F;
///     UNK5 = 0x10;
///     GUILD_BANK_UPDATE_MONEY = 0x11;
///     GUILD_BANK_MONEY_WITHDRAWN = 0x12;
///     GUILD_BANK_TEXT_CHANGED = 0x13;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildEvent {
    Promotion,
    Demotion,
    Motd,
    Joined,
    Left,
    Removed,
    LeaderIs,
    LeaderChanged,
    Disbanded,
    TabardChanged,
    /// string EVENT_GUILD_ROSTER_UPDATE tab content change?
    ///
    Unk1,
    /// EVENT_GUILD_ROSTER_UPDATE
    ///
    Unk2,
    /// ERR_FRIEND_ONLINE_SS
    ///
    SignedOn,
    /// ERR_FRIEND_OFFLINE_S
    ///
    SignedOff,
    /// EVENT_GUILDBANKBAGSLOTS_CHANGED
    ///
    GuildBankBagSlotsChanged,
    /// EVENT_GUILDBANK_UPDATE_TABS
    ///
    BanktabPurchased,
    /// EVENT_GUILDBANK_UPDATE_TABS
    ///
    Unk5,
    /// EVENT_GUILDBANK_UPDATE_MONEY. string 0000000000002710 is 1 gold
    ///
    GuildBankUpdateMoney,
    /// MSG_GUILD_BANK_MONEY_WITHDRAWN
    ///
    GuildBankMoneyWithdrawn,
    /// EVENT_GUILDBANK_TEXT_CHANGED
    ///
    GuildBankTextChanged,
}

impl GuildEvent {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Promotion => 0x0,
            Self::Demotion => 0x1,
            Self::Motd => 0x2,
            Self::Joined => 0x3,
            Self::Left => 0x4,
            Self::Removed => 0x5,
            Self::LeaderIs => 0x6,
            Self::LeaderChanged => 0x7,
            Self::Disbanded => 0x8,
            Self::TabardChanged => 0x9,
            Self::Unk1 => 0xa,
            Self::Unk2 => 0xb,
            Self::SignedOn => 0xc,
            Self::SignedOff => 0xd,
            Self::GuildBankBagSlotsChanged => 0xe,
            Self::BanktabPurchased => 0xf,
            Self::Unk5 => 0x10,
            Self::GuildBankUpdateMoney => 0x11,
            Self::GuildBankMoneyWithdrawn => 0x12,
            Self::GuildBankTextChanged => 0x13,
        }
    }

}

impl Default for GuildEvent {
    fn default() -> Self {
        Self::Promotion
    }
}

impl std::fmt::Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Promotion => f.write_str("Promotion"),
            Self::Demotion => f.write_str("Demotion"),
            Self::Motd => f.write_str("Motd"),
            Self::Joined => f.write_str("Joined"),
            Self::Left => f.write_str("Left"),
            Self::Removed => f.write_str("Removed"),
            Self::LeaderIs => f.write_str("LeaderIs"),
            Self::LeaderChanged => f.write_str("LeaderChanged"),
            Self::Disbanded => f.write_str("Disbanded"),
            Self::TabardChanged => f.write_str("TabardChanged"),
            Self::Unk1 => f.write_str("Unk1"),
            Self::Unk2 => f.write_str("Unk2"),
            Self::SignedOn => f.write_str("SignedOn"),
            Self::SignedOff => f.write_str("SignedOff"),
            Self::GuildBankBagSlotsChanged => f.write_str("GuildBankBagSlotsChanged"),
            Self::BanktabPurchased => f.write_str("BanktabPurchased"),
            Self::Unk5 => f.write_str("Unk5"),
            Self::GuildBankUpdateMoney => f.write_str("GuildBankUpdateMoney"),
            Self::GuildBankMoneyWithdrawn => f.write_str("GuildBankMoneyWithdrawn"),
            Self::GuildBankTextChanged => f.write_str("GuildBankTextChanged"),
        }
    }
}

impl TryFrom<u8> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Promotion),
            1 => Ok(Self::Demotion),
            2 => Ok(Self::Motd),
            3 => Ok(Self::Joined),
            4 => Ok(Self::Left),
            5 => Ok(Self::Removed),
            6 => Ok(Self::LeaderIs),
            7 => Ok(Self::LeaderChanged),
            8 => Ok(Self::Disbanded),
            9 => Ok(Self::TabardChanged),
            10 => Ok(Self::Unk1),
            11 => Ok(Self::Unk2),
            12 => Ok(Self::SignedOn),
            13 => Ok(Self::SignedOff),
            14 => Ok(Self::GuildBankBagSlotsChanged),
            15 => Ok(Self::BanktabPurchased),
            16 => Ok(Self::Unk5),
            17 => Ok(Self::GuildBankUpdateMoney),
            18 => Ok(Self::GuildBankMoneyWithdrawn),
            19 => Ok(Self::GuildBankTextChanged),
            v => Err(crate::errors::EnumError::new("GuildEvent", v as u32),)
        }
    }
}

