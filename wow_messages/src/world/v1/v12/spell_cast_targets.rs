use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{SpellCastTargetFlags};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:100`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L100):
/// ```text
/// struct SpellCastTargets {
///     SpellCastTargetFlags target_flags;
///     if (target_flags & UNIT) {
///         PackedGuid unit_target1;
///     }
///     if (target_flags & UNIT_ENEMY) {
///         PackedGuid unit_target2;
///     }
///     if (target_flags & GAMEOBJECT) {
///         PackedGuid object_target1;
///     }
///     if (target_flags & LOCKED) {
///         PackedGuid object_target2;
///     }
///     if (target_flags & ITEM) {
///         PackedGuid item_target1;
///     }
///     if (target_flags & TRADE_ITEM) {
///         PackedGuid item_target2;
///     }
///     if (target_flags & SOURCE_LOCATION) {
///         f32 position_x1;
///         f32 position_y1;
///         f32 position_z1;
///     }
///     if (target_flags & DEST_LOCATION) {
///         f32 position_x2;
///         f32 position_y2;
///         f32 position_z2;
///     }
///     if (target_flags & STRING) {
///         CString target_string;
///     }
///     if (target_flags & CORPSE_ALLY) {
///         PackedGuid corpse_target1;
///     }
///     if (target_flags & CORPSE_ENEMY) {
///         PackedGuid corpse_target2;
///     }
/// }
/// ```
pub struct SpellCastTargets {
    pub target_flags: SpellCastTargetsSpellCastTargetFlags,
}

impl ReadableAndWritable for SpellCastTargets {
    type Error = SpellCastTargetsError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::read(r)?;

