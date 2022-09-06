use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::InstanceResetFailedReason;
use crate::world::vanilla::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L16):
/// ```text
/// smsg SMSG_INSTANCE_RESET_FAILED = 0x031F {
///     InstanceResetFailedReason reason;
///     Map map;
/// }
/// ```
pub struct SMSG_INSTANCE_RESET_FAILED {
    pub reason: InstanceResetFailedReason,
    pub map: Map,
}

impl crate::Message for SMSG_INSTANCE_RESET_FAILED {
    const OPCODE: u32 = 0x031f;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reason: InstanceResetFailedReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // reason: InstanceResetFailedReason
        let reason: InstanceResetFailedReason = crate::util::read_u8_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            reason,
            map,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_INSTANCE_RESET_FAILED {}

