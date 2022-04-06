use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_action.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_action.wowm#L3):
/// ```text
/// cmsg CMSG_PET_ACTION = 0x175 {
///     u64 pet_guid;
///     u32 data;
///     u64 target_guid;
/// }
/// ```
pub struct CMSG_PET_ACTION {
    pub pet_guid: u64,
    pub data: u32,
    pub target_guid: u64,
}

impl WorldClientMessageWrite for CMSG_PET_ACTION {
    const OPCODE: u32 = 0x175;

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
impl WorldMessageBody for CMSG_PET_ACTION {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: u64
        let pet_guid = crate::util::read_u64_le(r)?;

        // data: u32
        let data = crate::util::read_u32_le(r)?;

        // target_guid: u64
        let target_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            pet_guid,
            data,
            target_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: u64
        w.write_all(&self.pet_guid.to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // target_guid: u64
        w.write_all(&self.target_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PET_ACTION {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_PET_ACTION {
    fn maximum_possible_size() -> usize {
        8 // pet_guid: u64
        + 4 // data: u32
        + 8 // target_guid: u64
    }
}

