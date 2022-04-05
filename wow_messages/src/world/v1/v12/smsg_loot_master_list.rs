use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:1030`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L1030):
/// ```text
/// smsg SMSG_LOOT_MASTER_LIST = 0x2A4 {
///     u8 amount_of_players;
///     Guid[amount_of_players] guids;
/// }
/// ```
pub struct SMSG_LOOT_MASTER_LIST {
    pub amount_of_players: u8,
    pub guids: Vec<Guid>,
}

impl WorldServerMessageWrite for SMSG_LOOT_MASTER_LIST {
    const OPCODE: u16 = 0x2a4;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_LOOT_MASTER_LIST {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_players: u8
        let amount_of_players = crate::util::read_u8_le(r)?;

        // guids: Guid[amount_of_players]
        let mut guids = Vec::with_capacity(amount_of_players as usize);
        for i in 0..amount_of_players {
            guids.push(Guid::read(r)?);
        }

        Ok(Self {
            amount_of_players,
            guids,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_players: u8
        w.write_all(&(self.guids.len() as u8).to_le_bytes())?;

        // guids: Guid[amount_of_players]
        for i in self.guids.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_LOOT_MASTER_LIST {
    fn size(&self) -> usize {
        1 // amount_of_players: u8
        + 8 // guids: Guid[amount_of_players]
    }
}

impl MaximumPossibleSized for SMSG_LOOT_MASTER_LIST {
    fn maximum_possible_size() -> usize {
        1 // amount_of_players: u8
        + 255 * 8 // guids: Guid[amount_of_players]
    }
}

