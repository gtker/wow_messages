use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:3219`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L3219):
/// ```text
/// cmsg CMSG_PET_UNLEARN = 0x2F0 {
///     u64 pet_guid;
/// }
/// ```
pub struct CMSG_PET_UNLEARN {
    pub pet_guid: u64,
}

impl WorldClientMessageWrite for CMSG_PET_UNLEARN {
    const OPCODE: u32 = 0x2f0;

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
impl WorldMessageBody for CMSG_PET_UNLEARN {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: u64
        let pet_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            pet_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: u64
        w.write_all(&self.pet_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PET_UNLEARN {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_PET_UNLEARN {
    fn maximum_possible_size() -> usize {
        8 // pet_guid: u64
    }
}

