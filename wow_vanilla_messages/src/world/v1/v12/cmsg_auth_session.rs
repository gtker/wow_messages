use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_AUTH_SESSION {
    pub build: u32,
    pub server_id: u32,
    pub username: String,
    pub client_seed: u32,
    pub client_proof: [u8; 20],
    pub decompressed_addon_info_size: u32,
    pub compressed_addon_info: Vec<u8>,
}

impl ClientMessage for CMSG_AUTH_SESSION {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    pub fn size(&self) -> usize {
        0
        + 4 // build: u32
        + 4 // server_id: u32
        + self.username.len() + 1 // username: CString
        + 4 // client_seed: u32
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 4 // decompressed_addon_info_size: u32
        + self.compressed_addon_info.len() * core::mem::size_of::<u8>() // compressed_addon_info: u8[-]
    }
}

