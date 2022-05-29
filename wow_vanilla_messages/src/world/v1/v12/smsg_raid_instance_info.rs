use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::RaidInfo;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_RAID_INSTANCE_INFO {
    pub raid_infos: Vec<RaidInfo>,
}

impl ServerMessage for SMSG_RAID_INSTANCE_INFO {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes())?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x02cc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_raid_infos: u32
        let amount_of_raid_infos = crate::util::read_u32_le(r)?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        let mut raid_infos = Vec::with_capacity(amount_of_raid_infos as usize);
        for i in 0..amount_of_raid_infos {
            raid_infos.push(RaidInfo::read(r)?);
        }

        Ok(Self {
            raid_infos,
        })
    }

}

impl SMSG_RAID_INSTANCE_INFO {
    pub(crate) fn size(&self) -> usize {
        0
        + 4 // amount_of_raid_infos: u32
        + self.raid_infos.len() * 12 // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

