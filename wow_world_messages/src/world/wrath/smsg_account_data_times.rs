use std::io::{Read, Write};

use crate::wrath::CacheMask;

/// Indicate when each piece of account data was last updated by a [`CMSG_UPDATE_ACCOUNT_DATA`](crate::wrath::CMSG_UPDATE_ACCOUNT_DATA). The client can check this against its own times to detect that more recent account data was written from a different client.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm:45`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm#L45):
/// ```text
/// smsg SMSG_ACCOUNT_DATA_TIMES = 0x0209 {
///     u32 unix_time;
///     u8 unknown1;
///     CacheMask mask;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ACCOUNT_DATA_TIMES {
    /// Seconds since Unix Epoch
    pub unix_time: u32,
    /// Both mangostwo and arcemu hardcode this to 1
    pub unknown1: u8,
    pub mask: CacheMask,
}

impl crate::private::Sealed for SMSG_ACCOUNT_DATA_TIMES {}
impl SMSG_ACCOUNT_DATA_TIMES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=137).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unix_time: u32
        let unix_time = crate::util::read_u32_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // mask: CacheMask
        let mask = CacheMask::read(&mut r)?;

        Ok(Self {
            unix_time,
            unknown1,
            mask,
        })
    }

}

impl crate::Message for SMSG_ACCOUNT_DATA_TIMES {
    const OPCODE: u32 = 0x0209;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ACCOUNT_DATA_TIMES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ACCOUNT_DATA_TIMES {{").unwrap();
        // Members
        writeln!(s, "    unix_time = {};", self.unix_time).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        panic!("unsupported type for test case printing: 'CacheMask' for variable 'mask'");

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 521_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unix_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        panic!("unsupported type CacheMask for variable 'mask'");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unix_time: u32
        w.write_all(&self.unix_time.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // mask: CacheMask
        self.mask.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(521, "SMSG_ACCOUNT_DATA_TIMES", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACCOUNT_DATA_TIMES {}

impl SMSG_ACCOUNT_DATA_TIMES {
    pub(crate) const fn size(&self) -> usize {
        4 // unix_time: u32
        + 1 // unknown1: u8
        + self.mask.size() // mask: CacheMask
    }
}

