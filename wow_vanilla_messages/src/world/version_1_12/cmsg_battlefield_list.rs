use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Map;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm#L3):
/// ```text
/// cmsg CMSG_BATTLEFIELD_LIST = 0x023C {
///     Map map;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_LIST {
    pub map: Map,
}

impl ClientMessage for CMSG_BATTLEFIELD_LIST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x023c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}

