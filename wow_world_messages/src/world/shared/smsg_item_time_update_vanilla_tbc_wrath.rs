use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_time_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_time_update.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_TIME_UPDATE = 0x01EA {
///     Guid guid;
///     u32 duration;
/// }
/// ```
pub struct SMSG_ITEM_TIME_UPDATE {
    pub guid: Guid,
    pub duration: u32,
}

impl crate::private::Sealed for SMSG_ITEM_TIME_UPDATE {}
impl SMSG_ITEM_TIME_UPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x01EA, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            duration,
        })
    }

}

impl crate::Message for SMSG_ITEM_TIME_UPDATE {
    const OPCODE: u32 = 0x01ea;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_TIME_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    duration = {};", self.duration).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 490_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ITEM_TIME_UPDATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ITEM_TIME_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_TIME_UPDATE {}

