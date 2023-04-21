use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_corpse_query_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_corpse_query_client.wowm#L3):
/// ```text
/// cmsg MSG_CORPSE_QUERY_Client = 0x0216 {
/// }
/// ```
pub struct MSG_CORPSE_QUERY_Client {
}

impl crate::private::Sealed for MSG_CORPSE_QUERY_Client {}
impl crate::Message for MSG_CORPSE_QUERY_Client {
    const OPCODE: u32 = 0x0216;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0216, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_CORPSE_QUERY_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_CORPSE_QUERY_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_CORPSE_QUERY_Client {}

