use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    SpellCastTargetFlags, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:206`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L206):
/// ```text
/// struct SpellCastTargets {
///     SpellCastTargetFlags target_flags;
///     if (target_flags & UNIT) {
///         PackedGuid unit_target;
///     }
///     else if (target_flags & UNIT_MINIPET) {
///         PackedGuid minipet_target;
///     }
///     else if (target_flags & GAMEOBJECT) {
///         PackedGuid gameobject_target;
///     }
///     else if (target_flags & CORPSE_ENEMY) {
///         PackedGuid enemy_corpse_target;
///     }
///     else if (target_flags & CORPSE_ALLY) {
///         PackedGuid ally_corpse_target;
///     }
///     if (target_flags & ITEM) {
///         PackedGuid item_target;
///     }
///     else if (target_flags & TRADE_ITEM) {
///         PackedGuid trade_item_target;
///     }
///     if (target_flags & SOURCE_LOCATION) {
///         Vector3d source;
///     }
///     if (target_flags & DEST_LOCATION) {
///         Vector3d destination;
///     }
///     if (target_flags & STRING) {
///         CString target_string;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets {
    pub target_flags: SpellCastTargets_SpellCastTargetFlags,
}

impl SpellCastTargets {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target_flags: SpellCastTargetFlags
        w.write_all(&(self.target_flags.as_int().to_le_bytes()))?;

        if let Some(if_statement) = &self.target_flags.unit {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    // unit_target: PackedGuid
                    crate::util::write_packed_guid(&unit_target, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                    minipet_target,
                } => {
                    // minipet_target: PackedGuid
                    crate::util::write_packed_guid(&minipet_target, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Unit::Gameobject {
                    gameobject_target,
                } => {
                    // gameobject_target: PackedGuid
                    crate::util::write_packed_guid(&gameobject_target, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Unit::CorpseEnemy {
                    enemy_corpse_target,
                } => {
                    // enemy_corpse_target: PackedGuid
                    crate::util::write_packed_guid(&enemy_corpse_target, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Unit::CorpseAlly {
                    ally_corpse_target,
                } => {
                    // ally_corpse_target: PackedGuid
                    crate::util::write_packed_guid(&ally_corpse_target, &mut w)?;

                }
            }
        }

        if let Some(if_statement) = &self.target_flags.item {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item_target,
                } => {
                    // item_target: PackedGuid
                    crate::util::write_packed_guid(&item_target, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item_target,
                } => {
                    // trade_item_target: PackedGuid
                    crate::util::write_packed_guid(&trade_item_target, &mut w)?;

                }
            }
        }

        if let Some(if_statement) = &self.target_flags.source_location {
            // source: Vector3d
            crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&if_statement.source, &mut w)?;

        }

        if let Some(if_statement) = &self.target_flags.dest_location {
            // destination: Vector3d
            crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&if_statement.destination, &mut w)?;

        }

        if let Some(if_statement) = &self.target_flags.string {
            // target_string: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(if_statement.target_string.as_bytes().iter().next_back(), Some(&0_u8), "String `target_string` must not be null-terminated.");
            w.write_all(if_statement.target_string.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        Ok(())
    }
}

impl SpellCastTargets {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::new(crate::util::read_u32_le(&mut r)?);

        let target_flags_unit = if target_flags.is_unit() {
            // unit_target: PackedGuid
            let unit_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                unit_target,
            })
        }
        else if target_flags.is_unit_minipet() {
            // minipet_target: PackedGuid
            let minipet_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                minipet_target,
            })
        }
        else if target_flags.is_gameobject() {
            // gameobject_target: PackedGuid
            let gameobject_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::Gameobject {
                gameobject_target,
            })
        }
        else if target_flags.is_corpse_enemy() {
            // enemy_corpse_target: PackedGuid
            let enemy_corpse_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::CorpseEnemy {
                enemy_corpse_target,
            })
        }
        else if target_flags.is_corpse_ally() {
            // ally_corpse_target: PackedGuid
            let ally_corpse_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::CorpseAlly {
                ally_corpse_target,
            })
        }
        else {
            None
        };

        let target_flags_item = if target_flags.is_item() {
            // item_target: PackedGuid
            let item_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Item::Item {
                item_target,
            })
        }
        else if target_flags.is_trade_item() {
            // trade_item_target: PackedGuid
            let trade_item_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                trade_item_target,
            })
        }
        else {
            None
        };

        let target_flags_source_location = if target_flags.is_source_location() {
            // source: Vector3d
            let source = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_SourceLocation {
                source,
            })
        }
        else {
            None
        };

        let target_flags_dest_location = if target_flags.is_dest_location() {
            // destination: Vector3d
            let destination = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_DestLocation {
                destination,
            })
        }
        else {
            None
        };

        let target_flags_string = if target_flags.is_string() {
            // target_string: CString
            let target_string = {
                let target_string = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(target_string)?
            };

            Some(SpellCastTargets_SpellCastTargetFlags_String {
                target_string,
            })
        }
        else {
            None
        };

        let target_flags = SpellCastTargets_SpellCastTargetFlags {
            inner: target_flags.as_int(),
            unit: target_flags_unit,
            item: target_flags_item,
            source_location: target_flags_source_location,
            dest_location: target_flags_dest_location,
            string: target_flags_string,
        };

        Ok(Self {
            target_flags,
        })
    }

}

