use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:215`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L215):
/// ```text
/// cmsg CMSG_LOOT_MASTER_GIVE = 0x2A3 {
///     u64 loot_guid;
///     u8 slot_id;
///     u64 target_player_guid;
/// }
/// ```
pub struct CMSG_LOOT_MASTER_GIVE {
    pub loot_guid: u64,
    pub slot_id: u8,
    pub target_player_guid: u64,
}

impl WorldClientMessageWrite for CMSG_LOOT_MASTER_GIVE {
    const OPCODE: u32 = 0x2a3;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_LOOT_MASTER_GIVE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_guid: u64
        let loot_guid = crate::util::read_u64_le(r)?;

        // slot_id: u8
        let slot_id = crate::util::read_u8_le(r)?;

        // target_player_guid: u64
        let target_player_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            loot_guid,
            slot_id,
            target_player_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_guid: u64
        w.write_all(&self.loot_guid.to_le_bytes())?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes())?;

        // target_player_guid: u64
        w.write_all(&self.target_player_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_LOOT_MASTER_GIVE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_LOOT_MASTER_GIVE {
    fn maximum_possible_size() -> usize {
        8 // loot_guid: u64
        + 1 // slot_id: u8
        + 8 // target_player_guid: u64
    }
}

