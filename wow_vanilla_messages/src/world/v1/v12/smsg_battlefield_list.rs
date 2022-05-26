use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::Map;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_BATTLEFIELD_LIST {
    pub battlemaster: Guid,
    pub map: Map,
    pub unknown1: u8,
    pub unknown2: u32,
    pub unknown3: u8,
    pub battlegrounds: Vec<u32>,
}

impl SMSG_BATTLEFIELD_LIST {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl ServerMessage for SMSG_BATTLEFIELD_LIST {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x023d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // battlemaster: Guid
        let battlemaster = Guid::read(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(r)?;

        // number_of_battlegrounds: u32
        let number_of_battlegrounds = crate::util::read_u32_le(r)?;

        // battlegrounds: u32[number_of_battlegrounds]
        let mut battlegrounds = Vec::with_capacity(number_of_battlegrounds as usize);
        for i in 0..number_of_battlegrounds {
            battlegrounds.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            battlemaster,
            map,
            unknown1,
            unknown2,
            unknown3,
            battlegrounds,
        })
    }

}

impl SMSG_BATTLEFIELD_LIST {
    pub fn size(&self) -> usize {
        0
        + 8 // battlemaster: Guid
        + 4 // map: Map
        + 1 // unknown1: u8
        + 4 // unknown2: u32
        + 1 // unknown3: u8
        + 4 // number_of_battlegrounds: u32
        + self.battlegrounds.len() * core::mem::size_of::<u32>() // battlegrounds: u32[number_of_battlegrounds]
    }
}

