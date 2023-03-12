use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::guild_event_tbc_wrath::GuildEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm#L9):
/// ```text
/// struct GuildLogEvent {
///     GuildEvent event;
///     Guid player1;
///     if (event == JOINED
///         || event == LEFT) {
///         Guid player2;
///     }
///     else if (event == PROMOTE_PLAYER
///         || event == DEMOTE_PLAYER) {
///         u8 new_rank;
///     }
///     u32 unix_time;
/// }
/// ```
pub struct GuildLogEvent {
    pub event: GuildLogEvent_GuildEvent,
    pub player1: Guid,
    pub unix_time: u32,
}

impl GuildLogEvent {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event: GuildEvent
        w.write_all(&u8::from(self.event.as_int()).to_le_bytes())?;

        // player1: Guid
        w.write_all(&self.player1.guid().to_le_bytes())?;

        match &self.event {
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
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // event: GuildEvent
        let event: GuildEvent = crate::util::read_u8_le(&mut r)?.try_into()?;

        // player1: Guid
        let player1 = Guid::read(&mut r)?;

        let event_if = match event {
            GuildEvent::Promotion => GuildLogEvent_GuildEvent::Promotion,
            GuildEvent::Demotion => GuildLogEvent_GuildEvent::Demotion,
            GuildEvent::Motd => GuildLogEvent_GuildEvent::Motd,
            GuildEvent::Joined => {
                // player2: Guid
                let player2 = Guid::read(&mut r)?;

                GuildLogEvent_GuildEvent::Joined {
                    player2,
                }
            }
            GuildEvent::Left => {
                // player2: Guid
                let player2 = Guid::read(&mut r)?;

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
    pub(crate) fn size(&self) -> usize {
        self.event.size() // event: GuildLogEvent_GuildEvent
        + 8 // player1: Guid
        + 4 // unix_time: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GuildLogEvent_GuildEvent {
    Promotion,
    Demotion,
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
        Self::Promotion
    }
}

impl GuildLogEvent_GuildEvent {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Promotion => 0,
            Self::Demotion => 1,
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

impl GuildLogEvent_GuildEvent {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Promotion => {
                1
            }
            Self::Demotion => {
                1
            }
            Self::Motd => {
                1
            }
            Self::Joined {
                player2,
            } => {
                1
                + 8 // player2: Guid
            }
            Self::Left {
                player2,
            } => {
                1
                + 8 // player2: Guid
            }
            Self::Removed => {
                1
            }
            Self::LeaderIs => {
                1
            }
            Self::LeaderChanged => {
                1
            }
            Self::Disbanded => {
                1
            }
            Self::TabardChanged => {
                1
            }
            Self::Unk1 => {
                1
            }
            Self::Unk2 => {
                1
            }
            Self::SignedOn => {
                1
            }
            Self::SignedOff => {
                1
            }
            Self::GuildBankBagSlotsChanged => {
                1
            }
            Self::BanktabPurchased => {
                1
            }
            Self::Unk5 => {
                1
            }
            Self::GuildBankUpdateMoney => {
                1
            }
            Self::GuildBankMoneyWithdrawn => {
                1
            }
            Self::GuildBankTextChanged => {
                1
            }
        }
    }
}