impl SpellCastTargets {
    pub(crate) fn size(&self) -> usize {
        self.target_flags.size() // target_flags: SpellCastTargets_SpellCastTargetFlags
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_Unit {
    Unit {
        unit_target: Guid,
    },
    UnitMinipet {
        minipet_target: Guid,
    },
    Gameobject {
        gameobject_target: Guid,
    },
    CorpseEnemy {
        enemy_corpse_target: Guid,
    },
    CorpseAlly {
        ally_corpse_target: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Unit { .. } => 2,
            Self::UnitMinipet { .. } => 65536,
            Self::Gameobject { .. } => 2048,
            Self::CorpseEnemy { .. } => 512,
            Self::CorpseAlly { .. } => 32768,
        }
    }

}

impl std::fmt::Display for SpellCastTargets_SpellCastTargetFlags_Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit{ .. } => f.write_str("Unit"),
            Self::UnitMinipet{ .. } => f.write_str("UnitMinipet"),
            Self::Gameobject{ .. } => f.write_str("Gameobject"),
            Self::CorpseEnemy{ .. } => f.write_str("CorpseEnemy"),
            Self::CorpseAlly{ .. } => f.write_str("CorpseAlly"),
        }
    }
}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Unit {
                unit_target,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&unit_target) // unit_target: PackedGuid
            }
            Self::UnitMinipet {
                minipet_target,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&minipet_target) // minipet_target: PackedGuid
            }
            Self::Gameobject {
                gameobject_target,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&gameobject_target) // gameobject_target: PackedGuid
            }
            Self::CorpseEnemy {
                enemy_corpse_target,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&enemy_corpse_target) // enemy_corpse_target: PackedGuid
            }
            Self::CorpseAlly {
                ally_corpse_target,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&ally_corpse_target) // ally_corpse_target: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_Item {
    Item {
        item_target: Guid,
    },
    TradeItem {
        trade_item_target: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Item {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Item { .. } => 16,
            Self::TradeItem { .. } => 4096,
        }
    }

}

impl std::fmt::Display for SpellCastTargets_SpellCastTargetFlags_Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Item{ .. } => f.write_str("Item"),
            Self::TradeItem{ .. } => f.write_str("TradeItem"),
        }
    }
}

impl SpellCastTargets_SpellCastTargetFlags_Item {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Item {
                item_target,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&item_target) // item_target: PackedGuid
            }
            Self::TradeItem {
                trade_item_target,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&trade_item_target) // trade_item_target: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags {
    inner: u32,
    unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,
    item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,
    source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,
    dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,
    string: Option<SpellCastTargets_SpellCastTargetFlags_String>,
}

