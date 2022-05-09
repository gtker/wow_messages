use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum SpellEffect {
    NONE,
    INSTAKILL,
    SCHOOL_DAMAGE,
    DUMMY,
    PORTAL_TELEPORT,
    TELEPORT_UNITS,
    APPLY_AURA,
    ENVIRONMENTAL_DAMAGE,
    POWER_DRAIN,
    HEALTH_LEECH,
    HEAL,
    BIND,
    PORTAL,
    RITUAL_BASE,
    RITUAL_SPECIALIZE,
    RITUAL_ACTIVATE_PORTAL,
    QUEST_COMPLETE,
    WEAPON_DAMAGE_NOSCHOOL,
    RESURRECT,
    ADD_EXTRA_ATTACKS,
    DODGE,
    EVADE,
    PARRY,
    BLOCK,
    CREATE_ITEM,
    WEAPON,
    DEFENSE,
    PERSISTENT_AREA_AURA,
    SUMMON,
    LEAP,
    ENERGIZE,
    WEAPON_PERCENT_DAMAGE,
    TRIGGER_MISSILE,
    OPEN_LOCK,
    SUMMON_CHANGE_ITEM,
    APPLY_AREA_AURA_PARTY,
    LEARN_SPELL,
    SPELL_DEFENSE,
    DISPEL,
    LANGUAGE,
    DUAL_WIELD,
    SUMMON_WILD,
    SUMMON_GUARDIAN,
    TELEPORT_UNITS_FACE_CASTER,
    SKILL_STEP,
    ADD_HONOR,
    SPAWN,
    TRADE_SKILL,
    STEALTH,
    DETECT,
    TRANS_DOOR,
    FORCE_CRITICAL_HIT,
    GUARANTEE_HIT,
    ENCHANT_ITEM,
    ENCHANT_ITEM_TEMPORARY,
    TAMECREATURE,
    SUMMON_PET,
    LEARN_PET_SPELL,
    WEAPON_DAMAGE,
    OPEN_LOCK_ITEM,
    PROFICIENCY,
    SEND_EVENT,
    POWER_BURN,
    THREAT,
    TRIGGER_SPELL,
    HEALTH_FUNNEL,
    POWER_FUNNEL,
    HEAL_MAX_HEALTH,
    INTERRUPT_CAST,
    DISTRACT,
    PULL,
    PICKPOCKET,
    ADD_FARSIGHT,
    SUMMON_POSSESSED,
    SUMMON_TOTEM,
    HEAL_MECHANICAL,
    SUMMON_OBJECT_WILD,
    SCRIPT_EFFECT,
    ATTACK,
    SANCTUARY,
    ADD_COMBO_POINTS,
    CREATE_HOUSE,
    BIND_SIGHT,
    DUEL,
    STUCK,
    SUMMON_PLAYER,
    ACTIVATE_OBJECT,
    SUMMON_TOTEM_SLOT1,
    SUMMON_TOTEM_SLOT2,
    SUMMON_TOTEM_SLOT3,
    SUMMON_TOTEM_SLOT4,
    THREAT_ALL,
    ENCHANT_HELD_ITEM,
    SUMMON_PHANTASM,
    SELF_RESURRECT,
    SKINNING,
    CHARGE,
    SUMMON_CRITTER,
    KNOCK_BACK,
    DISENCHANT,
    INEBRIATE,
    FEED_PET,
    DISMISS_PET,
    REPUTATION,
    SUMMON_OBJECT_SLOT1,
    SUMMON_OBJECT_SLOT2,
    SUMMON_OBJECT_SLOT3,
    SUMMON_OBJECT_SLOT4,
    DISPEL_MECHANIC,
    SUMMON_DEAD_PET,
    DESTROY_ALL_TOTEMS,
    DURABILITY_DAMAGE,
    SUMMON_DEMON,
    RESURRECT_NEW,
    ATTACK_ME,
    DURABILITY_DAMAGE_PCT,
    SKIN_PLAYER_CORPSE,
    SPIRIT_HEAL,
    SKILL,
    APPLY_AREA_AURA_PET,
    TELEPORT_GRAVEYARD,
    NORMALIZED_WEAPON_DMG,
    UNKNOWN122,
    SEND_TAXI,
    PLAYER_PULL,
    MODIFY_THREAT_PERCENT,
    UNKNOWN126,
    UNKNOWN127,
}

