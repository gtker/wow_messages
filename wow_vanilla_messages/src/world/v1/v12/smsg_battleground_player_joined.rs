use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm#L3):
/// ```text
/// smsg SMSG_BATTLEGROUND_PLAYER_JOINED = 0x2EC {
///     u64 player_guid;
/// }
/// ```
pub struct SMSG_BATTLEGROUND_PLAYER_JOINED {
    pub player_guid: u64,
}

impl WorldServerMessageWrite for SMSG_BATTLEGROUND_PLAYER_JOINED {
    const OPCODE: u16 = 0x2ec;

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
impl WorldMessageBody for SMSG_BATTLEGROUND_PLAYER_JOINED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_guid: u64
        let player_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            player_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_guid: u64
        w.write_all(&self.player_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_BATTLEGROUND_PLAYER_JOINED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_BATTLEGROUND_PLAYER_JOINED {
    fn maximum_possible_size() -> usize {
        8 // player_guid: u64
    }
}

