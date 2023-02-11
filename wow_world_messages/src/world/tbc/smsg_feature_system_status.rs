use crate::tbc::ComplaintStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm#L9):
/// ```text
/// smsg SMSG_FEATURE_SYSTEM_STATUS = 0x03C8 {
///     ComplaintStatus complaint_status;
///     Bool voice_chat_enabled;
/// }
/// ```
pub struct SMSG_FEATURE_SYSTEM_STATUS {
    pub complaint_status: ComplaintStatus,
    pub voice_chat_enabled: bool,
}

impl crate::Message for SMSG_FEATURE_SYSTEM_STATUS {
    const OPCODE: u32 = 0x03c8;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // complaint_status: ComplaintStatus
        w.write_all(&(self.complaint_status.as_int() as u8).to_le_bytes())?;

        // voice_chat_enabled: Bool
        w.write_all(u8::from(self.voice_chat_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C8, size: body_size as u32 });
        }

        // complaint_status: ComplaintStatus
        let complaint_status: ComplaintStatus = crate::util::read_u8_le(r)?.try_into()?;

        // voice_chat_enabled: Bool
        let voice_chat_enabled = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            complaint_status,
            voice_chat_enabled,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FEATURE_SYSTEM_STATUS {}

