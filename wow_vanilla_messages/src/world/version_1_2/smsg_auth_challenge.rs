use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm#L3):
/// ```text
/// smsg SMSG_AUTH_CHALLENGE = 0x01EC {
///     u32 server_seed;
/// }
/// ```
/// # Description
///
/// Seed used by the client to prove in [CMSG_AUTH_SESSION](crate::world::version_1_12::CMSG_AUTH_SESSION) that it has authenticated with the auth server.
/// # Comment
///
/// First thing sent when a client connects to the world server.
/// This message is always unencrypted.
pub struct SMSG_AUTH_CHALLENGE {
    pub server_seed: u32,
}

impl ServerMessage for SMSG_AUTH_CHALLENGE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // server_seed: u32
        w.write_all(&self.server_seed.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01ec;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // server_seed: u32
        let server_seed = crate::util::read_u32_le(r)?;

        Ok(Self {
            server_seed,
        })
    }

}

#[cfg(test)]
mod test {
    use super::SMSG_AUTH_CHALLENGE;
    use super::*;
    use super::super::*;
    use crate::world::version_1_2::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xEC, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
