use std::io::{Read, Write};

use wow_world_base::shared::far_sight_operation_vanilla_tbc_wrath::FarSightOperation;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm#L8):
/// ```text
/// cmsg CMSG_FAR_SIGHT = 0x027A {
///     FarSightOperation operation;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_FAR_SIGHT {
    pub operation: FarSightOperation,
}

impl crate::private::Sealed for CMSG_FAR_SIGHT {}
impl CMSG_FAR_SIGHT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // operation: FarSightOperation
        let operation = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            operation,
        })
    }

}

impl crate::Message for CMSG_FAR_SIGHT {
    const OPCODE: u32 = 0x027a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_FAR_SIGHT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_FAR_SIGHT {{").unwrap();
        // Members
        writeln!(s, "    operation = {};", self.operation.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 634_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "operation", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // operation: FarSightOperation
        w.write_all(&(self.operation.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(634, "CMSG_FAR_SIGHT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_FAR_SIGHT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_FAR_SIGHT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_FAR_SIGHT {}

