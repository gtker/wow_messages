use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_LAST_INSTANCE = 0x0320 {
///     Map map;
/// }
/// ```
pub struct SMSG_UPDATE_LAST_INSTANCE {
    pub map: Map,
}

impl ServerMessage for SMSG_UPDATE_LAST_INSTANCE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0320;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

        Ok(Self {
            map,
        })
    }

}

