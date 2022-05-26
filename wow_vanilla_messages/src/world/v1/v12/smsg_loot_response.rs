use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::LootMethod;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LOOT_RESPONSE {
    pub guid: Guid,
    pub loot_method: LootMethod,
}

impl SMSG_LOOT_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // loot_method: LootMethod
        w.write_all(&(self.loot_method.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_LOOT_RESPONSE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // loot_method: LootMethod
        w.write_all(&(self.loot_method.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0160;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // loot_method: LootMethod
        let loot_method: LootMethod = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            loot_method,
        })
    }

}

