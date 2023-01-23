use std::convert::{TryFrom, TryInto};
use crate::world::wrath::MiniMoveMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_multiple_moves.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_multiple_moves.wowm#L21):
/// ```text
/// smsg SMSG_MULTIPLE_MOVES = 0x051E {
///     u32 amount_of_moves;
///     MiniMoveMessage[amount_of_moves] moves;
/// }
/// ```
pub struct SMSG_MULTIPLE_MOVES {
    pub moves: Vec<MiniMoveMessage>,
}

impl crate::Message for SMSG_MULTIPLE_MOVES {
    const OPCODE: u32 = 0x051e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_moves: u32
        w.write_all(&(self.moves.len() as u32).to_le_bytes())?;

        // moves: MiniMoveMessage[amount_of_moves]
        for i in self.moves.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x051E, size: body_size as u32 });
        }

        // amount_of_moves: u32
        let amount_of_moves = crate::util::read_u32_le(r)?;

        // moves: MiniMoveMessage[amount_of_moves]
        let mut moves = Vec::with_capacity(amount_of_moves as usize);
        for i in 0..amount_of_moves {
            moves.push(MiniMoveMessage::read(r)?);
        }

        Ok(Self {
            moves,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_MULTIPLE_MOVES {}

impl SMSG_MULTIPLE_MOVES {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_moves: u32
        + self.moves.iter().fold(0, |acc, x| acc + x.size()) // moves: MiniMoveMessage[amount_of_moves]
    }
}

