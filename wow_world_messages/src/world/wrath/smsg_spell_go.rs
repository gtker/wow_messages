use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::SpellCastTargets;
use crate::world::wrath::Power;
use crate::world::wrath::GameobjectCastFlags;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_go.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_go.wowm#L49):
/// ```text
/// smsg SMSG_SPELL_GO = 0x0132 {
///     PackedGuid cast_item;
///     PackedGuid caster;
///     u8 extra_casts;
///     u32 spell;
///     GameobjectCastFlags flags;
///     u32 timestamp;
///     SpellCastTargets targets;
///     if (flags & POWER_UPDATE) {
///         (u32)Power power;
///     }
///     if (flags & RUNE_UPDATE) {
///         u8 rune_mask_initial;
///         u8 rune_mask_after_cast;
///         u8[6] rune_cooldowns;
///     }
///     if (flags & ADJUST_MISSILE) {
///         f32 elevation;
///         u32 delay_trajectory;
///     }
///     if (flags & AMMO) {
///         u32 ammo_display_id;
///         u32 ammo_inventory_type;
///     }
///     if (flags & VISUAL_CHAIN) {
///         u32 unknown1;
///         u32 unknown2;
///     }
///     if (flags & DEST_LOCATION) {
///         u8 unknown3;
///     }
/// }
/// ```
pub struct SMSG_SPELL_GO {
    /// cmangos/vmangos/mangoszero: if cast item is used, set this to guid of cast item, otherwise set it to same as caster.
    ///
    pub cast_item: Guid,
    pub caster: Guid,
    pub extra_casts: u8,
    pub spell: u32,
    pub flags: SMSG_SPELL_GO_GameobjectCastFlags,
    pub timestamp: u32,
    pub targets: SpellCastTargets,
}

impl crate::Message for SMSG_SPELL_GO {
    const OPCODE: u32 = 0x0132;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // cast_item: PackedGuid
        self.cast_item.write_packed_guid_into_vec(w);

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(w);

