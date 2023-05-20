use std::io::{Read, Write};

use crate::wrath::LfgJoinPlayer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm#L1):
/// ```text
/// smsg SMSG_LFG_JOIN_RESULT = 0x0364 {
///     u32 result;
///     u32 state;
///     LfgJoinPlayer[-] players;
/// }
/// ```
pub struct SMSG_LFG_JOIN_RESULT {
    pub result: u32,
    pub state: u32,
    pub players: Vec<LfgJoinPlayer>,
}

impl crate::private::Sealed for SMSG_LFG_JOIN_RESULT {}
impl crate::Message for SMSG_LFG_JOIN_RESULT {
    const OPCODE: u32 = 0x0364;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: u32
        w.write_all(&self.result.to_le_bytes())?;

        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // players: LfgJoinPlayer[-]
        for i in self.players.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(8..=65543).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0364, size: body_size });
        }

        // result: u32
        let result = crate::util::read_u32_le(&mut r)?;

        // state: u32
        let state = crate::util::read_u32_le(&mut r)?;

        // players: LfgJoinPlayer[-]
        let players = {
            let mut current_size = {
                4 // result: u32
                + 4 // state: u32
            };
            let mut players = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                players.push(LfgJoinPlayer::read(&mut r)?);
                current_size += 1;
            }
            players
        };

        Ok(Self {
            result,
            state,
            players,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_JOIN_RESULT {}

impl SMSG_LFG_JOIN_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // result: u32
        + 4 // state: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: LfgJoinPlayer[-]
    }
}

