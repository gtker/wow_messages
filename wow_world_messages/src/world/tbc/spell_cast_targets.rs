use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    SpellCastTargetFlags, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:204`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L204):
/// ```text
/// struct SpellCastTargets {
///     SpellCastTargetFlags target_flags;
///     if (target_flags & UNIT) {
///         PackedGuid unit_target;
///     }
///     else if (target_flags & UNIT_MINIPET) {
///         PackedGuid unit_minipet;
///     }
///     else if (target_flags & UNIT_ENEMY) {
///         PackedGuid unit_enemy;
///     }
///     if (target_flags & GAMEOBJECT) {
///         PackedGuid gameobject;
///     }
///     else if (target_flags & LOCKED) {
///         PackedGuid locked;
///     }
///     if (target_flags & ITEM) {
///         PackedGuid item;
///     }
///     else if (target_flags & TRADE_ITEM) {
///         PackedGuid trade_item;
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
///     if (target_flags & CORPSE_ALLY) {
///         PackedGuid corpse_ally;
///     }
///     else if (target_flags & CORPSE_ENEMY) {
///         PackedGuid corpse_enemy;
///     }
/// }
/// ```
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
                    unit_minipet,
                } => {
                    // unit_minipet: PackedGuid
                    crate::util::write_packed_guid(&unit_minipet, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Unit::UnitEnemy {
                    unit_enemy,
                } => {
                    // unit_enemy: PackedGuid
                    crate::util::write_packed_guid(&unit_enemy, &mut w)?;

                }
            }
        }

        if let Some(if_statement) = &self.target_flags.gameobject {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                    gameobject,
                } => {
                    // gameobject: PackedGuid
                    crate::util::write_packed_guid(&gameobject, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Gameobject::Locked {
                    locked,
                } => {
                    // locked: PackedGuid
                    crate::util::write_packed_guid(&locked, &mut w)?;

                }
            }
        }

        if let Some(if_statement) = &self.target_flags.item {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item,
                } => {
                    // item: PackedGuid
                    crate::util::write_packed_guid(&item, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item,
                } => {
                    // trade_item: PackedGuid
                    crate::util::write_packed_guid(&trade_item, &mut w)?;

                }
            }
        }

        if let Some(if_statement) = &self.target_flags.source_location {
            // source: Vector3d
            if_statement.source.write_into_vec(&mut w)?;

        }

        if let Some(if_statement) = &self.target_flags.dest_location {
            // destination: Vector3d
            if_statement.destination.write_into_vec(&mut w)?;

        }

        if let Some(if_statement) = &self.target_flags.string {
            // target_string: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(if_statement.target_string.as_bytes().iter().rev().next(), Some(&0_u8), "String `target_string` must not be null-terminated.");
            w.write_all(if_statement.target_string.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        if let Some(if_statement) = &self.target_flags.corpse_ally {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseAlly {
                    corpse_ally,
                } => {
                    // corpse_ally: PackedGuid
                    crate::util::write_packed_guid(&corpse_ally, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseEnemy {
                    corpse_enemy,
                } => {
                    // corpse_enemy: PackedGuid
                    crate::util::write_packed_guid(&corpse_enemy, &mut w)?;

                }
            }
        }

        Ok(())
    }
}

impl SpellCastTargets {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
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
            // unit_minipet: PackedGuid
            let unit_minipet = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                unit_minipet,
            })
        }
        else if target_flags.is_unit_enemy() {
            // unit_enemy: PackedGuid
            let unit_enemy = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::UnitEnemy {
                unit_enemy,
            })
        }
        else {
            None
        };

        let target_flags_gameobject = if target_flags.is_gameobject() {
            // gameobject: PackedGuid
            let gameobject = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                gameobject,
            })
        }
        else if target_flags.is_locked() {
            // locked: PackedGuid
            let locked = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Gameobject::Locked {
                locked,
            })
        }
        else {
            None
        };

        let target_flags_item = if target_flags.is_item() {
            // item: PackedGuid
            let item = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Item::Item {
                item,
            })
        }
        else if target_flags.is_trade_item() {
            // trade_item: PackedGuid
            let trade_item = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                trade_item,
            })
        }
        else {
            None
        };

        let target_flags_source_location = if target_flags.is_source_location() {
            // source: Vector3d
            let source = Vector3d::read(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_SourceLocation {
                source,
            })
        }
        else {
            None
        };

        let target_flags_dest_location = if target_flags.is_dest_location() {
            // destination: Vector3d
            let destination = Vector3d::read(&mut r)?;

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

        let target_flags_corpse_ally = if target_flags.is_corpse_ally() {
            // corpse_ally: PackedGuid
            let corpse_ally = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseAlly {
                corpse_ally,
            })
        }
        else if target_flags.is_corpse_enemy() {
            // corpse_enemy: PackedGuid
            let corpse_enemy = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseEnemy {
                corpse_enemy,
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
            gameobject: target_flags_gameobject,
            string: target_flags_string,
            corpse_ally: target_flags_corpse_ally,
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
        unit_minipet: Guid,
    },
    UnitEnemy {
        unit_enemy: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Unit { .. } => 2,
            Self::UnitMinipet { .. } => 65536,
            Self::UnitEnemy { .. } => 128,
        }
    }

}

