use std::io::{Read, Write};

use crate::wrath::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_queue_request_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_queue_request_response.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_QUEUE_REQUEST_RESPONSE = 0x04E4 {
///     u32 battle_id;
///     Area area;
///     Bool queued;
///     Bool full;
///     Bool warmup;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_QUEUE_REQUEST_RESPONSE {
    pub battle_id: u32,
    pub area: Area,
    pub queued: bool,
    pub full: bool,
    pub warmup: bool,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_QUEUE_REQUEST_RESPONSE {}
impl crate::Message for SMSG_BATTLEFIELD_MGR_QUEUE_REQUEST_RESPONSE {
    const OPCODE: u32 = 0x04e4;

    fn size_without_header(&self) -> u32 {
        11
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // queued: Bool
        w.write_all(u8::from(self.queued).to_le_bytes().as_slice())?;

        // full: Bool
        w.write_all(u8::from(self.full).to_le_bytes().as_slice())?;

        // warmup: Bool
        w.write_all(u8::from(self.warmup).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 11 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E4, size: body_size });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // queued: Bool
        let queued = crate::util::read_u8_le(&mut r)? != 0;

        // full: Bool
        let full = crate::util::read_u8_le(&mut r)? != 0;

        // warmup: Bool
        let warmup = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            battle_id,
            area,
            queued,
            full,
            warmup,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_QUEUE_REQUEST_RESPONSE {}

