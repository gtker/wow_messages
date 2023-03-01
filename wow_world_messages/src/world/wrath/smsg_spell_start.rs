use crate:: {
    Guid,
};
use crate::wrath::SpellCastTargets;
use crate::wrath::Power;
use crate::wrath::CastFlags;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_start.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_start.wowm#L36):
/// ```text
/// smsg SMSG_SPELL_START = 0x0131 {
///     PackedGuid cast_item;
///     PackedGuid caster;
///     u8 cast_count;
///     u32 spell;
///     CastFlags flags;
///     u32 timer;
///     SpellCastTargets targets;
///     if (flags & POWER_LEFT_SELF) {
///         (u32)Power power;
///     }
///     if (flags & AMMO) {
///         u32 ammo_display_id;
///         u32 ammo_inventory_type;
///     }
///     if (flags & UNKNOWN_23) {
///         u32 unknown1;
///         u32 unknown2;
///     }
/// }
/// ```
pub struct SMSG_SPELL_START {
    /// cmangos/vmangos/mangoszero: if cast item is used, set this to guid of cast item, otherwise set it to same as caster.
    ///
    pub cast_item: Guid,
    pub caster: Guid,
    pub cast_count: u8,
    pub spell: u32,
    pub flags: SMSG_SPELL_START_CastFlags,
    pub timer: u32,
    pub targets: SpellCastTargets,
}

impl crate::Message for SMSG_SPELL_START {
    const OPCODE: u32 = 0x0131;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // cast_item: PackedGuid
        self.cast_item.write_packed_guid_into_vec(&mut w)?;

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(&mut w)?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: CastFlags
        w.write_all(&u32::from(self.flags.as_int()).to_le_bytes())?;

