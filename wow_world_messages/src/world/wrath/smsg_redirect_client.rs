use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Only exists as a comment in azerothcore/trinitycore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_redirect_client.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_redirect_client.wowm#L1):
/// ```text
/// smsg SMSG_REDIRECT_CLIENT = 0x050D {
///     u32 ip_address;
///     u16 port;
///     u32 unknown;
///     u8[20] hash;
/// }
/// ```
pub struct SMSG_REDIRECT_CLIENT {
    pub ip_address: u32,
    pub port: u16,
    pub unknown: u32,
    /// azerothcore: ip + port, seed = sessionkey
    ///
    pub hash: [u8; 20],
}

impl crate::Message for SMSG_REDIRECT_CLIENT {
    const OPCODE: u32 = 0x050d;

    fn size_without_header(&self) -> u32 {
        30
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 30 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x050D, size: body_size as u32 });
        }

        // ip_address: u32
        let ip_address = crate::util::read_u32_le(r)?;

        // port: u16
        let port = crate::util::read_u16_le(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        // hash: u8[20]
        let mut hash = [0_u8; 20];
        r.read_exact(&mut hash)?;

        Ok(Self {
            ip_address,
            port,
            unknown,
            hash,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_REDIRECT_CLIENT {}

