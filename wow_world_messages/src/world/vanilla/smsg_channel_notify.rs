use crate::vanilla::ChatNotify;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm:100`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm#L100):
/// ```text
/// smsg SMSG_CHANNEL_NOTIFY = 0x0099 {
///     ChatNotify notify_type;
///     CString channel_name;
/// }
/// ```
pub struct SMSG_CHANNEL_NOTIFY {
    pub notify_type: ChatNotify,
    pub channel_name: String,
}

impl crate::Message for SMSG_CHANNEL_NOTIFY {
    const OPCODE: u32 = 0x0099;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // notify_type: ChatNotify
        w.write_all(&u8::from(self.notify_type.as_int()).to_le_bytes())?;

        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0099, size: body_size as u32 });
        }

        // notify_type: ChatNotify
        let notify_type: ChatNotify = crate::util::read_u8_le(&mut r)?.try_into()?;

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        Ok(Self {
            notify_type,
            channel_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CHANNEL_NOTIFY {}

impl SMSG_CHANNEL_NOTIFY {
    pub(crate) fn size(&self) -> usize {
        1 // notify_type: ChatNotify
        + self.channel_name.len() + 1 // channel_name: CString
    }
}

