use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm#L3):
/// ```text
/// cmsg CMSG_BATTLEFIELD_JOIN = 0x023E {
///     Map map;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_JOIN {
    pub map: Map,
}

impl ClientMessage for CMSG_BATTLEFIELD_JOIN {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x023e;

    fn client_size(&self) -> u16 {
        10
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

