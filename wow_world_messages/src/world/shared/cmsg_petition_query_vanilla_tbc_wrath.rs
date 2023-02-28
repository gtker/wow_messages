use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_petition_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_petition_query.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_QUERY = 0x01C6 {
///     u32 guild_id;
///     Guid petition;
/// }
/// ```
pub struct CMSG_PETITION_QUERY {
    pub guild_id: u32,
    pub petition: Guid,
}

impl crate::Message for CMSG_PETITION_QUERY {
    const OPCODE: u32 = 0x01c6;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C6, size: body_size as u32 });
        }

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(&mut r)?;

        // petition: Guid
        let petition = Guid::read(&mut r)?;

        Ok(Self {
            guild_id,
            petition,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PETITION_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PETITION_QUERY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PETITION_QUERY {}

