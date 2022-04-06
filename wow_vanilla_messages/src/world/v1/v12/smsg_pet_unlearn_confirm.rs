use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm#L3):
/// ```text
/// smsg SMSG_PET_UNLEARN_CONFIRM = 0x2F1 {
///     u64 pet_guid;
///     u32 talent_reset_cost;
/// }
/// ```
pub struct SMSG_PET_UNLEARN_CONFIRM {
    pub pet_guid: u64,
    pub talent_reset_cost: u32,
}

impl WorldServerMessageWrite for SMSG_PET_UNLEARN_CONFIRM {
    const OPCODE: u16 = 0x2f1;

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
impl WorldMessageBody for SMSG_PET_UNLEARN_CONFIRM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: u64
        let pet_guid = crate::util::read_u64_le(r)?;

        // talent_reset_cost: u32
        let talent_reset_cost = crate::util::read_u32_le(r)?;

        Ok(Self {
            pet_guid,
            talent_reset_cost,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: u64
        w.write_all(&self.pet_guid.to_le_bytes())?;

        // talent_reset_cost: u32
        w.write_all(&self.talent_reset_cost.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PET_UNLEARN_CONFIRM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PET_UNLEARN_CONFIRM {
    fn maximum_possible_size() -> usize {
        8 // pet_guid: u64
        + 4 // talent_reset_cost: u32
    }
}

