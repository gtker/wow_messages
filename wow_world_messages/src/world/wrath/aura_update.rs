use crate:: {
    Guid,
};
use crate::wrath:: {
    Level,
};
use crate::wrath::AuraFlag;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm#L18):
/// ```text
/// struct AuraUpdate {
///     u8 visual_slot;
///     u32 spell;
///     AuraFlag flags;
///     Level level;
///     u8 aura_stack_count;
///     if (flags & NOT_CASTER) {
///         PackedGuid caster;
///     }
///     if (flags & DURATION) {
///         u32 duration;
///         u32 time_left;
///     }
/// }
/// ```
pub struct AuraUpdate {
    pub visual_slot: u8,
    pub spell: u32,
    pub flags: AuraUpdate_AuraFlag,
    pub level: Level,
    pub aura_stack_count: u8,
}

impl AuraUpdate {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // visual_slot: u8
        w.write_all(&self.visual_slot.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: AuraFlag
        w.write_all(&u8::from(self.flags.as_int()).to_le_bytes())?;

        // level: Level
        w.write_all(&self.level.as_int().to_le_bytes())?;

        // aura_stack_count: u8
        w.write_all(&self.aura_stack_count.to_le_bytes())?;

        if let Some(if_statement) = &self.flags.not_caster {
            // caster: PackedGuid
            if_statement.caster.write_packed_guid_into_vec(&mut w)?;

        }

        if let Some(if_statement) = &self.flags.duration {
            // duration: u32
            w.write_all(&if_statement.duration.to_le_bytes())?;

            // time_left: u32
            w.write_all(&if_statement.time_left.to_le_bytes())?;

        }

        Ok(())
    }
}

impl AuraUpdate {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // visual_slot: u8
        let visual_slot = crate::util::read_u8_le(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // flags: AuraFlag
        let flags = AuraFlag::new(crate::util::read_u8_le(&mut r)?);

        // level: Level
        let level = Level::new(crate::util::read_u8_le(&mut r)?);

        // aura_stack_count: u8
        let aura_stack_count = crate::util::read_u8_le(&mut r)?;

        let flags_NOT_CASTER = if flags.is_NOT_CASTER() {
            // caster: PackedGuid
            let caster = Guid::read_packed(&mut r)?;

            Some(AuraUpdate_AuraFlag_NotCaster {
                caster,
            })
        }
        else {
            None
        };

        let flags_DURATION = if flags.is_DURATION() {
            // duration: u32
            let duration = crate::util::read_u32_le(&mut r)?;

            // time_left: u32
            let time_left = crate::util::read_u32_le(&mut r)?;

            Some(AuraUpdate_AuraFlag_Duration {
                duration,
                time_left,
            })
        }
        else {
            None
        };

        let flags = AuraUpdate_AuraFlag {
            inner: flags.as_int(),
            not_caster: flags_NOT_CASTER,
            duration: flags_DURATION,
        };

        Ok(Self {
            visual_slot,
            spell,
            flags,
            level,
            aura_stack_count,
        })
    }

}

