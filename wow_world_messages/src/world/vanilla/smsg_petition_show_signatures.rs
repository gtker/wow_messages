use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm#L3):
/// ```text
/// smsg SMSG_PETITION_SHOW_SIGNATURES = 0x01BF {
///     Guid item_guid;
///     Guid owner_guid;
///     Guid petition_guid;
///     u8 amount_of_signatures;
/// }
/// ```
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item_guid: Guid,
    pub owner_guid: Guid,
    pub petition_guid: Guid,
    pub amount_of_signatures: u8,
}

impl crate::Message for SMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u32 = 0x01bf;

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // owner_guid: Guid
        w.write_all(&self.owner_guid.guid().to_le_bytes())?;

        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // amount_of_signatures: u8
        w.write_all(&self.amount_of_signatures.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_guid,
            owner_guid,
            petition_guid,
            amount_of_signatures,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

