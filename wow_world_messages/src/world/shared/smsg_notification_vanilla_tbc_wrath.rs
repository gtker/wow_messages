use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_notification.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_notification.wowm#L3):
/// ```text
/// smsg SMSG_NOTIFICATION = 0x01CB {
///     CString notification;
/// }
/// ```
pub struct SMSG_NOTIFICATION {
    pub notification: String,
}

impl crate::Message for SMSG_NOTIFICATION {
    const OPCODE: u32 = 0x01cb;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // notification: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.notification.as_bytes().iter().rev().next(), Some(&0_u8), "String `notification` must not be null-terminated.");
        w.write_all(self.notification.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01CB, size: body_size as u32 });
        }

        // notification: CString
        let notification = crate::util::read_c_string_to_vec(r)?;
        let notification = String::from_utf8(notification)?;

        Ok(Self {
            notification,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_NOTIFICATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_NOTIFICATION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_NOTIFICATION {}

impl SMSG_NOTIFICATION {
    pub(crate) fn size(&self) -> usize {
        self.notification.len() + 1 // notification: CString
    }
}

