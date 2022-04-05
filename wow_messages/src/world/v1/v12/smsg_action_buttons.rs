use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:1736`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L1736):
/// ```text
/// smsg SMSG_ACTION_BUTTONS = 0x129 {
///     u32[120] data;
/// }
/// ```
pub struct SMSG_ACTION_BUTTONS {
    pub data: [u32; 120],
}

impl WorldServerMessageWrite for SMSG_ACTION_BUTTONS {
    const OPCODE: u16 = 0x129;

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
impl WorldMessageBody for SMSG_ACTION_BUTTONS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // data: u32[120]
        let mut data = [u32::default(); 120];
        for i in 0..120 {
            data[i] = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            data,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // data: u32[120]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl ConstantSized for SMSG_ACTION_BUTTONS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ACTION_BUTTONS {
    fn maximum_possible_size() -> usize {
        120 * core::mem::size_of::<u32>() // data: u32[120]
    }
}

