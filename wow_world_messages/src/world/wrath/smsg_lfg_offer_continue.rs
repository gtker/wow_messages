use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_offer_continue.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_offer_continue.wowm#L1):
/// ```text
/// smsg SMSG_LFG_OFFER_CONTINUE = 0x0293 {
///     u32 dungeon_entry;
/// }
/// ```
pub struct SMSG_LFG_OFFER_CONTINUE {
    pub dungeon_entry: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LFG_OFFER_CONTINUE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_OFFER_CONTINUE {{").unwrap();
        // Members
        writeln!(s, "    dungeon_entry = {};", self.dungeon_entry).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 659_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_entry");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_LFG_OFFER_CONTINUE {}
impl crate::Message for SMSG_LFG_OFFER_CONTINUE {
    const OPCODE: u32 = 0x0293;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // dungeon_entry: u32
        w.write_all(&self.dungeon_entry.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0293, size: body_size });
        }

        // dungeon_entry: u32
        let dungeon_entry = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            dungeon_entry,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_OFFER_CONTINUE {}

