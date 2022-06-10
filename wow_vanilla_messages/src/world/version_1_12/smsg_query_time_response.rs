use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm#L3):
/// ```text
/// smsg SMSG_QUERY_TIME_RESPONSE = 0x01CF {
///     u32 time;
/// }
/// ```
pub struct SMSG_QUERY_TIME_RESPONSE {
    pub time: u32,
}

impl ServerMessage for SMSG_QUERY_TIME_RESPONSE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01cf;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // time: u32
        let time = crate::util::read_u32_le(r)?;

        Ok(Self {
            time,
        })
    }

}

