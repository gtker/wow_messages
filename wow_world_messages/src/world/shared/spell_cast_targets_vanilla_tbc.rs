use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::spell_cast_target_flags_vanilla_tbc::SpellCastTargetFlags;
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:125`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L125):
/// ```text
/// struct SpellCastTargets {
///     SpellCastTargetFlags target_flags;
///     if (target_flags & UNIT) {
///         PackedGuid unit_target;
///     }
///     else if (target_flags & TRADE_ITEM) {
///         PackedGuid trade_item;
///     }
///     else if (target_flags & UNIT_ENEMY) {
///         PackedGuid enemy_target;
///     }
///     if (target_flags & GAMEOBJECT) {
///         PackedGuid object_target;
///     }
///     else if (target_flags & LOCKED) {
///         PackedGuid locked_target;
///     }
///     if (target_flags & ITEM) {
///         PackedGuid item_target;
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
///         PackedGuid corpse_target_ally;
///     }
///     else if (target_flags & CORPSE_ENEMY) {
///         PackedGuid corpse_target_enemy;
///     }
/// }
/// ```
pub struct SpellCastTargets {
    pub target_flags: SpellCastTargets_SpellCastTargetFlags,
}

impl SpellCastTargets {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target_flags: SpellCastTargetFlags
        w.write_all(&u16::from(self.target_flags.as_int()).to_le_bytes())?;

        if let Some(if_statement) = &self.target_flags.unit {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    // unit_target: PackedGuid
                    unit_target.write_packed_guid_into_vec(&mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Unit::TradeItem {
                    trade_item,
                } => {
                    // trade_item: PackedGuid
                    trade_item.write_packed_guid_into_vec(&mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Unit::UnitEnemy {
                    enemy_target,
                } => {
                    // enemy_target: PackedGuid
                    enemy_target.write_packed_guid_into_vec(&mut w)?;

                }
            }
        }

        if let Some(if_statement) = &self.target_flags.gameobject {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                    object_target,
                } => {
                    // object_target: PackedGuid
                    object_target.write_packed_guid_into_vec(&mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Gameobject::Locked {
                    locked_target,
                } => {
                    // locked_target: PackedGuid
                    locked_target.write_packed_guid_into_vec(&mut w)?;

                }
            }
        }

        if let Some(if_statement) = &self.target_flags.item {
            // item_target: PackedGuid
            if_statement.item_target.write_packed_guid_into_vec(&mut w)?;

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
                    corpse_target_ally,
                } => {
                    // corpse_target_ally: PackedGuid
                    corpse_target_ally.write_packed_guid_into_vec(&mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseEnemy {
                    corpse_target_enemy,
                } => {
                    // corpse_target_enemy: PackedGuid
                    corpse_target_enemy.write_packed_guid_into_vec(&mut w)?;

                }
            }
        }

        Ok(())
    }
}

impl SpellCastTargets {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::new(crate::util::read_u16_le(&mut r)?);

        let target_flags_UNIT = if target_flags.is_UNIT() {
            // unit_target: PackedGuid
            let unit_target = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                unit_target,
            })
        }
        else if target_flags.is_TRADE_ITEM() {
            // trade_item: PackedGuid
            let trade_item = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::TradeItem {
                trade_item,
            })
        }
        else if target_flags.is_UNIT_ENEMY() {
            // enemy_target: PackedGuid
            let enemy_target = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit::UnitEnemy {
                enemy_target,
            })
        }
        else {
            None
        };

        let target_flags_GAMEOBJECT = if target_flags.is_GAMEOBJECT() {
            // object_target: PackedGuid
            let object_target = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                object_target,
            })
        }
        else if target_flags.is_LOCKED() {
            // locked_target: PackedGuid
            let locked_target = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Gameobject::Locked {
                locked_target,
            })
        }
        else {
            None
        };

        let target_flags_ITEM = if target_flags.is_ITEM() {
            // item_target: PackedGuid
            let item_target = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Item {
                item_target,
            })
        }
        else {
            None
        };

        let target_flags_SOURCE_LOCATION = if target_flags.is_SOURCE_LOCATION() {
            // source: Vector3d
            let source = Vector3d::read(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_SourceLocation {
                source,
            })
        }
        else {
            None
        };

        let target_flags_DEST_LOCATION = if target_flags.is_DEST_LOCATION() {
            // destination: Vector3d
            let destination = Vector3d::read(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_DestLocation {
                destination,
            })
        }
        else {
            None
        };

        let target_flags_STRING = if target_flags.is_STRING() {
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

        let target_flags_CORPSE_ALLY = if target_flags.is_CORPSE_ALLY() {
            // corpse_target_ally: PackedGuid
            let corpse_target_ally = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseAlly {
                corpse_target_ally,
            })
        }
        else if target_flags.is_CORPSE_ENEMY() {
            // corpse_target_enemy: PackedGuid
            let corpse_target_enemy = Guid::read_packed(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseEnemy {
                corpse_target_enemy,
            })
        }
        else {
            None
        };

        let target_flags = SpellCastTargets_SpellCastTargetFlags {
            inner: target_flags.as_int(),
            unit: target_flags_UNIT,
            item: target_flags_ITEM,
            source_location: target_flags_SOURCE_LOCATION,
            dest_location: target_flags_DEST_LOCATION,
            gameobject: target_flags_GAMEOBJECT,
            string: target_flags_STRING,
            corpse_ally: target_flags_CORPSE_ALLY,
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
    TradeItem {
        trade_item: Guid,
    },
    UnitEnemy {
        enemy_target: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::Unit { .. } => 2,
            Self::TradeItem { .. } => 4096,
            Self::UnitEnemy { .. } => 128,
        }
    }

}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Unit {
                unit_target,
            } => {
                // Not an actual enum sent over the wire
                unit_target.size() // unit_target: PackedGuid
            }
            Self::TradeItem {
                trade_item,
            } => {
                // Not an actual enum sent over the wire
                trade_item.size() // trade_item: PackedGuid
            }
            Self::UnitEnemy {
                enemy_target,
            } => {
                // Not an actual enum sent over the wire
                enemy_target.size() // enemy_target: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_Gameobject {
    Gameobject {
        object_target: Guid,
    },
    Locked {
        locked_target: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Gameobject {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::Gameobject { .. } => 2048,
            Self::Locked { .. } => 16384,
        }
    }

}

