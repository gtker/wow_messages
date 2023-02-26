use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_client.wowm#L3):
/// ```text
/// cmsg MSG_INSPECT_HONOR_STATS_Client = 0x02D6 {
///     Guid guid;
/// }
/// ```
pub struct MSG_INSPECT_HONOR_STATS_Client {
    pub guid: Guid,
}

impl crate::Message for MSG_INSPECT_HONOR_STATS_Client {
    const OPCODE: u32 = 0x02d6;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D6, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_INSPECT_HONOR_STATS_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_INSPECT_HONOR_STATS_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_INSPECT_HONOR_STATS_Client {}

