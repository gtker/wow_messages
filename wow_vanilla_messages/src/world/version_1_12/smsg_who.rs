use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::WhoPlayer;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_who.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_who.wowm#L13):
/// ```text
/// smsg SMSG_WHO = 0x0063 {
///     u32 listed_players;
///     u32 online_players;
///     WhoPlayer[listed_players] players;
/// }
/// ```
pub struct SMSG_WHO {
    pub online_players: u32,
    pub players: Vec<WhoPlayer>,
}

impl ServerMessage for SMSG_WHO {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // listed_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // online_players: u32
        w.write_all(&self.online_players.to_le_bytes())?;

        // players: WhoPlayer[listed_players]
        for i in self.players.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0063;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // listed_players: u32
        let listed_players = crate::util::read_u32_le(r)?;

        // online_players: u32
        let online_players = crate::util::read_u32_le(r)?;

        // players: WhoPlayer[listed_players]
        let mut players = Vec::with_capacity(listed_players as usize);
        for i in 0..listed_players {
            players.push(WhoPlayer::read(r)?);
        }

        Ok(Self {
            online_players,
            players,
        })
    }

}

impl SMSG_WHO {
    pub(crate) fn size(&self) -> usize {
        4 // listed_players: u32
        + 4 // online_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: WhoPlayer[listed_players]
    }
}

