use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_creature_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_creature_query.wowm#L3):
/// ```text
/// cmsg CMSG_CREATURE_QUERY = 0x0060 {
///     u32 creature;
///     Guid guid;
/// }
/// ```
pub struct CMSG_CREATURE_QUERY {
    pub creature: u32,
    pub guid: Guid,
}

impl crate::Message for CMSG_CREATURE_QUERY {
    const OPCODE: u32 = 0x0060;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // creature: u32
        w.write_all(&self.creature.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0060, size: body_size as u32 });
        }

        // creature: u32
        let creature = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            creature,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_CREATURE_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CREATURE_QUERY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CREATURE_QUERY {}

