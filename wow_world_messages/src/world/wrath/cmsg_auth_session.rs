use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Sent after receiving [`SMSG_AUTH_CHALLENGE`](crate::world::wrath::SMSG_AUTH_CHALLENGE).
///
/// This message is never encrypted.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm:70`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm#L70):
/// ```text
/// cmsg CMSG_AUTH_SESSION = 0x01ED {
///     u32 client_build;
///     u32 login_server_id;
///     CString username;
///     u32 login_server_type;
///     u32 client_seed;
///     u32 region_id;
///     u32 battleground_id;
///     u32 realm_id;
///     u64 dos_response;
///     u8[20] client_proof;
///     u32 decompressed_addon_info_size;
///     u8[-] decompressed_addon_info;
/// }
/// ```
pub struct CMSG_AUTH_SESSION {
    pub client_build: u32,
    pub login_server_id: u32,
    pub username: String,
    pub login_server_type: u32,
    pub client_seed: u32,
    pub region_id: u32,
    pub battleground_id: u32,
    pub realm_id: u32,
    /// Purpose and exact meaning of name unknown.
    /// TrinityCore has this name but never uses the variable afterwards.
    ///
    pub dos_response: u64,
    pub client_proof: [u8; 20],
    pub decompressed_addon_info_size: u32,
    pub decompressed_addon_info: Vec<u8>,
}

impl crate::Message for CMSG_AUTH_SESSION {
    const OPCODE: u32 = 0x01ed;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // client_build: u32
        w.write_all(&self.client_build.to_le_bytes())?;

        // login_server_id: u32
        w.write_all(&self.login_server_id.to_le_bytes())?;

        // username: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.username.as_bytes().iter().rev().next(), Some(&0_u8), "String `username` must not be null-terminated.");
        w.write_all(self.username.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // login_server_type: u32
        w.write_all(&self.login_server_type.to_le_bytes())?;

        // client_seed: u32
        w.write_all(&self.client_seed.to_le_bytes())?;

        // region_id: u32
        w.write_all(&self.region_id.to_le_bytes())?;

        // battleground_id: u32
        w.write_all(&self.battleground_id.to_le_bytes())?;

        // realm_id: u32
        w.write_all(&self.realm_id.to_le_bytes())?;

        // dos_response: u64
        w.write_all(&self.dos_response.to_le_bytes())?;

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // decompressed_addon_info_size: u32
        w.write_all(&self.decompressed_addon_info_size.to_le_bytes())?;

        // decompressed_addon_info: u8[-]
        for i in self.decompressed_addon_info.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // client_build: u32
        let client_build = crate::util::read_u32_le(r)?;

        // login_server_id: u32
        let login_server_id = crate::util::read_u32_le(r)?;

        // username: CString
        let username = crate::util::read_c_string_to_vec(r)?;
        let username = String::from_utf8(username)?;

        // login_server_type: u32
        let login_server_type = crate::util::read_u32_le(r)?;

        // client_seed: u32
        let client_seed = crate::util::read_u32_le(r)?;

        // region_id: u32
        let region_id = crate::util::read_u32_le(r)?;

        // battleground_id: u32
        let battleground_id = crate::util::read_u32_le(r)?;

        // realm_id: u32
        let realm_id = crate::util::read_u32_le(r)?;

        // dos_response: u64
        let dos_response = crate::util::read_u64_le(r)?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof)?;

        // decompressed_addon_info_size: u32
        let decompressed_addon_info_size = crate::util::read_u32_le(r)?;

        // decompressed_addon_info: u8[-]
        let mut current_size = {
            4 // client_build: u32
            + 4 // login_server_id: u32
            + username.len() + 1 // username: CString
            + 4 // login_server_type: u32
            + 4 // client_seed: u32
            + 4 // region_id: u32
            + 4 // battleground_id: u32
            + 4 // realm_id: u32
            + 8 // dos_response: u64
            + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
            + 4 // decompressed_addon_info_size: u32
        };
        let mut decompressed_addon_info = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            decompressed_addon_info.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        let mut decompressed_addon_info_temp = Vec::new();
         {
            use flate2::read::ZlibDecoder;
            let mut decoder = ZlibDecoder::new(decompressed_addon_info.as_slice());
            decoder.read_to_end(&mut decompressed_addon_info_temp)?;
        }

        let mut decompressed_addon_info = decompressed_addon_info_temp;

        Ok(Self {
            client_build,
            login_server_id,
            username,
            login_server_type,
            client_seed,
            region_id,
            battleground_id,
            realm_id,
            dos_response,
            client_proof,
            decompressed_addon_info_size,
            decompressed_addon_info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_AUTH_SESSION {}

impl CMSG_AUTH_SESSION {
    pub(crate) fn size(&self) -> usize {
        4 // client_build: u32
        + 4 // login_server_id: u32
        + self.username.len() + 1 // username: CString
        + 4 // login_server_type: u32
        + 4 // client_seed: u32
        + 4 // region_id: u32
        + 4 // battleground_id: u32
        + 4 // realm_id: u32
        + 8 // dos_response: u64
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 4 // decompressed_addon_info_size: u32
        + self.decompressed_addon_info.len() * core::mem::size_of::<u8>() // decompressed_addon_info: u8[-]
    }
}