impl AuraUpdate {
    pub(crate) fn size(&self) -> usize {
        1 // visual_slot: u8
        + 4 // spell: u32
        + self.flags.size() // flags: AuraUpdate_AuraFlag
        + 1 // level: Level
        + 1 // aura_stack_count: u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct AuraUpdate_AuraFlag {
    inner: u8,
    not_caster: Option<AuraUpdate_AuraFlag_NotCaster>,
    duration: Option<AuraUpdate_AuraFlag_Duration>,
}

impl AuraUpdate_AuraFlag {
    pub const fn new(inner: u8, not_caster: Option<AuraUpdate_AuraFlag_NotCaster>,duration: Option<AuraUpdate_AuraFlag_Duration>,) -> Self {
        Self {
            inner,
            not_caster, 
            duration, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            not_caster: None,
            duration: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.not_caster.is_none()
        && self.duration.is_none()
    }

    pub const fn new_EFFECT_1() -> Self {
        Self {
            inner: AuraFlag::EFFECT_1,
            not_caster: None,
            duration: None,
        }
    }

    pub fn set_EFFECT_1(mut self) -> Self {
        self.inner |= AuraFlag::EFFECT_1;
        self
    }

    pub const fn get_EFFECT_1(&self) -> bool {
        (self.inner & AuraFlag::EFFECT_1) != 0
    }

    pub fn clear_EFFECT_1(mut self) -> Self {
        self.inner &= AuraFlag::EFFECT_1.reverse_bits();
        self
    }

    pub const fn new_EFFECT_2() -> Self {
        Self {
            inner: AuraFlag::EFFECT_2,
            not_caster: None,
            duration: None,
        }
    }

    pub fn set_EFFECT_2(mut self) -> Self {
        self.inner |= AuraFlag::EFFECT_2;
        self
    }

    pub const fn get_EFFECT_2(&self) -> bool {
        (self.inner & AuraFlag::EFFECT_2) != 0
    }

    pub fn clear_EFFECT_2(mut self) -> Self {
        self.inner &= AuraFlag::EFFECT_2.reverse_bits();
        self
    }

    pub const fn new_EFFECT_3() -> Self {
        Self {
            inner: AuraFlag::EFFECT_3,
            not_caster: None,
            duration: None,
        }
    }

    pub fn set_EFFECT_3(mut self) -> Self {
        self.inner |= AuraFlag::EFFECT_3;
        self
    }

    pub const fn get_EFFECT_3(&self) -> bool {
        (self.inner & AuraFlag::EFFECT_3) != 0
    }

    pub fn clear_EFFECT_3(mut self) -> Self {
        self.inner &= AuraFlag::EFFECT_3.reverse_bits();
        self
    }

    pub const fn new_NOT_CASTER(not_caster: AuraUpdate_AuraFlag_NotCaster) -> Self {
        Self {
            inner: AuraFlag::NOT_CASTER,
            not_caster: Some(not_caster),
            duration: None,
        }
    }

    pub fn set_NOT_CASTER(mut self, not_caster: AuraUpdate_AuraFlag_NotCaster) -> Self {
        self.inner |= AuraFlag::NOT_CASTER;
        self.not_caster = Some(not_caster);
        self
    }

    pub const fn get_NOT_CASTER(&self) -> Option<&AuraUpdate_AuraFlag_NotCaster> {
        self.not_caster.as_ref()
    }

    pub fn clear_NOT_CASTER(mut self) -> Self {
        self.inner &= AuraFlag::NOT_CASTER.reverse_bits();
        self.not_caster = None;
        self
    }

    pub const fn new_SET() -> Self {
        Self {
            inner: AuraFlag::SET,
            not_caster: None,
            duration: None,
        }
    }

    pub fn set_SET(mut self) -> Self {
        self.inner |= AuraFlag::SET;
        self
    }

    pub const fn get_SET(&self) -> bool {
        (self.inner & AuraFlag::SET) != 0
    }

    pub fn clear_SET(mut self) -> Self {
        self.inner &= AuraFlag::SET.reverse_bits();
        self
    }

    pub const fn new_CANCELLABLE() -> Self {
        Self {
            inner: AuraFlag::CANCELLABLE,
            not_caster: None,
            duration: None,
        }
    }

    pub fn set_CANCELLABLE(mut self) -> Self {
        self.inner |= AuraFlag::CANCELLABLE;
        self
    }

    pub const fn get_CANCELLABLE(&self) -> bool {
        (self.inner & AuraFlag::CANCELLABLE) != 0
    }

    pub fn clear_CANCELLABLE(mut self) -> Self {
        self.inner &= AuraFlag::CANCELLABLE.reverse_bits();
        self
    }

    pub const fn new_DURATION(duration: AuraUpdate_AuraFlag_Duration) -> Self {
        Self {
            inner: AuraFlag::DURATION,
            not_caster: None,
            duration: Some(duration),
        }
    }

    pub fn set_DURATION(mut self, duration: AuraUpdate_AuraFlag_Duration) -> Self {
        self.inner |= AuraFlag::DURATION;
        self.duration = Some(duration);
        self
    }

    pub const fn get_DURATION(&self) -> Option<&AuraUpdate_AuraFlag_Duration> {
        self.duration.as_ref()
    }

    pub fn clear_DURATION(mut self) -> Self {
        self.inner &= AuraFlag::DURATION.reverse_bits();
        self.duration = None;
        self
    }

    pub const fn new_HIDE() -> Self {
        Self {
            inner: AuraFlag::HIDE,
            not_caster: None,
            duration: None,
        }
    }

    pub fn set_HIDE(mut self) -> Self {
        self.inner |= AuraFlag::HIDE;
        self
    }

    pub const fn get_HIDE(&self) -> bool {
        (self.inner & AuraFlag::HIDE) != 0
    }

    pub fn clear_HIDE(mut self) -> Self {
        self.inner &= AuraFlag::HIDE.reverse_bits();
        self
    }

    pub const fn new_NEGATIVE() -> Self {
        Self {
            inner: AuraFlag::NEGATIVE,
            not_caster: None,
            duration: None,
        }
    }

    pub fn set_NEGATIVE(mut self) -> Self {
        self.inner |= AuraFlag::NEGATIVE;
        self
    }

    pub const fn get_NEGATIVE(&self) -> bool {
        (self.inner & AuraFlag::NEGATIVE) != 0
    }

    pub fn clear_NEGATIVE(mut self) -> Self {
        self.inner &= AuraFlag::NEGATIVE.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl AuraUpdate_AuraFlag {
    pub(crate) fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.not_caster {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.duration {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct AuraUpdate_AuraFlag_NotCaster {
    pub caster: Guid,
}

impl AuraUpdate_AuraFlag_NotCaster {
    pub(crate) fn size(&self) -> usize {
        self.caster.size() // caster: PackedGuid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct AuraUpdate_AuraFlag_Duration {
    pub duration: u32,
    pub time_left: u32,
}

impl AuraUpdate_AuraFlag_Duration {
    pub(crate) fn size(&self) -> usize {
        4 // duration: u32
        + 4 // time_left: u32
    }
}

