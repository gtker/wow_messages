use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::InstanceResetFailedReason;
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

impl ServerMessage for SMSG_INSTANCE_RESET_FAILED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reason: InstanceResetFailedReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x031f;

    fn server_size(&self) -> u16 {
        9
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // reason: InstanceResetFailedReason
        let reason: InstanceResetFailedReason = crate::util::read_u8_le(r)?.try_into()?;

        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

        Ok(Self {
            reason,
            map,
        })
    }

}

