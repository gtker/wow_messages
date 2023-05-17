use std::io::{Read, Write};

use crate::vanilla::{
    InstanceResetFailedReason, Map,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L16):
/// ```text
/// smsg SMSG_INSTANCE_RESET_FAILED = 0x031F {
///     (u32)InstanceResetFailedReason reason;
///     Map map;
/// }
/// ```
pub struct SMSG_INSTANCE_RESET_FAILED {
    pub reason: InstanceResetFailedReason,
    pub map: Map,
}

impl crate::private::Sealed for SMSG_INSTANCE_RESET_FAILED {}
impl crate::Message for SMSG_INSTANCE_RESET_FAILED {
    const OPCODE: u32 = 0x031f;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reason: InstanceResetFailedReason
        w.write_all(&u32::from(self.reason.as_int()).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x031F, size: body_size });
        }

        // reason: InstanceResetFailedReason
        let reason: InstanceResetFailedReason = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
            map,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_INSTANCE_RESET_FAILED {}

