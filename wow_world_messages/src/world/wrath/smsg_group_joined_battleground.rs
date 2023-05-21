use std::io::{Read, Write};

use crate::wrath::BgTypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm:58`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm#L58):
/// ```text
/// smsg SMSG_GROUP_JOINED_BATTLEGROUND = 0x02E8 {
///     BgTypeId id;
/// }
/// ```
pub struct SMSG_GROUP_JOINED_BATTLEGROUND {
    pub id: BgTypeId,
}

impl crate::private::Sealed for SMSG_GROUP_JOINED_BATTLEGROUND {}
impl SMSG_GROUP_JOINED_BATTLEGROUND {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // id: BgTypeId
        let id = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            id,
        })
    }

}

impl crate::Message for SMSG_GROUP_JOINED_BATTLEGROUND {
    const OPCODE: u32 = 0x02e8;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GROUP_JOINED_BATTLEGROUND {{").unwrap();
        // Members
        writeln!(s, "    id = {};", self.id.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 744_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: BgTypeId
        w.write_all(&(self.id.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(744, "SMSG_GROUP_JOINED_BATTLEGROUND", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GROUP_JOINED_BATTLEGROUND {}

