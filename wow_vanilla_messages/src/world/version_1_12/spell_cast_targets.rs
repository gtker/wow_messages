use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::SpellCastTargetFlags;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets {
    pub target_flags: SpellCastTargets_SpellCastTargetFlags,
}

impl SpellCastTargets {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target_flags: SpellCastTargetFlags
        w.write_all(&(self.target_flags.as_int() as u16).to_le_bytes())?;

        if let Some(if_statement) = &self.target_flags.unit {
            // unit_target1: PackedGuid
            w.write_all(&if_statement.unit_target1.packed_guid())?;

        }

        if let Some(if_statement) = &self.target_flags.unit_enemy {
            // unit_target2: PackedGuid
            w.write_all(&if_statement.unit_target2.packed_guid())?;

        }

        if let Some(if_statement) = &self.target_flags.gameobject {
            // object_target1: PackedGuid
            w.write_all(&if_statement.object_target1.packed_guid())?;

        }

        if let Some(if_statement) = &self.target_flags.locked {
            // object_target2: PackedGuid
            w.write_all(&if_statement.object_target2.packed_guid())?;

        }

        if let Some(if_statement) = &self.target_flags.item {
            // item_target1: PackedGuid
            w.write_all(&if_statement.item_target1.packed_guid())?;

        }

        if let Some(if_statement) = &self.target_flags.trade_item {
            // item_target2: PackedGuid
            w.write_all(&if_statement.item_target2.packed_guid())?;

        }

        if let Some(if_statement) = &self.target_flags.source_location {
            // position_x1: f32
            w.write_all(&if_statement.position_x1.to_le_bytes())?;

            // position_y1: f32
            w.write_all(&if_statement.position_y1.to_le_bytes())?;

            // position_z1: f32
            w.write_all(&if_statement.position_z1.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.target_flags.dest_location {
            // position_x2: f32
            w.write_all(&if_statement.position_x2.to_le_bytes())?;

            // position_y2: f32
            w.write_all(&if_statement.position_y2.to_le_bytes())?;

            // position_z2: f32
            w.write_all(&if_statement.position_z2.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.target_flags.string {
            // target_string: CString
            w.write_all(if_statement.target_string.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        if let Some(if_statement) = &self.target_flags.corpse_ally {
            // corpse_target1: PackedGuid
            w.write_all(&if_statement.corpse_target1.packed_guid())?;

        }

        if let Some(if_statement) = &self.target_flags.corpse_enemy {
            // corpse_target2: PackedGuid
            w.write_all(&if_statement.corpse_target2.packed_guid())?;

        }

        Ok(())
    }
}

impl SpellCastTargets {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::new(crate::util::read_u16_le(r)?);

        let target_flags_UNIT = if target_flags.is_UNIT() {
            // unit_target1: PackedGuid
            let unit_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_UNIT {
                unit_target1,
            })
        }
        else {
            None
        };

        let target_flags_UNIT_ENEMY = if target_flags.is_UNIT_ENEMY() {
            // unit_target2: PackedGuid
            let unit_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_UNIT_ENEMY {
                unit_target2,
            })
        }
        else {
            None
        };

        let target_flags_GAMEOBJECT = if target_flags.is_GAMEOBJECT() {
            // object_target1: PackedGuid
            let object_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_GAMEOBJECT {
                object_target1,
            })
        }
        else {
            None
        };

        let target_flags_LOCKED = if target_flags.is_LOCKED() {
            // object_target2: PackedGuid
            let object_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_LOCKED {
                object_target2,
            })
        }
        else {
            None
        };

        let target_flags_ITEM = if target_flags.is_ITEM() {
            // item_target1: PackedGuid
            let item_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_ITEM {
                item_target1,
            })
        }
        else {
            None
        };

