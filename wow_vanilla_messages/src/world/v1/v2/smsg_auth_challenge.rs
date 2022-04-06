use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm#L3):
/// ```text
/// smsg SMSG_AUTH_CHALLENGE = 0x1EC {
///     u32 server_seed;
/// }
/// ```
pub struct SMSG_AUTH_CHALLENGE {
    pub server_seed: u32,
}

impl WorldServerMessageWrite for SMSG_AUTH_CHALLENGE {
    const OPCODE: u16 = 0x1ec;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_AUTH_CHALLENGE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // server_seed: u32
        let server_seed = crate::util::read_u32_le(r)?;

        Ok(Self {
            server_seed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // server_seed: u32
        w.write_all(&self.server_seed.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_AUTH_CHALLENGE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_AUTH_CHALLENGE {
    fn maximum_possible_size() -> usize {
        4 // server_seed: u32
    }
}