        let target_flags_UNIT = if target_flags.is_UNIT() {
            // unit_target1: PackedGuid
            let unit_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsUNIT {
                unit_target1,
            })
        } else {
            None
        };

        let target_flags_UNIT_ENEMY = if target_flags.is_UNIT_ENEMY() {
            // unit_target2: PackedGuid
            let unit_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
                unit_target2,
            })
        } else {
            None
        };

        let target_flags_GAMEOBJECT = if target_flags.is_GAMEOBJECT() {
            // object_target1: PackedGuid
            let object_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
                object_target1,
            })
        } else {
            None
        };

        let target_flags_LOCKED = if target_flags.is_LOCKED() {
            // object_target2: PackedGuid
            let object_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsLOCKED {
                object_target2,
            })
        } else {
            None
        };

        let target_flags_ITEM = if target_flags.is_ITEM() {
            // item_target1: PackedGuid
            let item_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsITEM {
                item_target1,
            })
        } else {
            None
        };

        let target_flags_TRADE_ITEM = if target_flags.is_TRADE_ITEM() {
            // item_target2: PackedGuid
            let item_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
                item_target2,
            })
        } else {
            None
        };

        let target_flags_SOURCE_LOCATION = if target_flags.is_SOURCE_LOCATION() {
            // position_x1: f32
            let position_x1 = crate::util::read_f32_le(r)?;
            // position_y1: f32
            let position_y1 = crate::util::read_f32_le(r)?;
            // position_z1: f32
            let position_z1 = crate::util::read_f32_le(r)?;
            Some(SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
                position_x1,
                position_y1,
                position_z1,
            })
        } else {
            None
        };

        let target_flags_DEST_LOCATION = if target_flags.is_DEST_LOCATION() {
            // position_x2: f32
            let position_x2 = crate::util::read_f32_le(r)?;
            // position_y2: f32
            let position_y2 = crate::util::read_f32_le(r)?;
            // position_z2: f32
            let position_z2 = crate::util::read_f32_le(r)?;
            Some(SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
                position_x2,
                position_y2,
                position_z2,
            })
        } else {
            None
        };

        let target_flags_STRING = if target_flags.is_STRING() {
            // target_string: CString
            let target_string = crate::util::read_c_string_to_vec(r)?;
            let target_string = String::from_utf8(target_string)?;

            Some(SpellCastTargetsSpellCastTargetFlagsSTRING {
                target_string,
            })
        } else {
            None
        };

        let target_flags_CORPSE_ALLY = if target_flags.is_CORPSE_ALLY() {
            // corpse_target1: PackedGuid
            let corpse_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
                corpse_target1,
            })
        } else {
            None
        };

        let target_flags_CORPSE_ENEMY = if target_flags.is_CORPSE_ENEMY() {
            // corpse_target2: PackedGuid
            let corpse_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
                corpse_target2,
            })
        } else {
            None
        };

        let target_flags = SpellCastTargetsSpellCastTargetFlags {
            inner: target_flags.as_u16(),
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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_flags: SpellCastTargetFlags
        self.target_flags.write(w)?;

        if let Some(s) = &self.target_flags.unit {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.unit_enemy {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.gameobject {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.locked {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.item {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.trade_item {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.source_location {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.dest_location {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.string {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.corpse_ally {
            s.write(w)?;
        }

        if let Some(s) = &self.target_flags.corpse_enemy {
            s.write(w)?;
        }

        Ok(())
    }

}

impl VariableSized for SpellCastTargets {
    fn size(&self) -> usize {
        self.target_flags.size() // target_flags: SpellCastTargetFlags and subfields
    }
}

impl MaximumPossibleSized for SpellCastTargets {
    fn maximum_possible_size() -> usize {
        SpellCastTargetFlags::maximum_possible_size() // target_flags: SpellCastTargetFlags
    }
}

#[derive(Debug)]
pub enum SpellCastTargetsError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SpellCastTargetsError {}
impl std::fmt::Display for SpellCastTargetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellCastTargetsError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SpellCastTargetsError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlags {
    inner: u16,
    unit: Option<SpellCastTargetsSpellCastTargetFlagsUNIT>,
    item: Option<SpellCastTargetsSpellCastTargetFlagsITEM>,
    source_location: Option<SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION>,
    dest_location: Option<SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION>,
    unit_enemy: Option<SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY>,
    corpse_enemy: Option<SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY>,
    gameobject: Option<SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT>,
    trade_item: Option<SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM>,
    string: Option<SpellCastTargetsSpellCastTargetFlagsSTRING>,
    locked: Option<SpellCastTargetsSpellCastTargetFlagsLOCKED>,
    corpse_ally: Option<SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY>,
}

impl From<&SpellCastTargetsSpellCastTargetFlags> for SpellCastTargetFlags {
    fn from(e: &SpellCastTargetsSpellCastTargetFlags) -> Self {
        Self::new(e.inner)
    }
}

impl SpellCastTargetsSpellCastTargetFlags {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SpellCastTargetFlags = self.into();
        a.write(w)?;
        Ok(())
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

}
impl VariableSized for SpellCastTargetsSpellCastTargetFlags {
    fn size(&self) -> usize {
        2 // inner: SpellCastTargetFlags (u16)
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

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlags {
    fn maximum_possible_size() -> usize {
        2 // inner: SpellCastTargetFlags (u16)
        + SpellCastTargetsSpellCastTargetFlagsUNIT::maximum_possible_size() // UNIT enumerator
        + SpellCastTargetsSpellCastTargetFlagsITEM::maximum_possible_size() // ITEM enumerator
        + SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION::maximum_possible_size() // SOURCE_LOCATION enumerator
        + SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION::maximum_possible_size() // DEST_LOCATION enumerator
        + SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY::maximum_possible_size() // UNIT_ENEMY enumerator
        + SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY::maximum_possible_size() // CORPSE_ENEMY enumerator
        + SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT::maximum_possible_size() // GAMEOBJECT enumerator
        + SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM::maximum_possible_size() // TRADE_ITEM enumerator
        + SpellCastTargetsSpellCastTargetFlagsSTRING::maximum_possible_size() // STRING enumerator
        + SpellCastTargetsSpellCastTargetFlagsLOCKED::maximum_possible_size() // LOCKED enumerator
        + SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY::maximum_possible_size() // CORPSE_ALLY enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsUNIT {
    pub unit_target1: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsUNIT {
    fn size(&self) -> usize {
        self.unit_target1.size() // unit_target1: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsUNIT {
    fn maximum_possible_size() -> usize {
        9 // unit_target1: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsUNIT {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.unit_target1.write_packed(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsITEM {
    pub item_target1: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsITEM {
    fn size(&self) -> usize {
        self.item_target1.size() // item_target1: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsITEM {
    fn maximum_possible_size() -> usize {
        9 // item_target1: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsITEM {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.item_target1.write_packed(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
    pub position_x1: f32,
    pub position_y1: f32,
    pub position_z1: f32,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
    fn size(&self) -> usize {
        4 // position_x1: f32
        + 4 // position_y1: f32
        + 4 // position_z1: f32
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
    fn maximum_possible_size() -> usize {
        4 // position_x1: f32
        + 4 // position_y1: f32
        + 4 // position_z1: f32
    }
}

impl SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.position_x1.to_le_bytes())?;

        w.write_all(&self.position_y1.to_le_bytes())?;

        w.write_all(&self.position_z1.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
    pub position_x2: f32,
    pub position_y2: f32,
    pub position_z2: f32,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
    fn size(&self) -> usize {
        4 // position_x2: f32
        + 4 // position_y2: f32
        + 4 // position_z2: f32
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
    fn maximum_possible_size() -> usize {
        4 // position_x2: f32
        + 4 // position_y2: f32
        + 4 // position_z2: f32
    }
}

impl SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.position_x2.to_le_bytes())?;

        w.write_all(&self.position_y2.to_le_bytes())?;

        w.write_all(&self.position_z2.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
    pub unit_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
    fn size(&self) -> usize {
        self.unit_target2.size() // unit_target2: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
    fn maximum_possible_size() -> usize {
        9 // unit_target2: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.unit_target2.write_packed(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
    pub corpse_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
    fn size(&self) -> usize {
        self.corpse_target2.size() // corpse_target2: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
    fn maximum_possible_size() -> usize {
        9 // corpse_target2: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.corpse_target2.write_packed(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
    pub object_target1: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
    fn size(&self) -> usize {
        self.object_target1.size() // object_target1: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
    fn maximum_possible_size() -> usize {
        9 // object_target1: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.object_target1.write_packed(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
    pub item_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
    fn size(&self) -> usize {
        self.item_target2.size() // item_target2: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
    fn maximum_possible_size() -> usize {
        9 // item_target2: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.item_target2.write_packed(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsSTRING {
    pub target_string: String,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsSTRING {
    fn size(&self) -> usize {
        self.target_string.len() + 1 // target_string: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsSTRING {
    fn maximum_possible_size() -> usize {
        256 // target_string: CString
    }
}

impl SpellCastTargetsSpellCastTargetFlagsSTRING {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(self.target_string.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsLOCKED {
    pub object_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsLOCKED {
    fn size(&self) -> usize {
        self.object_target2.size() // object_target2: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsLOCKED {
    fn maximum_possible_size() -> usize {
        9 // object_target2: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsLOCKED {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.object_target2.write_packed(w)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
    pub corpse_target1: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
    fn size(&self) -> usize {
        self.corpse_target1.size() // corpse_target1: PackedGuid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
    fn maximum_possible_size() -> usize {
        9 // corpse_target1: PackedGuid
    }
}

impl SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.corpse_target1.write_packed(w)?;

        Ok(())
    }
}

