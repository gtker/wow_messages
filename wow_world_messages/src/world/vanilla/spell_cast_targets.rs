use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    SpellCastTargetFlags, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:118`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L118):
/// ```text
/// struct SpellCastTargets {
///     SpellCastTargetFlags target_flags;
///     if (target_flags & UNIT) {
///         PackedGuid unit_target;
///     }
///     if (target_flags & GAMEOBJECT) {
///         PackedGuid gameobject;
///     }
///     else if (target_flags & OBJECT_UNK) {
///         PackedGuid object_unk;
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
///     if (target_flags & CORPSE) {
///         PackedGuid corpse;
///     }
///     else if (target_flags & PVP_CORPSE) {
///         PackedGuid pvp_corpse;
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
            // unit_target: PackedGuid
            crate::util::write_packed_guid(&if_statement.unit_target, &mut w)?;

        }

        if let Some(if_statement) = &self.target_flags.gameobject {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                    gameobject,
                } => {
                    // gameobject: PackedGuid
                    crate::util::write_packed_guid(&gameobject, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Gameobject::ObjectUnk {
                    object_unk,
                } => {
                    // object_unk: PackedGuid
                    crate::util::write_packed_guid(&object_unk, &mut w)?;

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

        if let Some(if_statement) = &self.target_flags.corpse {
            match if_statement {
                SpellCastTargets_SpellCastTargetFlags_Corpse::Corpse {
                    corpse,
                } => {
                    // corpse: PackedGuid
                    crate::util::write_packed_guid(&corpse, &mut w)?;

                }
                SpellCastTargets_SpellCastTargetFlags_Corpse::PvpCorpse {
                    pvp_corpse,
                } => {
                    // pvp_corpse: PackedGuid
                    crate::util::write_packed_guid(&pvp_corpse, &mut w)?;

                }
            }
        }

        Ok(())
    }
}

impl SpellCastTargets {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::new(crate::util::read_u16_le(&mut r)?);

        let target_flags_unit = if target_flags.is_unit() {
            // unit_target: PackedGuid
            let unit_target = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Unit {
                unit_target,
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
        else if target_flags.is_object_unk() {
            // object_unk: PackedGuid
            let object_unk = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Gameobject::ObjectUnk {
                object_unk,
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

        let target_flags_corpse = if target_flags.is_corpse() {
            // corpse: PackedGuid
            let corpse = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Corpse::Corpse {
                corpse,
            })
        }
        else if target_flags.is_pvp_corpse() {
            // pvp_corpse: PackedGuid
            let pvp_corpse = crate::util::read_packed_guid(&mut r)?;

            Some(SpellCastTargets_SpellCastTargetFlags_Corpse::PvpCorpse {
                pvp_corpse,
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
            corpse: target_flags_corpse,
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
pub enum SpellCastTargets_SpellCastTargetFlags_Item {
    Item {
        item: Guid,
    },
    TradeItem {
        trade_item: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Item {
    pub(crate) const fn as_int(&self) -> u16 {
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
    ObjectUnk {
        object_unk: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Gameobject {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::Gameobject { .. } => 2048,
            Self::ObjectUnk { .. } => 128,
        }
    }

}

impl std::fmt::Display for SpellCastTargets_SpellCastTargetFlags_Gameobject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gameobject{ .. } => f.write_str("Gameobject"),
            Self::ObjectUnk{ .. } => f.write_str("ObjectUnk"),
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
            Self::ObjectUnk {
                object_unk,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&object_unk) // object_unk: PackedGuid
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellCastTargets_SpellCastTargetFlags_Corpse {
    Corpse {
        corpse: Guid,
    },
    PvpCorpse {
        pvp_corpse: Guid,
    },
}

impl SpellCastTargets_SpellCastTargetFlags_Corpse {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::Corpse { .. } => 32768,
            Self::PvpCorpse { .. } => 512,
        }
    }

}

impl std::fmt::Display for SpellCastTargets_SpellCastTargetFlags_Corpse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Corpse{ .. } => f.write_str("Corpse"),
            Self::PvpCorpse{ .. } => f.write_str("PvpCorpse"),
        }
    }
}

impl SpellCastTargets_SpellCastTargetFlags_Corpse {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Corpse {
                corpse,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&corpse) // corpse: PackedGuid
            }
            Self::PvpCorpse {
                pvp_corpse,
            } => {
                // Not an actual enum sent over the wire
                crate::util::packed_guid_size(&pvp_corpse) // pvp_corpse: PackedGuid
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
    corpse: Option<SpellCastTargets_SpellCastTargetFlags_Corpse>,
}

impl SpellCastTargets_SpellCastTargetFlags {
    pub const fn new(inner: u16, unit: Option<SpellCastTargets_SpellCastTargetFlags_Unit>,item: Option<SpellCastTargets_SpellCastTargetFlags_Item>,source_location: Option<SpellCastTargets_SpellCastTargetFlags_SourceLocation>,dest_location: Option<SpellCastTargets_SpellCastTargetFlags_DestLocation>,gameobject: Option<SpellCastTargets_SpellCastTargetFlags_Gameobject>,string: Option<SpellCastTargets_SpellCastTargetFlags_String>,corpse: Option<SpellCastTargets_SpellCastTargetFlags_Corpse>,) -> Self {
        Self {
            inner,
            unit, 
            item, 
            source_location, 
            dest_location, 
            gameobject, 
            string, 
            corpse, 
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
            corpse: None,
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
        && self.corpse.is_none()
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
            corpse: None,
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
            inner: SpellCastTargetFlags::UNIT,
            unit: Some(unit),
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit(mut self, unit: SpellCastTargets_SpellCastTargetFlags_Unit) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT;
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

    pub const fn new_unused2() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED2,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unused2(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED2;
        self
    }

    pub const fn get_unused2(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNUSED2) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unused2(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED2.reverse_bits();
        self
    }

    pub const fn new_unused3() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNUSED3,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unused3(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNUSED3;
        self
    }

    pub const fn get_unused3(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNUSED3) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unused3(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED3.reverse_bits();
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
            corpse: None,
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
            corpse: None,
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
            corpse: None,
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

    pub const fn new_unit_unk() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_UNK,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_unk(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_UNK;
        self
    }

    pub const fn get_unit_unk(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_UNK) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_unk(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_UNK.reverse_bits();
        self
    }

    pub const fn new_unit_corpse() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNIT_CORPSE,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unit_corpse(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_CORPSE;
        self
    }

    pub const fn get_unit_corpse(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNIT_CORPSE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unit_corpse(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_CORPSE.reverse_bits();
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
            corpse: None,
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
            corpse: None,
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

    pub const fn new_unk1() -> Self {
        Self {
            inner: SpellCastTargetFlags::UNK1,
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk1(mut self) -> Self {
        self.inner |= SpellCastTargetFlags::UNK1;
        self
    }

    pub const fn get_unk1(&self) -> bool {
        (self.inner & SpellCastTargetFlags::UNK1) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk1(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNK1.reverse_bits();
        self
    }

    pub const fn new_corpse(corpse: SpellCastTargets_SpellCastTargetFlags_Corpse) -> Self {
        Self {
            inner: corpse.as_int(),
            unit: None,
            item: None,
            source_location: None,
            dest_location: None,
            gameobject: None,
            string: None,
            corpse: Some(corpse),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_corpse(mut self, corpse: SpellCastTargets_SpellCastTargetFlags_Corpse) -> Self {
        self.inner |= corpse.as_int();
        self.corpse = Some(corpse);
        self
    }

    pub const fn get_corpse(&self) -> Option<&SpellCastTargets_SpellCastTargetFlags_Corpse> {
        self.corpse.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_corpse(mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE.reverse_bits();
        self.corpse = None;
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
            if let Some(s) = &self.corpse {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SpellCastTargets_SpellCastTargetFlags_Unit {
    pub unit_target: Guid,
}

impl SpellCastTargets_SpellCastTargetFlags_Unit {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit_target) // unit_target: PackedGuid
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

