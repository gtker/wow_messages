use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm#L3):
/// ```text
/// smsg SMSG_ACCOUNT_DATA_TIMES = 0x209 {
///     u32[32] data;
/// }
/// ```
pub struct SMSG_ACCOUNT_DATA_TIMES {
    pub data: [u32; 32],
}

impl WorldServerMessageWrite for SMSG_ACCOUNT_DATA_TIMES {
    const OPCODE: u16 = 0x209;

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
impl WorldMessageBody for SMSG_ACCOUNT_DATA_TIMES {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // data: u32[32]
        let mut data = [u32::default(); 32];
        for i in 0..32 {
            data[i] = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            data,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // data: u32[32]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl ConstantSized for SMSG_ACCOUNT_DATA_TIMES {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ACCOUNT_DATA_TIMES {
    fn maximum_possible_size() -> usize {
        32 * core::mem::size_of::<u32>() // data: u32[32]
    }
}

