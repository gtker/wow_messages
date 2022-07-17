use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_pending.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_pending.wowm#L3):
/// ```text
/// smsg SMSG_TRANSFER_PENDING = 0x003F {
///     Map map;
///     optional has_transport {
///         u32 transport;
///         Map transport_map;
///     }
/// }
/// ```
pub struct SMSG_TRANSFER_PENDING {
    pub map: Map,
    pub has_transport: Option<SMSG_TRANSFER_PENDING_has_transport>,
}

impl ServerMessage for SMSG_TRANSFER_PENDING {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        // optional has_transport
        if let Some(v) = &self.has_transport {
            // transport: u32
            w.write_all(&v.transport.to_le_bytes())?;

            // transport_map: Map
            w.write_all(&(map_as_int(&v.transport_map) as u32).to_le_bytes())?;

        }

        Ok(())
    }
    const OPCODE: u16 = 0x003f;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

        // optional has_transport
        let current_size = {
            4 // map: Map
        };
        let has_transport = if current_size < body_size as usize {
            // transport: u32
            let transport = crate::util::read_u32_le(r)?;

            // transport_map: Map
            let transport_map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

            Some(SMSG_TRANSFER_PENDING_has_transport {
                transport,
                transport_map,
            })
        } else {
            None
        };

        Ok(Self {
            map,
            has_transport,
        })
    }

}

impl SMSG_TRANSFER_PENDING {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + if let Some(has_transport) = &self.has_transport {
            4 // transport: u32
            + 4 // transport_map: Map
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_TRANSFER_PENDING_has_transport {
    pub transport: u32,
    pub transport_map: Map,
}

impl SMSG_TRANSFER_PENDING_has_transport {
    pub(crate) fn size(&self) -> usize {
        4 // transport: u32
        + 4 // transport_map: Map
    }

}

