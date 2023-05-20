use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_looking_for_group_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_looking_for_group_client.wowm#L3):
/// ```text
/// cmsg MSG_LOOKING_FOR_GROUP_Client = 0x01FF {
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Client {
}

impl crate::private::Sealed for MSG_LOOKING_FOR_GROUP_Client {}
impl crate::Message for MSG_LOOKING_FOR_GROUP_Client {
    const OPCODE: u32 = 0x01ff;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FF, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_LOOKING_FOR_GROUP_Client {}

