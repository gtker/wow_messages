use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    BattlegroundEndStatus, BattlegroundPlayer, BattlegroundWinner, PvpRank,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L29):
/// ```text
/// smsg MSG_PVP_LOG_DATA_Server = 0x02E0 {
///     BattlegroundEndStatus status;
///     if (status == ENDED) {
///         BattlegroundWinner winner;
///     }
///     u32 amount_of_players;
///     BattlegroundPlayer[amount_of_players] players;
/// }
/// ```
pub struct MSG_PVP_LOG_DATA_Server {
    pub status: MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus,
    pub players: Vec<BattlegroundPlayer>,
}

impl crate::private::Sealed for MSG_PVP_LOG_DATA_Server {}
impl crate::Message for MSG_PVP_LOG_DATA_Server {
    const OPCODE: u32 = 0x02e0;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_PVP_LOG_DATA_Server {{").unwrap();
        // Members
        writeln!(s, "    status = {};", BattlegroundEndStatus::try_from(self.status.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.status {
            crate::vanilla::MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::Ended {
                winner,
            } => {
                writeln!(s, "    winner = {};", winner.as_test_case_value()).unwrap();
            }
            _ => {}
        }

        writeln!(s, "    amount_of_players = {};", self.players.len()).unwrap();
        write!(s, "    players = [").unwrap();
        for v in self.players.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        player = {};", v.player.guid()).unwrap();
            writeln!(s, "        rank = {};", v.rank.as_test_case_value()).unwrap();
            writeln!(s, "        killing_blows = {};", v.killing_blows).unwrap();
            writeln!(s, "        honorable_kills = {};", v.honorable_kills).unwrap();
            writeln!(s, "        deaths = {};", v.deaths).unwrap();
            writeln!(s, "        bonus_honor = {};", v.bonus_honor).unwrap();
            writeln!(s, "        amount_of_extra_fields = {};", v.fields.len()).unwrap();
            write!(s, "        fields = [").unwrap();
            for v in v.fields.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 736_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "    ");
        match &self.status {
            crate::vanilla::MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::Ended {
                winner,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "winner", "    ");
            }
            _ => {}
        }

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_players", "    ");
        if !self.players.is_empty() {
            writeln!(s, "    /* players: BattlegroundPlayer[amount_of_players] start */").unwrap();
            for (i, v) in self.players.iter().enumerate() {
                writeln!(s, "    /* players: BattlegroundPlayer[amount_of_players] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "rank", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "killing_blows", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "honorable_kills", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "deaths", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "bonus_honor", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_extra_fields", "        ");
                if !v.fields.is_empty() {
                    writeln!(s, "    /* fields: u32[amount_of_extra_fields] start */").unwrap();
                    for (i, v) in v.fields.iter().enumerate() {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("fields {i}"), "        ");
                    }
                    writeln!(s, "    /* fields: u32[amount_of_extra_fields] end */").unwrap();
                }
                writeln!(s, "    /* players: BattlegroundPlayer[amount_of_players] {i} end */").unwrap();
            }
            writeln!(s, "    /* players: BattlegroundPlayer[amount_of_players] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // status: BattlegroundEndStatus
        w.write_all(&(self.status.as_int().to_le_bytes()))?;

        match &self.status {
            MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::Ended {
                winner,
            } => {
                // winner: BattlegroundWinner
                w.write_all(&(winner.as_int().to_le_bytes()))?;

            }
            _ => {}
        }

        // amount_of_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // players: BattlegroundPlayer[amount_of_players]
        for i in self.players.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E0, size: body_size });
        }

        // status: BattlegroundEndStatus
        let status = crate::util::read_u8_le(&mut r)?.try_into()?;

        let status_if = match status {
            BattlegroundEndStatus::NotEnded => MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::NotEnded,
            BattlegroundEndStatus::Ended => {
                // winner: BattlegroundWinner
                let winner = crate::util::read_u8_le(&mut r)?.try_into()?;

                MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::Ended {
                    winner,
                }
            }
        };

        // amount_of_players: u32
        let amount_of_players = crate::util::read_u32_le(&mut r)?;

        // players: BattlegroundPlayer[amount_of_players]
        let players = {
            let mut players = Vec::with_capacity(amount_of_players as usize);
            for _ in 0..amount_of_players {
                players.push(BattlegroundPlayer::read(&mut r)?);
            }
            players
        };

        Ok(Self {
            status: status_if,
            players,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_PVP_LOG_DATA_Server {}

impl MSG_PVP_LOG_DATA_Server {
    pub(crate) fn size(&self) -> usize {
        self.status.size() // status: MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus
        + 4 // amount_of_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: BattlegroundPlayer[amount_of_players]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    NotEnded,
    Ended {
        winner: BattlegroundWinner,
    },
}

impl Default for MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotEnded
    }
}

impl MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotEnded => 0,
            Self::Ended { .. } => 1,
        }
    }

}

impl std::fmt::Display for MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnded => f.write_str("NotEnded"),
            Self::Ended{ .. } => f.write_str("Ended"),
        }
    }
}

impl MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Ended {
                ..
            } => {
                1
                + 1 // winner: BattlegroundWinner
            }
            _ => 1,
        }
    }
}

