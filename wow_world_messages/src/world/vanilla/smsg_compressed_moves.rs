use crate::vanilla::CompressedMove;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm:46`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm#L46):
/// ```text
/// smsg SMSG_COMPRESSED_MOVES = 0x02FB {
///     CompressedMove[-] moves;
/// }
/// ```
pub struct SMSG_COMPRESSED_MOVES {
    pub moves: Vec<CompressedMove>,
}

impl crate::Message for SMSG_COMPRESSED_MOVES {
    const OPCODE: u32 = 0x02fb;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        w.write_all(&(self.size_uncompressed() as u32).to_le_bytes())?;

        let mut w = &mut flate2::write::ZlibEncoder::new(w, flate2::Compression::fast());

        // moves: CompressedMove[-]
        for i in self.moves.iter() {
            let mut vec = Vec::new();
            i.write_into_vec(&mut vec)?;
            w.write_all(vec.as_slice());
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size > 65535 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FB, size: body_size as u32 });
        }

        let decompressed_size = crate::util::read_u32_le(r)?;;
        let decompressed_buffer = vec![0; decompressed_size as usize];
        let mut r = &mut flate2::read::ZlibDecoder::new_with_buf(r, decompressed_buffer);

        // moves: CompressedMove[-]
        let mut current_size = {
            0
        };
        let mut moves = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            moves.push(CompressedMove::read(r)?);
            current_size += 1;
        }

        Ok(Self {
            moves,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_COMPRESSED_MOVES {}

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
        self.moves.iter().fold(0, |acc, x| acc + x.size()) // moves: CompressedMove[-]
    }
}

