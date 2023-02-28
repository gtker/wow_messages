use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// mangoszero/vmangos: No idea when this is called.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm#L3):
/// ```text
/// cmsg MSG_QUERY_NEXT_MAIL_TIME_Client = 0x0284 {
/// }
/// ```
pub struct MSG_QUERY_NEXT_MAIL_TIME_Client {
}

impl crate::Message for MSG_QUERY_NEXT_MAIL_TIME_Client {
    const OPCODE: u32 = 0x0284;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0284, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_QUERY_NEXT_MAIL_TIME_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_QUERY_NEXT_MAIL_TIME_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_QUERY_NEXT_MAIL_TIME_Client {}

