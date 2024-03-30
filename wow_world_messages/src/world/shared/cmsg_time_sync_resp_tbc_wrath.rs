use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_time_sync_resp.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_time_sync_resp.wowm#L3):
/// ```text
/// cmsg CMSG_TIME_SYNC_RESP = 0x0391 {
///     u32 time_sync;
///     u32 client_ticks;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_TIME_SYNC_RESP {
    /// Can be used to check if the client is still properly in sync
    /// This should be the same as the counter sent in [`SMSG_TIME_SYNC_REQ`](crate::tbc::SMSG_TIME_SYNC_REQ).
    pub time_sync: u32,
    /// You can check this against expected values to estimate client latency
    pub client_ticks: u32,
}

impl crate::private::Sealed for CMSG_TIME_SYNC_RESP {}
impl CMSG_TIME_SYNC_RESP {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // time_sync: u32
        let time_sync = crate::util::read_u32_le(&mut r)?;

        // client_ticks: u32
        let client_ticks = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            time_sync,
            client_ticks,
        })
    }

}

impl crate::Message for CMSG_TIME_SYNC_RESP {
    const OPCODE: u32 = 0x0391;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_TIME_SYNC_RESP"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_TIME_SYNC_RESP {{").unwrap();
        // Members
        writeln!(s, "    time_sync = {};", self.time_sync).unwrap();
        writeln!(s, "    client_ticks = {};", self.client_ticks).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 913_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_sync", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "client_ticks", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time_sync: u32
        w.write_all(&self.time_sync.to_le_bytes())?;

        // client_ticks: u32
        w.write_all(&self.client_ticks.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(913, "CMSG_TIME_SYNC_RESP", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TIME_SYNC_RESP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TIME_SYNC_RESP {}

