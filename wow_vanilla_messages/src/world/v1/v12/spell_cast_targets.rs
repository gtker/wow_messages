use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellCastTargetFlags};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellCastTargets {
    pub target_flags: SpellCastTargetsSpellCastTargetFlags,
}

impl ReadableAndWritable for SpellCastTargets {
    type Error = SpellCastTargetsError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // target_flags: SpellCastTargetFlags
        let target_flags = SpellCastTargetFlags::read(r)?;

        let target_flags_UNIT = if target_flags.is_UNIT() {
            // unit_target1: PackedGuid
            let unit_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsUNIT {
                unit_target1,
            })
        }
        else {
            None
        };

        let target_flags_UNIT_ENEMY = if target_flags.is_UNIT_ENEMY() {
            // unit_target2: PackedGuid
            let unit_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
                unit_target2,
            })
        }
        else {
            None
        };

        let target_flags_GAMEOBJECT = if target_flags.is_GAMEOBJECT() {
            // object_target1: PackedGuid
            let object_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
                object_target1,
            })
        }
        else {
            None
        };

        let target_flags_LOCKED = if target_flags.is_LOCKED() {
            // object_target2: PackedGuid
            let object_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsLOCKED {
                object_target2,
            })
        }
        else {
            None
        };

        let target_flags_ITEM = if target_flags.is_ITEM() {
            // item_target1: PackedGuid
            let item_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsITEM {
                item_target1,
            })
        }
        else {
            None
        };

        let target_flags_TRADE_ITEM = if target_flags.is_TRADE_ITEM() {
            // item_target2: PackedGuid
            let item_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
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
            Some(SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
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
            Some(SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
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

            Some(SpellCastTargetsSpellCastTargetFlagsSTRING {
                target_string,
            })
        }
        else {
            None
        };

        let target_flags_CORPSE_ALLY = if target_flags.is_CORPSE_ALLY() {
            // corpse_target1: PackedGuid
            let corpse_target1 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
                corpse_target1,
            })
        }
        else {
            None
        };

        let target_flags_CORPSE_ENEMY = if target_flags.is_CORPSE_ENEMY() {
            // corpse_target2: PackedGuid
            let corpse_target2 = Guid::read_packed(r)?;

            Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
                corpse_target2,
            })
        }
        else {
            None
        };

        let target_flags = SpellCastTargetsSpellCastTargetFlags {
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

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_flags: SpellCastTargetFlags
        self.target_flags.write(w)?;

        if let Some(if_statement) = &self.target_flags.unit {
            // unit_target1: PackedGuid
            if_statement.unit_target1.write_packed(w)?;

        }

        if let Some(if_statement) = &self.target_flags.unit_enemy {
            // unit_target2: PackedGuid
            if_statement.unit_target2.write_packed(w)?;

        }

        if let Some(if_statement) = &self.target_flags.gameobject {
            // object_target1: PackedGuid
            if_statement.object_target1.write_packed(w)?;

        }

        if let Some(if_statement) = &self.target_flags.locked {
            // object_target2: PackedGuid
            if_statement.object_target2.write_packed(w)?;

        }

        if let Some(if_statement) = &self.target_flags.item {
            // item_target1: PackedGuid
            if_statement.item_target1.write_packed(w)?;

        }

        if let Some(if_statement) = &self.target_flags.trade_item {
            // item_target2: PackedGuid
            if_statement.item_target2.write_packed(w)?;

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
            if_statement.corpse_target1.write_packed(w)?;

        }

        if let Some(if_statement) = &self.target_flags.corpse_enemy {
            // corpse_target2: PackedGuid
            if_statement.corpse_target2.write_packed(w)?;

        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_flags: SpellCastTargetFlags
            let target_flags = SpellCastTargetFlags::tokio_read(r).await?;

            let target_flags_UNIT = if target_flags.is_UNIT() {
                // unit_target1: PackedGuid
                let unit_target1 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsUNIT {
                    unit_target1,
                })
            }
            else {
                None
            };

            let target_flags_UNIT_ENEMY = if target_flags.is_UNIT_ENEMY() {
                // unit_target2: PackedGuid
                let unit_target2 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
                    unit_target2,
                })
            }
            else {
                None
            };

            let target_flags_GAMEOBJECT = if target_flags.is_GAMEOBJECT() {
                // object_target1: PackedGuid
                let object_target1 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
                    object_target1,
                })
            }
            else {
                None
            };

            let target_flags_LOCKED = if target_flags.is_LOCKED() {
                // object_target2: PackedGuid
                let object_target2 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsLOCKED {
                    object_target2,
                })
            }
            else {
                None
            };

            let target_flags_ITEM = if target_flags.is_ITEM() {
                // item_target1: PackedGuid
                let item_target1 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsITEM {
                    item_target1,
                })
            }
            else {
                None
            };

            let target_flags_TRADE_ITEM = if target_flags.is_TRADE_ITEM() {
                // item_target2: PackedGuid
                let item_target2 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
                    item_target2,
                })
            }
            else {
                None
            };

            let target_flags_SOURCE_LOCATION = if target_flags.is_SOURCE_LOCATION() {
                // position_x1: f32
                let position_x1 = crate::util::tokio_read_f32_le(r).await?;
                // position_y1: f32
                let position_y1 = crate::util::tokio_read_f32_le(r).await?;
                // position_z1: f32
                let position_z1 = crate::util::tokio_read_f32_le(r).await?;
                Some(SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
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
                let position_x2 = crate::util::tokio_read_f32_le(r).await?;
                // position_y2: f32
                let position_y2 = crate::util::tokio_read_f32_le(r).await?;
                // position_z2: f32
                let position_z2 = crate::util::tokio_read_f32_le(r).await?;
                Some(SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
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
                let target_string = crate::util::tokio_read_c_string_to_vec(r).await?;
                let target_string = String::from_utf8(target_string)?;

                Some(SpellCastTargetsSpellCastTargetFlagsSTRING {
                    target_string,
                })
            }
            else {
                None
            };

            let target_flags_CORPSE_ALLY = if target_flags.is_CORPSE_ALLY() {
                // corpse_target1: PackedGuid
                let corpse_target1 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
                    corpse_target1,
                })
            }
            else {
                None
            };

            let target_flags_CORPSE_ENEMY = if target_flags.is_CORPSE_ENEMY() {
                // corpse_target2: PackedGuid
                let corpse_target2 = Guid::tokio_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
                    corpse_target2,
                })
            }
            else {
                None
            };

            let target_flags = SpellCastTargetsSpellCastTargetFlags {
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
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_flags: SpellCastTargetFlags
            self.target_flags.tokio_write(w).await?;

            if let Some(if_statement) = &self.target_flags.unit {
                // unit_target1: PackedGuid
                if_statement.unit_target1.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.unit_enemy {
                // unit_target2: PackedGuid
                if_statement.unit_target2.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.gameobject {
                // object_target1: PackedGuid
                if_statement.object_target1.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.locked {
                // object_target2: PackedGuid
                if_statement.object_target2.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.item {
                // item_target1: PackedGuid
                if_statement.item_target1.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.trade_item {
                // item_target2: PackedGuid
                if_statement.item_target2.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.source_location {
                // position_x1: f32
                w.write_all(&if_statement.position_x1.to_le_bytes()).await?;

                // position_y1: f32
                w.write_all(&if_statement.position_y1.to_le_bytes()).await?;

                // position_z1: f32
                w.write_all(&if_statement.position_z1.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.target_flags.dest_location {
                // position_x2: f32
                w.write_all(&if_statement.position_x2.to_le_bytes()).await?;

                // position_y2: f32
                w.write_all(&if_statement.position_y2.to_le_bytes()).await?;

                // position_z2: f32
                w.write_all(&if_statement.position_z2.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.target_flags.string {
                // target_string: CString
                w.write_all(if_statement.target_string.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

            }

            if let Some(if_statement) = &self.target_flags.corpse_ally {
                // corpse_target1: PackedGuid
                if_statement.corpse_target1.tokio_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.corpse_enemy {
                // corpse_target2: PackedGuid
                if_statement.corpse_target2.tokio_write_packed(w).await?;

            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_flags: SpellCastTargetFlags
            let target_flags = SpellCastTargetFlags::astd_read(r).await?;

            let target_flags_UNIT = if target_flags.is_UNIT() {
                // unit_target1: PackedGuid
                let unit_target1 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsUNIT {
                    unit_target1,
                })
            }
            else {
                None
            };

            let target_flags_UNIT_ENEMY = if target_flags.is_UNIT_ENEMY() {
                // unit_target2: PackedGuid
                let unit_target2 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
                    unit_target2,
                })
            }
            else {
                None
            };

            let target_flags_GAMEOBJECT = if target_flags.is_GAMEOBJECT() {
                // object_target1: PackedGuid
                let object_target1 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
                    object_target1,
                })
            }
            else {
                None
            };

            let target_flags_LOCKED = if target_flags.is_LOCKED() {
                // object_target2: PackedGuid
                let object_target2 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsLOCKED {
                    object_target2,
                })
            }
            else {
                None
            };

            let target_flags_ITEM = if target_flags.is_ITEM() {
                // item_target1: PackedGuid
                let item_target1 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsITEM {
                    item_target1,
                })
            }
            else {
                None
            };

            let target_flags_TRADE_ITEM = if target_flags.is_TRADE_ITEM() {
                // item_target2: PackedGuid
                let item_target2 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
                    item_target2,
                })
            }
            else {
                None
            };

            let target_flags_SOURCE_LOCATION = if target_flags.is_SOURCE_LOCATION() {
                // position_x1: f32
                let position_x1 = crate::util::astd_read_f32_le(r).await?;
                // position_y1: f32
                let position_y1 = crate::util::astd_read_f32_le(r).await?;
                // position_z1: f32
                let position_z1 = crate::util::astd_read_f32_le(r).await?;
                Some(SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION {
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
                let position_x2 = crate::util::astd_read_f32_le(r).await?;
                // position_y2: f32
                let position_y2 = crate::util::astd_read_f32_le(r).await?;
                // position_z2: f32
                let position_z2 = crate::util::astd_read_f32_le(r).await?;
                Some(SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION {
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
                let target_string = crate::util::astd_read_c_string_to_vec(r).await?;
                let target_string = String::from_utf8(target_string)?;

                Some(SpellCastTargetsSpellCastTargetFlagsSTRING {
                    target_string,
                })
            }
            else {
                None
            };

            let target_flags_CORPSE_ALLY = if target_flags.is_CORPSE_ALLY() {
                // corpse_target1: PackedGuid
                let corpse_target1 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
                    corpse_target1,
                })
            }
            else {
                None
            };

            let target_flags_CORPSE_ENEMY = if target_flags.is_CORPSE_ENEMY() {
                // corpse_target2: PackedGuid
                let corpse_target2 = Guid::astd_read_packed(r).await?;

                Some(SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
                    corpse_target2,
                })
            }
            else {
                None
            };

            let target_flags = SpellCastTargetsSpellCastTargetFlags {
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
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_flags: SpellCastTargetFlags
            self.target_flags.astd_write(w).await?;

            if let Some(if_statement) = &self.target_flags.unit {
                // unit_target1: PackedGuid
                if_statement.unit_target1.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.unit_enemy {
                // unit_target2: PackedGuid
                if_statement.unit_target2.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.gameobject {
                // object_target1: PackedGuid
                if_statement.object_target1.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.locked {
                // object_target2: PackedGuid
                if_statement.object_target2.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.item {
                // item_target1: PackedGuid
                if_statement.item_target1.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.trade_item {
                // item_target2: PackedGuid
                if_statement.item_target2.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.source_location {
                // position_x1: f32
                w.write_all(&if_statement.position_x1.to_le_bytes()).await?;

                // position_y1: f32
                w.write_all(&if_statement.position_y1.to_le_bytes()).await?;

                // position_z1: f32
                w.write_all(&if_statement.position_z1.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.target_flags.dest_location {
                // position_x2: f32
                w.write_all(&if_statement.position_x2.to_le_bytes()).await?;

                // position_y2: f32
                w.write_all(&if_statement.position_y2.to_le_bytes()).await?;

                // position_z2: f32
                w.write_all(&if_statement.position_z2.to_le_bytes()).await?;

            }

            if let Some(if_statement) = &self.target_flags.string {
                // target_string: CString
                w.write_all(if_statement.target_string.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

            }

            if let Some(if_statement) = &self.target_flags.corpse_ally {
                // corpse_target1: PackedGuid
                if_statement.corpse_target1.astd_write_packed(w).await?;

            }

            if let Some(if_statement) = &self.target_flags.corpse_enemy {
                // corpse_target2: PackedGuid
                if_statement.corpse_target2.astd_write_packed(w).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for SpellCastTargets {
    fn size(&self) -> usize {
        0
        + self.target_flags.size() // target_flags: SpellCastTargetsSpellCastTargetFlags
    }
}

impl MaximumPossibleSized for SpellCastTargets {
    fn maximum_possible_size() -> usize {
        0
        + 354 // target_flags: SpellCastTargetsSpellCastTargetFlags
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

impl SpellCastTargetsSpellCastTargetFlags {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.inner.to_le_bytes()).await?;
        Ok(())
    }

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

    pub fn clear_SELF(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::SELF.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn clear_UNUSED1(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNUSED1.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNIT(unit: SpellCastTargetsSpellCastTargetFlagsUNIT) -> Self {
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

    pub fn set_UNIT(&mut self, unit: SpellCastTargetsSpellCastTargetFlagsUNIT) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT;
        self.unit = Some(unit);
        self.clone()
    }

    pub const fn get_UNIT(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsUNIT> {
        self.unit.as_ref()
    }

    pub fn clear_UNIT(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT.reverse_bits();
        self.unit = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn clear_UNIT_RAID(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_RAID.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn clear_UNIT_PARTY(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_PARTY.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_ITEM(item: SpellCastTargetsSpellCastTargetFlagsITEM) -> Self {
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

    pub fn set_ITEM(&mut self, item: SpellCastTargetsSpellCastTargetFlagsITEM) -> Self {
        self.inner |= SpellCastTargetFlags::ITEM;
        self.item = Some(item);
        self.clone()
    }

    pub const fn get_ITEM(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsITEM> {
        self.item.as_ref()
    }

    pub fn clear_ITEM(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::ITEM.reverse_bits();
        self.item = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SOURCE_LOCATION(source_location: SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION) -> Self {
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

    pub fn set_SOURCE_LOCATION(&mut self, source_location: SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION) -> Self {
        self.inner |= SpellCastTargetFlags::SOURCE_LOCATION;
        self.source_location = Some(source_location);
        self.clone()
    }

    pub const fn get_SOURCE_LOCATION(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsSOURCE_LOCATION> {
        self.source_location.as_ref()
    }

    pub fn clear_SOURCE_LOCATION(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::SOURCE_LOCATION.reverse_bits();
        self.source_location = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_DEST_LOCATION(dest_location: SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION) -> Self {
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

    pub fn set_DEST_LOCATION(&mut self, dest_location: SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION) -> Self {
        self.inner |= SpellCastTargetFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self.clone()
    }

    pub const fn get_DEST_LOCATION(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsDEST_LOCATION> {
        self.dest_location.as_ref()
    }

    pub fn clear_DEST_LOCATION(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNIT_ENEMY(unit_enemy: SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY) -> Self {
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

    pub fn set_UNIT_ENEMY(&mut self, unit_enemy: SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY) -> Self {
        self.inner |= SpellCastTargetFlags::UNIT_ENEMY;
        self.unit_enemy = Some(unit_enemy);
        self.clone()
    }

    pub const fn get_UNIT_ENEMY(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY> {
        self.unit_enemy.as_ref()
    }

    pub fn clear_UNIT_ENEMY(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ENEMY.reverse_bits();
        self.unit_enemy = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn clear_UNIT_ALLY(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_ALLY.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_CORPSE_ENEMY(corpse_enemy: SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY) -> Self {
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

    pub fn set_CORPSE_ENEMY(&mut self, corpse_enemy: SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ENEMY;
        self.corpse_enemy = Some(corpse_enemy);
        self.clone()
    }

    pub const fn get_CORPSE_ENEMY(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY> {
        self.corpse_enemy.as_ref()
    }

    pub fn clear_CORPSE_ENEMY(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE_ENEMY.reverse_bits();
        self.corpse_enemy = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
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

    pub fn clear_UNIT_DEAD(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::UNIT_DEAD.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_GAMEOBJECT(gameobject: SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT) -> Self {
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

    pub fn set_GAMEOBJECT(&mut self, gameobject: SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT) -> Self {
        self.inner |= SpellCastTargetFlags::GAMEOBJECT;
        self.gameobject = Some(gameobject);
        self.clone()
    }

    pub const fn get_GAMEOBJECT(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT> {
        self.gameobject.as_ref()
    }

    pub fn clear_GAMEOBJECT(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::GAMEOBJECT.reverse_bits();
        self.gameobject = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_TRADE_ITEM(trade_item: SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM) -> Self {
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

    pub fn set_TRADE_ITEM(&mut self, trade_item: SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM) -> Self {
        self.inner |= SpellCastTargetFlags::TRADE_ITEM;
        self.trade_item = Some(trade_item);
        self.clone()
    }

    pub const fn get_TRADE_ITEM(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM> {
        self.trade_item.as_ref()
    }

    pub fn clear_TRADE_ITEM(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::TRADE_ITEM.reverse_bits();
        self.trade_item = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_STRING(string: SpellCastTargetsSpellCastTargetFlagsSTRING) -> Self {
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

    pub fn set_STRING(&mut self, string: SpellCastTargetsSpellCastTargetFlagsSTRING) -> Self {
        self.inner |= SpellCastTargetFlags::STRING;
        self.string = Some(string);
        self.clone()
    }

    pub const fn get_STRING(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsSTRING> {
        self.string.as_ref()
    }

    pub fn clear_STRING(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::STRING.reverse_bits();
        self.string = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_LOCKED(locked: SpellCastTargetsSpellCastTargetFlagsLOCKED) -> Self {
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

    pub fn set_LOCKED(&mut self, locked: SpellCastTargetsSpellCastTargetFlagsLOCKED) -> Self {
        self.inner |= SpellCastTargetFlags::LOCKED;
        self.locked = Some(locked);
        self.clone()
    }

    pub const fn get_LOCKED(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsLOCKED> {
        self.locked.as_ref()
    }

    pub fn clear_LOCKED(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::LOCKED.reverse_bits();
        self.locked = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_CORPSE_ALLY(corpse_ally: SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY) -> Self {
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

    pub fn set_CORPSE_ALLY(&mut self, corpse_ally: SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY) -> Self {
        self.inner |= SpellCastTargetFlags::CORPSE_ALLY;
        self.corpse_ally = Some(corpse_ally);
        self.clone()
    }

    pub const fn get_CORPSE_ALLY(&self) -> Option<&SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY> {
        self.corpse_ally.as_ref()
    }

    pub fn clear_CORPSE_ALLY(&mut self) -> Self {
        self.inner &= SpellCastTargetFlags::CORPSE_ALLY.reverse_bits();
        self.corpse_ally = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for SpellCastTargetsSpellCastTargetFlags {
    fn size(&self) -> usize {
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

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlags {
    fn maximum_possible_size() -> usize {
        2 // inner
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
        self.unit_target1.size() // unit_target1: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsUNIT {
    fn maximum_possible_size() -> usize {
        9 // unit_target1: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsITEM {
    pub item_target1: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsITEM {
    fn size(&self) -> usize {
        self.item_target1.size() // item_target1: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsITEM {
    fn maximum_possible_size() -> usize {
        9 // item_target1: Guid
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

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
    pub unit_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
    fn size(&self) -> usize {
        self.unit_target2.size() // unit_target2: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsUNIT_ENEMY {
    fn maximum_possible_size() -> usize {
        9 // unit_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
    pub corpse_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
    fn size(&self) -> usize {
        self.corpse_target2.size() // corpse_target2: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ENEMY {
    fn maximum_possible_size() -> usize {
        9 // corpse_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
    pub object_target1: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
    fn size(&self) -> usize {
        self.object_target1.size() // object_target1: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsGAMEOBJECT {
    fn maximum_possible_size() -> usize {
        9 // object_target1: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
    pub item_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
    fn size(&self) -> usize {
        self.item_target2.size() // item_target2: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsTRADE_ITEM {
    fn maximum_possible_size() -> usize {
        9 // item_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsSTRING {
    pub target_string: String,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsSTRING {
    fn size(&self) -> usize {
        self.target_string.len() + 1 // target_string: CString
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsSTRING {
    fn maximum_possible_size() -> usize {
        256 // target_string: CString
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsLOCKED {
    pub object_target2: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsLOCKED {
    fn size(&self) -> usize {
        self.object_target2.size() // object_target2: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsLOCKED {
    fn maximum_possible_size() -> usize {
        9 // object_target2: Guid
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
    pub corpse_target1: Guid,
}

impl VariableSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
    fn size(&self) -> usize {
        self.corpse_target1.size() // corpse_target1: Guid
    }
}

impl MaximumPossibleSized for SpellCastTargetsSpellCastTargetFlagsCORPSE_ALLY {
    fn maximum_possible_size() -> usize {
        9 // corpse_target1: Guid
    }
}

