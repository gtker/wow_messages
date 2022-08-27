use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm#L21):
/// ```text
/// smsg SMSG_AUTH_CHALLENGE = 0x01EC {
///     u32 unknown1;
///     u32 realm_seed;
///     u32 seed1;
///     u32 seed2;
/// }
/// ```
pub struct SMSG_AUTH_CHALLENGE {
    pub unknown1: u32,
    pub realm_seed: u32,
    pub seed1: u32,
    pub seed2: u32,
}

impl ServerMessage for SMSG_AUTH_CHALLENGE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // realm_seed: u32
        w.write_all(&self.realm_seed.to_le_bytes())?;

        // seed1: u32
        w.write_all(&self.seed1.to_le_bytes())?;

        // seed2: u32
        w.write_all(&self.seed2.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01ec;

    fn server_size(&self) -> u16 {
        20
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // realm_seed: u32
        let realm_seed = crate::util::read_u32_le(r)?;

        // seed1: u32
        let seed1 = crate::util::read_u32_le(r)?;

        // seed2: u32
        let seed2 = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown1,
            realm_seed,
            seed1,
            seed2,
        })
    }

}

