use wow_world_base::shared::server_message_type_vanilla_tbc::ServerMessageType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_server_message.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_server_message.wowm#L11):
/// ```text
/// smsg SMSG_SERVER_MESSAGE = 0x0291 {
///     ServerMessageType message_type;
///     CString message;
/// }
/// ```
pub struct SMSG_SERVER_MESSAGE {
    pub message_type: ServerMessageType,
    pub message: String,
}

impl crate::Message for SMSG_SERVER_MESSAGE {
    const OPCODE: u32 = 0x0291;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // message_type: ServerMessageType
        w.write_all(&u32::from(self.message_type.as_int()).to_le_bytes())?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0291, size: body_size as u32 });
        }

        // message_type: ServerMessageType
        let message_type: ServerMessageType = crate::util::read_u32_le(r)?.try_into()?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            message_type,
            message,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SERVER_MESSAGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SERVER_MESSAGE {}

impl SMSG_SERVER_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        4 // message_type: ServerMessageType
        + self.message.len() + 1 // message: CString
    }
}

