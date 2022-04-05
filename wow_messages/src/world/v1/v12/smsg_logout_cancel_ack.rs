use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_cancel_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_cancel_ack.wowm#L3):
/// ```text
/// smsg SMSG_LOGOUT_CANCEL_ACK = 0x4F {
/// }
/// ```
pub struct SMSG_LOGOUT_CANCEL_ACK {
}

impl WorldServerMessageWrite for SMSG_LOGOUT_CANCEL_ACK {
    const OPCODE: u16 = 0x4f;

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
impl WorldMessageBody for SMSG_LOGOUT_CANCEL_ACK {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}

impl ConstantSized for SMSG_LOGOUT_CANCEL_ACK {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOGOUT_CANCEL_ACK {
    fn maximum_possible_size() -> usize {
        0
    }
}

