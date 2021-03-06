use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOEQUIP_ITEM_SLOT = 0x010F {
///     Guid guid;
///     u8 destination_slot;
/// }
/// ```
pub struct CMSG_AUTOEQUIP_ITEM_SLOT {
    pub guid: Guid,
    pub destination_slot: u8,
}

impl ClientMessage for CMSG_AUTOEQUIP_ITEM_SLOT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x010f;

    fn client_size(&self) -> u16 {
        15
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            destination_slot,
        })
    }

}

