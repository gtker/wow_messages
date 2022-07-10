use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Sent after receiving [`SMSG_AUTH_CHALLENGE`](crate::world::version_1_2::SMSG_AUTH_CHALLENGE).
///
/// This message is never encrypted.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm#L3):
/// ```text
/// cmsg CMSG_AUTH_SESSION = 0x01ED {
///     u32 build;
///     u32 server_id;
///     CString username;
///     u32 client_seed;
///     u8[20] client_proof;
///     u32 decompressed_addon_info_size;
///     u8[-] compressed_addon_info;
/// }
/// ```
pub struct CMSG_AUTH_SESSION {
    pub build: u32,
    /// This is sent to the client in `CMD_REALM_LIST_Server`.
    ///
    pub server_id: u32,
    pub username: String,
    pub client_seed: u32,
    pub client_proof: [u8; 20],
    pub decompressed_addon_info_size: u32,
    pub compressed_addon_info: Vec<u8>,
}

impl ClientMessage for CMSG_AUTH_SESSION {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // build: u32
        w.write_all(&self.build.to_le_bytes())?;

        // server_id: u32
        w.write_all(&self.server_id.to_le_bytes())?;

        // username: CString
        w.write_all(self.username.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // client_seed: u32
        w.write_all(&self.client_seed.to_le_bytes())?;

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // decompressed_addon_info_size: u32
        w.write_all(&self.decompressed_addon_info_size.to_le_bytes())?;

        // compressed_addon_info: u8[-]
        for i in self.compressed_addon_info.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x01ed;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // build: u32
        let build = crate::util::read_u32_le(r)?;

        // server_id: u32
        let server_id = crate::util::read_u32_le(r)?;

        // username: CString
        let username = crate::util::read_c_string_to_vec(r)?;
        let username = String::from_utf8(username)?;

        // client_seed: u32
        let client_seed = crate::util::read_u32_le(r)?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof)?;

        // decompressed_addon_info_size: u32
        let decompressed_addon_info_size = crate::util::read_u32_le(r)?;

        // compressed_addon_info: u8[-]
        let mut current_size = {
            4 // build: u32
            + 4 // server_id: u32
            + username.len() + 1 // username: CString
            + 4 // client_seed: u32
            + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
            + 4 // decompressed_addon_info_size: u32
        };
        let mut compressed_addon_info = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            compressed_addon_info.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            build,
            server_id,
            username,
            client_seed,
            client_proof,
            decompressed_addon_info_size,
            compressed_addon_info,
        })
    }

}

impl CMSG_AUTH_SESSION {
    pub(crate) fn size(&self) -> usize {
        4 // build: u32
        + 4 // server_id: u32
        + self.username.len() + 1 // username: CString
        + 4 // client_seed: u32
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 4 // decompressed_addon_info_size: u32
        + self.compressed_addon_info.len() * core::mem::size_of::<u8>() // compressed_addon_info: u8[-]
    }
}

