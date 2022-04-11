use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_UNSTABLE_PET {
    pub npc_guid: Guid,
    pub pet_number: u32,
}

impl WorldClientMessageWrite for CMSG_UNSTABLE_PET {
    const OPCODE: u32 = 0x271;

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
impl WorldMessageBody for CMSG_UNSTABLE_PET {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc_guid,
            pet_number,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.write(w)?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_UNSTABLE_PET {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_UNSTABLE_PET {
    fn maximum_possible_size() -> usize {
        8 // npc_guid: Guid
        + 4 // pet_number: u32
    }
}

