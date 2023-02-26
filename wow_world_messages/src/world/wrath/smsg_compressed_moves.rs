use crate::wrath::MiniMoveMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm#L27):
/// ```text
/// smsg SMSG_COMPRESSED_MOVES = 0x02FB {
///     u32 size = self.size;
///     MiniMoveMessage[-] moves;
/// }
/// ```
pub struct SMSG_COMPRESSED_MOVES {
    pub moves: Vec<MiniMoveMessage>,
}

impl crate::Message for SMSG_COMPRESSED_MOVES {
    const OPCODE: u32 = 0x02fb;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        w.write_all(&(self.size_uncompressed() as u32).to_le_bytes())?;

        let mut w = &mut flate2::write::ZlibEncoder::new(w, flate2::Compression::fast());

        // size: u32
        w.write_all(&((self.size() - 4) as u32).to_le_bytes())?;

        // moves: MiniMoveMessage[-]
        for i in self.moves.iter() {
            let mut vec = Vec::new();
            i.write_into_vec(&mut vec)?;
            w.write_all(vec.as_slice());
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=65539).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FB, size: body_size as u32 });
        }

        let decompressed_size = crate::util::read_u32_le(r)?;;
        let decompressed_buffer = vec![0; decompressed_size as usize];
        let mut r = &mut flate2::read::ZlibDecoder::new_with_buf(r, decompressed_buffer);

        // size: u32
        let _size = crate::util::read_u32_le(r)?;
        // size is expected to always be self.size (0)

        // moves: MiniMoveMessage[-]
        let moves = {
            let mut current_size = {
                4 // size: u32
            };
            let mut moves = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                moves.push(MiniMoveMessage::read(r)?);
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
impl crate::wrath::ServerMessage for SMSG_COMPRESSED_MOVES {}

impl SMSG_COMPRESSED_MOVES {
    pub(crate) fn size(&self) -> usize {
        use crate::traits::Message;

        let mut v = Vec::new();
        self.write_into_vec(&mut v);
        v.len()
    }
}

impl SMSG_COMPRESSED_MOVES {
    pub(crate) fn size_uncompressed(&self) -> usize {
        4 // size: u32
        + self.moves.iter().fold(0, |acc, x| acc + x.size()) // moves: MiniMoveMessage[-]
    }
}