        // extra_casts: u8
        w.write_all(&self.extra_casts.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: GameobjectCastFlags
        w.write_all(&(self.flags.as_int() as u32).to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        if let Some(if_statement) = &self.flags.power_update {
            // power: Power
            w.write_all(&(if_statement.power.as_int() as u32).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.rune_update {
            // rune_mask_initial: u8
            w.write_all(&if_statement.rune_mask_initial.to_le_bytes())?;

            // rune_mask_after_cast: u8
            w.write_all(&if_statement.rune_mask_after_cast.to_le_bytes())?;

            // rune_cooldowns: u8[6]
            for i in if_statement.rune_cooldowns.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        if let Some(if_statement) = &self.flags.adjust_missile {
            // elevation: f32
            w.write_all(&if_statement.elevation.to_le_bytes())?;

            // delay_trajectory: u32
            w.write_all(&if_statement.delay_trajectory.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.ammo {
            // ammo_display_id: u32
            w.write_all(&if_statement.ammo_display_id.to_le_bytes())?;

            // ammo_inventory_type: u32
            w.write_all(&if_statement.ammo_inventory_type.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.visual_chain {
            // unknown1: u32
            w.write_all(&if_statement.unknown1.to_le_bytes())?;

            // unknown2: u32
            w.write_all(&if_statement.unknown2.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.dest_location {
            // unknown3: u8
            w.write_all(&if_statement.unknown3.to_le_bytes())?;

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(21..=370).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0132, size: body_size as u32 });
        }

        // cast_item: PackedGuid
        let cast_item = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // extra_casts: u8
        let extra_casts = crate::util::read_u8_le(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // flags: GameobjectCastFlags
        let flags = GameobjectCastFlags::new(crate::util::read_u32_le(r)?);

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        let flags_POWER_UPDATE = if flags.is_POWER_UPDATE() {
            // power: Power
            let power: Power = (crate::util::read_u32_le(r)? as u8).try_into()?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate {
                power,
            })
        }
        else {
            None
        };

        let flags_RUNE_UPDATE = if flags.is_RUNE_UPDATE() {
            // rune_mask_initial: u8
            let rune_mask_initial = crate::util::read_u8_le(r)?;

            // rune_mask_after_cast: u8
            let rune_mask_after_cast = crate::util::read_u8_le(r)?;

            // rune_cooldowns: u8[6]
            let mut rune_cooldowns = [0_u8; 6];
            r.read_exact(&mut rune_cooldowns)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate {
                rune_cooldowns,
                rune_mask_after_cast,
                rune_mask_initial,
            })
        }
        else {
            None
        };

        let flags_ADJUST_MISSILE = if flags.is_ADJUST_MISSILE() {
            // elevation: f32
            let elevation = crate::util::read_f32_le(r)?;
            // delay_trajectory: u32
            let delay_trajectory = crate::util::read_u32_le(r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile {
                delay_trajectory,
                elevation,
            })
        }
        else {
            None
        };

        let flags_AMMO = if flags.is_AMMO() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::read_u32_le(r)?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::read_u32_le(r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_Ammo {
                ammo_display_id,
                ammo_inventory_type,
            })
        }
        else {
            None
        };

        let flags_VISUAL_CHAIN = if flags.is_VISUAL_CHAIN() {
            // unknown1: u32
            let unknown1 = crate::util::read_u32_le(r)?;

            // unknown2: u32
            let unknown2 = crate::util::read_u32_le(r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_VisualChain {
                unknown1,
                unknown2,
            })
        }
        else {
            None
        };

        let flags_DEST_LOCATION = if flags.is_DEST_LOCATION() {
            // unknown3: u8
            let unknown3 = crate::util::read_u8_le(r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_DestLocation {
                unknown3,
            })
        }
        else {
            None
        };

        let flags = SMSG_SPELL_GO_GameobjectCastFlags {
            inner: flags.as_int(),
            ammo: flags_AMMO,
            dest_location: flags_DEST_LOCATION,
            power_update: flags_POWER_UPDATE,
            adjust_missile: flags_ADJUST_MISSILE,
            visual_chain: flags_VISUAL_CHAIN,
            rune_update: flags_RUNE_UPDATE,
        };

        Ok(Self {
            cast_item,
            caster,
            extra_casts,
            spell,
            flags,
            timestamp,
            targets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SPELL_GO {}

impl SMSG_SPELL_GO {
    pub(crate) fn size(&self) -> usize {
        self.cast_item.size() // cast_item: Guid
        + self.caster.size() // caster: Guid
        + 1 // extra_casts: u8
        + 4 // spell: u32
        + self.flags.size() // flags: SMSG_SPELL_GO_GameobjectCastFlags
        + 4 // timestamp: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags {
    inner: u32,
    ammo: Option<SMSG_SPELL_GO_GameobjectCastFlags_Ammo>,
    dest_location: Option<SMSG_SPELL_GO_GameobjectCastFlags_DestLocation>,
    power_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate>,
    adjust_missile: Option<SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile>,
    visual_chain: Option<SMSG_SPELL_GO_GameobjectCastFlags_VisualChain>,
    rune_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate>,
}

impl SMSG_SPELL_GO_GameobjectCastFlags {
    pub const fn new(inner: u32, ammo: Option<SMSG_SPELL_GO_GameobjectCastFlags_Ammo>,dest_location: Option<SMSG_SPELL_GO_GameobjectCastFlags_DestLocation>,power_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate>,adjust_missile: Option<SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile>,visual_chain: Option<SMSG_SPELL_GO_GameobjectCastFlags_VisualChain>,rune_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate>,) -> Self {
        Self {
            inner,
            ammo, 
            dest_location, 
            power_update, 
            adjust_missile, 
            visual_chain, 
            rune_update, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.ammo.is_none()
        && self.dest_location.is_none()
        && self.power_update.is_none()
        && self.adjust_missile.is_none()
        && self.visual_chain.is_none()
        && self.rune_update.is_none()
    }

    pub const fn new_LOCK_PLAYER_CAST_ANIM() -> Self {
        Self {
            inner: GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_LOCK_PLAYER_CAST_ANIM(mut self) -> Self {
        self.inner |= GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM;
        self
    }

    pub const fn get_LOCK_PLAYER_CAST_ANIM(&self) -> bool {
        (self.inner & GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM) != 0
    }

    pub fn clear_LOCK_PLAYER_CAST_ANIM(mut self) -> Self {
        self.inner &= GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN2() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN2,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNKNOWN2(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN2;
        self
    }

    pub const fn get_UNKNOWN2(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN2) != 0
    }

    pub fn clear_UNKNOWN2(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN2.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN4() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN4,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNKNOWN4(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN4;
        self
    }

    pub const fn get_UNKNOWN4(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN4) != 0
    }

    pub fn clear_UNKNOWN4(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN4.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN8() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN8,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNKNOWN8(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN8;
        self
    }

    pub const fn get_UNKNOWN8(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN8) != 0
    }

    pub fn clear_UNKNOWN8(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN16() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN16,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNKNOWN16(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN16;
        self
    }

    pub const fn get_UNKNOWN16(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN16) != 0
    }

    pub fn clear_UNKNOWN16(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN16.reverse_bits();
        self
    }

    pub const fn new_AMMO(ammo: SMSG_SPELL_GO_GameobjectCastFlags_Ammo) -> Self {
        Self {
            inner: GameobjectCastFlags::AMMO,
            ammo: Some(ammo),
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_AMMO(mut self, ammo: SMSG_SPELL_GO_GameobjectCastFlags_Ammo) -> Self {
        self.inner |= GameobjectCastFlags::AMMO;
        self.ammo = Some(ammo);
        self
    }

    pub const fn get_AMMO(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_Ammo> {
        self.ammo.as_ref()
    }

    pub fn clear_AMMO(mut self) -> Self {
        self.inner &= GameobjectCastFlags::AMMO.reverse_bits();
        self.ammo = None;
        self
    }

    pub const fn new_DEST_LOCATION(dest_location: SMSG_SPELL_GO_GameobjectCastFlags_DestLocation) -> Self {
        Self {
            inner: GameobjectCastFlags::DEST_LOCATION,
            ammo: None,
            dest_location: Some(dest_location),
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_DEST_LOCATION(mut self, dest_location: SMSG_SPELL_GO_GameobjectCastFlags_DestLocation) -> Self {
        self.inner |= GameobjectCastFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self
    }

    pub const fn get_DEST_LOCATION(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_DestLocation> {
        self.dest_location.as_ref()
    }

    pub fn clear_DEST_LOCATION(mut self) -> Self {
        self.inner &= GameobjectCastFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        self
    }

    pub const fn new_ITEM_CASTER() -> Self {
        Self {
            inner: GameobjectCastFlags::ITEM_CASTER,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_ITEM_CASTER(mut self) -> Self {
        self.inner |= GameobjectCastFlags::ITEM_CASTER;
        self
    }

    pub const fn get_ITEM_CASTER(&self) -> bool {
        (self.inner & GameobjectCastFlags::ITEM_CASTER) != 0
    }

    pub fn clear_ITEM_CASTER(mut self) -> Self {
        self.inner &= GameobjectCastFlags::ITEM_CASTER.reverse_bits();
        self
    }

    pub const fn new_UNK200() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK200,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNK200(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK200;
        self
    }

    pub const fn get_UNK200(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK200) != 0
    }

    pub fn clear_UNK200(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK200.reverse_bits();
        self
    }

    pub const fn new_EXTRA_MESSAGE() -> Self {
        Self {
            inner: GameobjectCastFlags::EXTRA_MESSAGE,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_EXTRA_MESSAGE(mut self) -> Self {
        self.inner |= GameobjectCastFlags::EXTRA_MESSAGE;
        self
    }

    pub const fn get_EXTRA_MESSAGE(&self) -> bool {
        (self.inner & GameobjectCastFlags::EXTRA_MESSAGE) != 0
    }

    pub fn clear_EXTRA_MESSAGE(mut self) -> Self {
        self.inner &= GameobjectCastFlags::EXTRA_MESSAGE.reverse_bits();
        self
    }

    pub const fn new_POWER_UPDATE(power_update: SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate) -> Self {
        Self {
            inner: GameobjectCastFlags::POWER_UPDATE,
            ammo: None,
            dest_location: None,
            power_update: Some(power_update),
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_POWER_UPDATE(mut self, power_update: SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate) -> Self {
        self.inner |= GameobjectCastFlags::POWER_UPDATE;
        self.power_update = Some(power_update);
        self
    }

    pub const fn get_POWER_UPDATE(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate> {
        self.power_update.as_ref()
    }

    pub fn clear_POWER_UPDATE(mut self) -> Self {
        self.inner &= GameobjectCastFlags::POWER_UPDATE.reverse_bits();
        self.power_update = None;
        self
    }

    pub const fn new_UNK2000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK2000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNK2000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK2000;
        self
    }

    pub const fn get_UNK2000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK2000) != 0
    }

    pub fn clear_UNK2000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK2000.reverse_bits();
        self
    }

    pub const fn new_UNK1000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK1000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNK1000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK1000;
        self
    }

    pub const fn get_UNK1000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK1000) != 0
    }

    pub fn clear_UNK1000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK1000.reverse_bits();
        self
    }

    pub const fn new_UNK8000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK8000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNK8000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK8000;
        self
    }

    pub const fn get_UNK8000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK8000) != 0
    }

    pub fn clear_UNK8000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK8000.reverse_bits();
        self
    }

    pub const fn new_ADJUST_MISSILE(adjust_missile: SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile) -> Self {
        Self {
            inner: GameobjectCastFlags::ADJUST_MISSILE,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: Some(adjust_missile),
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_ADJUST_MISSILE(mut self, adjust_missile: SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile) -> Self {
        self.inner |= GameobjectCastFlags::ADJUST_MISSILE;
        self.adjust_missile = Some(adjust_missile);
        self
    }

    pub const fn get_ADJUST_MISSILE(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile> {
        self.adjust_missile.as_ref()
    }

    pub fn clear_ADJUST_MISSILE(mut self) -> Self {
        self.inner &= GameobjectCastFlags::ADJUST_MISSILE.reverse_bits();
        self.adjust_missile = None;
        self
    }

    pub const fn new_UNK40000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK40000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNK40000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK40000;
        self
    }

    pub const fn get_UNK40000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK40000) != 0
    }

    pub fn clear_UNK40000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK40000.reverse_bits();
        self
    }

    pub const fn new_VISUAL_CHAIN(visual_chain: SMSG_SPELL_GO_GameobjectCastFlags_VisualChain) -> Self {
        Self {
            inner: GameobjectCastFlags::VISUAL_CHAIN,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: Some(visual_chain),
            rune_update: None,
        }
    }

    pub fn set_VISUAL_CHAIN(mut self, visual_chain: SMSG_SPELL_GO_GameobjectCastFlags_VisualChain) -> Self {
        self.inner |= GameobjectCastFlags::VISUAL_CHAIN;
        self.visual_chain = Some(visual_chain);
        self
    }

    pub const fn get_VISUAL_CHAIN(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_VisualChain> {
        self.visual_chain.as_ref()
    }

    pub fn clear_VISUAL_CHAIN(mut self) -> Self {
        self.inner &= GameobjectCastFlags::VISUAL_CHAIN.reverse_bits();
        self.visual_chain = None;
        self
    }

    pub const fn new_RUNE_UPDATE(rune_update: SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate) -> Self {
        Self {
            inner: GameobjectCastFlags::RUNE_UPDATE,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: Some(rune_update),
        }
    }

    pub fn set_RUNE_UPDATE(mut self, rune_update: SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate) -> Self {
        self.inner |= GameobjectCastFlags::RUNE_UPDATE;
        self.rune_update = Some(rune_update);
        self
    }

    pub const fn get_RUNE_UPDATE(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate> {
        self.rune_update.as_ref()
    }

    pub fn clear_RUNE_UPDATE(mut self) -> Self {
        self.inner &= GameobjectCastFlags::RUNE_UPDATE.reverse_bits();
        self.rune_update = None;
        self
    }

    pub const fn new_UNK400000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK400000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub fn set_UNK400000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK400000;
        self
    }

    pub const fn get_UNK400000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK400000) != 0
    }

    pub fn clear_UNK400000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK400000.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_SPELL_GO_GameobjectCastFlags {
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
            if let Some(s) = &self.dest_location {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.power_update {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.adjust_missile {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.visual_chain {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.rune_update {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_Ammo {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl SMSG_SPELL_GO_GameobjectCastFlags_Ammo {
    pub(crate) fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_DestLocation {
    pub unknown3: u8,
}

impl SMSG_SPELL_GO_GameobjectCastFlags_DestLocation {
    pub(crate) fn size(&self) -> usize {
        1 // unknown3: u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate {
    pub power: Power,
}

impl SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate {
    pub(crate) fn size(&self) -> usize {
        4 // power: Power
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile {
    pub delay_trajectory: u32,
    pub elevation: f32,
}

impl SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile {
    pub(crate) fn size(&self) -> usize {
        4 // delay_trajectory: u32
        + 4 // elevation: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_VisualChain {
    pub unknown1: u32,
    pub unknown2: u32,
}

impl SMSG_SPELL_GO_GameobjectCastFlags_VisualChain {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + 4 // unknown2: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate {
    pub rune_cooldowns: [u8; 6],
    pub rune_mask_after_cast: u8,
    pub rune_mask_initial: u8,
}

impl SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate {
    pub(crate) fn size(&self) -> usize {
        6 * core::mem::size_of::<u8>() // rune_cooldowns: u8[6]
        + 1 // rune_mask_after_cast: u8
        + 1 // rune_mask_initial: u8
    }
}

