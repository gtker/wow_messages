use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::SpellLog;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLLOGEXECUTE {
    pub caster: Guid,
    pub spell: u32,
    pub logs: Vec<SpellLog>,
}

impl SMSG_SPELLLOGEXECUTE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // caster: PackedGuid
        w.write_all(&self.caster.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_effects: u32
        w.write_all(&(self.logs.len() as u32).to_le_bytes())?;

        // logs: SpellLog[amount_of_effects]
        for i in self.logs.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_SPELLLOGEXECUTE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // caster: PackedGuid
        w.write_all(&self.caster.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_effects: u32
        w.write_all(&(self.logs.len() as u32).to_le_bytes())?;

        // logs: SpellLog[amount_of_effects]
        for i in self.logs.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x024c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_effects: u32
        let amount_of_effects = crate::util::read_u32_le(r)?;

        // logs: SpellLog[amount_of_effects]
        let mut logs = Vec::with_capacity(amount_of_effects as usize);
        for i in 0..amount_of_effects {
            logs.push(SpellLog::read(r)?);
        }

        Ok(Self {
            caster,
            spell,
            logs,
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // caster: PackedGuid
            let caster = Guid::tokio_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_effects: u32
            let amount_of_effects = crate::util::tokio_read_u32_le(r).await?;

            // logs: SpellLog[amount_of_effects]
            let mut logs = Vec::with_capacity(amount_of_effects as usize);
            for i in 0..amount_of_effects {
                logs.push(SpellLog::tokio_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                logs,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // caster: PackedGuid
            let caster = Guid::astd_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // amount_of_effects: u32
            let amount_of_effects = crate::util::astd_read_u32_le(r).await?;

            // logs: SpellLog[amount_of_effects]
            let mut logs = Vec::with_capacity(amount_of_effects as usize);
            for i in 0..amount_of_effects {
                logs.push(SpellLog::astd_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                logs,
            })
        })
    }

}

impl SMSG_SPELLLOGEXECUTE {
    pub fn size(&self) -> usize {
        0
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_effects: u32
        + self.logs.iter().fold(0, |acc, x| acc + x.size()) // logs: SpellLog[amount_of_effects]
    }
}

