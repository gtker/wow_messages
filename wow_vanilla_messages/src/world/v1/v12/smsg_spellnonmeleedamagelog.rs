use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::SpellSchool;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLNONMELEEDAMAGELOG {
    pub target: Guid,
    pub attacker: Guid,
    pub spell: u32,
    pub damage: u32,
    pub school: SpellSchool,
    pub absorbed_damage: u32,
    pub resisted: u32,
    pub periodic_log: u8,
    pub unused: u8,
    pub blocked: u32,
    pub hit_info: u32,
    pub extend_flag: u8,
}

impl SMSG_SPELLNONMELEEDAMAGELOG {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // target: PackedGuid
        w.write_all(&self.target.packed_guid())?;

        // attacker: PackedGuid
        w.write_all(&self.attacker.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int() as u8).to_le_bytes())?;

        // absorbed_damage: u32
        w.write_all(&self.absorbed_damage.to_le_bytes())?;

        // resisted: u32
        w.write_all(&self.resisted.to_le_bytes())?;

        // periodic_log: u8
        w.write_all(&self.periodic_log.to_le_bytes())?;

        // unused: u8
        w.write_all(&self.unused.to_le_bytes())?;

        // blocked: u32
        w.write_all(&self.blocked.to_le_bytes())?;

        // hit_info: u32
        w.write_all(&self.hit_info.to_le_bytes())?;

        // extend_flag: u8
        w.write_all(&self.extend_flag.to_le_bytes())?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_SPELLNONMELEEDAMAGELOG {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target: PackedGuid
        w.write_all(&self.target.packed_guid())?;

        // attacker: PackedGuid
        w.write_all(&self.attacker.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int() as u8).to_le_bytes())?;

        // absorbed_damage: u32
        w.write_all(&self.absorbed_damage.to_le_bytes())?;

        // resisted: u32
        w.write_all(&self.resisted.to_le_bytes())?;

        // periodic_log: u8
        w.write_all(&self.periodic_log.to_le_bytes())?;

        // unused: u8
        w.write_all(&self.unused.to_le_bytes())?;

        // blocked: u32
        w.write_all(&self.blocked.to_le_bytes())?;

        // hit_info: u32
        w.write_all(&self.hit_info.to_le_bytes())?;

        // extend_flag: u8
        w.write_all(&self.extend_flag.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0250;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // attacker: PackedGuid
        let attacker = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // school: SpellSchool
        let school: SpellSchool = crate::util::read_u8_le(r)?.try_into()?;

        // absorbed_damage: u32
        let absorbed_damage = crate::util::read_u32_le(r)?;

        // resisted: u32
        let resisted = crate::util::read_u32_le(r)?;

        // periodic_log: u8
        let periodic_log = crate::util::read_u8_le(r)?;

        // unused: u8
        let unused = crate::util::read_u8_le(r)?;

        // blocked: u32
        let blocked = crate::util::read_u32_le(r)?;

        // hit_info: u32
        let hit_info = crate::util::read_u32_le(r)?;

        // extend_flag: u8
        let extend_flag = crate::util::read_u8_le(r)?;

        Ok(Self {
            target,
            attacker,
            spell,
            damage,
            school,
            absorbed_damage,
            resisted,
            periodic_log,
            unused,
            blocked,
            hit_info,
            extend_flag,
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
            // target: PackedGuid
            let target = Guid::tokio_read_packed(r).await?;

            // attacker: PackedGuid
            let attacker = Guid::tokio_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // damage: u32
            let damage = crate::util::tokio_read_u32_le(r).await?;

            // school: SpellSchool
            let school: SpellSchool = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // absorbed_damage: u32
            let absorbed_damage = crate::util::tokio_read_u32_le(r).await?;

            // resisted: u32
            let resisted = crate::util::tokio_read_u32_le(r).await?;

            // periodic_log: u8
            let periodic_log = crate::util::tokio_read_u8_le(r).await?;

            // unused: u8
            let unused = crate::util::tokio_read_u8_le(r).await?;

            // blocked: u32
            let blocked = crate::util::tokio_read_u32_le(r).await?;

            // hit_info: u32
            let hit_info = crate::util::tokio_read_u32_le(r).await?;

            // extend_flag: u8
            let extend_flag = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                target,
                attacker,
                spell,
                damage,
                school,
                absorbed_damage,
                resisted,
                periodic_log,
                unused,
                blocked,
                hit_info,
                extend_flag,
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
            // target: PackedGuid
            let target = Guid::astd_read_packed(r).await?;

            // attacker: PackedGuid
            let attacker = Guid::astd_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // damage: u32
            let damage = crate::util::astd_read_u32_le(r).await?;

            // school: SpellSchool
            let school: SpellSchool = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // absorbed_damage: u32
            let absorbed_damage = crate::util::astd_read_u32_le(r).await?;

            // resisted: u32
            let resisted = crate::util::astd_read_u32_le(r).await?;

            // periodic_log: u8
            let periodic_log = crate::util::astd_read_u8_le(r).await?;

            // unused: u8
            let unused = crate::util::astd_read_u8_le(r).await?;

            // blocked: u32
            let blocked = crate::util::astd_read_u32_le(r).await?;

            // hit_info: u32
            let hit_info = crate::util::astd_read_u32_le(r).await?;

            // extend_flag: u8
            let extend_flag = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                target,
                attacker,
                spell,
                damage,
                school,
                absorbed_damage,
                resisted,
                periodic_log,
                unused,
                blocked,
                hit_info,
                extend_flag,
            })
        })
    }

}

impl SMSG_SPELLNONMELEEDAMAGELOG {
    pub fn size(&self) -> usize {
        0
        + self.target.size() // target: Guid
        + self.attacker.size() // attacker: Guid
        + 4 // spell: u32
        + 4 // damage: u32
        + 1 // school: SpellSchool
        + 4 // absorbed_damage: u32
        + 4 // resisted: u32
        + 1 // periodic_log: u8
        + 1 // unused: u8
        + 4 // blocked: u32
        + 4 // hit_info: u32
        + 1 // extend_flag: u8
    }
}

