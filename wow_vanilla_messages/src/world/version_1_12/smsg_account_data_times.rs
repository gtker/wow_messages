use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm#L3):
/// ```text
/// smsg SMSG_ACCOUNT_DATA_TIMES = 0x0209 {
///     u32[32] data;
/// }
/// ```
pub struct SMSG_ACCOUNT_DATA_TIMES {
    pub data: [u32; 32],
}

impl ServerMessage for SMSG_ACCOUNT_DATA_TIMES {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // data: u32[32]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0209;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        128
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // data: u32[32]
        let mut data = [u32::default(); 32];
        for i in data.iter_mut() {
            *i = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            data,
        })
    }

}

