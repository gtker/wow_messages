use crate:: {
};
use crate::wrath::MiniMoveMessage;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm#L20):
/// ```text
/// smsg SMSG_MULTIPLE_MOVES = 0x051E {
///     u32 size = self.size;
///     MiniMoveMessage[-] moves;
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // size: u32
        w.write_all(&((self.size() - 4) as u32).to_le_bytes())?;

        // moves: MiniMoveMessage[-]
        for i in self.moves.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=65539).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x051E, size: body_size as u32 });
        }

        // size: u32
        let _size = crate::util::read_u32_le(&mut r)?;
        // size is expected to always be self.size (0)

        // moves: MiniMoveMessage[-]
        let moves = {
            let mut current_size = {
                4 // size: u32
            };
            let mut moves = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                moves.push(MiniMoveMessage::read(&mut r)?);
                current_size += 1;
            }
            moves
        };

        Ok(Self {
            moves,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MULTIPLE_MOVES {}

impl SMSG_MULTIPLE_MOVES {
    pub(crate) fn size(&self) -> usize {
        4 // size: u32
        + self.moves.iter().fold(0, |acc, x| acc + x.size()) // moves: MiniMoveMessage[-]
    }
}