impl SpellCastTargets_SpellCastTargetFlags_Gameobject {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Gameobject {
                object_target,
            } => {
                // Not an actual enum sent over the wire
                object_target.size() // object_target: PackedGuid
            }
            Self::Locked {
                locked_target,
            } => {
                // Not an actual enum sent over the wire
                locked_target.size() // locked_target: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    CorpseAlly {
        corpse_target_ally: Guid,
    },
    CorpseEnemy {
        corpse_target_enemy: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::CorpseAlly { .. } => 32768,
            Self::CorpseEnemy { .. } => 512,
        }
    }

}

impl SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::CorpseAlly {
                corpse_target_ally,
            } => {
                // Not an actual enum sent over the wire
                corpse_target_ally.size() // corpse_target_ally: PackedGuid
            }
            Self::CorpseEnemy {
                corpse_target_enemy,
            } => {
                // Not an actual enum sent over the wire
                corpse_target_enemy.size() // corpse_target_enemy: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags {
    inner: u16,
    unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,
    item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,
    source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,
    dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,
    gameobject: Option<SpellCastTargets_SpellCastTargetFlags_Gameobject>,
    string: Option<SpellCastTargets_SpellCastTargetFlags_String>,
    corpse_ally: Option<SpellCastTargets_SpellCastTargetFlags_CorpseAlly>,
}

