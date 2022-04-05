use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:703`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L703):
/// ```text
/// smsg SMSG_GMTICKET_SYSTEMSTATUS = 0x21B {
///     u32 will_accept_tickets;
/// }
/// ```
pub struct SMSG_GMTICKET_SYSTEMSTATUS {
    pub will_accept_tickets: u32,
}

impl WorldServerMessageWrite for SMSG_GMTICKET_SYSTEMSTATUS {
    const OPCODE: u16 = 0x21b;

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
impl WorldMessageBody for SMSG_GMTICKET_SYSTEMSTATUS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // will_accept_tickets: u32
        let will_accept_tickets = crate::util::read_u32_le(r)?;

        Ok(Self {
            will_accept_tickets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // will_accept_tickets: u32
        w.write_all(&self.will_accept_tickets.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_GMTICKET_SYSTEMSTATUS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_GMTICKET_SYSTEMSTATUS {
    fn maximum_possible_size() -> usize {
        4 // will_accept_tickets: u32
    }
}

