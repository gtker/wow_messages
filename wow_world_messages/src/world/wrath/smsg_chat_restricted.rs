use std::io::{Read, Write};

use crate::wrath::ChatRestrictionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm#L22):
/// ```text
/// smsg SMSG_CHAT_RESTRICTED = 0x02FD {
///     ChatRestrictionType restriction;
/// }
/// ```
pub struct SMSG_CHAT_RESTRICTED {
    pub restriction: ChatRestrictionType,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CHAT_RESTRICTED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHAT_RESTRICTED {{").unwrap();
        // Members
        writeln!(s, "    restriction = {};", self.restriction.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 765_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "restriction");
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

impl crate::private::Sealed for SMSG_CHAT_RESTRICTED {}
impl crate::Message for SMSG_CHAT_RESTRICTED {
    const OPCODE: u32 = 0x02fd;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // restriction: ChatRestrictionType
        w.write_all(&(self.restriction.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FD, size: body_size });
        }

        // restriction: ChatRestrictionType
        let restriction = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            restriction,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHAT_RESTRICTED {}

