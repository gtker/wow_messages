use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellCastTargetFlags;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:81`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L81):
/// ```text
/// struct SpellCastTargets {
///     SpellCastTargetFlags target_flags;
///     if (target_flags & UNIT) {
///         PackedGuid unit_target;
///     }
///     if (target_flags & UNIT_ENEMY) {
///         PackedGuid unit_enemy_target;
///     }
///     if (target_flags & GAMEOBJECT) {
///         PackedGuid object_target;
///     }
///     if (target_flags & LOCKED) {
///         PackedGuid object_target_locked;
///     }
///     if (target_flags & ITEM) {
///         PackedGuid item_target;
///     }
///     if (target_flags & TRADE_ITEM) {
///         PackedGuid item_trade_target;
///     }
///     if (target_flags & SOURCE_LOCATION) {
///         f32 source_position_x;
///         f32 source_position_y;
///         f32 source_position_z;
///     }
///     if (target_flags & DEST_LOCATION) {
///         f32 destination_position_x;
///         f32 destination_position_y;
///         f32 destination_position_z;
///     }
///     if (target_flags & STRING) {
///         CString target_string;
///     }
///     if (target_flags & CORPSE_ALLY) {
///         PackedGuid corpse_target_ally;
///     }
///     if (target_flags & CORPSE_ENEMY) {
///         PackedGuid corpse_target_enemy;
///     }
/// }
/// ```
pub struct SpellCastTargets {
    pub target_flags: SpellCastTargets_SpellCastTargetFlags,
}

impl SpellCastTargets {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target_flags: SpellCastTargetFlags
        w.write_all(&(self.target_flags.as_int() as u16).to_le_bytes())?;

