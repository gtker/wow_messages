use std::io::{Read, Write};

use crate::vanilla::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_pending.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_pending.wowm#L1):
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

impl crate::private::Sealed for SMSG_TRANSFER_PENDING {}
impl crate::Message for SMSG_TRANSFER_PENDING {
    const OPCODE: u32 = 0x003f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // optional has_transport
        if let Some(v) = &self.has_transport {
            // transport: u32
            w.write_all(&v.transport.to_le_bytes())?;

            // transport_map: Map
            w.write_all(&(v.transport_map.as_int().to_le_bytes()))?;

        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=12).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003F, size: body_size });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // optional has_transport
        let current_size = {
            4 // map: Map
        };
        let has_transport = if current_size < body_size as usize {
            // transport: u32
            let transport = crate::util::read_u32_le(&mut r)?;

            // transport_map: Map
            let transport_map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TRANSFER_PENDING {}

impl SMSG_TRANSFER_PENDING {
    pub(crate) const fn size(&self) -> usize {
        4 // map: Map
        + if let Some(has_transport) = &self.has_transport {
            4 // transport: u32
            + 4 // transport_map: Map
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_TRANSFER_PENDING_has_transport {
    pub transport: u32,
    pub transport_map: Map,
}

