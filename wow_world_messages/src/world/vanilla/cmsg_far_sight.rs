use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::FarSightOperation;
use crate::world::vanilla::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm#L8):
/// ```text
/// cmsg CMSG_FAR_SIGHT = 0x027A {
///     FarSightOperation operation;
/// }
/// ```
pub struct CMSG_FAR_SIGHT {
    pub operation: FarSightOperation,
}

impl crate::Message for CMSG_FAR_SIGHT {
    const OPCODE: u32 = 0x027a;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // operation: FarSightOperation
        w.write_all(&(self.operation.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // operation: FarSightOperation
        let operation: FarSightOperation = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            operation,
        })
    }

}
impl ClientMessage for CMSG_FAR_SIGHT {}

