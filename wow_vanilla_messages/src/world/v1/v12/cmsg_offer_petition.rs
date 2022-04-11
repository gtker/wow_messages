use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm#L3):
/// ```text
/// cmsg CMSG_OFFER_PETITION = 0x1C3 {
///     Guid petition_guid;
///     Guid target_guid;
/// }
/// ```
pub struct CMSG_OFFER_PETITION {
    pub petition_guid: Guid,
    pub target_guid: Guid,
}

impl WorldClientMessageWrite for CMSG_OFFER_PETITION {
    const OPCODE: u32 = 0x1c3;

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
impl WorldMessageBody for CMSG_OFFER_PETITION {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            petition_guid,
            target_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_OFFER_PETITION {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_OFFER_PETITION {
    fn maximum_possible_size() -> usize {
        8 // petition_guid: Guid
        + 8 // target_guid: Guid
    }
}