impl ReadableAndWritable for SpellEffect {
    type Error = SpellEffectError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let a = crate::util::tokio_read_u32_le(r).await?;

            Ok(a.try_into()?)
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let a = crate::util::astd_read_u32_le(r).await?;

            Ok(a.try_into()?)
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }

}

impl SpellEffect {
    #[cfg(feature = "sync")]
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::astd_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::astd_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::astd_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellEffectError> {
        let a = crate::util::astd_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NONE => 0x0,
            Self::INSTAKILL => 0x1,
            Self::SCHOOL_DAMAGE => 0x2,
            Self::DUMMY => 0x3,
            Self::PORTAL_TELEPORT => 0x4,
            Self::TELEPORT_UNITS => 0x5,
            Self::APPLY_AURA => 0x6,
            Self::ENVIRONMENTAL_DAMAGE => 0x7,
            Self::POWER_DRAIN => 0x8,
            Self::HEALTH_LEECH => 0x9,
            Self::HEAL => 0xa,
            Self::BIND => 0xb,
            Self::PORTAL => 0xc,
            Self::RITUAL_BASE => 0xd,
            Self::RITUAL_SPECIALIZE => 0xe,
            Self::RITUAL_ACTIVATE_PORTAL => 0xf,
            Self::QUEST_COMPLETE => 0x10,
            Self::WEAPON_DAMAGE_NOSCHOOL => 0x11,
            Self::RESURRECT => 0x12,
            Self::ADD_EXTRA_ATTACKS => 0x13,
            Self::DODGE => 0x14,
            Self::EVADE => 0x15,
            Self::PARRY => 0x16,
            Self::BLOCK => 0x17,
            Self::CREATE_ITEM => 0x18,
            Self::WEAPON => 0x19,
            Self::DEFENSE => 0x1a,
            Self::PERSISTENT_AREA_AURA => 0x1b,
            Self::SUMMON => 0x1c,
            Self::LEAP => 0x1d,
            Self::ENERGIZE => 0x1e,
            Self::WEAPON_PERCENT_DAMAGE => 0x1f,
            Self::TRIGGER_MISSILE => 0x20,
            Self::OPEN_LOCK => 0x21,
            Self::SUMMON_CHANGE_ITEM => 0x22,
            Self::APPLY_AREA_AURA_PARTY => 0x23,
            Self::LEARN_SPELL => 0x24,
            Self::SPELL_DEFENSE => 0x25,
            Self::DISPEL => 0x26,
            Self::LANGUAGE => 0x27,
            Self::DUAL_WIELD => 0x28,
            Self::SUMMON_WILD => 0x29,
            Self::SUMMON_GUARDIAN => 0x2a,
            Self::TELEPORT_UNITS_FACE_CASTER => 0x2b,
            Self::SKILL_STEP => 0x2c,
            Self::ADD_HONOR => 0x2d,
            Self::SPAWN => 0x2e,
            Self::TRADE_SKILL => 0x2f,
            Self::STEALTH => 0x30,
            Self::DETECT => 0x31,
            Self::TRANS_DOOR => 0x32,
            Self::FORCE_CRITICAL_HIT => 0x33,
            Self::GUARANTEE_HIT => 0x34,
            Self::ENCHANT_ITEM => 0x35,
            Self::ENCHANT_ITEM_TEMPORARY => 0x36,
            Self::TAMECREATURE => 0x37,
            Self::SUMMON_PET => 0x38,
            Self::LEARN_PET_SPELL => 0x39,
            Self::WEAPON_DAMAGE => 0x3a,
            Self::OPEN_LOCK_ITEM => 0x3b,
            Self::PROFICIENCY => 0x3c,
            Self::SEND_EVENT => 0x3d,
            Self::POWER_BURN => 0x3e,
            Self::THREAT => 0x3f,
            Self::TRIGGER_SPELL => 0x40,
            Self::HEALTH_FUNNEL => 0x41,
            Self::POWER_FUNNEL => 0x42,
            Self::HEAL_MAX_HEALTH => 0x43,
            Self::INTERRUPT_CAST => 0x44,
            Self::DISTRACT => 0x45,
            Self::PULL => 0x46,
            Self::PICKPOCKET => 0x47,
            Self::ADD_FARSIGHT => 0x48,
            Self::SUMMON_POSSESSED => 0x49,
            Self::SUMMON_TOTEM => 0x4a,
            Self::HEAL_MECHANICAL => 0x4b,
            Self::SUMMON_OBJECT_WILD => 0x4c,
            Self::SCRIPT_EFFECT => 0x4d,
            Self::ATTACK => 0x4e,
            Self::SANCTUARY => 0x4f,
            Self::ADD_COMBO_POINTS => 0x50,
            Self::CREATE_HOUSE => 0x51,
            Self::BIND_SIGHT => 0x52,
            Self::DUEL => 0x53,
            Self::STUCK => 0x54,
            Self::SUMMON_PLAYER => 0x55,
            Self::ACTIVATE_OBJECT => 0x56,
            Self::SUMMON_TOTEM_SLOT1 => 0x57,
            Self::SUMMON_TOTEM_SLOT2 => 0x58,
            Self::SUMMON_TOTEM_SLOT3 => 0x59,
            Self::SUMMON_TOTEM_SLOT4 => 0x5a,
            Self::THREAT_ALL => 0x5b,
            Self::ENCHANT_HELD_ITEM => 0x5c,
            Self::SUMMON_PHANTASM => 0x5d,
            Self::SELF_RESURRECT => 0x5e,
            Self::SKINNING => 0x5f,
            Self::CHARGE => 0x60,
            Self::SUMMON_CRITTER => 0x61,
            Self::KNOCK_BACK => 0x62,
            Self::DISENCHANT => 0x63,
            Self::INEBRIATE => 0x64,
            Self::FEED_PET => 0x65,
            Self::DISMISS_PET => 0x66,
            Self::REPUTATION => 0x67,
            Self::SUMMON_OBJECT_SLOT1 => 0x68,
            Self::SUMMON_OBJECT_SLOT2 => 0x69,
            Self::SUMMON_OBJECT_SLOT3 => 0x6a,
            Self::SUMMON_OBJECT_SLOT4 => 0x6b,
            Self::DISPEL_MECHANIC => 0x6c,
            Self::SUMMON_DEAD_PET => 0x6d,
            Self::DESTROY_ALL_TOTEMS => 0x6e,
            Self::DURABILITY_DAMAGE => 0x6f,
            Self::SUMMON_DEMON => 0x70,
            Self::RESURRECT_NEW => 0x71,
            Self::ATTACK_ME => 0x72,
            Self::DURABILITY_DAMAGE_PCT => 0x73,
            Self::SKIN_PLAYER_CORPSE => 0x74,
            Self::SPIRIT_HEAL => 0x75,
            Self::SKILL => 0x76,
            Self::APPLY_AREA_AURA_PET => 0x77,
            Self::TELEPORT_GRAVEYARD => 0x78,
            Self::NORMALIZED_WEAPON_DMG => 0x79,
            Self::UNKNOWN122 => 0x7a,
            Self::SEND_TAXI => 0x7b,
            Self::PLAYER_PULL => 0x7c,
            Self::MODIFY_THREAT_PERCENT => 0x7d,
            Self::UNKNOWN126 => 0x7e,
            Self::UNKNOWN127 => 0x7f,
        }
    }

    pub const fn new() -> Self {
        Self::NONE
    }

}

