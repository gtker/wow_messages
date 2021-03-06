use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_stop_attack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_stop_attack.wowm#L3):
/// ```text
/// cmsg CMSG_PET_STOP_ATTACK = 0x02EA {
///     Guid pet_guid;
/// }
/// ```
pub struct CMSG_PET_STOP_ATTACK {
    pub pet_guid: Guid,
}

impl ClientMessage for CMSG_PET_STOP_ATTACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02ea;

    fn client_size(&self) -> u16 {
        14
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        Ok(Self {
            pet_guid,
        })
    }

}

