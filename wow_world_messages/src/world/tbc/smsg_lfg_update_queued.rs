use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_queued.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_queued.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_QUEUED = 0x036F {
///     Bool queued;
/// }
/// ```
pub struct SMSG_LFG_UPDATE_QUEUED {
    pub queued: bool,
}

impl crate::private::Sealed for SMSG_LFG_UPDATE_QUEUED {}
impl crate::Message for SMSG_LFG_UPDATE_QUEUED {
    const OPCODE: u32 = 0x036f;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // queued: Bool
        w.write_all(u8::from(self.queued).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036F, size: body_size });
        }

        // queued: Bool
        let queued = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            queued,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LFG_UPDATE_QUEUED {}