impl std::fmt::Display for SpellCastTargets_SpellCastTargetFlags_Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit{ .. } => f.write_str("Unit"),
            Self::UnitMinipet{ .. } => f.write_str("UnitMinipet"),
            Self::UnitEnemy{ .. } => f.write_str("UnitEnemy"),
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
                unit_minipet,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&unit_minipet) // unit_minipet: PackedGuid
            }
            Self::UnitEnemy {
                unit_enemy,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&unit_enemy) // unit_enemy: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_Item {
    Item {
        item: Guid,
    },
    TradeItem {
        trade_item: Guid,
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
                item,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&item) // item: PackedGuid
            }
            Self::TradeItem {
                trade_item,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&trade_item) // trade_item: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_Gameobject {
    Gameobject {
        gameobject: Guid,
    },
    Locked {
        locked: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Gameobject {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Gameobject { .. } => 2048,
            Self::Locked { .. } => 16384,
        }
    }

}

impl std::fmt::Display for SpellCastTargets_SpellCastTargetFlags_Gameobject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gameobject{ .. } => f.write_str("Gameobject"),
            Self::Locked{ .. } => f.write_str("Locked"),
        }
    }
}

impl SpellCastTargets_SpellCastTargetFlags_Gameobject {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Gameobject {
                gameobject,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&gameobject) // gameobject: PackedGuid
            }
            Self::Locked {
                locked,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&locked) // locked: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    CorpseAlly {
        corpse_ally: Guid,
    },
    CorpseEnemy {
        corpse_enemy: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::CorpseAlly { .. } => 32768,
            Self::CorpseEnemy { .. } => 512,
        }
    }

}

impl std::fmt::Display for SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CorpseAlly{ .. } => f.write_str("CorpseAlly"),
            Self::CorpseEnemy{ .. } => f.write_str("CorpseEnemy"),
        }
    }
}

impl SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::CorpseAlly {
                corpse_ally,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&corpse_ally) // corpse_ally: PackedGuid
            }
            Self::CorpseEnemy {
                corpse_enemy,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&corpse_enemy) // corpse_enemy: PackedGuid
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
    gameobject: Option<SpellCastTargets_SpellCastTargetFlags_Gameobject>,
    string: Option<SpellCastTargets_SpellCastTargetFlags_String>,
    corpse_ally: Option<SpellCastTargets_SpellCastTargetFlags_CorpseAlly>,
}

impl SpellCastTargets_SpellCastTargetFlags {
    pub const fn new(inner: u32, unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,gameobject: Option<SpellCastTargets_SpellCastTargetFlags_Gameobject>,string: Option<SpellCastTargets_SpellCastTargetFlags_String>,corpse_ally: Option<SpellCastTargets_SpellCastTargetFlags_CorpseAlly>,) -> Self {
        Self {
            inner,
            unit, 
            item, 
            source_location, 
            dest_location, 
            gameobject, 
            string, 
            corpse_ally, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse_ally: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.unit.is_none()
        && self.item.is_none()
        && self.source_location.is_none()
        && self.dest_location.is_none()
        && self.gameobject.is_none()
        && self.string.is_none()
        && self.corpse_ally.is_none()
    }

    pub const fn new_unused1() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED1,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse_ally: None,
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
            gameobject: None,
            string: None,
            corpse_ally: None,
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
            gameobject: None,
            string: None,
            corpse_ally: None,
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
            gameobject: None,
            string: None,
            corpse_ally: None,
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
            gameobject: None,
            string: None,
            corpse_ally: None,
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
            gameobject: None,
            string: None,
            corpse_ally: None,
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
            gameobject: None,
            string: None,
            corpse_ally: None,
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

    pub const fn new_unit_ally() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_ALLY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse_ally: None,
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
            gameobject: None,
            string: None,
            corpse_ally: None,
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

    pub const fn new_gameobject(gameobject: SpellCastTargets_SpellCastTargetFlags_Gameobject) -> Self {
        Self {
            inner: gameobject.as_int(),
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: Some(gameobject),
            string: None,
            corpse_ally: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_gameobject(mut self, gameobject: SpellCastTargets_SpellCastTargetFlags_Gameobject) -> Self {
        self.inner |= gameobject.as_int();
        self.gameobject = Some(gameobject);
        self
    }

    pub const fn get_gameobject(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Gameobject> {
        self.gameobject.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_gameobject(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GAMEOBJECT.reverse_bits();
        self.gameobject = None;
        self
    }

    pub const fn new_string(string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        Self {
            inner: SpellCastTargetFlags::STRING,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: Some(string),
            corpse_ally: None,
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

    pub const fn new_corpse_ally(corpse_ally: SpellCastTargets_SpellCastTargetFlags_CorpseAlly) -> Self {
        Self {
            inner: corpse_ally.as_int(),
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse_ally: Some(corpse_ally),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_corpse_ally(mut self, corpse_ally: SpellCastTargets_SpellCastTargetFlags_CorpseAlly) -> Self {
        self.inner |= corpse_ally.as_int();
        self.corpse_ally = Some(corpse_ally);
        self
    }

    pub const fn get_corpse_ally(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_CorpseAlly> {
        self.corpse_ally.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_corpse_ally(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE_ALLY.reverse_bits();
        self.corpse_ally = None;
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
            if let Some(s) = &self.gameobject {
                s.size()
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
        + {
            if let Some(s) = &self.corpse_ally {
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

impl SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub(crate) const fn size(&self) -> usize {
        12 // source: Vector3d
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub destination: Vector3d,
}

impl SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub(crate) const fn size(&self) -> usize {
        12 // destination: Vector3d
    }
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