#[cfg(test)]
mod test {
    use super::CMSG_AUTH_SESSION;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 174] = [ 0x00, 0xAC, 0xED, 0x01, 0x00, 0x00, 0xF3, 0x16, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x88, 0x02, 0xD8, 0x49, 0x88,
         0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7, 0x8A, 0xDB, 0xA4, 0xFB,
         0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, 0x56, 0x01, 0x00, 0x00, 0x78,
         0x9C, 0x75, 0xCC, 0xBD, 0x0E, 0xC2, 0x30, 0x0C, 0x04, 0xE0, 0xF2, 0x1E,
         0xBC, 0x0C, 0x61, 0x40, 0x95, 0xC8, 0x42, 0xC3, 0x8C, 0x4C, 0xE2, 0x22,
         0x0B, 0xC7, 0xA9, 0x8C, 0xCB, 0x4F, 0x9F, 0x1E, 0x16, 0x24, 0x06, 0x73,
         0xEB, 0x77, 0x77, 0x81, 0x69, 0x59, 0x40, 0xCB, 0x69, 0x33, 0x67, 0xA3,
         0x26, 0xC7, 0xBE, 0x5B, 0xD5, 0xC7, 0x7A, 0xDF, 0x7D, 0x12, 0xBE, 0x16,
         0xC0, 0x8C, 0x71, 0x24, 0xE4, 0x12, 0x49, 0xA8, 0xC2, 0xE4, 0x95, 0x48,
         0x0A, 0xC9, 0xC5, 0x3D, 0xD8, 0xB6, 0x7A, 0x06, 0x4B, 0xF8, 0x34, 0x0F,
         0x15, 0x46, 0x73, 0x67, 0xBB, 0x38, 0xCC, 0x7A, 0xC7, 0x97, 0x8B, 0xBD,
         0xDC, 0x26, 0xCC, 0xFE, 0x30, 0x42, 0xD6, 0xE6, 0xCA, 0x01, 0xA8, 0xB8,
         0x90, 0x80, 0x51, 0xFC, 0xB7, 0xA4, 0x50, 0x70, 0xB8, 0x12, 0xF3, 0x3F,
         0x26, 0x41, 0xFD, 0xB5, 0x37, 0x90, 0x19, 0x66, 0x8F, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            compressed_addon_info: vec![ 0x78, 0x9C, 0x75, 0xCC, 0xBD, 0x0E, 0xC2,
                 0x30, 0x0C, 0x04, 0xE0, 0xF2, 0x1E, 0xBC, 0x0C, 0x61, 0x40, 0x95,
                 0xC8, 0x42, 0xC3, 0x8C, 0x4C, 0xE2, 0x22, 0x0B, 0xC7, 0xA9, 0x8C,
                 0xCB, 0x4F, 0x9F, 0x1E, 0x16, 0x24, 0x06, 0x73, 0xEB, 0x77, 0x77,
                 0x81, 0x69, 0x59, 0x40, 0xCB, 0x69, 0x33, 0x67, 0xA3, 0x26, 0xC7,
                 0xBE, 0x5B, 0xD5, 0xC7, 0x7A, 0xDF, 0x7D, 0x12, 0xBE, 0x16, 0xC0,
                 0x8C, 0x71, 0x24, 0xE4, 0x12, 0x49, 0xA8, 0xC2, 0xE4, 0x95, 0x48,
                 0x0A, 0xC9, 0xC5, 0x3D, 0xD8, 0xB6, 0x7A, 0x06, 0x4B, 0xF8, 0x34,
                 0x0F, 0x15, 0x46, 0x73, 0x67, 0xBB, 0x38, 0xCC, 0x7A, 0xC7, 0x97,
                 0x8B, 0xBD, 0xDC, 0x26, 0xCC, 0xFE, 0x30, 0x42, 0xD6, 0xE6, 0xCA,
                 0x01, 0xA8, 0xB8, 0x90, 0x80, 0x51, 0xFC, 0xB7, 0xA4, 0x50, 0x70,
                 0xB8, 0x12, 0xF3, 0x3F, 0x26, 0x41, 0xFD, 0xB5, 0x37, 0x90, 0x19,
                 0x66, 0x8F, ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.compressed_addon_info, expected.compressed_addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            compressed_addon_info: vec![ 0x78, 0x9C, 0x75, 0xCC, 0xBD, 0x0E, 0xC2,
                 0x30, 0x0C, 0x04, 0xE0, 0xF2, 0x1E, 0xBC, 0x0C, 0x61, 0x40, 0x95,
                 0xC8, 0x42, 0xC3, 0x8C, 0x4C, 0xE2, 0x22, 0x0B, 0xC7, 0xA9, 0x8C,
                 0xCB, 0x4F, 0x9F, 0x1E, 0x16, 0x24, 0x06, 0x73, 0xEB, 0x77, 0x77,
                 0x81, 0x69, 0x59, 0x40, 0xCB, 0x69, 0x33, 0x67, 0xA3, 0x26, 0xC7,
                 0xBE, 0x5B, 0xD5, 0xC7, 0x7A, 0xDF, 0x7D, 0x12, 0xBE, 0x16, 0xC0,
                 0x8C, 0x71, 0x24, 0xE4, 0x12, 0x49, 0xA8, 0xC2, 0xE4, 0x95, 0x48,
                 0x0A, 0xC9, 0xC5, 0x3D, 0xD8, 0xB6, 0x7A, 0x06, 0x4B, 0xF8, 0x34,
                 0x0F, 0x15, 0x46, 0x73, 0x67, 0xBB, 0x38, 0xCC, 0x7A, 0xC7, 0x97,
                 0x8B, 0xBD, 0xDC, 0x26, 0xCC, 0xFE, 0x30, 0x42, 0xD6, 0xE6, 0xCA,
                 0x01, 0xA8, 0xB8, 0x90, 0x80, 0x51, 0xFC, 0xB7, 0xA4, 0x50, 0x70,
                 0xB8, 0x12, 0xF3, 0x3F, 0x26, 0x41, 0xFD, 0xB5, 0x37, 0x90, 0x19,
                 0x66, 0x8F, ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.compressed_addon_info, expected.compressed_addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            compressed_addon_info: vec![ 0x78, 0x9C, 0x75, 0xCC, 0xBD, 0x0E, 0xC2,
                 0x30, 0x0C, 0x04, 0xE0, 0xF2, 0x1E, 0xBC, 0x0C, 0x61, 0x40, 0x95,
                 0xC8, 0x42, 0xC3, 0x8C, 0x4C, 0xE2, 0x22, 0x0B, 0xC7, 0xA9, 0x8C,
                 0xCB, 0x4F, 0x9F, 0x1E, 0x16, 0x24, 0x06, 0x73, 0xEB, 0x77, 0x77,
                 0x81, 0x69, 0x59, 0x40, 0xCB, 0x69, 0x33, 0x67, 0xA3, 0x26, 0xC7,
                 0xBE, 0x5B, 0xD5, 0xC7, 0x7A, 0xDF, 0x7D, 0x12, 0xBE, 0x16, 0xC0,
                 0x8C, 0x71, 0x24, 0xE4, 0x12, 0x49, 0xA8, 0xC2, 0xE4, 0x95, 0x48,
                 0x0A, 0xC9, 0xC5, 0x3D, 0xD8, 0xB6, 0x7A, 0x06, 0x4B, 0xF8, 0x34,
                 0x0F, 0x15, 0x46, 0x73, 0x67, 0xBB, 0x38, 0xCC, 0x7A, 0xC7, 0x97,
                 0x8B, 0xBD, 0xDC, 0x26, 0xCC, 0xFE, 0x30, 0x42, 0xD6, 0xE6, 0xCA,
                 0x01, 0xA8, 0xB8, 0x90, 0x80, 0x51, 0xFC, 0xB7, 0xA4, 0x50, 0x70,
                 0xB8, 0x12, 0xF3, 0x3F, 0x26, 0x41, 0xFD, 0xB5, 0x37, 0x90, 0x19,
                 0x66, 0x8F, ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.compressed_addon_info, expected.compressed_addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
