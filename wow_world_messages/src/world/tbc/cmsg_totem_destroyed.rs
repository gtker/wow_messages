use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_totem_destroyed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_totem_destroyed.wowm#L1):
/// ```text
/// cmsg CMSG_TOTEM_DESTROYED = 0x0413 {
///     u8 slot;
/// }
/// ```
pub struct CMSG_TOTEM_DESTROYED {
    pub slot: u8,
}

impl crate::private::Sealed for CMSG_TOTEM_DESTROYED {}
impl CMSG_TOTEM_DESTROYED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            slot,
        })
    }

}

impl crate::Message for CMSG_TOTEM_DESTROYED {
    const OPCODE: u32 = 0x0413;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_TOTEM_DESTROYED {{").unwrap();
        // Members
        writeln!(s, "    slot = {};", self.slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1043_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1043, "CMSG_TOTEM_DESTROYED", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TOTEM_DESTROYED {}

