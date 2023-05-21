use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_clientcache_version.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_clientcache_version.wowm#L1):
/// ```text
/// smsg SMSG_CLIENTCACHE_VERSION = 0x04AB {
///     u32 version;
/// }
/// ```
pub struct SMSG_CLIENTCACHE_VERSION {
    pub version: u32,
}

impl crate::private::Sealed for SMSG_CLIENTCACHE_VERSION {}
impl SMSG_CLIENTCACHE_VERSION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // version: u32
        let version = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            version,
        })
    }

}

impl crate::Message for SMSG_CLIENTCACHE_VERSION {
    const OPCODE: u32 = 0x04ab;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CLIENTCACHE_VERSION {{").unwrap();
        // Members
        writeln!(s, "    version = {};", self.version).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1195_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "version", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // version: u32
        w.write_all(&self.version.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1195, "SMSG_CLIENTCACHE_VERSION", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CLIENTCACHE_VERSION {}

