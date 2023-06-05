use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questlog_swap_quest.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questlog_swap_quest.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTLOG_SWAP_QUEST = 0x0193 {
///     u8 slot1;
///     u8 slot2;
/// }
/// ```
pub struct CMSG_QUESTLOG_SWAP_QUEST {
    pub slot1: u8,
    pub slot2: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_QUESTLOG_SWAP_QUEST {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_QUESTLOG_SWAP_QUEST {{").unwrap();
        // Members
        writeln!(s, "    slot1 = {};", self.slot1).unwrap();
        writeln!(s, "    slot2 = {};", self.slot2).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 403_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot1");
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

impl crate::private::Sealed for CMSG_QUESTLOG_SWAP_QUEST {}
impl crate::Message for CMSG_QUESTLOG_SWAP_QUEST {
    const OPCODE: u32 = 0x0193;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot1: u8
        w.write_all(&self.slot1.to_le_bytes())?;

        // slot2: u8
        w.write_all(&self.slot2.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0193, size: body_size });
        }

        // slot1: u8
        let slot1 = crate::util::read_u8_le(&mut r)?;

        // slot2: u8
        let slot2 = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            slot1,
            slot2,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_QUESTLOG_SWAP_QUEST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_QUESTLOG_SWAP_QUEST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUESTLOG_SWAP_QUEST {}