impl SpellCastTargets_SpellCastTargetFlags {
    pub const fn new(inner: u16, unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,gameobject: Option<SpellCastTargets_SpellCastTargetFlags_Gameobject>,string: Option<SpellCastTargets_SpellCastTargetFlags_String>,corpse_ally: Option<SpellCastTargets_SpellCastTargetFlags_CorpseAlly>,) -> Self {
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

    pub const fn new_UNUSED1() -> Self {
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

    pub fn set_UNUSED1(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED1;
        self
    }

    pub const fn get_UNUSED1(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNUSED1) != 0
    }

    pub fn clear_UNUSED1(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED1.reverse_bits();
        self
    }

    pub const fn new_UNIT(unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
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

    pub fn set_UNIT(mut self, unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
        self.inner |= unit.as_int();
        self.unit = Some(unit);
        self
    }

    pub const fn get_UNIT(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Unit> {
        self.unit.as_ref()
    }

    pub fn clear_UNIT(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT.reverse_bits();
        self.unit = None;
        self
    }

    pub const fn new_UNIT_RAID() -> Self {
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

    pub fn set_UNIT_RAID(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_RAID;
        self
    }

    pub const fn get_UNIT_RAID(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_RAID) != 0
    }

    pub fn clear_UNIT_RAID(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_RAID.reverse_bits();
        self
    }

    pub const fn new_UNIT_PARTY() -> Self {
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

    pub fn set_UNIT_PARTY(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_PARTY;
        self
    }

    pub const fn get_UNIT_PARTY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_PARTY) != 0
    }

    pub fn clear_UNIT_PARTY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_PARTY.reverse_bits();
        self
    }

    pub const fn new_ITEM(item: SpellCastTargets_SpellCastTargetFlags_Item) -> Self {
        Self {
            inner: SpellCastTargetFlags::ITEM,
            unit: None,
            item: Some(item),
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse_ally: None,
        }
    }

    pub fn set_ITEM(mut self, item: SpellCastTargets_SpellCastTargetFlags_Item) -> Self {
        self.inner |= SpellCastTargetFlags::ITEM;
        self.item = Some(item);
        self
    }

    pub const fn get_ITEM(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Item> {
        self.item.as_ref()
    }

    pub fn clear_ITEM(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::ITEM.reverse_bits();
        self.item = None;
        self
    }

    pub const fn new_SOURCE_LOCATION(source_location: SpellCastTargets_SpellCastTargetFlags_SourceLocation) -> Self {
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

    pub fn set_SOURCE_LOCATION(mut self, source_location: SpellCastTargets_SpellCastTargetFlags_SourceLocation) -> Self {
        self.inner |= SpellCastTargetFlags::SOURCE_LOCATION;
        self.source_location = Some(source_location);
        self
    }

    pub const fn get_SOURCE_LOCATION(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_SourceLocation> {
        self.source_location.as_ref()
    }

    pub fn clear_SOURCE_LOCATION(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::SOURCE_LOCATION.reverse_bits();
        self.source_location = None;
        self
    }

    pub const fn new_DEST_LOCATION(dest_location: SpellCastTargets_SpellCastTargetFlags_DestLocation) -> Self {
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

    pub fn set_DEST_LOCATION(mut self, dest_location: SpellCastTargets_SpellCastTargetFlags_DestLocation) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self
    }

    pub const fn get_DEST_LOCATION(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_DestLocation> {
        self.dest_location.as_ref()
    }

    pub fn clear_DEST_LOCATION(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        self
    }

    pub const fn new_UNIT_ALLY() -> Self {
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

    pub fn set_UNIT_ALLY(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ALLY;
        self
    }

    pub const fn get_UNIT_ALLY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_ALLY) != 0
    }

    pub fn clear_UNIT_ALLY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ALLY.reverse_bits();
        self
    }

    pub const fn new_UNIT_DEAD() -> Self {
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

    pub fn set_UNIT_DEAD(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_DEAD;
        self
    }

    pub const fn get_UNIT_DEAD(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_DEAD) != 0
    }

    pub fn clear_UNIT_DEAD(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_DEAD.reverse_bits();
        self
    }

    pub const fn new_GAMEOBJECT(gameobject: SpellCastTargets_SpellCastTargetFlags_Gameobject) -> Self {
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

    pub fn set_GAMEOBJECT(mut self, gameobject: SpellCastTargets_SpellCastTargetFlags_Gameobject) -> Self {
        self.inner |= gameobject.as_int();
        self.gameobject = Some(gameobject);
        self
    }

    pub const fn get_GAMEOBJECT(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Gameobject> {
        self.gameobject.as_ref()
    }

    pub fn clear_GAMEOBJECT(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GAMEOBJECT.reverse_bits();
        self.gameobject = None;
        self
    }

    pub const fn new_STRING(string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
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

    pub fn set_STRING(mut self, string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        self.inner |= SpellCastTargetFlags::STRING;
        self.string = Some(string);
        self
    }

    pub const fn get_STRING(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_String> {
        self.string.as_ref()
    }

    pub fn clear_STRING(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::STRING.reverse_bits();
        self.string = None;
        self
    }

    pub const fn new_CORPSE_ALLY(corpse_ally: SpellCastTargets_SpellCastTargetFlags_CorpseAlly) -> Self {
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

    pub fn set_CORPSE_ALLY(mut self, corpse_ally: SpellCastTargets_SpellCastTargetFlags_CorpseAlly) -> Self {
        self.inner |= corpse_ally.as_int();
        self.corpse_ally = Some(corpse_ally);
        self
    }

    pub const fn get_CORPSE_ALLY(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_CorpseAlly> {
        self.corpse_ally.as_ref()
    }

    pub fn clear_CORPSE_ALLY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE_ALLY.reverse_bits();
        self.corpse_ally = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}
impl SpellCastTargets_SpellCastTargetFlags {
    pub(crate) fn size(&self) -> usize {
        2 // inner
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Item {
    pub item_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Item {
    pub(crate) fn size(&self) -> usize {
        self.item_target.size() // item_target: PackedGuid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub source: Vector3d,
}

impl SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub(crate) fn size(&self) -> usize {
        12 // source: Vector3d
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub destination: Vector3d,
}

impl SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub(crate) fn size(&self) -> usize {
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

