use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_client.wowm#L3):
/// ```text
/// cmsg MSG_PVP_LOG_DATA_Client = 0x02E0 {
/// }
/// ```
pub struct MSG_PVP_LOG_DATA_Client {
}

impl crate::private::Sealed for MSG_PVP_LOG_DATA_Client {}
impl crate::Message for MSG_PVP_LOG_DATA_Client {
    const OPCODE: u32 = 0x02e0;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E0, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_PVP_LOG_DATA_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_PVP_LOG_DATA_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_PVP_LOG_DATA_Client {}

