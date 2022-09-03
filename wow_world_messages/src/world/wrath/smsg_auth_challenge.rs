use std::convert::{TryFrom, TryInto};
use crate::world::wrath::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm#L21):
/// ```text
/// smsg SMSG_AUTH_CHALLENGE = 0x01EC {
///     u32 unknown1;
///     u32 realm_seed;
///     u8[32] seed;
/// }
/// ```
pub struct SMSG_AUTH_CHALLENGE {
    /// TrinityCore/ArcEmu/mangostwo always set to 1.
    /// TrinityCore/mangostwo: 1...31
    ///
    pub unknown1: u32,
    pub realm_seed: u32,
    /// Randomized values. Is not used at all by TrinityCore/mangostwo/ArcEmu.
    ///
    pub seed: [u8; 32],
}

impl crate::Message for SMSG_AUTH_CHALLENGE {
    const OPCODE: u32 = 0x01ec;

    fn size_without_header(&self) -> u32 {
        40
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // realm_seed: u32
        w.write_all(&self.realm_seed.to_le_bytes())?;

        // seed: u8[32]
        for i in self.seed.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 40 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // realm_seed: u32
        let realm_seed = crate::util::read_u32_le(r)?;

        // seed: u8[32]
        let mut seed = [0_u8; 32];
        r.read_exact(&mut seed)?;

        Ok(Self {
            unknown1,
            realm_seed,
            seed,
        })
    }

}
impl ServerMessage for SMSG_AUTH_CHALLENGE {}

