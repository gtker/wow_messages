use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/cmsg_instance_lock_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/cmsg_instance_lock_response.wowm#L1):
/// ```text
/// cmsg CMSG_INSTANCE_LOCK_RESPONSE = 0x013F {
///     Bool accept;
/// }
/// ```
pub struct CMSG_INSTANCE_LOCK_RESPONSE {
    pub accept: bool,
}

impl crate::private::Sealed for CMSG_INSTANCE_LOCK_RESPONSE {}
impl CMSG_INSTANCE_LOCK_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // accept: Bool
        let accept = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            accept,
        })
    }

}

impl crate::Message for CMSG_INSTANCE_LOCK_RESPONSE {
    const OPCODE: u32 = 0x013f;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_INSTANCE_LOCK_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    accept = {};", if self.accept { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 319_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "accept", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // accept: Bool
        w.write_all(u8::from(self.accept).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(319, "CMSG_INSTANCE_LOCK_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_INSTANCE_LOCK_RESPONSE {}