impl ConstantSized for SpellEffect {}

impl MaximumPossibleSized for SpellEffect {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for SpellEffect {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for SpellEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::INSTAKILL => f.write_str("INSTAKILL"),
            Self::SCHOOL_DAMAGE => f.write_str("SCHOOL_DAMAGE"),
            Self::DUMMY => f.write_str("DUMMY"),
            Self::PORTAL_TELEPORT => f.write_str("PORTAL_TELEPORT"),
            Self::TELEPORT_UNITS => f.write_str("TELEPORT_UNITS"),
            Self::APPLY_AURA => f.write_str("APPLY_AURA"),
            Self::ENVIRONMENTAL_DAMAGE => f.write_str("ENVIRONMENTAL_DAMAGE"),
            Self::POWER_DRAIN => f.write_str("POWER_DRAIN"),
            Self::HEALTH_LEECH => f.write_str("HEALTH_LEECH"),
            Self::HEAL => f.write_str("HEAL"),
            Self::BIND => f.write_str("BIND"),
            Self::PORTAL => f.write_str("PORTAL"),
            Self::RITUAL_BASE => f.write_str("RITUAL_BASE"),
            Self::RITUAL_SPECIALIZE => f.write_str("RITUAL_SPECIALIZE"),
            Self::RITUAL_ACTIVATE_PORTAL => f.write_str("RITUAL_ACTIVATE_PORTAL"),
            Self::QUEST_COMPLETE => f.write_str("QUEST_COMPLETE"),
            Self::WEAPON_DAMAGE_NOSCHOOL => f.write_str("WEAPON_DAMAGE_NOSCHOOL"),
            Self::RESURRECT => f.write_str("RESURRECT"),
            Self::ADD_EXTRA_ATTACKS => f.write_str("ADD_EXTRA_ATTACKS"),
            Self::DODGE => f.write_str("DODGE"),
            Self::EVADE => f.write_str("EVADE"),
            Self::PARRY => f.write_str("PARRY"),
            Self::BLOCK => f.write_str("BLOCK"),
            Self::CREATE_ITEM => f.write_str("CREATE_ITEM"),
            Self::WEAPON => f.write_str("WEAPON"),
            Self::DEFENSE => f.write_str("DEFENSE"),
            Self::PERSISTENT_AREA_AURA => f.write_str("PERSISTENT_AREA_AURA"),
            Self::SUMMON => f.write_str("SUMMON"),
            Self::LEAP => f.write_str("LEAP"),
            Self::ENERGIZE => f.write_str("ENERGIZE"),
            Self::WEAPON_PERCENT_DAMAGE => f.write_str("WEAPON_PERCENT_DAMAGE"),
            Self::TRIGGER_MISSILE => f.write_str("TRIGGER_MISSILE"),
            Self::OPEN_LOCK => f.write_str("OPEN_LOCK"),
            Self::SUMMON_CHANGE_ITEM => f.write_str("SUMMON_CHANGE_ITEM"),
            Self::APPLY_AREA_AURA_PARTY => f.write_str("APPLY_AREA_AURA_PARTY"),
            Self::LEARN_SPELL => f.write_str("LEARN_SPELL"),
            Self::SPELL_DEFENSE => f.write_str("SPELL_DEFENSE"),
            Self::DISPEL => f.write_str("DISPEL"),
            Self::LANGUAGE => f.write_str("LANGUAGE"),
            Self::DUAL_WIELD => f.write_str("DUAL_WIELD"),
            Self::SUMMON_WILD => f.write_str("SUMMON_WILD"),
            Self::SUMMON_GUARDIAN => f.write_str("SUMMON_GUARDIAN"),
            Self::TELEPORT_UNITS_FACE_CASTER => f.write_str("TELEPORT_UNITS_FACE_CASTER"),
            Self::SKILL_STEP => f.write_str("SKILL_STEP"),
            Self::ADD_HONOR => f.write_str("ADD_HONOR"),
            Self::SPAWN => f.write_str("SPAWN"),
            Self::TRADE_SKILL => f.write_str("TRADE_SKILL"),
            Self::STEALTH => f.write_str("STEALTH"),
            Self::DETECT => f.write_str("DETECT"),
            Self::TRANS_DOOR => f.write_str("TRANS_DOOR"),
            Self::FORCE_CRITICAL_HIT => f.write_str("FORCE_CRITICAL_HIT"),
            Self::GUARANTEE_HIT => f.write_str("GUARANTEE_HIT"),
            Self::ENCHANT_ITEM => f.write_str("ENCHANT_ITEM"),
            Self::ENCHANT_ITEM_TEMPORARY => f.write_str("ENCHANT_ITEM_TEMPORARY"),
            Self::TAMECREATURE => f.write_str("TAMECREATURE"),
            Self::SUMMON_PET => f.write_str("SUMMON_PET"),
            Self::LEARN_PET_SPELL => f.write_str("LEARN_PET_SPELL"),
            Self::WEAPON_DAMAGE => f.write_str("WEAPON_DAMAGE"),
            Self::OPEN_LOCK_ITEM => f.write_str("OPEN_LOCK_ITEM"),
            Self::PROFICIENCY => f.write_str("PROFICIENCY"),
            Self::SEND_EVENT => f.write_str("SEND_EVENT"),
            Self::POWER_BURN => f.write_str("POWER_BURN"),
            Self::THREAT => f.write_str("THREAT"),
            Self::TRIGGER_SPELL => f.write_str("TRIGGER_SPELL"),
            Self::HEALTH_FUNNEL => f.write_str("HEALTH_FUNNEL"),
            Self::POWER_FUNNEL => f.write_str("POWER_FUNNEL"),
            Self::HEAL_MAX_HEALTH => f.write_str("HEAL_MAX_HEALTH"),
            Self::INTERRUPT_CAST => f.write_str("INTERRUPT_CAST"),
            Self::DISTRACT => f.write_str("DISTRACT"),
            Self::PULL => f.write_str("PULL"),
            Self::PICKPOCKET => f.write_str("PICKPOCKET"),
            Self::ADD_FARSIGHT => f.write_str("ADD_FARSIGHT"),
            Self::SUMMON_POSSESSED => f.write_str("SUMMON_POSSESSED"),
            Self::SUMMON_TOTEM => f.write_str("SUMMON_TOTEM"),
            Self::HEAL_MECHANICAL => f.write_str("HEAL_MECHANICAL"),
            Self::SUMMON_OBJECT_WILD => f.write_str("SUMMON_OBJECT_WILD"),
            Self::SCRIPT_EFFECT => f.write_str("SCRIPT_EFFECT"),
            Self::ATTACK => f.write_str("ATTACK"),
            Self::SANCTUARY => f.write_str("SANCTUARY"),
            Self::ADD_COMBO_POINTS => f.write_str("ADD_COMBO_POINTS"),
            Self::CREATE_HOUSE => f.write_str("CREATE_HOUSE"),
            Self::BIND_SIGHT => f.write_str("BIND_SIGHT"),
            Self::DUEL => f.write_str("DUEL"),
            Self::STUCK => f.write_str("STUCK"),
            Self::SUMMON_PLAYER => f.write_str("SUMMON_PLAYER"),
            Self::ACTIVATE_OBJECT => f.write_str("ACTIVATE_OBJECT"),
            Self::SUMMON_TOTEM_SLOT1 => f.write_str("SUMMON_TOTEM_SLOT1"),
            Self::SUMMON_TOTEM_SLOT2 => f.write_str("SUMMON_TOTEM_SLOT2"),
            Self::SUMMON_TOTEM_SLOT3 => f.write_str("SUMMON_TOTEM_SLOT3"),
            Self::SUMMON_TOTEM_SLOT4 => f.write_str("SUMMON_TOTEM_SLOT4"),
            Self::THREAT_ALL => f.write_str("THREAT_ALL"),
            Self::ENCHANT_HELD_ITEM => f.write_str("ENCHANT_HELD_ITEM"),
            Self::SUMMON_PHANTASM => f.write_str("SUMMON_PHANTASM"),
            Self::SELF_RESURRECT => f.write_str("SELF_RESURRECT"),
            Self::SKINNING => f.write_str("SKINNING"),
            Self::CHARGE => f.write_str("CHARGE"),
            Self::SUMMON_CRITTER => f.write_str("SUMMON_CRITTER"),
            Self::KNOCK_BACK => f.write_str("KNOCK_BACK"),
            Self::DISENCHANT => f.write_str("DISENCHANT"),
            Self::INEBRIATE => f.write_str("INEBRIATE"),
            Self::FEED_PET => f.write_str("FEED_PET"),
            Self::DISMISS_PET => f.write_str("DISMISS_PET"),
            Self::REPUTATION => f.write_str("REPUTATION"),
            Self::SUMMON_OBJECT_SLOT1 => f.write_str("SUMMON_OBJECT_SLOT1"),
            Self::SUMMON_OBJECT_SLOT2 => f.write_str("SUMMON_OBJECT_SLOT2"),
            Self::SUMMON_OBJECT_SLOT3 => f.write_str("SUMMON_OBJECT_SLOT3"),
            Self::SUMMON_OBJECT_SLOT4 => f.write_str("SUMMON_OBJECT_SLOT4"),
            Self::DISPEL_MECHANIC => f.write_str("DISPEL_MECHANIC"),
            Self::SUMMON_DEAD_PET => f.write_str("SUMMON_DEAD_PET"),
            Self::DESTROY_ALL_TOTEMS => f.write_str("DESTROY_ALL_TOTEMS"),
            Self::DURABILITY_DAMAGE => f.write_str("DURABILITY_DAMAGE"),
            Self::SUMMON_DEMON => f.write_str("SUMMON_DEMON"),
            Self::RESURRECT_NEW => f.write_str("RESURRECT_NEW"),
            Self::ATTACK_ME => f.write_str("ATTACK_ME"),
            Self::DURABILITY_DAMAGE_PCT => f.write_str("DURABILITY_DAMAGE_PCT"),
            Self::SKIN_PLAYER_CORPSE => f.write_str("SKIN_PLAYER_CORPSE"),
            Self::SPIRIT_HEAL => f.write_str("SPIRIT_HEAL"),
            Self::SKILL => f.write_str("SKILL"),
            Self::APPLY_AREA_AURA_PET => f.write_str("APPLY_AREA_AURA_PET"),
            Self::TELEPORT_GRAVEYARD => f.write_str("TELEPORT_GRAVEYARD"),
            Self::NORMALIZED_WEAPON_DMG => f.write_str("NORMALIZED_WEAPON_DMG"),
            Self::UNKNOWN122 => f.write_str("UNKNOWN122"),
            Self::SEND_TAXI => f.write_str("SEND_TAXI"),
            Self::PLAYER_PULL => f.write_str("PLAYER_PULL"),
            Self::MODIFY_THREAT_PERCENT => f.write_str("MODIFY_THREAT_PERCENT"),
            Self::UNKNOWN126 => f.write_str("UNKNOWN126"),
            Self::UNKNOWN127 => f.write_str("UNKNOWN127"),
        }
    }
}

