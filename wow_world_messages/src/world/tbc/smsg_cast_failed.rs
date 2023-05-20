use std::io::{Read, Write};

use crate::tbc::{
    Area, SpellCastResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cast_failed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cast_failed.wowm#L1):
/// ```text
/// smsg SMSG_CAST_FAILED = 0x0130 {
///     u32 id;
///     SpellCastResult result;
///     Bool multiple_casts;
///     if (result == REQUIRES_SPELL_FOCUS) {
///         u32 spell_focus;
///     }
///     else if (result == REQUIRES_AREA) {
///         Area area;
///     }
///     else if (result == TOTEMS) {
///         u32[2] totems;
///     }
///     else if (result == TOTEM_CATEGORY) {
///         u32[2] totem_categories;
///     }
///     else if (result == EQUIPPED_ITEM_CLASS) {
///         u32 item_class;
///         u32 item_sub_class;
///         u32 item_inventory_type;
///     }
/// }
/// ```
pub struct SMSG_CAST_FAILED {
    pub id: u32,
    pub result: SMSG_CAST_FAILED_SpellCastResult,
    pub multiple_casts: bool,
}

impl crate::private::Sealed for SMSG_CAST_FAILED {}
impl crate::Message for SMSG_CAST_FAILED {
    const OPCODE: u32 = 0x0130;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        // multiple_casts: Bool
        w.write_all(u8::from(self.multiple_casts).to_le_bytes().as_slice())?;

        match &self.result {
            SMSG_CAST_FAILED_SpellCastResult::EquippedItemClass {
                item_class,
                item_inventory_type,
                item_sub_class,
            } => {
                // item_class: u32
                w.write_all(&item_class.to_le_bytes())?;

                // item_sub_class: u32
                w.write_all(&item_sub_class.to_le_bytes())?;

                // item_inventory_type: u32
                w.write_all(&item_inventory_type.to_le_bytes())?;

            }
            SMSG_CAST_FAILED_SpellCastResult::RequiresArea {
                area,
            } => {
                // area: Area
                w.write_all(&(area.as_int().to_le_bytes()))?;

            }
            SMSG_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                spell_focus,
            } => {
                // spell_focus: u32
                w.write_all(&spell_focus.to_le_bytes())?;

            }
            SMSG_CAST_FAILED_SpellCastResult::TotemCategory {
                totem_categories,
            } => {
                // totem_categories: u32[2]
                for i in totem_categories.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            SMSG_CAST_FAILED_SpellCastResult::Totems {
                totems,
            } => {
                // totems: u32[2]
                for i in totems.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            _ => {}
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=18).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0130, size: body_size });
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        // multiple_casts: Bool
        let multiple_casts = crate::util::read_u8_le(&mut r)? != 0;

        let result_if = match result {
            SpellCastResult::AffectingCombat => SMSG_CAST_FAILED_SpellCastResult::AffectingCombat,
            SpellCastResult::AlreadyAtFullHealth => SMSG_CAST_FAILED_SpellCastResult::AlreadyAtFullHealth,
            SpellCastResult::AlreadyAtFullMana => SMSG_CAST_FAILED_SpellCastResult::AlreadyAtFullMana,
            SpellCastResult::AlreadyAtFullPower => SMSG_CAST_FAILED_SpellCastResult::AlreadyAtFullPower,
            SpellCastResult::AlreadyBeingTamed => SMSG_CAST_FAILED_SpellCastResult::AlreadyBeingTamed,
            SpellCastResult::AlreadyHaveCharm => SMSG_CAST_FAILED_SpellCastResult::AlreadyHaveCharm,
            SpellCastResult::AlreadyHaveSummon => SMSG_CAST_FAILED_SpellCastResult::AlreadyHaveSummon,
            SpellCastResult::AlreadyOpen => SMSG_CAST_FAILED_SpellCastResult::AlreadyOpen,
            SpellCastResult::AuraBounced => SMSG_CAST_FAILED_SpellCastResult::AuraBounced,
            SpellCastResult::AutotrackInterrupted => SMSG_CAST_FAILED_SpellCastResult::AutotrackInterrupted,
            SpellCastResult::BadImplicitTargets => SMSG_CAST_FAILED_SpellCastResult::BadImplicitTargets,
            SpellCastResult::BadTargets => SMSG_CAST_FAILED_SpellCastResult::BadTargets,
            SpellCastResult::CantBeCharmed => SMSG_CAST_FAILED_SpellCastResult::CantBeCharmed,
            SpellCastResult::CantBeDisenchanted => SMSG_CAST_FAILED_SpellCastResult::CantBeDisenchanted,
            SpellCastResult::CantBeDisenchantedSkill => SMSG_CAST_FAILED_SpellCastResult::CantBeDisenchantedSkill,
            SpellCastResult::CantBeProspected => SMSG_CAST_FAILED_SpellCastResult::CantBeProspected,
            SpellCastResult::CantCastOnTapped => SMSG_CAST_FAILED_SpellCastResult::CantCastOnTapped,
            SpellCastResult::CantDuelWhileInvisible => SMSG_CAST_FAILED_SpellCastResult::CantDuelWhileInvisible,
            SpellCastResult::CantDuelWhileStealthed => SMSG_CAST_FAILED_SpellCastResult::CantDuelWhileStealthed,
            SpellCastResult::CantStealth => SMSG_CAST_FAILED_SpellCastResult::CantStealth,
            SpellCastResult::CasterAurastate => SMSG_CAST_FAILED_SpellCastResult::CasterAurastate,
            SpellCastResult::CasterDead => SMSG_CAST_FAILED_SpellCastResult::CasterDead,
            SpellCastResult::Charmed => SMSG_CAST_FAILED_SpellCastResult::Charmed,
            SpellCastResult::ChestInUse => SMSG_CAST_FAILED_SpellCastResult::ChestInUse,
            SpellCastResult::Confused => SMSG_CAST_FAILED_SpellCastResult::Confused,
            SpellCastResult::DontReport => SMSG_CAST_FAILED_SpellCastResult::DontReport,
            SpellCastResult::EquippedItem => SMSG_CAST_FAILED_SpellCastResult::EquippedItem,
            SpellCastResult::EquippedItemClass => {
                // item_class: u32
                let item_class = crate::util::read_u32_le(&mut r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(&mut r)?;

                // item_inventory_type: u32
                let item_inventory_type = crate::util::read_u32_le(&mut r)?;

                SMSG_CAST_FAILED_SpellCastResult::EquippedItemClass {
                    item_class,
                    item_inventory_type,
                    item_sub_class,
                }
            }
            SpellCastResult::EquippedItemClassMainhand => SMSG_CAST_FAILED_SpellCastResult::EquippedItemClassMainhand,
            SpellCastResult::EquippedItemClassOffhand => SMSG_CAST_FAILED_SpellCastResult::EquippedItemClassOffhand,
            SpellCastResult::ErrorX => SMSG_CAST_FAILED_SpellCastResult::ErrorX,
            SpellCastResult::Fizzle => SMSG_CAST_FAILED_SpellCastResult::Fizzle,
            SpellCastResult::Fleeing => SMSG_CAST_FAILED_SpellCastResult::Fleeing,
            SpellCastResult::FoodLowlevel => SMSG_CAST_FAILED_SpellCastResult::FoodLowlevel,
            SpellCastResult::Highlevel => SMSG_CAST_FAILED_SpellCastResult::Highlevel,
            SpellCastResult::HungerSatiated => SMSG_CAST_FAILED_SpellCastResult::HungerSatiated,
            SpellCastResult::Immune => SMSG_CAST_FAILED_SpellCastResult::Immune,
            SpellCastResult::Interrupted => SMSG_CAST_FAILED_SpellCastResult::Interrupted,
            SpellCastResult::InterruptedCombat => SMSG_CAST_FAILED_SpellCastResult::InterruptedCombat,
            SpellCastResult::ItemAlreadyEnchanted => SMSG_CAST_FAILED_SpellCastResult::ItemAlreadyEnchanted,
            SpellCastResult::ItemGone => SMSG_CAST_FAILED_SpellCastResult::ItemGone,
            SpellCastResult::ItemNotFound => SMSG_CAST_FAILED_SpellCastResult::ItemNotFound,
            SpellCastResult::ItemNotReady => SMSG_CAST_FAILED_SpellCastResult::ItemNotReady,
            SpellCastResult::LevelRequirement => SMSG_CAST_FAILED_SpellCastResult::LevelRequirement,
            SpellCastResult::LineOfSight => SMSG_CAST_FAILED_SpellCastResult::LineOfSight,
            SpellCastResult::Lowlevel => SMSG_CAST_FAILED_SpellCastResult::Lowlevel,
            SpellCastResult::LowCastlevel => SMSG_CAST_FAILED_SpellCastResult::LowCastlevel,
            SpellCastResult::MainhandEmpty => SMSG_CAST_FAILED_SpellCastResult::MainhandEmpty,
            SpellCastResult::Moving => SMSG_CAST_FAILED_SpellCastResult::Moving,
            SpellCastResult::NeedAmmo => SMSG_CAST_FAILED_SpellCastResult::NeedAmmo,
            SpellCastResult::NeedAmmoPouch => SMSG_CAST_FAILED_SpellCastResult::NeedAmmoPouch,
            SpellCastResult::NeedExoticAmmo => SMSG_CAST_FAILED_SpellCastResult::NeedExoticAmmo,
            SpellCastResult::Nopath => SMSG_CAST_FAILED_SpellCastResult::Nopath,
            SpellCastResult::NotBehind => SMSG_CAST_FAILED_SpellCastResult::NotBehind,
            SpellCastResult::NotFishable => SMSG_CAST_FAILED_SpellCastResult::NotFishable,
            SpellCastResult::NotFlying => SMSG_CAST_FAILED_SpellCastResult::NotFlying,
            SpellCastResult::NotHere => SMSG_CAST_FAILED_SpellCastResult::NotHere,
            SpellCastResult::NotInfront => SMSG_CAST_FAILED_SpellCastResult::NotInfront,
            SpellCastResult::NotInControl => SMSG_CAST_FAILED_SpellCastResult::NotInControl,
            SpellCastResult::NotKnown => SMSG_CAST_FAILED_SpellCastResult::NotKnown,
            SpellCastResult::NotMounted => SMSG_CAST_FAILED_SpellCastResult::NotMounted,
            SpellCastResult::NotOnTaxi => SMSG_CAST_FAILED_SpellCastResult::NotOnTaxi,
            SpellCastResult::NotOnTransport => SMSG_CAST_FAILED_SpellCastResult::NotOnTransport,
            SpellCastResult::NotReady => SMSG_CAST_FAILED_SpellCastResult::NotReady,
            SpellCastResult::NotShapeshift => SMSG_CAST_FAILED_SpellCastResult::NotShapeshift,
            SpellCastResult::NotStanding => SMSG_CAST_FAILED_SpellCastResult::NotStanding,
            SpellCastResult::NotTradeable => SMSG_CAST_FAILED_SpellCastResult::NotTradeable,
            SpellCastResult::NotTrading => SMSG_CAST_FAILED_SpellCastResult::NotTrading,
            SpellCastResult::NotUnsheathed => SMSG_CAST_FAILED_SpellCastResult::NotUnsheathed,
            SpellCastResult::NotWhileGhost => SMSG_CAST_FAILED_SpellCastResult::NotWhileGhost,
            SpellCastResult::NoAmmo => SMSG_CAST_FAILED_SpellCastResult::NoAmmo,
            SpellCastResult::NoChargesRemain => SMSG_CAST_FAILED_SpellCastResult::NoChargesRemain,
            SpellCastResult::NoChampion => SMSG_CAST_FAILED_SpellCastResult::NoChampion,
            SpellCastResult::NoComboPoints => SMSG_CAST_FAILED_SpellCastResult::NoComboPoints,
            SpellCastResult::NoDueling => SMSG_CAST_FAILED_SpellCastResult::NoDueling,
            SpellCastResult::NoEndurance => SMSG_CAST_FAILED_SpellCastResult::NoEndurance,
            SpellCastResult::NoFish => SMSG_CAST_FAILED_SpellCastResult::NoFish,
            SpellCastResult::NoItemsWhileShapeshifted => SMSG_CAST_FAILED_SpellCastResult::NoItemsWhileShapeshifted,
            SpellCastResult::NoMountsAllowed => SMSG_CAST_FAILED_SpellCastResult::NoMountsAllowed,
            SpellCastResult::NoPet => SMSG_CAST_FAILED_SpellCastResult::NoPet,
            SpellCastResult::NoPower => SMSG_CAST_FAILED_SpellCastResult::NoPower,
            SpellCastResult::NothingToDispel => SMSG_CAST_FAILED_SpellCastResult::NothingToDispel,
            SpellCastResult::NothingToSteal => SMSG_CAST_FAILED_SpellCastResult::NothingToSteal,
            SpellCastResult::OnlyAbovewater => SMSG_CAST_FAILED_SpellCastResult::OnlyAbovewater,
            SpellCastResult::OnlyDaytime => SMSG_CAST_FAILED_SpellCastResult::OnlyDaytime,
            SpellCastResult::OnlyIndoors => SMSG_CAST_FAILED_SpellCastResult::OnlyIndoors,
            SpellCastResult::OnlyMounted => SMSG_CAST_FAILED_SpellCastResult::OnlyMounted,
            SpellCastResult::OnlyNighttime => SMSG_CAST_FAILED_SpellCastResult::OnlyNighttime,
            SpellCastResult::OnlyOutdoors => SMSG_CAST_FAILED_SpellCastResult::OnlyOutdoors,
            SpellCastResult::OnlyShapeshift => SMSG_CAST_FAILED_SpellCastResult::OnlyShapeshift,
            SpellCastResult::OnlyStealthed => SMSG_CAST_FAILED_SpellCastResult::OnlyStealthed,
            SpellCastResult::OnlyUnderwater => SMSG_CAST_FAILED_SpellCastResult::OnlyUnderwater,
            SpellCastResult::OutOfRange => SMSG_CAST_FAILED_SpellCastResult::OutOfRange,
            SpellCastResult::Pacified => SMSG_CAST_FAILED_SpellCastResult::Pacified,
            SpellCastResult::Possessed => SMSG_CAST_FAILED_SpellCastResult::Possessed,
            SpellCastResult::Reagents => SMSG_CAST_FAILED_SpellCastResult::Reagents,
            SpellCastResult::RequiresArea => {
                // area: Area
                let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

                SMSG_CAST_FAILED_SpellCastResult::RequiresArea {
                    area,
                }
            }
            SpellCastResult::RequiresSpellFocus => {
                // spell_focus: u32
                let spell_focus = crate::util::read_u32_le(&mut r)?;

                SMSG_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                    spell_focus,
                }
            }
            SpellCastResult::Rooted => SMSG_CAST_FAILED_SpellCastResult::Rooted,
            SpellCastResult::Silenced => SMSG_CAST_FAILED_SpellCastResult::Silenced,
            SpellCastResult::SpellInProgress => SMSG_CAST_FAILED_SpellCastResult::SpellInProgress,
            SpellCastResult::SpellLearned => SMSG_CAST_FAILED_SpellCastResult::SpellLearned,
            SpellCastResult::SpellUnavailable => SMSG_CAST_FAILED_SpellCastResult::SpellUnavailable,
            SpellCastResult::Stunned => SMSG_CAST_FAILED_SpellCastResult::Stunned,
            SpellCastResult::TargetsDead => SMSG_CAST_FAILED_SpellCastResult::TargetsDead,
            SpellCastResult::TargetAffectingCombat => SMSG_CAST_FAILED_SpellCastResult::TargetAffectingCombat,
            SpellCastResult::TargetAurastate => SMSG_CAST_FAILED_SpellCastResult::TargetAurastate,
            SpellCastResult::TargetDueling => SMSG_CAST_FAILED_SpellCastResult::TargetDueling,
            SpellCastResult::TargetEnemy => SMSG_CAST_FAILED_SpellCastResult::TargetEnemy,
            SpellCastResult::TargetEnraged => SMSG_CAST_FAILED_SpellCastResult::TargetEnraged,
            SpellCastResult::TargetFriendly => SMSG_CAST_FAILED_SpellCastResult::TargetFriendly,
            SpellCastResult::TargetInCombat => SMSG_CAST_FAILED_SpellCastResult::TargetInCombat,
            SpellCastResult::TargetIsPlayer => SMSG_CAST_FAILED_SpellCastResult::TargetIsPlayer,
            SpellCastResult::TargetIsPlayerControlled => SMSG_CAST_FAILED_SpellCastResult::TargetIsPlayerControlled,
            SpellCastResult::TargetNotDead => SMSG_CAST_FAILED_SpellCastResult::TargetNotDead,
            SpellCastResult::TargetNotInParty => SMSG_CAST_FAILED_SpellCastResult::TargetNotInParty,
            SpellCastResult::TargetNotLooted => SMSG_CAST_FAILED_SpellCastResult::TargetNotLooted,
            SpellCastResult::TargetNotPlayer => SMSG_CAST_FAILED_SpellCastResult::TargetNotPlayer,
            SpellCastResult::TargetNoPockets => SMSG_CAST_FAILED_SpellCastResult::TargetNoPockets,
            SpellCastResult::TargetNoWeapons => SMSG_CAST_FAILED_SpellCastResult::TargetNoWeapons,
            SpellCastResult::TargetUnskinnable => SMSG_CAST_FAILED_SpellCastResult::TargetUnskinnable,
            SpellCastResult::ThirstSatiated => SMSG_CAST_FAILED_SpellCastResult::ThirstSatiated,
            SpellCastResult::TooClose => SMSG_CAST_FAILED_SpellCastResult::TooClose,
            SpellCastResult::TooManyOfItem => SMSG_CAST_FAILED_SpellCastResult::TooManyOfItem,
            SpellCastResult::TotemCategory => {
                // totem_categories: u32[2]
                let totem_categories = {
                    let mut totem_categories = [u32::default(); 2];
                    for i in totem_categories.iter_mut() {
                        *i = crate::util::read_u32_le(&mut r)?;
                    }
                    totem_categories
                };

                SMSG_CAST_FAILED_SpellCastResult::TotemCategory {
                    totem_categories,
                }
            }
            SpellCastResult::Totems => {
                // totems: u32[2]
                let totems = {
                    let mut totems = [u32::default(); 2];
                    for i in totems.iter_mut() {
                        *i = crate::util::read_u32_le(&mut r)?;
                    }
                    totems
                };

                SMSG_CAST_FAILED_SpellCastResult::Totems {
                    totems,
                }
            }
            SpellCastResult::TrainingPoints => SMSG_CAST_FAILED_SpellCastResult::TrainingPoints,
            SpellCastResult::TryAgain => SMSG_CAST_FAILED_SpellCastResult::TryAgain,
            SpellCastResult::UnitNotBehind => SMSG_CAST_FAILED_SpellCastResult::UnitNotBehind,
            SpellCastResult::UnitNotInfront => SMSG_CAST_FAILED_SpellCastResult::UnitNotInfront,
            SpellCastResult::WrongPetFood => SMSG_CAST_FAILED_SpellCastResult::WrongPetFood,
            SpellCastResult::NotWhileFatigued => SMSG_CAST_FAILED_SpellCastResult::NotWhileFatigued,
            SpellCastResult::TargetNotInInstance => SMSG_CAST_FAILED_SpellCastResult::TargetNotInInstance,
            SpellCastResult::NotWhileTrading => SMSG_CAST_FAILED_SpellCastResult::NotWhileTrading,
            SpellCastResult::TargetNotInRaid => SMSG_CAST_FAILED_SpellCastResult::TargetNotInRaid,
            SpellCastResult::DisenchantWhileLooting => SMSG_CAST_FAILED_SpellCastResult::DisenchantWhileLooting,
            SpellCastResult::ProspectWhileLooting => SMSG_CAST_FAILED_SpellCastResult::ProspectWhileLooting,
            SpellCastResult::ProspectNeedMore => SMSG_CAST_FAILED_SpellCastResult::ProspectNeedMore,
            SpellCastResult::TargetFreeforall => SMSG_CAST_FAILED_SpellCastResult::TargetFreeforall,
            SpellCastResult::NoEdibleCorpses => SMSG_CAST_FAILED_SpellCastResult::NoEdibleCorpses,
            SpellCastResult::OnlyBattlegrounds => SMSG_CAST_FAILED_SpellCastResult::OnlyBattlegrounds,
            SpellCastResult::TargetNotGhost => SMSG_CAST_FAILED_SpellCastResult::TargetNotGhost,
            SpellCastResult::TooManySkills => SMSG_CAST_FAILED_SpellCastResult::TooManySkills,
            SpellCastResult::TransformUnusable => SMSG_CAST_FAILED_SpellCastResult::TransformUnusable,
            SpellCastResult::WrongWeather => SMSG_CAST_FAILED_SpellCastResult::WrongWeather,
            SpellCastResult::DamageImmune => SMSG_CAST_FAILED_SpellCastResult::DamageImmune,
            SpellCastResult::PreventedByMechanic => SMSG_CAST_FAILED_SpellCastResult::PreventedByMechanic,
            SpellCastResult::PlayTime => SMSG_CAST_FAILED_SpellCastResult::PlayTime,
            SpellCastResult::Reputation => SMSG_CAST_FAILED_SpellCastResult::Reputation,
            SpellCastResult::MinSkill => SMSG_CAST_FAILED_SpellCastResult::MinSkill,
            SpellCastResult::NotInArena => SMSG_CAST_FAILED_SpellCastResult::NotInArena,
            SpellCastResult::NotOnShapeshift => SMSG_CAST_FAILED_SpellCastResult::NotOnShapeshift,
            SpellCastResult::NotOnStealthed => SMSG_CAST_FAILED_SpellCastResult::NotOnStealthed,
            SpellCastResult::NotOnDamageImmune => SMSG_CAST_FAILED_SpellCastResult::NotOnDamageImmune,
            SpellCastResult::NotOnMounted => SMSG_CAST_FAILED_SpellCastResult::NotOnMounted,
            SpellCastResult::TooShallow => SMSG_CAST_FAILED_SpellCastResult::TooShallow,
            SpellCastResult::TargetNotInSanctuary => SMSG_CAST_FAILED_SpellCastResult::TargetNotInSanctuary,
            SpellCastResult::TargetIsTrivial => SMSG_CAST_FAILED_SpellCastResult::TargetIsTrivial,
            SpellCastResult::BmOrInvisgod => SMSG_CAST_FAILED_SpellCastResult::BmOrInvisgod,
            SpellCastResult::ExpertRidingRequirement => SMSG_CAST_FAILED_SpellCastResult::ExpertRidingRequirement,
            SpellCastResult::ArtisanRidingRequirement => SMSG_CAST_FAILED_SpellCastResult::ArtisanRidingRequirement,
            SpellCastResult::NotIdle => SMSG_CAST_FAILED_SpellCastResult::NotIdle,
            SpellCastResult::NotInactive => SMSG_CAST_FAILED_SpellCastResult::NotInactive,
            SpellCastResult::PartialPlaytime => SMSG_CAST_FAILED_SpellCastResult::PartialPlaytime,
            SpellCastResult::NoPlaytime => SMSG_CAST_FAILED_SpellCastResult::NoPlaytime,
            SpellCastResult::NotInBattleground => SMSG_CAST_FAILED_SpellCastResult::NotInBattleground,
            SpellCastResult::OnlyInArena => SMSG_CAST_FAILED_SpellCastResult::OnlyInArena,
            SpellCastResult::TargetLockedToRaidInstance => SMSG_CAST_FAILED_SpellCastResult::TargetLockedToRaidInstance,
            SpellCastResult::Unknown => SMSG_CAST_FAILED_SpellCastResult::Unknown,
        };

        Ok(Self {
            id,
            result: result_if,
            multiple_casts,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CAST_FAILED {}

impl SMSG_CAST_FAILED {
    pub(crate) const fn size(&self) -> usize {
        4 // id: u32
        + self.result.size() // result: SMSG_CAST_FAILED_SpellCastResult
        + 1 // multiple_casts: Bool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_CAST_FAILED_SpellCastResult {
    AffectingCombat,
    AlreadyAtFullHealth,
    AlreadyAtFullMana,
    AlreadyAtFullPower,
    AlreadyBeingTamed,
    AlreadyHaveCharm,
    AlreadyHaveSummon,
    AlreadyOpen,
    AuraBounced,
    AutotrackInterrupted,
    BadImplicitTargets,
    BadTargets,
    CantBeCharmed,
    CantBeDisenchanted,
    CantBeDisenchantedSkill,
    CantBeProspected,
    CantCastOnTapped,
    CantDuelWhileInvisible,
    CantDuelWhileStealthed,
    CantStealth,
    CasterAurastate,
    CasterDead,
    Charmed,
    ChestInUse,
    Confused,
    DontReport,
    EquippedItem,
    EquippedItemClass {
        item_class: u32,
        item_inventory_type: u32,
        item_sub_class: u32,
    },
    EquippedItemClassMainhand,
    EquippedItemClassOffhand,
    ErrorX,
    Fizzle,
    Fleeing,
    FoodLowlevel,
    Highlevel,
    HungerSatiated,
    Immune,
    Interrupted,
    InterruptedCombat,
    ItemAlreadyEnchanted,
    ItemGone,
    ItemNotFound,
    ItemNotReady,
    LevelRequirement,
    LineOfSight,
    Lowlevel,
    LowCastlevel,
    MainhandEmpty,
    Moving,
    NeedAmmo,
    NeedAmmoPouch,
    NeedExoticAmmo,
    Nopath,
    NotBehind,
    NotFishable,
    NotFlying,
    NotHere,
    NotInfront,
    NotInControl,
    NotKnown,
    NotMounted,
    NotOnTaxi,
    NotOnTransport,
    NotReady,
    NotShapeshift,
    NotStanding,
    NotTradeable,
    NotTrading,
    NotUnsheathed,
    NotWhileGhost,
    NoAmmo,
    NoChargesRemain,
    NoChampion,
    NoComboPoints,
    NoDueling,
    NoEndurance,
    NoFish,
    NoItemsWhileShapeshifted,
    NoMountsAllowed,
    NoPet,
    NoPower,
    NothingToDispel,
    NothingToSteal,
    OnlyAbovewater,
    OnlyDaytime,
    OnlyIndoors,
    OnlyMounted,
    OnlyNighttime,
    OnlyOutdoors,
    OnlyShapeshift,
    OnlyStealthed,
    OnlyUnderwater,
    OutOfRange,
    Pacified,
    Possessed,
    Reagents,
    RequiresArea {
        area: Area,
    },
    RequiresSpellFocus {
        spell_focus: u32,
    },
    Rooted,
    Silenced,
    SpellInProgress,
    SpellLearned,
    SpellUnavailable,
    Stunned,
    TargetsDead,
    TargetAffectingCombat,
    TargetAurastate,
    TargetDueling,
    TargetEnemy,
    TargetEnraged,
    TargetFriendly,
    TargetInCombat,
    TargetIsPlayer,
    TargetIsPlayerControlled,
    TargetNotDead,
    TargetNotInParty,
    TargetNotLooted,
    TargetNotPlayer,
    TargetNoPockets,
    TargetNoWeapons,
    TargetUnskinnable,
    ThirstSatiated,
    TooClose,
    TooManyOfItem,
    TotemCategory {
        totem_categories: [u32; 2],
    },
    Totems {
        totems: [u32; 2],
    },
    TrainingPoints,
    TryAgain,
    UnitNotBehind,
    UnitNotInfront,
    WrongPetFood,
    NotWhileFatigued,
    TargetNotInInstance,
    NotWhileTrading,
    TargetNotInRaid,
    DisenchantWhileLooting,
    ProspectWhileLooting,
    ProspectNeedMore,
    TargetFreeforall,
    NoEdibleCorpses,
    OnlyBattlegrounds,
    TargetNotGhost,
    TooManySkills,
    TransformUnusable,
    WrongWeather,
    DamageImmune,
    PreventedByMechanic,
    PlayTime,
    Reputation,
    MinSkill,
    NotInArena,
    NotOnShapeshift,
    NotOnStealthed,
    NotOnDamageImmune,
    NotOnMounted,
    TooShallow,
    TargetNotInSanctuary,
    TargetIsTrivial,
    BmOrInvisgod,
    ExpertRidingRequirement,
    ArtisanRidingRequirement,
    NotIdle,
    NotInactive,
    PartialPlaytime,
    NoPlaytime,
    NotInBattleground,
    OnlyInArena,
    TargetLockedToRaidInstance,
    Unknown,
}

impl Default for SMSG_CAST_FAILED_SpellCastResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::AffectingCombat
    }
}

impl SMSG_CAST_FAILED_SpellCastResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::AffectingCombat => 0,
            Self::AlreadyAtFullHealth => 1,
            Self::AlreadyAtFullMana => 2,
            Self::AlreadyAtFullPower => 3,
            Self::AlreadyBeingTamed => 4,
            Self::AlreadyHaveCharm => 5,
            Self::AlreadyHaveSummon => 6,
            Self::AlreadyOpen => 7,
            Self::AuraBounced => 8,
            Self::AutotrackInterrupted => 9,
            Self::BadImplicitTargets => 10,
            Self::BadTargets => 11,
            Self::CantBeCharmed => 12,
            Self::CantBeDisenchanted => 13,
            Self::CantBeDisenchantedSkill => 14,
            Self::CantBeProspected => 15,
            Self::CantCastOnTapped => 16,
            Self::CantDuelWhileInvisible => 17,
            Self::CantDuelWhileStealthed => 18,
            Self::CantStealth => 19,
            Self::CasterAurastate => 20,
            Self::CasterDead => 21,
            Self::Charmed => 22,
            Self::ChestInUse => 23,
            Self::Confused => 24,
            Self::DontReport => 25,
            Self::EquippedItem => 26,
            Self::EquippedItemClass { .. } => 27,
            Self::EquippedItemClassMainhand => 28,
            Self::EquippedItemClassOffhand => 29,
            Self::ErrorX => 30,
            Self::Fizzle => 31,
            Self::Fleeing => 32,
            Self::FoodLowlevel => 33,
            Self::Highlevel => 34,
            Self::HungerSatiated => 35,
            Self::Immune => 36,
            Self::Interrupted => 37,
            Self::InterruptedCombat => 38,
            Self::ItemAlreadyEnchanted => 39,
            Self::ItemGone => 40,
            Self::ItemNotFound => 41,
            Self::ItemNotReady => 42,
            Self::LevelRequirement => 43,
            Self::LineOfSight => 44,
            Self::Lowlevel => 45,
            Self::LowCastlevel => 46,
            Self::MainhandEmpty => 47,
            Self::Moving => 48,
            Self::NeedAmmo => 49,
            Self::NeedAmmoPouch => 50,
            Self::NeedExoticAmmo => 51,
            Self::Nopath => 52,
            Self::NotBehind => 53,
            Self::NotFishable => 54,
            Self::NotFlying => 55,
            Self::NotHere => 56,
            Self::NotInfront => 57,
            Self::NotInControl => 58,
            Self::NotKnown => 59,
            Self::NotMounted => 60,
            Self::NotOnTaxi => 61,
            Self::NotOnTransport => 62,
            Self::NotReady => 63,
            Self::NotShapeshift => 64,
            Self::NotStanding => 65,
            Self::NotTradeable => 66,
            Self::NotTrading => 67,
            Self::NotUnsheathed => 68,
            Self::NotWhileGhost => 69,
            Self::NoAmmo => 70,
            Self::NoChargesRemain => 71,
            Self::NoChampion => 72,
            Self::NoComboPoints => 73,
            Self::NoDueling => 74,
            Self::NoEndurance => 75,
            Self::NoFish => 76,
            Self::NoItemsWhileShapeshifted => 77,
            Self::NoMountsAllowed => 78,
            Self::NoPet => 79,
            Self::NoPower => 80,
            Self::NothingToDispel => 81,
            Self::NothingToSteal => 82,
            Self::OnlyAbovewater => 83,
            Self::OnlyDaytime => 84,
            Self::OnlyIndoors => 85,
            Self::OnlyMounted => 86,
            Self::OnlyNighttime => 87,
            Self::OnlyOutdoors => 88,
            Self::OnlyShapeshift => 89,
            Self::OnlyStealthed => 90,
            Self::OnlyUnderwater => 91,
            Self::OutOfRange => 92,
            Self::Pacified => 93,
            Self::Possessed => 94,
            Self::Reagents => 95,
            Self::RequiresArea { .. } => 96,
            Self::RequiresSpellFocus { .. } => 97,
            Self::Rooted => 98,
            Self::Silenced => 99,
            Self::SpellInProgress => 100,
            Self::SpellLearned => 101,
            Self::SpellUnavailable => 102,
            Self::Stunned => 103,
            Self::TargetsDead => 104,
            Self::TargetAffectingCombat => 105,
            Self::TargetAurastate => 106,
            Self::TargetDueling => 107,
            Self::TargetEnemy => 108,
            Self::TargetEnraged => 109,
            Self::TargetFriendly => 110,
            Self::TargetInCombat => 111,
            Self::TargetIsPlayer => 112,
            Self::TargetIsPlayerControlled => 113,
            Self::TargetNotDead => 114,
            Self::TargetNotInParty => 115,
            Self::TargetNotLooted => 116,
            Self::TargetNotPlayer => 117,
            Self::TargetNoPockets => 118,
            Self::TargetNoWeapons => 119,
            Self::TargetUnskinnable => 120,
            Self::ThirstSatiated => 121,
            Self::TooClose => 122,
            Self::TooManyOfItem => 123,
            Self::TotemCategory { .. } => 124,
            Self::Totems { .. } => 125,
            Self::TrainingPoints => 126,
            Self::TryAgain => 127,
            Self::UnitNotBehind => 128,
            Self::UnitNotInfront => 129,
            Self::WrongPetFood => 130,
            Self::NotWhileFatigued => 131,
            Self::TargetNotInInstance => 132,
            Self::NotWhileTrading => 133,
            Self::TargetNotInRaid => 134,
            Self::DisenchantWhileLooting => 135,
            Self::ProspectWhileLooting => 136,
            Self::ProspectNeedMore => 137,
            Self::TargetFreeforall => 138,
            Self::NoEdibleCorpses => 139,
            Self::OnlyBattlegrounds => 140,
            Self::TargetNotGhost => 141,
            Self::TooManySkills => 142,
            Self::TransformUnusable => 143,
            Self::WrongWeather => 144,
            Self::DamageImmune => 145,
            Self::PreventedByMechanic => 146,
            Self::PlayTime => 147,
            Self::Reputation => 148,
            Self::MinSkill => 149,
            Self::NotInArena => 150,
            Self::NotOnShapeshift => 151,
            Self::NotOnStealthed => 152,
            Self::NotOnDamageImmune => 153,
            Self::NotOnMounted => 154,
            Self::TooShallow => 155,
            Self::TargetNotInSanctuary => 156,
            Self::TargetIsTrivial => 157,
            Self::BmOrInvisgod => 158,
            Self::ExpertRidingRequirement => 159,
            Self::ArtisanRidingRequirement => 160,
            Self::NotIdle => 161,
            Self::NotInactive => 162,
            Self::PartialPlaytime => 163,
            Self::NoPlaytime => 164,
            Self::NotInBattleground => 165,
            Self::OnlyInArena => 166,
            Self::TargetLockedToRaidInstance => 167,
            Self::Unknown => 168,
        }
    }

}

impl SMSG_CAST_FAILED_SpellCastResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::EquippedItemClass {
                ..
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_inventory_type: u32
                + 4 // item_sub_class: u32
            }
            Self::RequiresArea {
                ..
            } => {
                1
                + 4 // area: Area
            }
            Self::RequiresSpellFocus {
                ..
            } => {
                1
                + 4 // spell_focus: u32
            }
            Self::TotemCategory {
                ..
            } => {
                1
                + 8 // totem_categories: u32[2]
            }
            Self::Totems {
                ..
            } => {
                1
                + 8 // totems: u32[2]
            }
            _ => 1,
        }
    }
}

