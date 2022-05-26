use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ItemSpells {
    pub spell: u32,
    pub spell_trigger: u32,
    pub spell_charges: u32,
    pub spell_cooldown: u32,
    pub spell_category: u32,
    pub spell_category_cooldown: u32,
}

impl ItemSpells {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // spell_trigger: u32
        w.write_all(&self.spell_trigger.to_le_bytes())?;

        // spell_charges: u32
        w.write_all(&self.spell_charges.to_le_bytes())?;

        // spell_cooldown: u32
        w.write_all(&self.spell_cooldown.to_le_bytes())?;

        // spell_category: u32
        w.write_all(&self.spell_category.to_le_bytes())?;

        // spell_category_cooldown: u32
        w.write_all(&self.spell_category_cooldown.to_le_bytes())?;

        Ok(())
    }
}

impl ItemSpells {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // spell_trigger: u32
        let spell_trigger = crate::util::read_u32_le(r)?;

        // spell_charges: u32
        let spell_charges = crate::util::read_u32_le(r)?;

        // spell_cooldown: u32
        let spell_cooldown = crate::util::read_u32_le(r)?;

        // spell_category: u32
        let spell_category = crate::util::read_u32_le(r)?;

        // spell_category_cooldown: u32
        let spell_category_cooldown = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell,
            spell_trigger,
            spell_charges,
            spell_cooldown,
            spell_category,
            spell_category_cooldown,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // spell: u32
        let spell = crate::util::tokio_read_u32_le(r).await?;

        // spell_trigger: u32
        let spell_trigger = crate::util::tokio_read_u32_le(r).await?;

        // spell_charges: u32
        let spell_charges = crate::util::tokio_read_u32_le(r).await?;

        // spell_cooldown: u32
        let spell_cooldown = crate::util::tokio_read_u32_le(r).await?;

        // spell_category: u32
        let spell_category = crate::util::tokio_read_u32_le(r).await?;

        // spell_category_cooldown: u32
        let spell_category_cooldown = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            spell,
            spell_trigger,
            spell_charges,
            spell_cooldown,
            spell_category,
            spell_category_cooldown,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // spell: u32
        let spell = crate::util::astd_read_u32_le(r).await?;

        // spell_trigger: u32
        let spell_trigger = crate::util::astd_read_u32_le(r).await?;

        // spell_charges: u32
        let spell_charges = crate::util::astd_read_u32_le(r).await?;

        // spell_cooldown: u32
        let spell_cooldown = crate::util::astd_read_u32_le(r).await?;

        // spell_category: u32
        let spell_category = crate::util::astd_read_u32_le(r).await?;

        // spell_category_cooldown: u32
        let spell_category_cooldown = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            spell,
            spell_trigger,
            spell_charges,
            spell_cooldown,
            spell_category,
            spell_category_cooldown,
        })
    }

}

