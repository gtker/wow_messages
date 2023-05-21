use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm#L3):
/// ```text
/// smsg SMSG_TIME_SYNC_REQ = 0x0390 {
///     u32 time_sync;
/// }
/// ```
pub struct SMSG_TIME_SYNC_REQ {
    pub time_sync: u32,
}

impl crate::private::Sealed for SMSG_TIME_SYNC_REQ {}
impl SMSG_TIME_SYNC_REQ {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // time_sync: u32
        let time_sync = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            time_sync,
        })
    }

}

impl crate::Message for SMSG_TIME_SYNC_REQ {
    const OPCODE: u32 = 0x0390;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TIME_SYNC_REQ {{").unwrap();
        // Members
        writeln!(s, "    time_sync = {};", self.time_sync).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 912_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_sync", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time_sync: u32
        w.write_all(&self.time_sync.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(912, "SMSG_TIME_SYNC_REQ", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TIME_SYNC_REQ {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TIME_SYNC_REQ {}

