use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::PvpRank;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PVP_CREDIT {
    pub honor_points: u32,
    pub victim: Guid,
    pub rank: PvpRank,
}

impl ServerMessage for SMSG_PVP_CREDIT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // honor_points: u32
        w.write_all(&self.honor_points.to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        // rank: PvpRank
        w.write_all(&(self.rank.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x028c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        13
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // honor_points: u32
        let honor_points = crate::util::read_u32_le(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        // rank: PvpRank
        let rank: PvpRank = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            honor_points,
            victim,
            rank,
        })
    }

}

