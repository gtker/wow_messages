use std::io::{Read, Write};

/// Only exists as a comment in azerothcore/trinitycore.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_redirect_client.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_redirect_client.wowm#L2):
/// ```text
/// smsg SMSG_REDIRECT_CLIENT = 0x050D {
///     u32 ip_address;
///     u16 port;
///     u32 unknown;
///     u8[20] hash;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_REDIRECT_CLIENT {
    pub ip_address: u32,
    pub port: u16,
    pub unknown: u32,
    /// azerothcore: ip + port, seed = sessionkey
    pub hash: [u8; 20],
}

impl crate::private::Sealed for SMSG_REDIRECT_CLIENT {}
impl SMSG_REDIRECT_CLIENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 30 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // ip_address: u32
        let ip_address = crate::util::read_u32_le(&mut r)?;

        // port: u16
        let port = crate::util::read_u16_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // hash: u8[20]
        let hash = {
            let mut hash = [0_u8; 20];
            r.read_exact(&mut hash)?;
            hash
        };

        Ok(Self {
            ip_address,
            port,
            unknown,
            hash,
        })
    }

}

impl crate::Message for SMSG_REDIRECT_CLIENT {
    const OPCODE: u32 = 0x050d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_REDIRECT_CLIENT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_REDIRECT_CLIENT {{").unwrap();
        // Members
        writeln!(s, "    ip_address = {};", self.ip_address).unwrap();
        writeln!(s, "    port = {};", self.port).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        writeln!(s, "    hash = [").unwrap();
        for v in self.hash.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 32_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1293_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "ip_address", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "port", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.hash.len(), "hash", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        30
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // ip_address: u32
        w.write_all(&self.ip_address.to_le_bytes())?;

        // port: u16
        w.write_all(&self.port.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // hash: u8[20]
        for i in self.hash.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1293, "SMSG_REDIRECT_CLIENT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_REDIRECT_CLIENT {}