        if let Some(if_statement) = &self.target_flags.unit {
            // unit_target: PackedGuid
            if_statement.unit_target.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.unit_enemy {
            // unit_enemy_target: PackedGuid
            if_statement.unit_enemy_target.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.gameobject {
            // object_target: PackedGuid
            if_statement.object_target.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.locked {
            // object_target_locked: PackedGuid
            if_statement.object_target_locked.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.item {
            // item_target: PackedGuid
            if_statement.item_target.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.trade_item {
            // item_trade_target: PackedGuid
            if_statement.item_trade_target.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.source_location {
            // source_position_x: f32
            w.write_all(&if_statement.source_position_x.to_le_bytes())?;

            // source_position_y: f32
            w.write_all(&if_statement.source_position_y.to_le_bytes())?;

            // source_position_z: f32
            w.write_all(&if_statement.source_position_z.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.target_flags.dest_location {
            // destination_position_x: f32
            w.write_all(&if_statement.destination_position_x.to_le_bytes())?;

            // destination_position_y: f32
            w.write_all(&if_statement.destination_position_y.to_le_bytes())?;

            // destination_position_z: f32
            w.write_all(&if_statement.destination_position_z.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.target_flags.string {
            // target_string: CString
            w.write_all(if_statement.target_string.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        if let Some(if_statement) = &self.target_flags.corpse_ally {
            // corpse_target_ally: PackedGuid
            if_statement.corpse_target_ally.write_packed_guid_into_vec(w);

        }

        if let Some(if_statement) = &self.target_flags.corpse_enemy {
            // corpse_target_enemy: PackedGuid
            if_statement.corpse_target_enemy.write_packed_guid_into_vec(w);

        }

        Ok(())
    }
}

impl SpellCastTargets {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::new(crate::util::read_u16_le(r)?);

        let target_flags_UNIT = if target_flags.is_UNIT() {
            // unit_target: PackedGuid
            let unit_target = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit {
                unit_target,
            })
        }
        else {
            None
        };

        let target_flags_UNIT_ENEMY = if target_flags.is_UNIT_ENEMY() {
            // unit_enemy_target: PackedGuid
            let unit_enemy_target = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_UnitEnemy {
                unit_enemy_target,
            })
        }
        else {
            None
        };

        let target_flags_GAMEOBJECT = if target_flags.is_GAMEOBJECT() {
            // object_target: PackedGuid
            let object_target = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Gameobject {
                object_target,
            })
        }
        else {
            None
        };

        let target_flags_LOCKED = if target_flags.is_LOCKED() {
            // object_target_locked: PackedGuid
            let object_target_locked = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Locked {
                object_target_locked,
            })
        }
        else {
            None
        };

        let target_flags_ITEM = if target_flags.is_ITEM() {
            // item_target: PackedGuid
            let item_target = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Item {
                item_target,
            })
        }
        else {
            None
        };

        let target_flags_TRADE_ITEM = if target_flags.is_TRADE_ITEM() {
            // item_trade_target: PackedGuid
            let item_trade_target = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_TradeItem {
                item_trade_target,
            })
        }
        else {
            None
        };

        let target_flags_SOURCE_LOCATION = if target_flags.is_SOURCE_LOCATION() {
            // source_position_x: f32
            let source_position_x = crate::util::read_f32_le(r)?;
            // source_position_y: f32
            let source_position_y = crate::util::read_f32_le(r)?;
            // source_position_z: f32
            let source_position_z = crate::util::read_f32_le(r)?;
            Some(SpellCastTargets_SpellCastTargetFlags_SourceLocation {
                source_position_x,
                source_position_y,
                source_position_z,
            })
        }
        else {
            None
        };

        let target_flags_DEST_LOCATION = if target_flags.is_DEST_LOCATION() {
            // destination_position_x: f32
            let destination_position_x = crate::util::read_f32_le(r)?;
            // destination_position_y: f32
            let destination_position_y = crate::util::read_f32_le(r)?;
            // destination_position_z: f32
            let destination_position_z = crate::util::read_f32_le(r)?;
            Some(SpellCastTargets_SpellCastTargetFlags_DestLocation {
                destination_position_x,
                destination_position_y,
                destination_position_z,
            })
        }
        else {
            None
        };

        let target_flags_STRING = if target_flags.is_STRING() {
            // target_string: CString
            let target_string = crate::util::read_c_string_to_vec(r)?;
            let target_string = String::from_utf8(target_string)?;

            Some(SpellCastTargets_SpellCastTargetFlags_String {
                target_string,
            })
        }
        else {
            None
        };

        let target_flags_CORPSE_ALLY = if target_flags.is_CORPSE_ALLY() {
            // corpse_target_ally: PackedGuid
            let corpse_target_ally = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
                corpse_target_ally,
            })
        }
        else {
            None
        };

        let target_flags_CORPSE_ENEMY = if target_flags.is_CORPSE_ENEMY() {
            // corpse_target_enemy: PackedGuid
            let corpse_target_enemy = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CorpseEnemy {
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
            unit_enemy: target_flags_UNIT_ENEMY,
            corpse_enemy: target_flags_CORPSE_ENEMY,
            gameobject: target_flags_GAMEOBJECT,
            trade_item: target_flags_TRADE_ITEM,
            string: target_flags_STRING,
            locked: target_flags_LOCKED,
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags {
    inner: u16,
    unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,
    item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,
    source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,
    dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,
    unit_enemy: Option<SpellCastTargets_SpellCastTargetFlags_UnitEnemy>,
    corpse_enemy: Option<SpellCastTargets_SpellCastTargetFlags_CorpseEnemy>,
    gameobject: Option<SpellCastTargets_SpellCastTargetFlags_Gameobject>,
    trade_item: Option<SpellCastTargets_SpellCastTargetFlags_TradeItem>,
    string: Option<SpellCastTargets_SpellCastTargetFlags_String>,
    locked: Option<SpellCastTargets_SpellCastTargetFlags_Locked>,
    corpse_ally: Option<SpellCastTargets_SpellCastTargetFlags_CorpseAlly>,
}

impl SpellCastTargets_SpellCastTargetFlags {
    pub const fn empty() -> Self {
        Self {
            inner: 0,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.unit.is_none()
        && self.item.is_none()
        && self.source_location.is_none()
        && self.dest_location.is_none()
        && self.unit_enemy.is_none()
        && self.corpse_enemy.is_none()
        && self.gameobject.is_none()
        && self.trade_item.is_none()
        && self.string.is_none()
        && self.locked.is_none()
        && self.corpse_ally.is_none()
    }

    pub const fn new_UNUSED1() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED1,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_UNUSED1(&mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED1;
        self.clone()
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
            inner: SpellCastTargetFlags::UNIT,
            unit: Some(unit),
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_UNIT(&mut self, unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT;
        self.unit = Some(unit);
        self.clone()
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
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_UNIT_RAID(&mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_RAID;
        self.clone()
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
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_UNIT_PARTY(&mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_PARTY;
        self.clone()
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
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_ITEM(&mut self, item: SpellCastTargets_SpellCastTargetFlags_Item) -> Self {
        self.inner |= SpellCastTargetFlags::ITEM;
        self.item = Some(item);
        self.clone()
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
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_SOURCE_LOCATION(&mut self, source_location: SpellCastTargets_SpellCastTargetFlags_SourceLocation) -> Self {
        self.inner |= SpellCastTargetFlags::SOURCE_LOCATION;
        self.source_location = Some(source_location);
        self.clone()
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
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_DEST_LOCATION(&mut self, dest_location: SpellCastTargets_SpellCastTargetFlags_DestLocation) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self.clone()
    }

    pub const fn get_DEST_LOCATION(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_DestLocation> {
        self.dest_location.as_ref()
    }

    pub fn clear_DEST_LOCATION(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        self
    }

    pub const fn new_UNIT_ENEMY(unit_enemy: SpellCastTargets_SpellCastTargetFlags_UnitEnemy) -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_ENEMY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: Some(unit_enemy),
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_UNIT_ENEMY(&mut self, unit_enemy: SpellCastTargets_SpellCastTargetFlags_UnitEnemy) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ENEMY;
        self.unit_enemy = Some(unit_enemy);
        self.clone()
    }

    pub const fn get_UNIT_ENEMY(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_UnitEnemy> {
        self.unit_enemy.as_ref()
    }

    pub fn clear_UNIT_ENEMY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ENEMY.reverse_bits();
        self.unit_enemy = None;
        self
    }

    pub const fn new_UNIT_ALLY() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_ALLY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_UNIT_ALLY(&mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ALLY;
        self.clone()
    }

    pub const fn get_UNIT_ALLY(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_ALLY) != 0
    }

    pub fn clear_UNIT_ALLY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ALLY.reverse_bits();
        self
    }

    pub const fn new_CORPSE_ENEMY(corpse_enemy: SpellCastTargets_SpellCastTargetFlags_CorpseEnemy) -> Self {
        Self {
            inner: SpellCastTargetFlags::CORPSE_ENEMY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: Some(corpse_enemy),
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_CORPSE_ENEMY(&mut self, corpse_enemy: SpellCastTargets_SpellCastTargetFlags_CorpseEnemy) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ENEMY;
        self.corpse_enemy = Some(corpse_enemy);
        self.clone()
    }

    pub const fn get_CORPSE_ENEMY(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_CorpseEnemy> {
        self.corpse_enemy.as_ref()
    }

    pub fn clear_CORPSE_ENEMY(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE_ENEMY.reverse_bits();
        self.corpse_enemy = None;
        self
    }

    pub const fn new_UNIT_DEAD() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_DEAD,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_UNIT_DEAD(&mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_DEAD;
        self.clone()
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
            inner: SpellCastTargetFlags::GAMEOBJECT,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: Some(gameobject),
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_GAMEOBJECT(&mut self, gameobject: SpellCastTargets_SpellCastTargetFlags_Gameobject) -> Self {
        self.inner |= SpellCastTargetFlags::GAMEOBJECT;
        self.gameobject = Some(gameobject);
        self.clone()
    }

    pub const fn get_GAMEOBJECT(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Gameobject> {
        self.gameobject.as_ref()
    }

    pub fn clear_GAMEOBJECT(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GAMEOBJECT.reverse_bits();
        self.gameobject = None;
        self
    }

    pub const fn new_TRADE_ITEM(trade_item: SpellCastTargets_SpellCastTargetFlags_TradeItem) -> Self {
        Self {
            inner: SpellCastTargetFlags::TRADE_ITEM,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: Some(trade_item),
            string: None,
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_TRADE_ITEM(&mut self, trade_item: SpellCastTargets_SpellCastTargetFlags_TradeItem) -> Self {
        self.inner |= SpellCastTargetFlags::TRADE_ITEM;
        self.trade_item = Some(trade_item);
        self.clone()
    }

    pub const fn get_TRADE_ITEM(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_TradeItem> {
        self.trade_item.as_ref()
    }

    pub fn clear_TRADE_ITEM(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::TRADE_ITEM.reverse_bits();
        self.trade_item = None;
        self
    }

    pub const fn new_STRING(string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        Self {
            inner: SpellCastTargetFlags::STRING,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: Some(string),
            locked: None,
            corpse_ally: None,
        }
    }

    pub fn set_STRING(&mut self, string: SpellCastTargets_SpellCastTargetFlags_String) -> Self {
        self.inner |= SpellCastTargetFlags::STRING;
        self.string = Some(string);
        self.clone()
    }

    pub const fn get_STRING(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_String> {
        self.string.as_ref()
    }

    pub fn clear_STRING(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::STRING.reverse_bits();
        self.string = None;
        self
    }

    pub const fn new_LOCKED(locked: SpellCastTargets_SpellCastTargetFlags_Locked) -> Self {
        Self {
            inner: SpellCastTargetFlags::LOCKED,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: Some(locked),
            corpse_ally: None,
        }
    }

    pub fn set_LOCKED(&mut self, locked: SpellCastTargets_SpellCastTargetFlags_Locked) -> Self {
        self.inner |= SpellCastTargetFlags::LOCKED;
        self.locked = Some(locked);
        self.clone()
    }

    pub const fn get_LOCKED(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Locked> {
        self.locked.as_ref()
    }

    pub fn clear_LOCKED(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::LOCKED.reverse_bits();
        self.locked = None;
        self
    }

    pub const fn new_CORPSE_ALLY(corpse_ally: SpellCastTargets_SpellCastTargetFlags_CorpseAlly) -> Self {
        Self {
            inner: SpellCastTargetFlags::CORPSE_ALLY,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            unit_enemy: None,
            corpse_enemy: None,
            gameobject: None,
            trade_item: None,
            string: None,
            locked: None,
            corpse_ally: Some(corpse_ally),
        }
    }

    pub fn set_CORPSE_ALLY(&mut self, corpse_ally: SpellCastTargets_SpellCastTargetFlags_CorpseAlly) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ALLY;
        self.corpse_ally = Some(corpse_ally);
        self.clone()
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
            if let Some(s) = &self.unit_enemy {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.corpse_enemy {
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
            if let Some(s) = &self.trade_item {
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
            if let Some(s) = &self.locked {
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Unit {
    pub unit_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) fn size(&self) -> usize {
        self.unit_target.size() // unit_target: Guid
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Item {
    pub item_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Item {
    pub(crate) fn size(&self) -> usize {
        self.item_target.size() // item_target: Guid
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub source_position_x: f32,
    pub source_position_y: f32,
    pub source_position_z: f32,
}

impl SpellCastTargets_SpellCastTargetFlags_SourceLocation {
    pub(crate) fn size(&self) -> usize {
        4 // source_position_x: f32
        + 4 // source_position_y: f32
        + 4 // source_position_z: f32
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub destination_position_x: f32,
    pub destination_position_y: f32,
    pub destination_position_z: f32,
}

impl SpellCastTargets_SpellCastTargetFlags_DestLocation {
    pub(crate) fn size(&self) -> usize {
        4 // destination_position_x: f32
        + 4 // destination_position_y: f32
        + 4 // destination_position_z: f32
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_UnitEnemy {
    pub unit_enemy_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_UnitEnemy {
    pub(crate) fn size(&self) -> usize {
        self.unit_enemy_target.size() // unit_enemy_target: Guid
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_CorpseEnemy {
    pub corpse_target_enemy: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_CorpseEnemy {
    pub(crate) fn size(&self) -> usize {
        self.corpse_target_enemy.size() // corpse_target_enemy: Guid
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Gameobject {
    pub object_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Gameobject {
    pub(crate) fn size(&self) -> usize {
        self.object_target.size() // object_target: Guid
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_TradeItem {
    pub item_trade_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_TradeItem {
    pub(crate) fn size(&self) -> usize {
        self.item_trade_target.size() // item_trade_target: Guid
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_String {
    pub target_string: String,
}

impl SpellCastTargets_SpellCastTargetFlags_String {
    pub(crate) fn size(&self) -> usize {
        self.target_string.len() + 1 // target_string: CString
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Locked {
    pub object_target_locked: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Locked {
    pub(crate) fn size(&self) -> usize {
        self.object_target_locked.size() // object_target_locked: Guid
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    pub corpse_target_ally: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_CorpseAlly {
    pub(crate) fn size(&self) -> usize {
        self.corpse_target_ally.size() // corpse_target_ally: Guid
    }
}