        // timer: u32
        w.write_all(&self.timer.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        if let Some(if_statement) = &self.flags.power_left_self {
            // power: Power
            w.write_all(&u32::from(if_statement.power.as_int()).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.ammo {
            // ammo_display_id: u32
            w.write_all(&if_statement.ammo_display_id.to_le_bytes())?;

            // ammo_inventory_type: u32
            w.write_all(&if_statement.ammo_inventory_type.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.unknown_23 {
            // unknown1: u32
            w.write_all(&if_statement.unknown1.to_le_bytes())?;

            // unknown2: u32
            w.write_all(&if_statement.unknown2.to_le_bytes())?;

        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(21..=353).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0131, size: body_size as u32 });
        }

        // cast_item: PackedGuid
        let cast_item = Guid::read_packed(&mut r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(&mut r)?;

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // flags: CastFlags
        let flags = CastFlags::new(crate::util::read_u32_le(&mut r)?);

        // timer: u32
        let timer = crate::util::read_u32_le(&mut r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        let flags_POWER_LEFT_SELF = if flags.is_POWER_LEFT_SELF() {
            // power: Power
            let power: Power = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            Some(SMSG_SPELL_START_CastFlags_PowerLeftSelf {
                power,
            })
        }
        else {
            None
        };

        let flags_AMMO = if flags.is_AMMO() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::read_u32_le(&mut r)?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_SPELL_START_CastFlags_Ammo {
                ammo_display_id,
                ammo_inventory_type,
            })
        }
        else {
            None
        };

        let flags_UNKNOWN_23 = if flags.is_UNKNOWN_23() {
            // unknown1: u32
            let unknown1 = crate::util::read_u32_le(&mut r)?;

            // unknown2: u32
            let unknown2 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_SPELL_START_CastFlags_Unknown23 {
                unknown1,
                unknown2,
            })
        }
        else {
            None
        };

        let flags = SMSG_SPELL_START_CastFlags {
            inner: flags.as_int(),
            ammo: flags_AMMO,
            power_left_self: flags_POWER_LEFT_SELF,
            unknown_23: flags_UNKNOWN_23,
        };

        Ok(Self {
            cast_item,
            caster,
            cast_count,
            spell,
            flags,
            timer,
            targets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_START {}

impl SMSG_SPELL_START {
    pub(crate) fn size(&self) -> usize {
        self.cast_item.size() // cast_item: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 1 // cast_count: u8
        + 4 // spell: u32
        + self.flags.size() // flags: SMSG_SPELL_START_CastFlags
        + 4 // timer: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_START_CastFlags {
    inner: u32,
    ammo: Option<SMSG_SPELL_START_CastFlags_Ammo>,
    power_left_self: Option<SMSG_SPELL_START_CastFlags_PowerLeftSelf>,
    unknown_23: Option<SMSG_SPELL_START_CastFlags_Unknown23>,
}

impl SMSG_SPELL_START_CastFlags {
    pub const fn new(inner: u32, ammo: Option<SMSG_SPELL_START_CastFlags_Ammo>,power_left_self: Option<SMSG_SPELL_START_CastFlags_PowerLeftSelf>,unknown_23: Option<SMSG_SPELL_START_CastFlags_Unknown23>,) -> Self {
        Self {
            inner,
            ammo, 
            power_left_self, 
            unknown_23, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.ammo.is_none()
        && self.power_left_self.is_none()
        && self.unknown_23.is_none()
    }

    pub const fn new_PENDING() -> Self {
        Self {
            inner: CastFlags::PENDING,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_PENDING(mut self) -> Self {
        self.inner |= CastFlags::PENDING;
        self
    }

    pub const fn get_PENDING(&self) -> bool {
        (self.inner & CastFlags::PENDING) != 0
    }

    pub fn clear_PENDING(mut self) -> Self {
        self.inner &= CastFlags::PENDING.reverse_bits();
        self
    }

    pub const fn new_HAS_TRAJECTORY() -> Self {
        Self {
            inner: CastFlags::HAS_TRAJECTORY,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_HAS_TRAJECTORY(mut self) -> Self {
        self.inner |= CastFlags::HAS_TRAJECTORY;
        self
    }

    pub const fn get_HAS_TRAJECTORY(&self) -> bool {
        (self.inner & CastFlags::HAS_TRAJECTORY) != 0
    }

    pub fn clear_HAS_TRAJECTORY(mut self) -> Self {
        self.inner &= CastFlags::HAS_TRAJECTORY.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_3() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_3,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_3(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_3;
        self
    }

    pub const fn get_UNKNOWN_3(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_3) != 0
    }

    pub fn clear_UNKNOWN_3(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_3.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_4() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_4,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_4(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_4;
        self
    }

    pub const fn get_UNKNOWN_4(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_4) != 0
    }

    pub fn clear_UNKNOWN_4(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_4.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_5() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_5,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_5(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_5;
        self
    }

    pub const fn get_UNKNOWN_5(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_5) != 0
    }

    pub fn clear_UNKNOWN_5(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_5.reverse_bits();
        self
    }

    pub const fn new_AMMO(ammo: SMSG_SPELL_START_CastFlags_Ammo) -> Self {
        Self {
            inner: CastFlags::AMMO,
            ammo: Some(ammo),
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_AMMO(mut self, ammo: SMSG_SPELL_START_CastFlags_Ammo) -> Self {
        self.inner |= CastFlags::AMMO;
        self.ammo = Some(ammo);
        self
    }

    pub const fn get_AMMO(&self) -> Option<&SMSG_SPELL_START_CastFlags_Ammo> {
        self.ammo.as_ref()
    }

    pub fn clear_AMMO(mut self) -> Self {
        self.inner &= CastFlags::AMMO.reverse_bits();
        self.ammo = None;
        self
    }

    pub const fn new_UNKNOWN_7() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_7,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_7(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_7;
        self
    }

    pub const fn get_UNKNOWN_7(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_7) != 0
    }

    pub fn clear_UNKNOWN_7(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_7.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_8() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_8,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_8(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_8;
        self
    }

    pub const fn get_UNKNOWN_8(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_8) != 0
    }

    pub fn clear_UNKNOWN_8(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_8.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_9() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_9,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_9(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_9;
        self
    }

    pub const fn get_UNKNOWN_9(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_9) != 0
    }

    pub fn clear_UNKNOWN_9(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_9.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_10() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_10,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_10(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_10;
        self
    }

    pub const fn get_UNKNOWN_10(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_10) != 0
    }

    pub fn clear_UNKNOWN_10(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_10.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_11() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_11,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_11(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_11;
        self
    }

    pub const fn get_UNKNOWN_11(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_11) != 0
    }

    pub fn clear_UNKNOWN_11(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_11.reverse_bits();
        self
    }

    pub const fn new_POWER_LEFT_SELF(power_left_self: SMSG_SPELL_START_CastFlags_PowerLeftSelf) -> Self {
        Self {
            inner: CastFlags::POWER_LEFT_SELF,
            ammo: None,
            power_left_self: Some(power_left_self),
            unknown_23: None,
        }
    }

    pub fn set_POWER_LEFT_SELF(mut self, power_left_self: SMSG_SPELL_START_CastFlags_PowerLeftSelf) -> Self {
        self.inner |= CastFlags::POWER_LEFT_SELF;
        self.power_left_self = Some(power_left_self);
        self
    }

    pub const fn get_POWER_LEFT_SELF(&self) -> Option<&SMSG_SPELL_START_CastFlags_PowerLeftSelf> {
        self.power_left_self.as_ref()
    }

    pub fn clear_POWER_LEFT_SELF(mut self) -> Self {
        self.inner &= CastFlags::POWER_LEFT_SELF.reverse_bits();
        self.power_left_self = None;
        self
    }

    pub const fn new_UNKNOWN_13() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_13,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_13(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_13;
        self
    }

    pub const fn get_UNKNOWN_13(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_13) != 0
    }

    pub fn clear_UNKNOWN_13(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_13.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_14() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_14,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_14(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_14;
        self
    }

    pub const fn get_UNKNOWN_14(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_14) != 0
    }

    pub fn clear_UNKNOWN_14(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_14.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_15() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_15,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_15(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_15;
        self
    }

    pub const fn get_UNKNOWN_15(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_15) != 0
    }

    pub fn clear_UNKNOWN_15(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_15.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_16() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_16,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_16(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_16;
        self
    }

    pub const fn get_UNKNOWN_16(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_16) != 0
    }

    pub fn clear_UNKNOWN_16(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_16.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_17() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_17,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_17(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_17;
        self
    }

    pub const fn get_UNKNOWN_17(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_17) != 0
    }

    pub fn clear_UNKNOWN_17(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_17.reverse_bits();
        self
    }

    pub const fn new_ADJUST_MISSILE() -> Self {
        Self {
            inner: CastFlags::ADJUST_MISSILE,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_ADJUST_MISSILE(mut self) -> Self {
        self.inner |= CastFlags::ADJUST_MISSILE;
        self
    }

    pub const fn get_ADJUST_MISSILE(&self) -> bool {
        (self.inner & CastFlags::ADJUST_MISSILE) != 0
    }

    pub fn clear_ADJUST_MISSILE(mut self) -> Self {
        self.inner &= CastFlags::ADJUST_MISSILE.reverse_bits();
        self
    }

    pub const fn new_NO_GCD() -> Self {
        Self {
            inner: CastFlags::NO_GCD,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_NO_GCD(mut self) -> Self {
        self.inner |= CastFlags::NO_GCD;
        self
    }

    pub const fn get_NO_GCD(&self) -> bool {
        (self.inner & CastFlags::NO_GCD) != 0
    }

    pub fn clear_NO_GCD(mut self) -> Self {
        self.inner &= CastFlags::NO_GCD.reverse_bits();
        self
    }

    pub const fn new_VISUAL_CHAIN() -> Self {
        Self {
            inner: CastFlags::VISUAL_CHAIN,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_VISUAL_CHAIN(mut self) -> Self {
        self.inner |= CastFlags::VISUAL_CHAIN;
        self
    }

    pub const fn get_VISUAL_CHAIN(&self) -> bool {
        (self.inner & CastFlags::VISUAL_CHAIN) != 0
    }

    pub fn clear_VISUAL_CHAIN(mut self) -> Self {
        self.inner &= CastFlags::VISUAL_CHAIN.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_21() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_21,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_21(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_21;
        self
    }

    pub const fn get_UNKNOWN_21(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_21) != 0
    }

    pub fn clear_UNKNOWN_21(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_21.reverse_bits();
        self
    }

    pub const fn new_RUNE_LIST() -> Self {
        Self {
            inner: CastFlags::RUNE_LIST,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_RUNE_LIST(mut self) -> Self {
        self.inner |= CastFlags::RUNE_LIST;
        self
    }

    pub const fn get_RUNE_LIST(&self) -> bool {
        (self.inner & CastFlags::RUNE_LIST) != 0
    }

    pub fn clear_RUNE_LIST(mut self) -> Self {
        self.inner &= CastFlags::RUNE_LIST.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_23(unknown_23: SMSG_SPELL_START_CastFlags_Unknown23) -> Self {
        Self {
            inner: CastFlags::UNKNOWN_23,
            ammo: None,
            power_left_self: None,
            unknown_23: Some(unknown_23),
        }
    }

    pub fn set_UNKNOWN_23(mut self, unknown_23: SMSG_SPELL_START_CastFlags_Unknown23) -> Self {
        self.inner |= CastFlags::UNKNOWN_23;
        self.unknown_23 = Some(unknown_23);
        self
    }

    pub const fn get_UNKNOWN_23(&self) -> Option<&SMSG_SPELL_START_CastFlags_Unknown23> {
        self.unknown_23.as_ref()
    }

    pub fn clear_UNKNOWN_23(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_23.reverse_bits();
        self.unknown_23 = None;
        self
    }

    pub const fn new_UNKNOWN_24() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_24,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_24(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_24;
        self
    }

    pub const fn get_UNKNOWN_24(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_24) != 0
    }

    pub fn clear_UNKNOWN_24(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_24.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_25() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_25,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_25(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_25;
        self
    }

    pub const fn get_UNKNOWN_25(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_25) != 0
    }

    pub fn clear_UNKNOWN_25(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_25.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_26() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_26,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_26(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_26;
        self
    }

    pub const fn get_UNKNOWN_26(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_26) != 0
    }

    pub fn clear_UNKNOWN_26(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_26.reverse_bits();
        self
    }

    pub const fn new_IMMUNITY() -> Self {
        Self {
            inner: CastFlags::IMMUNITY,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_IMMUNITY(mut self) -> Self {
        self.inner |= CastFlags::IMMUNITY;
        self
    }

    pub const fn get_IMMUNITY(&self) -> bool {
        (self.inner & CastFlags::IMMUNITY) != 0
    }

    pub fn clear_IMMUNITY(mut self) -> Self {
        self.inner &= CastFlags::IMMUNITY.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_28() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_28,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_28(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_28;
        self
    }

    pub const fn get_UNKNOWN_28(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_28) != 0
    }

    pub fn clear_UNKNOWN_28(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_28.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_29() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_29,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_29(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_29;
        self
    }

    pub const fn get_UNKNOWN_29(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_29) != 0
    }

    pub fn clear_UNKNOWN_29(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_29.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_30() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_30,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_30(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_30;
        self
    }

    pub const fn get_UNKNOWN_30(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_30) != 0
    }

    pub fn clear_UNKNOWN_30(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_30.reverse_bits();
        self
    }

    pub const fn new_HEAL_PREDICTION() -> Self {
        Self {
            inner: CastFlags::HEAL_PREDICTION,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_HEAL_PREDICTION(mut self) -> Self {
        self.inner |= CastFlags::HEAL_PREDICTION;
        self
    }

    pub const fn get_HEAL_PREDICTION(&self) -> bool {
        (self.inner & CastFlags::HEAL_PREDICTION) != 0
    }

    pub fn clear_HEAL_PREDICTION(mut self) -> Self {
        self.inner &= CastFlags::HEAL_PREDICTION.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN_32() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_32,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    pub fn set_UNKNOWN_32(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_32;
        self
    }

    pub const fn get_UNKNOWN_32(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_32) != 0
    }

    pub fn clear_UNKNOWN_32(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_32.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_SPELL_START_CastFlags {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.ammo {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.power_left_self {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.unknown_23 {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_START_CastFlags_Ammo {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl SMSG_SPELL_START_CastFlags_Ammo {
    pub(crate) fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_START_CastFlags_PowerLeftSelf {
    pub power: Power,
}

impl SMSG_SPELL_START_CastFlags_PowerLeftSelf {
    pub(crate) fn size(&self) -> usize {
        4 // power: Power
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_START_CastFlags_Unknown23 {
    pub unknown1: u32,
    pub unknown2: u32,
}

impl SMSG_SPELL_START_CastFlags_Unknown23 {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + 4 // unknown2: u32
    }
}