        let target_flags_TRADE_ITEM = if target_flags.is_TRADE_ITEM() {
            // item_target2: PackedGuid
            let item_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_TRADE_ITEM {
                item_target2,
            })
        }
        else {
            None
        };

        let target_flags_SOURCE_LOCATION = if target_flags.is_SOURCE_LOCATION() {
            // position_x1: f32
            let position_x1 = crate::util::read_f32_le(r)?;
            // position_y1: f32
            let position_y1 = crate::util::read_f32_le(r)?;
            // position_z1: f32
            let position_z1 = crate::util::read_f32_le(r)?;
            Some(SpellCastTargets_SpellCastTargetFlags_SOURCE_LOCATION {
                position_x1,
                position_y1,
                position_z1,
            })
        }
        else {
            None
        };

        let target_flags_DEST_LOCATION = if target_flags.is_DEST_LOCATION() {
            // position_x2: f32
            let position_x2 = crate::util::read_f32_le(r)?;
            // position_y2: f32
            let position_y2 = crate::util::read_f32_le(r)?;
            // position_z2: f32
            let position_z2 = crate::util::read_f32_le(r)?;
            Some(SpellCastTargets_SpellCastTargetFlags_DEST_LOCATION {
                position_x2,
                position_y2,
                position_z2,
            })
        }
        else {
            None
        };

        let target_flags_STRING = if target_flags.is_STRING() {
            // target_string: CString
            let target_string = crate::util::read_c_string_to_vec(r)?;
            let target_string = String::from_utf8(target_string)?;

            Some(SpellCastTargets_SpellCastTargetFlags_STRING {
                target_string,
            })
        }
        else {
            None
        };

        let target_flags_CORPSE_ALLY = if target_flags.is_CORPSE_ALLY() {
            // corpse_target1: PackedGuid
            let corpse_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CORPSE_ALLY {
                corpse_target1,
            })
        }
        else {
            None
        };

        let target_flags_CORPSE_ENEMY = if target_flags.is_CORPSE_ENEMY() {
            // corpse_target2: PackedGuid
            let corpse_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_CORPSE_ENEMY {
                corpse_target2,
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags {
    inner: u16,
    unit: Option<SpellCastTargets_SpellCastTargetFlags_UNIT>,
    item: Option<SpellCastTargets_SpellCastTargetFlags_ITEM>,
    source_location: Option<SpellCastTargets_SpellCastTargetFlags_SOURCE_LOCATION>,
    dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DEST_LOCATION>,
    unit_enemy: Option<SpellCastTargets_SpellCastTargetFlags_UNIT_ENEMY>,
    corpse_enemy: Option<SpellCastTargets_SpellCastTargetFlags_CORPSE_ENEMY>,
    gameobject: Option<SpellCastTargets_SpellCastTargetFlags_GAMEOBJECT>,
    trade_item: Option<SpellCastTargets_SpellCastTargetFlags_TRADE_ITEM>,
    string: Option<SpellCastTargets_SpellCastTargetFlags_STRING>,
    locked: Option<SpellCastTargets_SpellCastTargetFlags_LOCKED>,
    corpse_ally: Option<SpellCastTargets_SpellCastTargetFlags_CORPSE_ALLY>,
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

    pub const fn new_SELF() -> Self {
        Self {
            inner: SpellCastTargetFlags::SELF,
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

    pub fn set_SELF(&mut self) -> Self {
        self.inner |= SpellCastTargetFlags::SELF;
        self.clone()
    }

    pub const fn get_SELF(&self) -> bool {
        // Underlying value is 0
        self.inner == SpellCastTargetFlags::SELF
    }

    pub fn clear_SELF(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::SELF.reverse_bits();
        self
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

    pub const fn new_UNIT(unit: SpellCastTargets_SpellCastTargetFlags_UNIT) -> Self {
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

    pub fn set_UNIT(&mut self, unit: SpellCastTargets_SpellCastTargetFlags_UNIT) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT;
        self.unit = Some(unit);
        self.clone()
    }

    pub const fn get_UNIT(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_UNIT> {
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

    pub const fn new_ITEM(item: SpellCastTargets_SpellCastTargetFlags_ITEM) -> Self {
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

    pub fn set_ITEM(&mut self, item: SpellCastTargets_SpellCastTargetFlags_ITEM) -> Self {
        self.inner |= SpellCastTargetFlags::ITEM;
        self.item = Some(item);
        self.clone()
    }

    pub const fn get_ITEM(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_ITEM> {
        self.item.as_ref()
    }

    pub fn clear_ITEM(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::ITEM.reverse_bits();
        self.item = None;
        self
    }

    pub const fn new_SOURCE_LOCATION(source_location: SpellCastTargets_SpellCastTargetFlags_SOURCE_LOCATION) -> Self {
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

    pub fn set_SOURCE_LOCATION(&mut self, source_location: SpellCastTargets_SpellCastTargetFlags_SOURCE_LOCATION) -> Self {
        self.inner |= SpellCastTargetFlags::SOURCE_LOCATION;
        self.source_location = Some(source_location);
        self.clone()
    }

    pub const fn get_SOURCE_LOCATION(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_SOURCE_LOCATION> {
        self.source_location.as_ref()
    }

    pub fn clear_SOURCE_LOCATION(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::SOURCE_LOCATION.reverse_bits();
        self.source_location = None;
        self
    }

    pub const fn new_DEST_LOCATION(dest_location: SpellCastTargets_SpellCastTargetFlags_DEST_LOCATION) -> Self {
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

    pub fn set_DEST_LOCATION(&mut self, dest_location: SpellCastTargets_SpellCastTargetFlags_DEST_LOCATION) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self.clone()
    }

    pub const fn get_DEST_LOCATION(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_DEST_LOCATION> {
        self.dest_location.as_ref()
    }

    pub fn clear_DEST_LOCATION(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        self
    }

    pub const fn new_UNIT_ENEMY(unit_enemy: SpellCastTargets_SpellCastTargetFlags_UNIT_ENEMY) -> Self {
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

    pub fn set_UNIT_ENEMY(&mut self, unit_enemy: SpellCastTargets_SpellCastTargetFlags_UNIT_ENEMY) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ENEMY;
        self.unit_enemy = Some(unit_enemy);
        self.clone()
    }

    pub const fn get_UNIT_ENEMY(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_UNIT_ENEMY> {
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

    pub const fn new_CORPSE_ENEMY(corpse_enemy: SpellCastTargets_SpellCastTargetFlags_CORPSE_ENEMY) -> Self {
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

    pub fn set_CORPSE_ENEMY(&mut self, corpse_enemy: SpellCastTargets_SpellCastTargetFlags_CORPSE_ENEMY) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ENEMY;
        self.corpse_enemy = Some(corpse_enemy);
        self.clone()
    }

    pub const fn get_CORPSE_ENEMY(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_CORPSE_ENEMY> {
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

    pub const fn new_GAMEOBJECT(gameobject: SpellCastTargets_SpellCastTargetFlags_GAMEOBJECT) -> Self {
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

    pub fn set_GAMEOBJECT(&mut self, gameobject: SpellCastTargets_SpellCastTargetFlags_GAMEOBJECT) -> Self {
        self.inner |= SpellCastTargetFlags::GAMEOBJECT;
        self.gameobject = Some(gameobject);
        self.clone()
    }

    pub const fn get_GAMEOBJECT(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_GAMEOBJECT> {
        self.gameobject.as_ref()
    }

    pub fn clear_GAMEOBJECT(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GAMEOBJECT.reverse_bits();
        self.gameobject = None;
        self
    }

    pub const fn new_TRADE_ITEM(trade_item: SpellCastTargets_SpellCastTargetFlags_TRADE_ITEM) -> Self {
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

    pub fn set_TRADE_ITEM(&mut self, trade_item: SpellCastTargets_SpellCastTargetFlags_TRADE_ITEM) -> Self {
        self.inner |= SpellCastTargetFlags::TRADE_ITEM;
        self.trade_item = Some(trade_item);
        self.clone()
    }

    pub const fn get_TRADE_ITEM(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_TRADE_ITEM> {
        self.trade_item.as_ref()
    }

    pub fn clear_TRADE_ITEM(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::TRADE_ITEM.reverse_bits();
        self.trade_item = None;
        self
    }

    pub const fn new_STRING(string: SpellCastTargets_SpellCastTargetFlags_STRING) -> Self {
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

    pub fn set_STRING(&mut self, string: SpellCastTargets_SpellCastTargetFlags_STRING) -> Self {
        self.inner |= SpellCastTargetFlags::STRING;
        self.string = Some(string);
        self.clone()
    }

    pub const fn get_STRING(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_STRING> {
        self.string.as_ref()
    }

    pub fn clear_STRING(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::STRING.reverse_bits();
        self.string = None;
        self
    }

    pub const fn new_LOCKED(locked: SpellCastTargets_SpellCastTargetFlags_LOCKED) -> Self {
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

    pub fn set_LOCKED(&mut self, locked: SpellCastTargets_SpellCastTargetFlags_LOCKED) -> Self {
        self.inner |= SpellCastTargetFlags::LOCKED;
        self.locked = Some(locked);
        self.clone()
    }

    pub const fn get_LOCKED(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_LOCKED> {
        self.locked.as_ref()
    }

    pub fn clear_LOCKED(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::LOCKED.reverse_bits();
        self.locked = None;
        self
    }

    pub const fn new_CORPSE_ALLY(corpse_ally: SpellCastTargets_SpellCastTargetFlags_CORPSE_ALLY) -> Self {
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

    pub fn set_CORPSE_ALLY(&mut self, corpse_ally: SpellCastTargets_SpellCastTargetFlags_CORPSE_ALLY) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ALLY;
        self.corpse_ally = Some(corpse_ally);
        self.clone()
    }

    pub const fn get_CORPSE_ALLY(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_CORPSE_ALLY> {
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

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_UNIT {
    pub unit_target1: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_UNIT {
    pub(crate) fn size(&self) -> usize {
        self.unit_target1.size() // unit_target1: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_ITEM {
    pub item_target1: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_ITEM {
    pub(crate) fn size(&self) -> usize {
        self.item_target1.size() // item_target1: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_SOURCE_LOCATION {
    pub position_x1: f32,
    pub position_y1: f32,
    pub position_z1: f32,
}

impl SpellCastTargets_SpellCastTargetFlags_SOURCE_LOCATION {
    pub(crate) fn size(&self) -> usize {
        4 // position_x1: f32
        + 4 // position_y1: f32
        + 4 // position_z1: f32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_DEST_LOCATION {
    pub position_x2: f32,
    pub position_y2: f32,
    pub position_z2: f32,
}

impl SpellCastTargets_SpellCastTargetFlags_DEST_LOCATION {
    pub(crate) fn size(&self) -> usize {
        4 // position_x2: f32
        + 4 // position_y2: f32
        + 4 // position_z2: f32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_UNIT_ENEMY {
    pub unit_target2: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_UNIT_ENEMY {
    pub(crate) fn size(&self) -> usize {
        self.unit_target2.size() // unit_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_CORPSE_ENEMY {
    pub corpse_target2: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_CORPSE_ENEMY {
    pub(crate) fn size(&self) -> usize {
        self.corpse_target2.size() // corpse_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_GAMEOBJECT {
    pub object_target1: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_GAMEOBJECT {
    pub(crate) fn size(&self) -> usize {
        self.object_target1.size() // object_target1: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_TRADE_ITEM {
    pub item_target2: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_TRADE_ITEM {
    pub(crate) fn size(&self) -> usize {
        self.item_target2.size() // item_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_STRING {
    pub target_string: String,
}

impl SpellCastTargets_SpellCastTargetFlags_STRING {
    pub(crate) fn size(&self) -> usize {
        self.target_string.len() + 1 // target_string: CString
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_LOCKED {
    pub object_target2: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_LOCKED {
    pub(crate) fn size(&self) -> usize {
        self.object_target2.size() // object_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargets_SpellCastTargetFlags_CORPSE_ALLY {
    pub corpse_target1: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_CORPSE_ALLY {
    pub(crate) fn size(&self) -> usize {
        self.corpse_target1.size() // corpse_target1: Guid
    }
}

