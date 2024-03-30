use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_cancel_temp_enchantment.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_cancel_temp_enchantment.wowm#L1):
/// ```text
/// cmsg CMSG_CANCEL_TEMP_ENCHANTMENT = 0x0379 {
///     u32 slot;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_CANCEL_TEMP_ENCHANTMENT {
    pub slot: u32,
}

impl crate::private::Sealed for CMSG_CANCEL_TEMP_ENCHANTMENT {}
impl CMSG_CANCEL_TEMP_ENCHANTMENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // slot: u32
        let slot = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            slot,
        })
    }

}

impl crate::Message for CMSG_CANCEL_TEMP_ENCHANTMENT {
    const OPCODE: u32 = 0x0379;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CANCEL_TEMP_ENCHANTMENT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CANCEL_TEMP_ENCHANTMENT {{").unwrap();
        // Members
        writeln!(s, "    slot = {};", self.slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 889_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(889, "CMSG_CANCEL_TEMP_ENCHANTMENT", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CANCEL_TEMP_ENCHANTMENT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CANCEL_TEMP_ENCHANTMENT {}

