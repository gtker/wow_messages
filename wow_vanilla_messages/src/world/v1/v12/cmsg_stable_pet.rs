use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_stable_pet.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_stable_pet.wowm#L3):
/// ```text
/// cmsg CMSG_STABLE_PET = 0x270 {
///     Guid npc_guid;
/// }
/// ```
pub struct CMSG_STABLE_PET {
    pub npc_guid: Guid,
}

impl WorldClientMessageWrite for CMSG_STABLE_PET {
    const OPCODE: u32 = 0x270;

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
impl WorldMessageBody for CMSG_STABLE_PET {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        Ok(Self {
            npc_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_STABLE_PET {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_STABLE_PET {
    fn maximum_possible_size() -> usize {
        8 // npc_guid: Guid
    }
}

