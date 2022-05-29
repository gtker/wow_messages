use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Map;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_TRANSFER_PENDING {
    pub map: Map,
    pub has_transport: Option<SMSG_TRANSFER_PENDINGhas_transport>,
}

impl ServerMessage for SMSG_TRANSFER_PENDING {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // optional has_transport
        if let Some(v) = &self.has_transport {
            // transport: u32
            w.write_all(&v.transport.to_le_bytes())?;

            // transport_map: Map
            w.write_all(&(v.transport_map.as_int() as u32).to_le_bytes())?;

        }

        Ok(())
    }
    const OPCODE: u16 = 0x003f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // optional has_transport
        let current_size = {
            4 // map: Map
        };
        let has_transport = if current_size < body_size as usize {
            // transport: u32
            let transport = crate::util::read_u32_le(r)?;

            // transport_map: Map
            let transport_map: Map = crate::util::read_u32_le(r)?.try_into()?;

            Some(SMSG_TRANSFER_PENDINGhas_transport {
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
pub struct SMSG_TRANSFER_PENDINGhas_transport {
    pub transport: u32,
    pub transport_map: Map,
}

impl SMSG_TRANSFER_PENDINGhas_transport {
    pub(crate) fn size(&self) -> usize {
        4 // transport: u32
        + 4 // transport_map: Map
    }

}

