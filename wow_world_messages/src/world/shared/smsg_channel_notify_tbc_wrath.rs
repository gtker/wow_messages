use crate:: {
};
use wow_world_base::shared::chat_notify_tbc_wrath::ChatNotify;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm:218`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm#L218):
/// ```text
/// smsg SMSG_CHANNEL_NOTIFY = 0x0099 {
///     ChatNotify notify_type;
///     CString channel_name;
///     optional unknown1 {
///         u32 unknown2;
///         u32 unkwown3;
///     }
/// }
/// ```
pub struct SMSG_CHANNEL_NOTIFY {
    pub notify_type: ChatNotify,
    pub channel_name: String,
    pub unknown1: Option<SMSG_CHANNEL_NOTIFY_unknown1>,
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

        // optional unknown1
        if let Some(v) = &self.unknown1 {
            // unknown2: u32
            w.write_all(&v.unknown2.to_le_bytes())?;

            // unkwown3: u32
            w.write_all(&v.unkwown3.to_le_bytes())?;

        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=265).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0099, size: body_size as u32 });
        }

        // notify_type: ChatNotify
        let notify_type: ChatNotify = crate::util::read_u8_le(&mut r)?.try_into()?;

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        // optional unknown1
        let current_size = {
            1 // notify_type: ChatNotify
            + channel_name.len() + 1 // channel_name: CString
        };
        let unknown1 = if current_size < body_size as usize {
            // unknown2: u32
            let unknown2 = crate::util::read_u32_le(&mut r)?;

            // unkwown3: u32
            let unkwown3 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_CHANNEL_NOTIFY_unknown1 {
                unknown2,
                unkwown3,
            })
        } else {
            None
        };

        Ok(Self {
            notify_type,
            channel_name,
            unknown1,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHANNEL_NOTIFY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHANNEL_NOTIFY {}

impl SMSG_CHANNEL_NOTIFY {
    pub(crate) fn size(&self) -> usize {
        1 // notify_type: ChatNotify
        + self.channel_name.len() + 1 // channel_name: CString
        + if let Some(unknown1) = &self.unknown1 {
            4 // unknown2: u32
            + 4 // unkwown3: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CHANNEL_NOTIFY_unknown1 {
    pub unknown2: u32,
    pub unkwown3: u32,
}

impl SMSG_CHANNEL_NOTIFY_unknown1 {
    pub(crate) fn size(&self) -> usize {
        4 // unknown2: u32
        + 4 // unkwown3: u32
    }

}