impl TryFrom<u32> for SpellEffect {
    type Error = TryFromSpellEffectError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::INSTAKILL),
            2 => Ok(Self::SCHOOL_DAMAGE),
            3 => Ok(Self::DUMMY),
            4 => Ok(Self::PORTAL_TELEPORT),
            5 => Ok(Self::TELEPORT_UNITS),
            6 => Ok(Self::APPLY_AURA),
            7 => Ok(Self::ENVIRONMENTAL_DAMAGE),
            8 => Ok(Self::POWER_DRAIN),
            9 => Ok(Self::HEALTH_LEECH),
            10 => Ok(Self::HEAL),
            11 => Ok(Self::BIND),
            12 => Ok(Self::PORTAL),
            13 => Ok(Self::RITUAL_BASE),
            14 => Ok(Self::RITUAL_SPECIALIZE),
            15 => Ok(Self::RITUAL_ACTIVATE_PORTAL),
            16 => Ok(Self::QUEST_COMPLETE),
            17 => Ok(Self::WEAPON_DAMAGE_NOSCHOOL),
            18 => Ok(Self::RESURRECT),
            19 => Ok(Self::ADD_EXTRA_ATTACKS),
            20 => Ok(Self::DODGE),
            21 => Ok(Self::EVADE),
            22 => Ok(Self::PARRY),
            23 => Ok(Self::BLOCK),
            24 => Ok(Self::CREATE_ITEM),
            25 => Ok(Self::WEAPON),
            26 => Ok(Self::DEFENSE),
            27 => Ok(Self::PERSISTENT_AREA_AURA),
            28 => Ok(Self::SUMMON),
            29 => Ok(Self::LEAP),
            30 => Ok(Self::ENERGIZE),
            31 => Ok(Self::WEAPON_PERCENT_DAMAGE),
            32 => Ok(Self::TRIGGER_MISSILE),
            33 => Ok(Self::OPEN_LOCK),
            34 => Ok(Self::SUMMON_CHANGE_ITEM),
            35 => Ok(Self::APPLY_AREA_AURA_PARTY),
            36 => Ok(Self::LEARN_SPELL),
            37 => Ok(Self::SPELL_DEFENSE),
            38 => Ok(Self::DISPEL),
            39 => Ok(Self::LANGUAGE),
            40 => Ok(Self::DUAL_WIELD),
            41 => Ok(Self::SUMMON_WILD),
            42 => Ok(Self::SUMMON_GUARDIAN),
            43 => Ok(Self::TELEPORT_UNITS_FACE_CASTER),
            44 => Ok(Self::SKILL_STEP),
            45 => Ok(Self::ADD_HONOR),
            46 => Ok(Self::SPAWN),
            47 => Ok(Self::TRADE_SKILL),
            48 => Ok(Self::STEALTH),
            49 => Ok(Self::DETECT),
            50 => Ok(Self::TRANS_DOOR),
            51 => Ok(Self::FORCE_CRITICAL_HIT),
            52 => Ok(Self::GUARANTEE_HIT),
            53 => Ok(Self::ENCHANT_ITEM),
            54 => Ok(Self::ENCHANT_ITEM_TEMPORARY),
            55 => Ok(Self::TAMECREATURE),
            56 => Ok(Self::SUMMON_PET),
            57 => Ok(Self::LEARN_PET_SPELL),
            58 => Ok(Self::WEAPON_DAMAGE),
            59 => Ok(Self::OPEN_LOCK_ITEM),
            60 => Ok(Self::PROFICIENCY),
            61 => Ok(Self::SEND_EVENT),
            62 => Ok(Self::POWER_BURN),
            63 => Ok(Self::THREAT),
            64 => Ok(Self::TRIGGER_SPELL),
            65 => Ok(Self::HEALTH_FUNNEL),
            66 => Ok(Self::POWER_FUNNEL),
            67 => Ok(Self::HEAL_MAX_HEALTH),
            68 => Ok(Self::INTERRUPT_CAST),
            69 => Ok(Self::DISTRACT),
            70 => Ok(Self::PULL),
            71 => Ok(Self::PICKPOCKET),
            72 => Ok(Self::ADD_FARSIGHT),
            73 => Ok(Self::SUMMON_POSSESSED),
            74 => Ok(Self::SUMMON_TOTEM),
            75 => Ok(Self::HEAL_MECHANICAL),
            76 => Ok(Self::SUMMON_OBJECT_WILD),
            77 => Ok(Self::SCRIPT_EFFECT),
            78 => Ok(Self::ATTACK),
            79 => Ok(Self::SANCTUARY),
            80 => Ok(Self::ADD_COMBO_POINTS),
            81 => Ok(Self::CREATE_HOUSE),
            82 => Ok(Self::BIND_SIGHT),
            83 => Ok(Self::DUEL),
            84 => Ok(Self::STUCK),
            85 => Ok(Self::SUMMON_PLAYER),
            86 => Ok(Self::ACTIVATE_OBJECT),
            87 => Ok(Self::SUMMON_TOTEM_SLOT1),
            88 => Ok(Self::SUMMON_TOTEM_SLOT2),
            89 => Ok(Self::SUMMON_TOTEM_SLOT3),
            90 => Ok(Self::SUMMON_TOTEM_SLOT4),
            91 => Ok(Self::THREAT_ALL),
            92 => Ok(Self::ENCHANT_HELD_ITEM),
            93 => Ok(Self::SUMMON_PHANTASM),
            94 => Ok(Self::SELF_RESURRECT),
            95 => Ok(Self::SKINNING),
            96 => Ok(Self::CHARGE),
            97 => Ok(Self::SUMMON_CRITTER),
            98 => Ok(Self::KNOCK_BACK),
            99 => Ok(Self::DISENCHANT),
            100 => Ok(Self::INEBRIATE),
            101 => Ok(Self::FEED_PET),
            102 => Ok(Self::DISMISS_PET),
            103 => Ok(Self::REPUTATION),
            104 => Ok(Self::SUMMON_OBJECT_SLOT1),
            105 => Ok(Self::SUMMON_OBJECT_SLOT2),
            106 => Ok(Self::SUMMON_OBJECT_SLOT3),
            107 => Ok(Self::SUMMON_OBJECT_SLOT4),
            108 => Ok(Self::DISPEL_MECHANIC),
            109 => Ok(Self::SUMMON_DEAD_PET),
            110 => Ok(Self::DESTROY_ALL_TOTEMS),
            111 => Ok(Self::DURABILITY_DAMAGE),
            112 => Ok(Self::SUMMON_DEMON),
            113 => Ok(Self::RESURRECT_NEW),
            114 => Ok(Self::ATTACK_ME),
            115 => Ok(Self::DURABILITY_DAMAGE_PCT),
            116 => Ok(Self::SKIN_PLAYER_CORPSE),
            117 => Ok(Self::SPIRIT_HEAL),
            118 => Ok(Self::SKILL),
            119 => Ok(Self::APPLY_AREA_AURA_PET),
            120 => Ok(Self::TELEPORT_GRAVEYARD),
            121 => Ok(Self::NORMALIZED_WEAPON_DMG),
            122 => Ok(Self::UNKNOWN122),
            123 => Ok(Self::SEND_TAXI),
            124 => Ok(Self::PLAYER_PULL),
            125 => Ok(Self::MODIFY_THREAT_PERCENT),
            126 => Ok(Self::UNKNOWN126),
            127 => Ok(Self::UNKNOWN127),
            _ => Err(TryFromSpellEffectError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromSpellEffectError {
    value: u32,
}

impl TryFromSpellEffectError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum SpellEffectError {
    Read(std::io::Error),
    TryFrom(TryFromSpellEffectError),
}

impl std::error::Error for SpellEffectError {}
impl std::fmt::Display for TryFromSpellEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SpellEffect': '{}'", self.value))
    }
}

impl std::fmt::Display for SpellEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellEffectError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromSpellEffectError> for SpellEffectError {
    fn from(value: TryFromSpellEffectError) -> Self {
        Self::TryFrom(value)
    }
}

