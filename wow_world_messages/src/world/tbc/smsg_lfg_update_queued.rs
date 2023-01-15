use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update_queued.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update_queued.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_QUEUED = 0x036F {
///     Bool queued;
/// }
/// ```
pub struct SMSG_LFG_UPDATE_QUEUED {
    pub queued: bool,
}

impl crate::Message for SMSG_LFG_UPDATE_QUEUED {
    const OPCODE: u32 = 0x036f;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // queued: Bool
        w.write_all(u8::from(self.queued).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036F, size: body_size as u32 });
        }

        // queued: Bool
        let queued = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            queued,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_LFG_UPDATE_QUEUED {}

