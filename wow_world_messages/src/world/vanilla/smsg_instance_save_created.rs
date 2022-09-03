use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm#L3):
/// ```text
/// smsg SMSG_INSTANCE_SAVE_CREATED = 0x02CB {
///     u32 unknown;
/// }
/// ```
pub struct SMSG_INSTANCE_SAVE_CREATED {
    /// cmangos/vmangos/mangoszero set to 0
    ///
    pub unknown: u32,
}

impl crate::Message for SMSG_INSTANCE_SAVE_CREATED {
    const OPCODE: u32 = 0x02cb;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown,
        })
    }

}
impl ServerMessage for SMSG_INSTANCE_SAVE_CREATED {}

