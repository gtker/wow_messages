use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::FriendResult;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_status.wowm:62`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_status.wowm#L62):
/// ```text
/// smsg SMSG_FRIEND_STATUS = 0x0068 {
///     FriendResult result;
///     Guid guid;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_FRIEND_STATUS {
    pub result: FriendResult,
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_FRIEND_STATUS {}
impl SMSG_FRIEND_STATUS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for SMSG_FRIEND_STATUS {
    const OPCODE: u32 = 0x0068;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_FRIEND_STATUS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_FRIEND_STATUS {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 104_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(104, "SMSG_FRIEND_STATUS", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FRIEND_STATUS {}

