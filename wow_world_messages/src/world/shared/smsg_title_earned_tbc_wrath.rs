use std::io::{Read, Write};

use wow_world_base::shared::title_earn_status_tbc_wrath::TitleEarnStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_title_earned.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_title_earned.wowm#L8):
/// ```text
/// smsg SMSG_TITLE_EARNED = 0x0373 {
///     u32 title;
///     TitleEarnStatus status;
/// }
/// ```
pub struct SMSG_TITLE_EARNED {
    pub title: u32,
    pub status: TitleEarnStatus,
}

impl crate::private::Sealed for SMSG_TITLE_EARNED {}
impl crate::Message for SMSG_TITLE_EARNED {
    const OPCODE: u32 = 0x0373;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TITLE_EARNED {{").unwrap();
        // Members
        writeln!(s, "    title = {};", self.title).unwrap();
        writeln!(s, "    status = {};", self.status.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 883_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "status", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // title: u32
        w.write_all(&self.title.to_le_bytes())?;

        // status: TitleEarnStatus
        w.write_all(&(self.status.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0373, size: body_size });
        }

        // title: u32
        let title = crate::util::read_u32_le(&mut r)?;

        // status: TitleEarnStatus
        let status = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            title,
            status,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TITLE_EARNED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TITLE_EARNED {}

