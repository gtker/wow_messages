use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::ChatNotify;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
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

impl ServerMessage for SMSG_CHANNEL_NOTIFY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // notify_type: ChatNotify
        w.write_all(&(self.notify_type.as_int() as u8).to_le_bytes())?;

        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0099;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // notify_type: ChatNotify
        let notify_type: ChatNotify = crate::util::read_u8_le(r)?.try_into()?;

        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        Ok(Self {
            notify_type,
            channel_name,
        })
    }

}

impl SMSG_CHANNEL_NOTIFY {
    pub(crate) fn size(&self) -> usize {
        1 // notify_type: ChatNotify
        + self.channel_name.len() + 1 // channel_name: CString
    }
}

