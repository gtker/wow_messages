use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_RESISTLOG {
    pub guid1: Guid,
    pub guid2: Guid,
    pub unknown1: u32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: u32,
    pub unknown5: u32,
}

impl ServerMessage for SMSG_RESISTLOG {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid1: Guid
        w.write_all(&self.guid1.guid().to_le_bytes())?;

        // guid2: Guid
        w.write_all(&self.guid2.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01d6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        36
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid1: Guid
        let guid1 = Guid::read(r)?;

        // guid2: Guid
        let guid2 = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(r)?;
        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(r)?;
        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

}

