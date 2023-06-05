use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::FriendResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_status.wowm:62`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_status.wowm#L62):
/// ```text
/// smsg SMSG_FRIEND_STATUS = 0x0068 {
///     FriendResult result;
///     Guid guid;
/// }
/// ```
pub struct SMSG_FRIEND_STATUS {
    pub result: FriendResult,
    pub guid: Guid,
}

#[cfg(feature = "print-testcase")]
impl SMSG_FRIEND_STATUS {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_FRIEND_STATUS {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 104_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result");
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

impl crate::private::Sealed for SMSG_FRIEND_STATUS {}
impl crate::Message for SMSG_FRIEND_STATUS {
    const OPCODE: u32 = 0x0068;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: FriendResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0068, size: body_size });
        }

        // result: FriendResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            result,
            guid,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FRIEND_STATUS {}

