use crate:: {
};
use crate::vanilla::WhoPlayer;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_who.wowm:25`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_who.wowm#L25):
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

impl crate::Message for SMSG_WHO {
    const OPCODE: u32 = 0x0063;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // listed_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // online_players: u32
        w.write_all(&self.online_players.to_le_bytes())?;

        // players: WhoPlayer[listed_players]
        for i in self.players.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0063, size: body_size as u32 });
        }

        // listed_players: u32
        let listed_players = crate::util::read_u32_le(&mut r)?;

        // online_players: u32
        let online_players = crate::util::read_u32_le(&mut r)?;

        // players: WhoPlayer[listed_players]
        let players = {
            let mut players = Vec::with_capacity(listed_players as usize);
            for i in 0..listed_players {
                players.push(WhoPlayer::read(&mut r)?);
            }
            players
        };

        Ok(Self {
            online_players,
            players,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_WHO {}

impl SMSG_WHO {
    pub(crate) fn size(&self) -> usize {
        4 // listed_players: u32
        + 4 // online_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: WhoPlayer[listed_players]
    }
}

