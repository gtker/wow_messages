use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_update_account_data_complete.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_update_account_data_complete.wowm#L1):
/// ```text
/// smsg SMSG_UPDATE_ACCOUNT_DATA_COMPLETE = 0x0463 {
///     u32 data_type;
///     u32 unknown1;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {
    pub data_type: u32,
    /// mangostwo hardcodes this to 0
    pub unknown1: u32,
}

impl crate::private::Sealed for SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {}
impl SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // data_type: u32
        let data_type = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            data_type,
            unknown1,
        })
    }

}

impl crate::Message for SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {
    const OPCODE: u32 = 0x0463;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_ACCOUNT_DATA_COMPLETE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {{").unwrap();
        // Members
        writeln!(s, "    data_type = {};", self.data_type).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1123_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "data_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // data_type: u32
        w.write_all(&self.data_type.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1123, "SMSG_UPDATE_ACCOUNT_DATA_COMPLETE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {}

