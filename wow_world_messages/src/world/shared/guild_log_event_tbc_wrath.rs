use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::guild_event_tbc_wrath::GuildEvent;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm#L9):
/// ```text
/// struct GuildLogEvent {
///     GuildEvent event;
///     Guid player1;
///     if (event == JOINED
///         || event == LEFT) {
///         Guid player2;
///     }
///     else if (event == PROMOTION
///         || event == DEMOTION) {
///         u8 new_rank;
///     }
///     u32 unix_time;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct GuildLogEvent {
    pub event: GuildLogEvent_GuildEvent,
    pub player1: Guid,
    pub unix_time: u32,
}

impl GuildLogEvent {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event: GuildEvent
        w.write_all(&(self.event.as_int().to_le_bytes()))?;

        // player1: Guid
        w.write_all(&self.player1.guid().to_le_bytes())?;

        match &self.event {
            GuildLogEvent_GuildEvent::Promotion {
                new_rank,
            } => {
                // new_rank: u8
                w.write_all(&new_rank.to_le_bytes())?;

            }
            GuildLogEvent_GuildEvent::Demotion {
                new_rank,
            } => {
                // new_rank: u8
                w.write_all(&new_rank.to_le_bytes())?;

            }
            GuildLogEvent_GuildEvent::Joined {
                player2,
            } => {
                // player2: Guid
                w.write_all(&player2.guid().to_le_bytes())?;

            }
            GuildLogEvent_GuildEvent::Left {
                player2,
            } => {
                // player2: Guid
                w.write_all(&player2.guid().to_le_bytes())?;

            }
            _ => {}
        }

        // unix_time: u32
        w.write_all(&self.unix_time.to_le_bytes())?;

        Ok(())
    }
}

impl GuildLogEvent {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // event: GuildEvent
        let event = crate::util::read_u8_le(&mut r)?.try_into()?;

        // player1: Guid
        let player1 = crate::util::read_guid(&mut r)?;

        let event_if = match event {
            GuildEvent::Promotion => {
                // new_rank: u8
                let new_rank = crate::util::read_u8_le(&mut r)?;

                GuildLogEvent_GuildEvent::Promotion {
                    new_rank,
                }
            }
            GuildEvent::Demotion => {
                // new_rank: u8
                let new_rank = crate::util::read_u8_le(&mut r)?;

                GuildLogEvent_GuildEvent::Demotion {
                    new_rank,
                }
            }
            GuildEvent::Motd => GuildLogEvent_GuildEvent::Motd,
            GuildEvent::Joined => {
                // player2: Guid
                let player2 = crate::util::read_guid(&mut r)?;

                GuildLogEvent_GuildEvent::Joined {
                    player2,
                }
            }
            GuildEvent::Left => {
                // player2: Guid
                let player2 = crate::util::read_guid(&mut r)?;

                GuildLogEvent_GuildEvent::Left {
                    player2,
                }
            }
            GuildEvent::Removed => GuildLogEvent_GuildEvent::Removed,
            GuildEvent::LeaderIs => GuildLogEvent_GuildEvent::LeaderIs,
            GuildEvent::LeaderChanged => GuildLogEvent_GuildEvent::LeaderChanged,
            GuildEvent::Disbanded => GuildLogEvent_GuildEvent::Disbanded,
            GuildEvent::TabardChanged => GuildLogEvent_GuildEvent::TabardChanged,
            GuildEvent::Unk1 => GuildLogEvent_GuildEvent::Unk1,
            GuildEvent::Unk2 => GuildLogEvent_GuildEvent::Unk2,
            GuildEvent::SignedOn => GuildLogEvent_GuildEvent::SignedOn,
            GuildEvent::SignedOff => GuildLogEvent_GuildEvent::SignedOff,
            GuildEvent::GuildBankBagSlotsChanged => GuildLogEvent_GuildEvent::GuildBankBagSlotsChanged,
            GuildEvent::BanktabPurchased => GuildLogEvent_GuildEvent::BanktabPurchased,
            GuildEvent::Unk5 => GuildLogEvent_GuildEvent::Unk5,
            GuildEvent::GuildBankUpdateMoney => GuildLogEvent_GuildEvent::GuildBankUpdateMoney,
            GuildEvent::GuildBankMoneyWithdrawn => GuildLogEvent_GuildEvent::GuildBankMoneyWithdrawn,
            GuildEvent::GuildBankTextChanged => GuildLogEvent_GuildEvent::GuildBankTextChanged,
        };

        // unix_time: u32
        let unix_time = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            event: event_if,
            player1,
            unix_time,
        })
    }

}

impl GuildLogEvent {
    pub(crate) const fn size(&self) -> usize {
        self.event.size() // event: GuildLogEvent_GuildEvent
        + 8 // player1: Guid
        + 4 // unix_time: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GuildLogEvent_GuildEvent {
    Promotion {
        new_rank: u8,
    },
    Demotion {
        new_rank: u8,
    },
    Motd,
    Joined {
        player2: Guid,
    },
    Left {
        player2: Guid,
    },
    Removed,
    LeaderIs,
    LeaderChanged,
    Disbanded,
    TabardChanged,
    Unk1,
    Unk2,
    SignedOn,
    SignedOff,
    GuildBankBagSlotsChanged,
    BanktabPurchased,
    Unk5,
    GuildBankUpdateMoney,
    GuildBankMoneyWithdrawn,
    GuildBankTextChanged,
}

impl Default for GuildLogEvent_GuildEvent {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Motd
    }
}

impl GuildLogEvent_GuildEvent {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Promotion { .. } => 0,
            Self::Demotion { .. } => 1,
            Self::Motd => 2,
            Self::Joined { .. } => 3,
            Self::Left { .. } => 4,
            Self::Removed => 5,
            Self::LeaderIs => 6,
            Self::LeaderChanged => 7,
            Self::Disbanded => 8,
            Self::TabardChanged => 9,
            Self::Unk1 => 10,
            Self::Unk2 => 11,
            Self::SignedOn => 12,
            Self::SignedOff => 13,
            Self::GuildBankBagSlotsChanged => 14,
            Self::BanktabPurchased => 15,
            Self::Unk5 => 16,
            Self::GuildBankUpdateMoney => 17,
            Self::GuildBankMoneyWithdrawn => 18,
            Self::GuildBankTextChanged => 19,
        }
    }

}

impl std::fmt::Display for GuildLogEvent_GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Promotion{ .. } => f.write_str("Promotion"),
            Self::Demotion{ .. } => f.write_str("Demotion"),
            Self::Motd => f.write_str("Motd"),
            Self::Joined{ .. } => f.write_str("Joined"),
            Self::Left{ .. } => f.write_str("Left"),
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

impl GuildLogEvent_GuildEvent {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Promotion {
                ..
            } => {
                1
                + 1 // new_rank: u8
            }
            Self::Demotion {
                ..
            } => {
                1
                + 1 // new_rank: u8
            }
            Self::Joined {
                ..
            } => {
                1
                + 8 // player2: Guid
            }
            Self::Left {
                ..
            } => {
                1
                + 8 // player2: Guid
            }
            _ => 1,
        }
    }
}

