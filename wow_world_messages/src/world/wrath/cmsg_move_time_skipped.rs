use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped_3_3_5.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped_3_3_5.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_TIME_SKIPPED = 0x02CE {
///     PackedGuid guid;
///     u32 lag;
/// }
/// ```
pub struct CMSG_MOVE_TIME_SKIPPED {
    pub guid: Guid,
    pub lag: u32,
}

impl crate::Message for CMSG_MOVE_TIME_SKIPPED {
    const OPCODE: u32 = 0x02ce;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w)?;

        // lag: u32
        w.write_all(&self.lag.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02CE, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // lag: u32
        let lag = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            lag,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_TIME_SKIPPED {}

impl CMSG_MOVE_TIME_SKIPPED {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // lag: u32
    }
}

