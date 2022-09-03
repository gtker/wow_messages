use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_action.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_action.wowm#L3):
/// ```text
/// cmsg CMSG_PET_ACTION = 0x0175 {
///     Guid pet_guid;
///     u32 data;
///     Guid target_guid;
/// }
/// ```
pub struct CMSG_PET_ACTION {
    pub pet_guid: Guid,
    pub data: u32,
    pub target_guid: Guid,
}

impl crate::Message for CMSG_PET_ACTION {
    const OPCODE: u32 = 0x0175;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // data: u32
        let data = crate::util::read_u32_le(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            pet_guid,
            data,
            target_guid,
        })
    }

}
impl ClientMessage for CMSG_PET_ACTION {}