impl SpellCastTargets_SpellCastTargetFlags {
    pub const fn new(inner: u32, unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,string: Option<SpellCastTargets_SpellCastTargetFlags_String>,) -> Self {
        Self {
            inner,
            unit, 
            item, 
            source_location, 
            dest_location, 
            string, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.unit.is_none()
        && self.item.is_none()
        && self.source_location.is_none()
        && self.dest_location.is_none()
        && self.string.is_none()
    }

    pub const fn new_unused1() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED1,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unused1(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED1;
        self
    }

    pub const fn get_unused1(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNUSED1) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unused1(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED1.reverse_bits();
        self
    }

    pub const fn new_unit(unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
        Self {
            inner: unit.as_int(),
            unit: Some(unit),
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit(mut self, unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
        self.inner |= unit.as_int();
        self.unit = Some(unit);
        self
    }

    pub const fn get_unit(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Unit> {
        self.unit.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT.reverse_bits();
        self.unit = None;
        self
    }

    pub const fn new_unit_raid() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_RAID,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_raid(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_RAID;
        self
    }

    pub const fn get_unit_raid(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_RAID) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_raid(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_RAID.reverse_bits();
        self
    }

    pub const fn new_unit_party() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_PARTY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_party(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_PARTY;
        self
    }

    pub const fn get_unit_party(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_PARTY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_party(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_PARTY.reverse_bits();
        self
    }

    pub const fn new_item(item: SpellCastTargets_SpellCastTargetFlags_Item) -> Self {
        Self {
            inner: item.as_int(),
            unit: None,
            item: Some(item),
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_item(mut self, item: SpellCastTargets_SpellCastTargetFlags_Item) -> Self {
        self.inner |= item.as_int();
        self.item = Some(item);
        self
    }

    pub const fn get_item(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Item> {
        self.item.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_item(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::ITEM.reverse_bits();
        self.item = None;
        self
    }

    pub const fn new_source_location(source_location: SpellCastTargets_SpellCastTargetFlags_SourceLocation) -> Self {
        Self {
            inner: SpellCastTargetFlags::SOURCE_LOCATION,
            unit: None,
            item: None,
            source_location: Some(source_location),
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_source_location(mut self, source_location: SpellCastTargets_SpellCastTargetFlags_SourceLocation) -> Self {
        self.inner |= SpellCastTargetFlags::SOURCE_LOCATION;
        self.source_location = Some(source_location);
        self
    }

    pub const fn get_source_location(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_SourceLocation> {
        self.source_location.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_source_location(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::SOURCE_LOCATION.reverse_bits();
        self.source_location = None;
        self
    }

    pub const fn new_dest_location(dest_location: SpellCastTargets_SpellCastTargetFlags_DestLocation) -> Self {
        Self {
            inner: SpellCastTargetFlags::DEST_LOCATION,
            unit: None,
            item: None,
            source_location: None,
            dest_location: Some(dest_location),
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_dest_location(mut self, dest_location: SpellCastTargets_SpellCastTargetFlags_DestLocation) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self
    }

    pub const fn get_dest_location(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_DestLocation> {
        self.dest_location.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_dest_location(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        self
    }

    pub const fn new_unit_enemy() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_ENEMY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_enemy(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ENEMY;
        self
    }

    pub const fn get_unit_enemy(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_ENEMY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_enemy(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ENEMY.reverse_bits();
        self
    }

    pub const fn new_unit_ally() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_ALLY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_ally(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ALLY;
        self
    }

    pub const fn get_unit_ally(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_ALLY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_ally(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ALLY.reverse_bits();
        self
    }

    pub const fn new_unit_dead() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_DEAD,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_dead(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_DEAD;
        self
    }

    pub const fn get_unit_dead(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_DEAD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_dead(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_DEAD.reverse_bits();
        self
    }

    pub const fn new_string(string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        Self {
            inner: SpellCastTargetFlags::STRING,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: Some(string),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_string(mut self, string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        self.inner |= SpellCastTargetFlags::STRING;
        self.string = Some(string);
        self
    }

    pub const fn get_string(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_String> {
        self.string.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_string(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::STRING.reverse_bits();
        self.string = None;
        self
    }

    pub const fn new_locked() -> Self {
        Self {
            inner: SpellCastTargetFlags::LOCKED,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_locked(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::LOCKED;
        self
    }

    pub const fn get_locked(&self) -> bool {
        (self.inner & SpellCastTargetFlags::LOCKED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_locked(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::LOCKED.reverse_bits();
        self
    }

    pub const fn new_glyph_slot() -> Self {
        Self {
            inner: SpellCastTargetFlags::GLYPH_SLOT,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_glyph_slot(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::GLYPH_SLOT;
        self
    }

    pub const fn get_glyph_slot(&self) -> bool {
        (self.inner & SpellCastTargetFlags::GLYPH_SLOT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_glyph_slot(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GLYPH_SLOT.reverse_bits();
        self
    }

    pub const fn new_dest_target() -> Self {
        Self {
            inner: SpellCastTargetFlags::DEST_TARGET,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_dest_target(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_TARGET;
        self
    }

    pub const fn get_dest_target(&self) -> bool {
        (self.inner & SpellCastTargetFlags::DEST_TARGET) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_dest_target(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_TARGET.reverse_bits();
        self
    }

    pub const fn new_unused20() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED20,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unused20(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED20;
        self
    }

    pub const fn get_unused20(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNUSED20) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unused20(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED20.reverse_bits();
        self
    }

    pub const fn new_unit_passenger() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_PASSENGER,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            string: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_passenger(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_PASSENGER;
        self
    }

    pub const fn get_unit_passenger(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_PASSENGER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_passenger(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_PASSENGER.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SpellCastTargets_SpellCastTargetFlags {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.unit {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.item {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.source_location {
                12
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.dest_location {
                12
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.string {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub source: Vector3d,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub destination: Vector3d,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_String {
    pub target_string: String,
}

impl SpellCastTargets_SpellCastTargetFlags_String {
    pub(crate) fn size(&self) -> usize {
        self.target_string.len() + 1 // target_string: CString
    }
}

