use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    CastFlags, Power, SpellCastTargetFlags, SpellCastTargets, Vector3d,
};

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
    pub cast_item: Guid,
    pub caster: Guid,
    pub cast_count: u8,
    pub spell: u32,
    pub flags: SMSG_SPELL_START_CastFlags,
    pub timer: u32,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for SMSG_SPELL_START {}
impl SMSG_SPELL_START {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(19..=353).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // cast_item: PackedGuid
        let cast_item = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

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

        let flags_power_left_self = if flags.is_power_left_self() {
            // power: Power
            let power = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            Some(SMSG_SPELL_START_CastFlags_PowerLeftSelf {
                power,
            })
        }
        else {
            None
        };

        let flags_ammo = if flags.is_ammo() {
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

        let flags_unknown_23 = if flags.is_unknown_23() {
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
            ammo: flags_ammo,
            power_left_self: flags_power_left_self,
            unknown_23: flags_unknown_23,
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

impl crate::Message for SMSG_SPELL_START {
    const OPCODE: u32 = 0x0131;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELL_START"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELL_START {{").unwrap();
        // Members
        writeln!(s, "    cast_item = {};", self.cast_item.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    cast_count = {};", self.cast_count).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    flags = {};", CastFlags::new(self.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    timer = {};", self.timer).unwrap();
        // targets: SpellCastTargets
        writeln!(s, "    targets = {{").unwrap();
        // Members
        writeln!(s, "        target_flags = {};", SpellCastTargetFlags::new(self.targets.target_flags.as_int()).as_test_case_value()).unwrap();
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    writeln!(s, "        unit_target = {};", unit_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                    minipet_target,
                } => {
                    writeln!(s, "        minipet_target = {};", minipet_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Gameobject {
                    gameobject_target,
                } => {
                    writeln!(s, "        gameobject_target = {};", gameobject_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseEnemy {
                    enemy_corpse_target,
                } => {
                    writeln!(s, "        enemy_corpse_target = {};", enemy_corpse_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseAlly {
                    ally_corpse_target,
                } => {
                    writeln!(s, "        ally_corpse_target = {};", ally_corpse_target.guid()).unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item_target,
                } => {
                    writeln!(s, "        item_target = {};", item_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item_target,
                } => {
                    writeln!(s, "        trade_item_target = {};", trade_item_target.guid()).unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_source_location() {
            // source: Vector3d
            writeln!(s, "        source = {{").unwrap();
            // Members
            writeln!(s, "            x = {};", if if_statement.source.x.to_string().contains('.') { if_statement.source.x.to_string() } else { format!("{}.0", if_statement.source.x) }).unwrap();
            writeln!(s, "            y = {};", if if_statement.source.y.to_string().contains('.') { if_statement.source.y.to_string() } else { format!("{}.0", if_statement.source.y) }).unwrap();
            writeln!(s, "            z = {};", if if_statement.source.z.to_string().contains('.') { if_statement.source.z.to_string() } else { format!("{}.0", if_statement.source.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_dest_location() {
            // destination: Vector3d
            writeln!(s, "        destination = {{").unwrap();
            // Members
            writeln!(s, "            x = {};", if if_statement.destination.x.to_string().contains('.') { if_statement.destination.x.to_string() } else { format!("{}.0", if_statement.destination.x) }).unwrap();
            writeln!(s, "            y = {};", if if_statement.destination.y.to_string().contains('.') { if_statement.destination.y.to_string() } else { format!("{}.0", if_statement.destination.y) }).unwrap();
            writeln!(s, "            z = {};", if if_statement.destination.z.to_string().contains('.') { if_statement.destination.z.to_string() } else { format!("{}.0", if_statement.destination.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_string() {
            writeln!(s, "        target_string = \"{}\";", if_statement.target_string).unwrap();
        }


        writeln!(s, "    }};").unwrap();
        if let Some(if_statement) = &self.flags.get_power_left_self() {
            writeln!(s, "    power = {};", if_statement.power.as_test_case_value()).unwrap();
        }

        if let Some(if_statement) = &self.flags.get_ammo() {
            writeln!(s, "    ammo_display_id = {};", if_statement.ammo_display_id).unwrap();
            writeln!(s, "    ammo_inventory_type = {};", if_statement.ammo_inventory_type).unwrap();
        }

        if let Some(if_statement) = &self.flags.get_unknown_23() {
            writeln!(s, "    unknown1 = {};", if_statement.unknown1).unwrap();
            writeln!(s, "    unknown2 = {};", if_statement.unknown2).unwrap();
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 305_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.cast_item), "cast_item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "cast_count", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "timer", "    ");
        writeln!(s, "    /* targets: SpellCastTargets start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "target_flags", "        ");
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&unit_target), "unit_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                    minipet_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&minipet_target), "minipet_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Gameobject {
                    gameobject_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&gameobject_target), "gameobject_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseEnemy {
                    enemy_corpse_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&enemy_corpse_target), "enemy_corpse_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseAlly {
                    ally_corpse_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&ally_corpse_target), "ally_corpse_target", "        ");
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&item_target), "item_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&trade_item_target), "trade_item_target", "        ");
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_source_location() {
            writeln!(s, "    /* source: Vector3d start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "            ");
            writeln!(s, "    /* source: Vector3d end */").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_dest_location() {
            writeln!(s, "    /* destination: Vector3d start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "            ");
            writeln!(s, "    /* destination: Vector3d end */").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_string() {
            crate::util::write_bytes(&mut s, &mut bytes, if_statement.target_string.len() + 1, "target_string", "        ");
        }

        writeln!(s, "    /* targets: SpellCastTargets end */").unwrap();
        if let Some(if_statement) = &self.flags.get_power_left_self() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "power", "    ");
        }

        if let Some(if_statement) = &self.flags.get_ammo() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ammo_display_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ammo_inventory_type", "    ");
        }

        if let Some(if_statement) = &self.flags.get_unknown_23() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // cast_item: PackedGuid
        crate::util::write_packed_guid(&self.cast_item, &mut w)?;

        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: CastFlags
        w.write_all(&(self.flags.as_int().to_le_bytes()))?;

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(305, "SMSG_SPELL_START", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_START {}

impl SMSG_SPELL_START {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.cast_item) // cast_item: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
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

    pub const fn new_pending() -> Self {
        Self {
            inner: CastFlags::PENDING,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending(mut self) -> Self {
        self.inner |= CastFlags::PENDING;
        self
    }

    pub const fn get_pending(&self) -> bool {
        (self.inner & CastFlags::PENDING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending(mut self) -> Self {
        self.inner &= CastFlags::PENDING.reverse_bits();
        self
    }

    pub const fn new_has_trajectory() -> Self {
        Self {
            inner: CastFlags::HAS_TRAJECTORY,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_has_trajectory(mut self) -> Self {
        self.inner |= CastFlags::HAS_TRAJECTORY;
        self
    }

    pub const fn get_has_trajectory(&self) -> bool {
        (self.inner & CastFlags::HAS_TRAJECTORY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_has_trajectory(mut self) -> Self {
        self.inner &= CastFlags::HAS_TRAJECTORY.reverse_bits();
        self
    }

    pub const fn new_unknown_3() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_3,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_3(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_3;
        self
    }

    pub const fn get_unknown_3(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_3) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_3(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_3.reverse_bits();
        self
    }

    pub const fn new_unknown_4() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_4,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_4(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_4;
        self
    }

    pub const fn get_unknown_4(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_4) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_4(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_4.reverse_bits();
        self
    }

    pub const fn new_unknown_5() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_5,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_5(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_5;
        self
    }

    pub const fn get_unknown_5(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_5) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_5(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_5.reverse_bits();
        self
    }

    pub const fn new_ammo(ammo: SMSG_SPELL_START_CastFlags_Ammo) -> Self {
        Self {
            inner: CastFlags::AMMO,
            ammo: Some(ammo),
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ammo(mut self, ammo: SMSG_SPELL_START_CastFlags_Ammo) -> Self {
        self.inner |= CastFlags::AMMO;
        self.ammo = Some(ammo);
        self
    }

    pub const fn get_ammo(&self) -> Option<&SMSG_SPELL_START_CastFlags_Ammo> {
        self.ammo.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ammo(mut self) -> Self {
        self.inner &= CastFlags::AMMO.reverse_bits();
        self.ammo = None;
        self
    }

    pub const fn new_unknown_7() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_7,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_7(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_7;
        self
    }

    pub const fn get_unknown_7(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_7) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_7(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_7.reverse_bits();
        self
    }

    pub const fn new_unknown_8() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_8,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_8(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_8;
        self
    }

    pub const fn get_unknown_8(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_8(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_8.reverse_bits();
        self
    }

    pub const fn new_unknown_9() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_9,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_9(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_9;
        self
    }

    pub const fn get_unknown_9(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_9) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_9(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_9.reverse_bits();
        self
    }

    pub const fn new_unknown_10() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_10,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_10(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_10;
        self
    }

    pub const fn get_unknown_10(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_10) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_10(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_10.reverse_bits();
        self
    }

    pub const fn new_unknown_11() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_11,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_11(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_11;
        self
    }

    pub const fn get_unknown_11(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_11) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_11(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_11.reverse_bits();
        self
    }

    pub const fn new_power_left_self(power_left_self: SMSG_SPELL_START_CastFlags_PowerLeftSelf) -> Self {
        Self {
            inner: CastFlags::POWER_LEFT_SELF,
            ammo: None,
            power_left_self: Some(power_left_self),
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_power_left_self(mut self, power_left_self: SMSG_SPELL_START_CastFlags_PowerLeftSelf) -> Self {
        self.inner |= CastFlags::POWER_LEFT_SELF;
        self.power_left_self = Some(power_left_self);
        self
    }

    pub const fn get_power_left_self(&self) -> Option<&SMSG_SPELL_START_CastFlags_PowerLeftSelf> {
        self.power_left_self.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_power_left_self(mut self) -> Self {
        self.inner &= CastFlags::POWER_LEFT_SELF.reverse_bits();
        self.power_left_self = None;
        self
    }

    pub const fn new_unknown_13() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_13,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_13(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_13;
        self
    }

    pub const fn get_unknown_13(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_13) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_13(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_13.reverse_bits();
        self
    }

    pub const fn new_unknown_14() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_14,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_14(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_14;
        self
    }

    pub const fn get_unknown_14(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_14) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_14(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_14.reverse_bits();
        self
    }

    pub const fn new_unknown_15() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_15,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_15(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_15;
        self
    }

    pub const fn get_unknown_15(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_15) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_15(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_15.reverse_bits();
        self
    }

    pub const fn new_unknown_16() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_16,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_16(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_16;
        self
    }

    pub const fn get_unknown_16(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_16) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_16(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_16.reverse_bits();
        self
    }

    pub const fn new_unknown_17() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_17,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_17(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_17;
        self
    }

    pub const fn get_unknown_17(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_17) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_17(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_17.reverse_bits();
        self
    }

    pub const fn new_adjust_missile() -> Self {
        Self {
            inner: CastFlags::ADJUST_MISSILE,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_adjust_missile(mut self) -> Self {
        self.inner |= CastFlags::ADJUST_MISSILE;
        self
    }

    pub const fn get_adjust_missile(&self) -> bool {
        (self.inner & CastFlags::ADJUST_MISSILE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_adjust_missile(mut self) -> Self {
        self.inner &= CastFlags::ADJUST_MISSILE.reverse_bits();
        self
    }

    pub const fn new_no_gcd() -> Self {
        Self {
            inner: CastFlags::NO_GCD,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_gcd(mut self) -> Self {
        self.inner |= CastFlags::NO_GCD;
        self
    }

    pub const fn get_no_gcd(&self) -> bool {
        (self.inner & CastFlags::NO_GCD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_gcd(mut self) -> Self {
        self.inner &= CastFlags::NO_GCD.reverse_bits();
        self
    }

    pub const fn new_visual_chain() -> Self {
        Self {
            inner: CastFlags::VISUAL_CHAIN,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_visual_chain(mut self) -> Self {
        self.inner |= CastFlags::VISUAL_CHAIN;
        self
    }

    pub const fn get_visual_chain(&self) -> bool {
        (self.inner & CastFlags::VISUAL_CHAIN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_visual_chain(mut self) -> Self {
        self.inner &= CastFlags::VISUAL_CHAIN.reverse_bits();
        self
    }

    pub const fn new_unknown_21() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_21,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_21(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_21;
        self
    }

    pub const fn get_unknown_21(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_21) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_21(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_21.reverse_bits();
        self
    }

    pub const fn new_rune_list() -> Self {
        Self {
            inner: CastFlags::RUNE_LIST,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_rune_list(mut self) -> Self {
        self.inner |= CastFlags::RUNE_LIST;
        self
    }

    pub const fn get_rune_list(&self) -> bool {
        (self.inner & CastFlags::RUNE_LIST) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_rune_list(mut self) -> Self {
        self.inner &= CastFlags::RUNE_LIST.reverse_bits();
        self
    }

    pub const fn new_unknown_23(unknown_23: SMSG_SPELL_START_CastFlags_Unknown23) -> Self {
        Self {
            inner: CastFlags::UNKNOWN_23,
            ammo: None,
            power_left_self: None,
            unknown_23: Some(unknown_23),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_23(mut self, unknown_23: SMSG_SPELL_START_CastFlags_Unknown23) -> Self {
        self.inner |= CastFlags::UNKNOWN_23;
        self.unknown_23 = Some(unknown_23);
        self
    }

    pub const fn get_unknown_23(&self) -> Option<&SMSG_SPELL_START_CastFlags_Unknown23> {
        self.unknown_23.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_23(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_23.reverse_bits();
        self.unknown_23 = None;
        self
    }

    pub const fn new_unknown_24() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_24,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_24(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_24;
        self
    }

    pub const fn get_unknown_24(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_24) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_24(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_24.reverse_bits();
        self
    }

    pub const fn new_unknown_25() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_25,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_25(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_25;
        self
    }

    pub const fn get_unknown_25(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_25) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_25(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_25.reverse_bits();
        self
    }

    pub const fn new_unknown_26() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_26,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_26(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_26;
        self
    }

    pub const fn get_unknown_26(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_26) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_26(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_26.reverse_bits();
        self
    }

    pub const fn new_immunity() -> Self {
        Self {
            inner: CastFlags::IMMUNITY,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_immunity(mut self) -> Self {
        self.inner |= CastFlags::IMMUNITY;
        self
    }

    pub const fn get_immunity(&self) -> bool {
        (self.inner & CastFlags::IMMUNITY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_immunity(mut self) -> Self {
        self.inner &= CastFlags::IMMUNITY.reverse_bits();
        self
    }

    pub const fn new_unknown_28() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_28,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_28(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_28;
        self
    }

    pub const fn get_unknown_28(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_28) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_28(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_28.reverse_bits();
        self
    }

    pub const fn new_unknown_29() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_29,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_29(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_29;
        self
    }

    pub const fn get_unknown_29(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_29) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_29(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_29.reverse_bits();
        self
    }

    pub const fn new_unknown_30() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_30,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_30(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_30;
        self
    }

    pub const fn get_unknown_30(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_30) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_30(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_30.reverse_bits();
        self
    }

    pub const fn new_heal_prediction() -> Self {
        Self {
            inner: CastFlags::HEAL_PREDICTION,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_heal_prediction(mut self) -> Self {
        self.inner |= CastFlags::HEAL_PREDICTION;
        self
    }

    pub const fn get_heal_prediction(&self) -> bool {
        (self.inner & CastFlags::HEAL_PREDICTION) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_heal_prediction(mut self) -> Self {
        self.inner &= CastFlags::HEAL_PREDICTION.reverse_bits();
        self
    }

    pub const fn new_unknown_32() -> Self {
        Self {
            inner: CastFlags::UNKNOWN_32,
            ammo: None,
            power_left_self: None,
            unknown_23: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown_32(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN_32;
        self
    }

    pub const fn get_unknown_32(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN_32) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown_32(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN_32.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_SPELL_START_CastFlags {
    pub(crate) const fn size(&self) -> usize {
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
    pub(crate) const fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_START_CastFlags_PowerLeftSelf {
    pub power: Power,
}

impl SMSG_SPELL_START_CastFlags_PowerLeftSelf {
    pub(crate) const fn size(&self) -> usize {
        4 // power: Power
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_START_CastFlags_Unknown23 {
    pub unknown1: u32,
    pub unknown2: u32,
}

impl SMSG_SPELL_START_CastFlags_Unknown23 {
    pub(crate) const fn size(&self) -> usize {
        4 // unknown1: u32
        + 4 // unknown2: u32
    }
}

