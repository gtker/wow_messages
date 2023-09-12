use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MiniMoveMessage, MiniMoveOpcode,
};

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

impl crate::private::Sealed for SMSG_MULTIPLE_MOVES {}
impl SMSG_MULTIPLE_MOVES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=65539).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // size: u32
        let _size = crate::util::read_u32_le(&mut r)?;
        // size is dynamic size of the object

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

impl crate::Message for SMSG_MULTIPLE_MOVES {
    const OPCODE: u32 = 0x051e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MULTIPLE_MOVES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MULTIPLE_MOVES {{").unwrap();
        // Members
        write!(s, "    moves = [").unwrap();
        for v in self.moves.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        opcode = {};", v.opcode.as_test_case_value()).unwrap();
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "        movement_counter = {};", v.movement_counter).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1310_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "size", "    ");
        if !self.moves.is_empty() {
            writeln!(s, "    /* moves: MiniMoveMessage[-] start */").unwrap();
            for (i, v) in self.moves.iter().enumerate() {
                writeln!(s, "    /* moves: MiniMoveMessage[-] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "size", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 2, "opcode", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.guid), "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "movement_counter", "        ");
                writeln!(s, "    /* moves: MiniMoveMessage[-] {i} end */").unwrap();
            }
            writeln!(s, "    /* moves: MiniMoveMessage[-] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // size: u32
        w.write_all(&((self.size() - 4) as u32).to_le_bytes())?;

        // moves: MiniMoveMessage[-]
        for i in self.moves.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1310, "SMSG_MULTIPLE_MOVES", body_size, a))
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

