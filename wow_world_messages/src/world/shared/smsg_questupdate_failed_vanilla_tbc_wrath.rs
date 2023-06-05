use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_FAILED = 0x0196 {
///     u32 quest_id;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_FAILED {
    pub quest_id: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_QUESTUPDATE_FAILED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTUPDATE_FAILED {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 406_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_QUESTUPDATE_FAILED {}
impl crate::Message for SMSG_QUESTUPDATE_FAILED {
    const OPCODE: u32 = 0x0196;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0196, size: body_size });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

