use std::io::{Read, Write};

use crate::wrath::{
    Area, Skill, SpellCastResult,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm#L35):
/// ```text
/// smsg SMSG_PET_CAST_FAILED = 0x0138 {
///     u8 cast_count;
///     Spell id;
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
///     else if (result == EQUIPPED_ITEM_CLASS
///         || result == EQUIPPED_ITEM_CLASS_OFFHAND
///         || result == EQUIPPED_ITEM_CLASS_MAINHAND) {
///         u32 item_class;
///         u32 item_sub_class;
///     }
///     else if (result == TOO_MANY_OF_ITEM) {
///         u32 item_limit_category;
///     }
///     else if (result == CUSTOM_ERROR) {
///         u32 custom_error;
///     }
///     else if (result == REAGENTS) {
///         u32 missing_item;
///     }
///     else if (result == PREVENTED_BY_MECHANIC) {
///         u32 mechanic;
///     }
///     else if (result == NEED_EXOTIC_AMMO) {
///         u32 equipped_item_sub_class;
///     }
///     else if (result == NEED_MORE_ITEMS) {
///         Item item;
///         u32 count;
///     }
///     else if (result == MIN_SKILL) {
///         (u32)Skill skill;
///         u32 skill_required;
///     }
///     else if (result == FISHING_TOO_LOW) {
///         u32 fishing_skill_required;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PET_CAST_FAILED {
    pub cast_count: u8,
    pub id: u32,
    pub result: SMSG_PET_CAST_FAILED_SpellCastResult,
    pub multiple_casts: bool,
}

impl crate::private::Sealed for SMSG_PET_CAST_FAILED {}
impl SMSG_PET_CAST_FAILED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(7..=15).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(&mut r)?;

        // id: Spell
        let id = crate::util::read_u32_le(&mut r)?;

        // result: SpellCastResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        // multiple_casts: Bool
        let multiple_casts = crate::util::read_bool_u8(&mut r)?;

        let result_if = match result {
            SpellCastResult::Success => SMSG_PET_CAST_FAILED_SpellCastResult::Success,
            SpellCastResult::AffectingCombat => SMSG_PET_CAST_FAILED_SpellCastResult::AffectingCombat,
            SpellCastResult::AlreadyAtFullHealth => SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyAtFullHealth,
            SpellCastResult::AlreadyAtFullMana => SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyAtFullMana,
            SpellCastResult::AlreadyAtFullPower => SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyAtFullPower,
            SpellCastResult::AlreadyBeingTamed => SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyBeingTamed,
            SpellCastResult::AlreadyHaveCharm => SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyHaveCharm,
            SpellCastResult::AlreadyHaveSummon => SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyHaveSummon,
            SpellCastResult::AlreadyOpen => SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyOpen,
            SpellCastResult::AuraBounced => SMSG_PET_CAST_FAILED_SpellCastResult::AuraBounced,
            SpellCastResult::AutotrackInterrupted => SMSG_PET_CAST_FAILED_SpellCastResult::AutotrackInterrupted,
            SpellCastResult::BadImplicitTargets => SMSG_PET_CAST_FAILED_SpellCastResult::BadImplicitTargets,
            SpellCastResult::BadTargets => SMSG_PET_CAST_FAILED_SpellCastResult::BadTargets,
            SpellCastResult::CantBeCharmed => SMSG_PET_CAST_FAILED_SpellCastResult::CantBeCharmed,
            SpellCastResult::CantBeDisenchanted => SMSG_PET_CAST_FAILED_SpellCastResult::CantBeDisenchanted,
            SpellCastResult::CantBeDisenchantedSkill => SMSG_PET_CAST_FAILED_SpellCastResult::CantBeDisenchantedSkill,
            SpellCastResult::CantBeMilled => SMSG_PET_CAST_FAILED_SpellCastResult::CantBeMilled,
            SpellCastResult::CantBeProspected => SMSG_PET_CAST_FAILED_SpellCastResult::CantBeProspected,
            SpellCastResult::CantCastOnTapped => SMSG_PET_CAST_FAILED_SpellCastResult::CantCastOnTapped,
            SpellCastResult::CantDuelWhileInvisible => SMSG_PET_CAST_FAILED_SpellCastResult::CantDuelWhileInvisible,
            SpellCastResult::CantDuelWhileStealthed => SMSG_PET_CAST_FAILED_SpellCastResult::CantDuelWhileStealthed,
            SpellCastResult::CantStealth => SMSG_PET_CAST_FAILED_SpellCastResult::CantStealth,
            SpellCastResult::CasterAurastate => SMSG_PET_CAST_FAILED_SpellCastResult::CasterAurastate,
            SpellCastResult::CasterDead => SMSG_PET_CAST_FAILED_SpellCastResult::CasterDead,
            SpellCastResult::Charmed => SMSG_PET_CAST_FAILED_SpellCastResult::Charmed,
            SpellCastResult::ChestInUse => SMSG_PET_CAST_FAILED_SpellCastResult::ChestInUse,
            SpellCastResult::Confused => SMSG_PET_CAST_FAILED_SpellCastResult::Confused,
            SpellCastResult::DontReport => SMSG_PET_CAST_FAILED_SpellCastResult::DontReport,
            SpellCastResult::EquippedItem => SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItem,
            SpellCastResult::EquippedItemClass => {
                // item_class: u32
                let item_class = crate::util::read_u32_le(&mut r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClass {
                    item_class,
                    item_sub_class,
                }
            }
            SpellCastResult::EquippedItemClassMainhand => {
                // item_class: u32
                let item_class = crate::util::read_u32_le(&mut r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassMainhand {
                    item_class,
                    item_sub_class,
                }
            }
            SpellCastResult::EquippedItemClassOffhand => {
                // item_class: u32
                let item_class = crate::util::read_u32_le(&mut r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassOffhand {
                    item_class,
                    item_sub_class,
                }
            }
            SpellCastResult::ErrorX => SMSG_PET_CAST_FAILED_SpellCastResult::ErrorX,
            SpellCastResult::Fizzle => SMSG_PET_CAST_FAILED_SpellCastResult::Fizzle,
            SpellCastResult::Fleeing => SMSG_PET_CAST_FAILED_SpellCastResult::Fleeing,
            SpellCastResult::FoodLowlevel => SMSG_PET_CAST_FAILED_SpellCastResult::FoodLowlevel,
            SpellCastResult::Highlevel => SMSG_PET_CAST_FAILED_SpellCastResult::Highlevel,
            SpellCastResult::HungerSatiated => SMSG_PET_CAST_FAILED_SpellCastResult::HungerSatiated,
            SpellCastResult::Immune => SMSG_PET_CAST_FAILED_SpellCastResult::Immune,
            SpellCastResult::IncorrectArea => SMSG_PET_CAST_FAILED_SpellCastResult::IncorrectArea,
            SpellCastResult::Interrupted => SMSG_PET_CAST_FAILED_SpellCastResult::Interrupted,
            SpellCastResult::InterruptedCombat => SMSG_PET_CAST_FAILED_SpellCastResult::InterruptedCombat,
            SpellCastResult::ItemAlreadyEnchanted => SMSG_PET_CAST_FAILED_SpellCastResult::ItemAlreadyEnchanted,
            SpellCastResult::ItemGone => SMSG_PET_CAST_FAILED_SpellCastResult::ItemGone,
            SpellCastResult::ItemNotFound => SMSG_PET_CAST_FAILED_SpellCastResult::ItemNotFound,
            SpellCastResult::ItemNotReady => SMSG_PET_CAST_FAILED_SpellCastResult::ItemNotReady,
            SpellCastResult::LevelRequirement => SMSG_PET_CAST_FAILED_SpellCastResult::LevelRequirement,
            SpellCastResult::LineOfSight => SMSG_PET_CAST_FAILED_SpellCastResult::LineOfSight,
            SpellCastResult::Lowlevel => SMSG_PET_CAST_FAILED_SpellCastResult::Lowlevel,
            SpellCastResult::LowCastlevel => SMSG_PET_CAST_FAILED_SpellCastResult::LowCastlevel,
            SpellCastResult::MainhandEmpty => SMSG_PET_CAST_FAILED_SpellCastResult::MainhandEmpty,
            SpellCastResult::Moving => SMSG_PET_CAST_FAILED_SpellCastResult::Moving,
            SpellCastResult::NeedAmmo => SMSG_PET_CAST_FAILED_SpellCastResult::NeedAmmo,
            SpellCastResult::NeedAmmoPouch => SMSG_PET_CAST_FAILED_SpellCastResult::NeedAmmoPouch,
            SpellCastResult::NeedExoticAmmo => {
                // equipped_item_sub_class: u32
                let equipped_item_sub_class = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::NeedExoticAmmo {
                    equipped_item_sub_class,
                }
            }
            SpellCastResult::NeedMoreItems => {
                // item: Item
                let item = crate::util::read_u32_le(&mut r)?;

                // count: u32
                let count = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::NeedMoreItems {
                    count,
                    item,
                }
            }
            SpellCastResult::Nopath => SMSG_PET_CAST_FAILED_SpellCastResult::Nopath,
            SpellCastResult::NotBehind => SMSG_PET_CAST_FAILED_SpellCastResult::NotBehind,
            SpellCastResult::NotFishable => SMSG_PET_CAST_FAILED_SpellCastResult::NotFishable,
            SpellCastResult::NotFlying => SMSG_PET_CAST_FAILED_SpellCastResult::NotFlying,
            SpellCastResult::NotHere => SMSG_PET_CAST_FAILED_SpellCastResult::NotHere,
            SpellCastResult::NotInfront => SMSG_PET_CAST_FAILED_SpellCastResult::NotInfront,
            SpellCastResult::NotInControl => SMSG_PET_CAST_FAILED_SpellCastResult::NotInControl,
            SpellCastResult::NotKnown => SMSG_PET_CAST_FAILED_SpellCastResult::NotKnown,
            SpellCastResult::NotMounted => SMSG_PET_CAST_FAILED_SpellCastResult::NotMounted,
            SpellCastResult::NotOnTaxi => SMSG_PET_CAST_FAILED_SpellCastResult::NotOnTaxi,
            SpellCastResult::NotOnTransport => SMSG_PET_CAST_FAILED_SpellCastResult::NotOnTransport,
            SpellCastResult::NotReady => SMSG_PET_CAST_FAILED_SpellCastResult::NotReady,
            SpellCastResult::NotShapeshift => SMSG_PET_CAST_FAILED_SpellCastResult::NotShapeshift,
            SpellCastResult::NotStanding => SMSG_PET_CAST_FAILED_SpellCastResult::NotStanding,
            SpellCastResult::NotTradeable => SMSG_PET_CAST_FAILED_SpellCastResult::NotTradeable,
            SpellCastResult::NotTrading => SMSG_PET_CAST_FAILED_SpellCastResult::NotTrading,
            SpellCastResult::NotUnsheathed => SMSG_PET_CAST_FAILED_SpellCastResult::NotUnsheathed,
            SpellCastResult::NotWhileGhost => SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileGhost,
            SpellCastResult::NotWhileLooting => SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileLooting,
            SpellCastResult::NoAmmo => SMSG_PET_CAST_FAILED_SpellCastResult::NoAmmo,
            SpellCastResult::NoChargesRemain => SMSG_PET_CAST_FAILED_SpellCastResult::NoChargesRemain,
            SpellCastResult::NoChampion => SMSG_PET_CAST_FAILED_SpellCastResult::NoChampion,
            SpellCastResult::NoComboPoints => SMSG_PET_CAST_FAILED_SpellCastResult::NoComboPoints,
            SpellCastResult::NoDueling => SMSG_PET_CAST_FAILED_SpellCastResult::NoDueling,
            SpellCastResult::NoEndurance => SMSG_PET_CAST_FAILED_SpellCastResult::NoEndurance,
            SpellCastResult::NoFish => SMSG_PET_CAST_FAILED_SpellCastResult::NoFish,
            SpellCastResult::NoItemsWhileShapeshifted => SMSG_PET_CAST_FAILED_SpellCastResult::NoItemsWhileShapeshifted,
            SpellCastResult::NoMountsAllowed => SMSG_PET_CAST_FAILED_SpellCastResult::NoMountsAllowed,
            SpellCastResult::NoPet => SMSG_PET_CAST_FAILED_SpellCastResult::NoPet,
            SpellCastResult::NoPower => SMSG_PET_CAST_FAILED_SpellCastResult::NoPower,
            SpellCastResult::NothingToDispel => SMSG_PET_CAST_FAILED_SpellCastResult::NothingToDispel,
            SpellCastResult::NothingToSteal => SMSG_PET_CAST_FAILED_SpellCastResult::NothingToSteal,
            SpellCastResult::OnlyAbovewater => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyAbovewater,
            SpellCastResult::OnlyDaytime => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyDaytime,
            SpellCastResult::OnlyIndoors => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyIndoors,
            SpellCastResult::OnlyMounted => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyMounted,
            SpellCastResult::OnlyNighttime => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyNighttime,
            SpellCastResult::OnlyOutdoors => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyOutdoors,
            SpellCastResult::OnlyShapeshift => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyShapeshift,
            SpellCastResult::OnlyStealthed => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyStealthed,
            SpellCastResult::OnlyUnderwater => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyUnderwater,
            SpellCastResult::OutOfRange => SMSG_PET_CAST_FAILED_SpellCastResult::OutOfRange,
            SpellCastResult::Pacified => SMSG_PET_CAST_FAILED_SpellCastResult::Pacified,
            SpellCastResult::Possessed => SMSG_PET_CAST_FAILED_SpellCastResult::Possessed,
            SpellCastResult::Reagents => {
                // missing_item: u32
                let missing_item = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::Reagents {
                    missing_item,
                }
            }
            SpellCastResult::RequiresArea => {
                // area: Area
                let area = crate::util::read_u32_le(&mut r)?.try_into()?;

                SMSG_PET_CAST_FAILED_SpellCastResult::RequiresArea {
                    area,
                }
            }
            SpellCastResult::RequiresSpellFocus => {
                // spell_focus: u32
                let spell_focus = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                    spell_focus,
                }
            }
            SpellCastResult::Rooted => SMSG_PET_CAST_FAILED_SpellCastResult::Rooted,
            SpellCastResult::Silenced => SMSG_PET_CAST_FAILED_SpellCastResult::Silenced,
            SpellCastResult::SpellInProgress => SMSG_PET_CAST_FAILED_SpellCastResult::SpellInProgress,
            SpellCastResult::SpellLearned => SMSG_PET_CAST_FAILED_SpellCastResult::SpellLearned,
            SpellCastResult::SpellUnavailable => SMSG_PET_CAST_FAILED_SpellCastResult::SpellUnavailable,
            SpellCastResult::Stunned => SMSG_PET_CAST_FAILED_SpellCastResult::Stunned,
            SpellCastResult::TargetsDead => SMSG_PET_CAST_FAILED_SpellCastResult::TargetsDead,
            SpellCastResult::TargetAffectingCombat => SMSG_PET_CAST_FAILED_SpellCastResult::TargetAffectingCombat,
            SpellCastResult::TargetAurastate => SMSG_PET_CAST_FAILED_SpellCastResult::TargetAurastate,
            SpellCastResult::TargetDueling => SMSG_PET_CAST_FAILED_SpellCastResult::TargetDueling,
            SpellCastResult::TargetEnemy => SMSG_PET_CAST_FAILED_SpellCastResult::TargetEnemy,
            SpellCastResult::TargetEnraged => SMSG_PET_CAST_FAILED_SpellCastResult::TargetEnraged,
            SpellCastResult::TargetFriendly => SMSG_PET_CAST_FAILED_SpellCastResult::TargetFriendly,
            SpellCastResult::TargetInCombat => SMSG_PET_CAST_FAILED_SpellCastResult::TargetInCombat,
            SpellCastResult::TargetIsPlayer => SMSG_PET_CAST_FAILED_SpellCastResult::TargetIsPlayer,
            SpellCastResult::TargetIsPlayerControlled => SMSG_PET_CAST_FAILED_SpellCastResult::TargetIsPlayerControlled,
            SpellCastResult::TargetNotDead => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotDead,
            SpellCastResult::TargetNotInParty => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInParty,
            SpellCastResult::TargetNotLooted => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotLooted,
            SpellCastResult::TargetNotPlayer => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotPlayer,
            SpellCastResult::TargetNoPockets => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNoPockets,
            SpellCastResult::TargetNoWeapons => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNoWeapons,
            SpellCastResult::TargetNoRangedWeapons => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNoRangedWeapons,
            SpellCastResult::TargetUnskinnable => SMSG_PET_CAST_FAILED_SpellCastResult::TargetUnskinnable,
            SpellCastResult::ThirstSatiated => SMSG_PET_CAST_FAILED_SpellCastResult::ThirstSatiated,
            SpellCastResult::TooClose => SMSG_PET_CAST_FAILED_SpellCastResult::TooClose,
            SpellCastResult::TooManyOfItem => {
                // item_limit_category: u32
                let item_limit_category = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::TooManyOfItem {
                    item_limit_category,
                }
            }
            SpellCastResult::TotemCategory => {
                // totem_categories: u32[2]
                let totem_categories = {
                    let mut totem_categories = [u32::default(); 2];
                    for i in totem_categories.iter_mut() {
                        *i = crate::util::read_u32_le(&mut r)?;
                    }
                    totem_categories
                };

                SMSG_PET_CAST_FAILED_SpellCastResult::TotemCategory {
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

                SMSG_PET_CAST_FAILED_SpellCastResult::Totems {
                    totems,
                }
            }
            SpellCastResult::TryAgain => SMSG_PET_CAST_FAILED_SpellCastResult::TryAgain,
            SpellCastResult::UnitNotBehind => SMSG_PET_CAST_FAILED_SpellCastResult::UnitNotBehind,
            SpellCastResult::UnitNotInfront => SMSG_PET_CAST_FAILED_SpellCastResult::UnitNotInfront,
            SpellCastResult::WrongPetFood => SMSG_PET_CAST_FAILED_SpellCastResult::WrongPetFood,
            SpellCastResult::NotWhileFatigued => SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileFatigued,
            SpellCastResult::TargetNotInInstance => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInInstance,
            SpellCastResult::NotWhileTrading => SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileTrading,
            SpellCastResult::TargetNotInRaid => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInRaid,
            SpellCastResult::TargetFreeforall => SMSG_PET_CAST_FAILED_SpellCastResult::TargetFreeforall,
            SpellCastResult::NoEdibleCorpses => SMSG_PET_CAST_FAILED_SpellCastResult::NoEdibleCorpses,
            SpellCastResult::OnlyBattlegrounds => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyBattlegrounds,
            SpellCastResult::TargetNotGhost => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotGhost,
            SpellCastResult::TransformUnusable => SMSG_PET_CAST_FAILED_SpellCastResult::TransformUnusable,
            SpellCastResult::WrongWeather => SMSG_PET_CAST_FAILED_SpellCastResult::WrongWeather,
            SpellCastResult::DamageImmune => SMSG_PET_CAST_FAILED_SpellCastResult::DamageImmune,
            SpellCastResult::PreventedByMechanic => {
                // mechanic: u32
                let mechanic = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::PreventedByMechanic {
                    mechanic,
                }
            }
            SpellCastResult::PlayTime => SMSG_PET_CAST_FAILED_SpellCastResult::PlayTime,
            SpellCastResult::Reputation => SMSG_PET_CAST_FAILED_SpellCastResult::Reputation,
            SpellCastResult::MinSkill => {
                // skill: Skill
                let skill = (crate::util::read_u32_le(&mut r)? as u16).try_into()?;

                // skill_required: u32
                let skill_required = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::MinSkill {
                    skill,
                    skill_required,
                }
            }
            SpellCastResult::NotInArena => SMSG_PET_CAST_FAILED_SpellCastResult::NotInArena,
            SpellCastResult::NotOnShapeshift => SMSG_PET_CAST_FAILED_SpellCastResult::NotOnShapeshift,
            SpellCastResult::NotOnStealthed => SMSG_PET_CAST_FAILED_SpellCastResult::NotOnStealthed,
            SpellCastResult::NotOnDamageImmune => SMSG_PET_CAST_FAILED_SpellCastResult::NotOnDamageImmune,
            SpellCastResult::NotOnMounted => SMSG_PET_CAST_FAILED_SpellCastResult::NotOnMounted,
            SpellCastResult::TooShallow => SMSG_PET_CAST_FAILED_SpellCastResult::TooShallow,
            SpellCastResult::TargetNotInSanctuary => SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInSanctuary,
            SpellCastResult::TargetIsTrivial => SMSG_PET_CAST_FAILED_SpellCastResult::TargetIsTrivial,
            SpellCastResult::BmOrInvisgod => SMSG_PET_CAST_FAILED_SpellCastResult::BmOrInvisgod,
            SpellCastResult::ExpertRidingRequirement => SMSG_PET_CAST_FAILED_SpellCastResult::ExpertRidingRequirement,
            SpellCastResult::ArtisanRidingRequirement => SMSG_PET_CAST_FAILED_SpellCastResult::ArtisanRidingRequirement,
            SpellCastResult::NotIdle => SMSG_PET_CAST_FAILED_SpellCastResult::NotIdle,
            SpellCastResult::NotInactive => SMSG_PET_CAST_FAILED_SpellCastResult::NotInactive,
            SpellCastResult::PartialPlaytime => SMSG_PET_CAST_FAILED_SpellCastResult::PartialPlaytime,
            SpellCastResult::NoPlaytime => SMSG_PET_CAST_FAILED_SpellCastResult::NoPlaytime,
            SpellCastResult::NotInBattleground => SMSG_PET_CAST_FAILED_SpellCastResult::NotInBattleground,
            SpellCastResult::NotInRaidInstance => SMSG_PET_CAST_FAILED_SpellCastResult::NotInRaidInstance,
            SpellCastResult::OnlyInArena => SMSG_PET_CAST_FAILED_SpellCastResult::OnlyInArena,
            SpellCastResult::TargetLockedToRaidInstance => SMSG_PET_CAST_FAILED_SpellCastResult::TargetLockedToRaidInstance,
            SpellCastResult::OnUseEnchant => SMSG_PET_CAST_FAILED_SpellCastResult::OnUseEnchant,
            SpellCastResult::NotOnGround => SMSG_PET_CAST_FAILED_SpellCastResult::NotOnGround,
            SpellCastResult::CustomError => {
                // custom_error: u32
                let custom_error = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::CustomError {
                    custom_error,
                }
            }
            SpellCastResult::CantDoThatRightNow => SMSG_PET_CAST_FAILED_SpellCastResult::CantDoThatRightNow,
            SpellCastResult::TooManySockets => SMSG_PET_CAST_FAILED_SpellCastResult::TooManySockets,
            SpellCastResult::InvalidGlyph => SMSG_PET_CAST_FAILED_SpellCastResult::InvalidGlyph,
            SpellCastResult::UniqueGlyph => SMSG_PET_CAST_FAILED_SpellCastResult::UniqueGlyph,
            SpellCastResult::GlyphSocketLocked => SMSG_PET_CAST_FAILED_SpellCastResult::GlyphSocketLocked,
            SpellCastResult::NoValidTargets => SMSG_PET_CAST_FAILED_SpellCastResult::NoValidTargets,
            SpellCastResult::ItemAtMaxCharges => SMSG_PET_CAST_FAILED_SpellCastResult::ItemAtMaxCharges,
            SpellCastResult::NotInBarbershop => SMSG_PET_CAST_FAILED_SpellCastResult::NotInBarbershop,
            SpellCastResult::FishingTooLow => {
                // fishing_skill_required: u32
                let fishing_skill_required = crate::util::read_u32_le(&mut r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::FishingTooLow {
                    fishing_skill_required,
                }
            }
            SpellCastResult::ItemEnchantTradeWindow => SMSG_PET_CAST_FAILED_SpellCastResult::ItemEnchantTradeWindow,
            SpellCastResult::SummonPending => SMSG_PET_CAST_FAILED_SpellCastResult::SummonPending,
            SpellCastResult::MaxSockets => SMSG_PET_CAST_FAILED_SpellCastResult::MaxSockets,
            SpellCastResult::PetCanRename => SMSG_PET_CAST_FAILED_SpellCastResult::PetCanRename,
            SpellCastResult::TargetCannotBeResurrected => SMSG_PET_CAST_FAILED_SpellCastResult::TargetCannotBeResurrected,
            SpellCastResult::Unknown => SMSG_PET_CAST_FAILED_SpellCastResult::Unknown,
        };

        Ok(Self {
            cast_count,
            id,
            result: result_if,
            multiple_casts,
        })
    }

}

impl crate::Message for SMSG_PET_CAST_FAILED {
    const OPCODE: u32 = 0x0138;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PET_CAST_FAILED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_CAST_FAILED {{").unwrap();
        // Members
        writeln!(s, "    cast_count = {};", self.cast_count).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    result = {};", SpellCastResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        writeln!(s, "    multiple_casts = {};", if self.multiple_casts { "TRUE" } else { "FALSE" }).unwrap();
        match &self.result {
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClass {
                item_class,
                item_sub_class,
            } => {
                writeln!(s, "    item_class = {};", item_class).unwrap();
                writeln!(s, "    item_sub_class = {};", item_sub_class).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassMainhand {
                item_class,
                item_sub_class,
            } => {
                writeln!(s, "    item_class = {};", item_class).unwrap();
                writeln!(s, "    item_sub_class = {};", item_sub_class).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassOffhand {
                item_class,
                item_sub_class,
            } => {
                writeln!(s, "    item_class = {};", item_class).unwrap();
                writeln!(s, "    item_sub_class = {};", item_sub_class).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::NeedExoticAmmo {
                equipped_item_sub_class,
            } => {
                writeln!(s, "    equipped_item_sub_class = {};", equipped_item_sub_class).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::NeedMoreItems {
                count,
                item,
            } => {
                writeln!(s, "    item = {};", item).unwrap();
                writeln!(s, "    count = {};", count).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::Reagents {
                missing_item,
            } => {
                writeln!(s, "    missing_item = {};", missing_item).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::RequiresArea {
                area,
            } => {
                writeln!(s, "    area = {};", area.as_test_case_value()).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                spell_focus,
            } => {
                writeln!(s, "    spell_focus = {};", spell_focus).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::TooManyOfItem {
                item_limit_category,
            } => {
                writeln!(s, "    item_limit_category = {};", item_limit_category).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::TotemCategory {
                totem_categories,
            } => {
                writeln!(s, "    totem_categories = [").unwrap();
                for v in totem_categories.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::Totems {
                totems,
            } => {
                writeln!(s, "    totems = [").unwrap();
                for v in totems.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::PreventedByMechanic {
                mechanic,
            } => {
                writeln!(s, "    mechanic = {};", mechanic).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::MinSkill {
                skill,
                skill_required,
            } => {
                writeln!(s, "    skill = {};", skill.as_test_case_value()).unwrap();
                writeln!(s, "    skill_required = {};", skill_required).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::CustomError {
                custom_error,
            } => {
                writeln!(s, "    custom_error = {};", custom_error).unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::FishingTooLow {
                fishing_skill_required,
            } => {
                writeln!(s, "    fishing_skill_required = {};", fishing_skill_required).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 312_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "cast_count", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "multiple_casts", "    ");
        match &self.result {
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClass {
                item_class,
                item_sub_class,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_class", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_sub_class", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassMainhand {
                item_class,
                item_sub_class,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_class", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_sub_class", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassOffhand {
                item_class,
                item_sub_class,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_class", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_sub_class", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::NeedExoticAmmo {
                equipped_item_sub_class,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "equipped_item_sub_class", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::NeedMoreItems {
                count,
                item,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "count", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::Reagents {
                missing_item,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "missing_item", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::RequiresArea {
                area,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                spell_focus,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_focus", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::TooManyOfItem {
                item_limit_category,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_limit_category", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::TotemCategory {
                totem_categories,
            } => {
                writeln!(s, "    /* totem_categories: u32[2] start */").unwrap();
                for (i, v) in totem_categories.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("totem_categories {i}"), "    ");
                }
                writeln!(s, "    /* totem_categories: u32[2] end */").unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::Totems {
                totems,
            } => {
                writeln!(s, "    /* totems: u32[2] start */").unwrap();
                for (i, v) in totems.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("totems {i}"), "    ");
                }
                writeln!(s, "    /* totems: u32[2] end */").unwrap();
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::PreventedByMechanic {
                mechanic,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "mechanic", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::MinSkill {
                skill,
                skill_required,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "skill", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "skill_required", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::CustomError {
                custom_error,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "custom_error", "    ");
            }
            crate::wrath::SMSG_PET_CAST_FAILED_SpellCastResult::FishingTooLow {
                fishing_skill_required,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "fishing_skill_required", "    ");
            }
            _ => {}
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
        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // id: Spell
        w.write_all(&self.id.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        // multiple_casts: Bool
        w.write_all(u8::from(self.multiple_casts).to_le_bytes().as_slice())?;

        match &self.result {
            SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClass {
                item_class,
                item_sub_class,
            } => {
                // item_class: u32
                w.write_all(&item_class.to_le_bytes())?;

                // item_sub_class: u32
                w.write_all(&item_sub_class.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassMainhand {
                item_class,
                item_sub_class,
            } => {
                // item_class: u32
                w.write_all(&item_class.to_le_bytes())?;

                // item_sub_class: u32
                w.write_all(&item_sub_class.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassOffhand {
                item_class,
                item_sub_class,
            } => {
                // item_class: u32
                w.write_all(&item_class.to_le_bytes())?;

                // item_sub_class: u32
                w.write_all(&item_sub_class.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::NeedExoticAmmo {
                equipped_item_sub_class,
            } => {
                // equipped_item_sub_class: u32
                w.write_all(&equipped_item_sub_class.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::NeedMoreItems {
                count,
                item,
            } => {
                // item: Item
                w.write_all(&item.to_le_bytes())?;

                // count: u32
                w.write_all(&count.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::Reagents {
                missing_item,
            } => {
                // missing_item: u32
                w.write_all(&missing_item.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::RequiresArea {
                area,
            } => {
                // area: Area
                w.write_all(&(area.as_int().to_le_bytes()))?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                spell_focus,
            } => {
                // spell_focus: u32
                w.write_all(&spell_focus.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::TooManyOfItem {
                item_limit_category,
            } => {
                // item_limit_category: u32
                w.write_all(&item_limit_category.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::TotemCategory {
                totem_categories,
            } => {
                // totem_categories: u32[2]
                for i in totem_categories.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::Totems {
                totems,
            } => {
                // totems: u32[2]
                for i in totems.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::PreventedByMechanic {
                mechanic,
            } => {
                // mechanic: u32
                w.write_all(&mechanic.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::MinSkill {
                skill,
                skill_required,
            } => {
                // skill: Skill
                w.write_all(&u32::from(skill.as_int()).to_le_bytes())?;

                // skill_required: u32
                w.write_all(&skill_required.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::CustomError {
                custom_error,
            } => {
                // custom_error: u32
                w.write_all(&custom_error.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::FishingTooLow {
                fishing_skill_required,
            } => {
                // fishing_skill_required: u32
                w.write_all(&fishing_skill_required.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(312, "SMSG_PET_CAST_FAILED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_CAST_FAILED {}

impl SMSG_PET_CAST_FAILED {
    pub(crate) const fn size(&self) -> usize {
        1 // cast_count: u8
        + 4 // id: Spell
        + self.result.size() // result: SMSG_PET_CAST_FAILED_SpellCastResult
        + 1 // multiple_casts: Bool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_PET_CAST_FAILED_SpellCastResult {
    Success,
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
    CantBeMilled,
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
        item_sub_class: u32,
    },
    EquippedItemClassMainhand {
        item_class: u32,
        item_sub_class: u32,
    },
    EquippedItemClassOffhand {
        item_class: u32,
        item_sub_class: u32,
    },
    ErrorX,
    Fizzle,
    Fleeing,
    FoodLowlevel,
    Highlevel,
    HungerSatiated,
    Immune,
    IncorrectArea,
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
    NeedExoticAmmo {
        equipped_item_sub_class: u32,
    },
    NeedMoreItems {
        count: u32,
        item: u32,
    },
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
    NotWhileLooting,
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
    Reagents {
        missing_item: u32,
    },
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
    TargetNoRangedWeapons,
    TargetUnskinnable,
    ThirstSatiated,
    TooClose,
    TooManyOfItem {
        item_limit_category: u32,
    },
    TotemCategory {
        totem_categories: [u32; 2],
    },
    Totems {
        totems: [u32; 2],
    },
    TryAgain,
    UnitNotBehind,
    UnitNotInfront,
    WrongPetFood,
    NotWhileFatigued,
    TargetNotInInstance,
    NotWhileTrading,
    TargetNotInRaid,
    TargetFreeforall,
    NoEdibleCorpses,
    OnlyBattlegrounds,
    TargetNotGhost,
    TransformUnusable,
    WrongWeather,
    DamageImmune,
    PreventedByMechanic {
        mechanic: u32,
    },
    PlayTime,
    Reputation,
    MinSkill {
        skill: Skill,
        skill_required: u32,
    },
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
    NotInRaidInstance,
    OnlyInArena,
    TargetLockedToRaidInstance,
    OnUseEnchant,
    NotOnGround,
    CustomError {
        custom_error: u32,
    },
    CantDoThatRightNow,
    TooManySockets,
    InvalidGlyph,
    UniqueGlyph,
    GlyphSocketLocked,
    NoValidTargets,
    ItemAtMaxCharges,
    NotInBarbershop,
    FishingTooLow {
        fishing_skill_required: u32,
    },
    ItemEnchantTradeWindow,
    SummonPending,
    MaxSockets,
    PetCanRename,
    TargetCannotBeResurrected,
    Unknown,
}

impl Default for SMSG_PET_CAST_FAILED_SpellCastResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Success
    }
}

impl SMSG_PET_CAST_FAILED_SpellCastResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::AffectingCombat => 1,
            Self::AlreadyAtFullHealth => 2,
            Self::AlreadyAtFullMana => 3,
            Self::AlreadyAtFullPower => 4,
            Self::AlreadyBeingTamed => 5,
            Self::AlreadyHaveCharm => 6,
            Self::AlreadyHaveSummon => 7,
            Self::AlreadyOpen => 8,
            Self::AuraBounced => 9,
            Self::AutotrackInterrupted => 10,
            Self::BadImplicitTargets => 11,
            Self::BadTargets => 12,
            Self::CantBeCharmed => 13,
            Self::CantBeDisenchanted => 14,
            Self::CantBeDisenchantedSkill => 15,
            Self::CantBeMilled => 16,
            Self::CantBeProspected => 17,
            Self::CantCastOnTapped => 18,
            Self::CantDuelWhileInvisible => 19,
            Self::CantDuelWhileStealthed => 20,
            Self::CantStealth => 21,
            Self::CasterAurastate => 22,
            Self::CasterDead => 23,
            Self::Charmed => 24,
            Self::ChestInUse => 25,
            Self::Confused => 26,
            Self::DontReport => 27,
            Self::EquippedItem => 28,
            Self::EquippedItemClass { .. } => 29,
            Self::EquippedItemClassMainhand { .. } => 30,
            Self::EquippedItemClassOffhand { .. } => 31,
            Self::ErrorX => 32,
            Self::Fizzle => 33,
            Self::Fleeing => 34,
            Self::FoodLowlevel => 35,
            Self::Highlevel => 36,
            Self::HungerSatiated => 37,
            Self::Immune => 38,
            Self::IncorrectArea => 39,
            Self::Interrupted => 40,
            Self::InterruptedCombat => 41,
            Self::ItemAlreadyEnchanted => 42,
            Self::ItemGone => 43,
            Self::ItemNotFound => 44,
            Self::ItemNotReady => 45,
            Self::LevelRequirement => 46,
            Self::LineOfSight => 47,
            Self::Lowlevel => 48,
            Self::LowCastlevel => 49,
            Self::MainhandEmpty => 50,
            Self::Moving => 51,
            Self::NeedAmmo => 52,
            Self::NeedAmmoPouch => 53,
            Self::NeedExoticAmmo { .. } => 54,
            Self::NeedMoreItems { .. } => 55,
            Self::Nopath => 56,
            Self::NotBehind => 57,
            Self::NotFishable => 58,
            Self::NotFlying => 59,
            Self::NotHere => 60,
            Self::NotInfront => 61,
            Self::NotInControl => 62,
            Self::NotKnown => 63,
            Self::NotMounted => 64,
            Self::NotOnTaxi => 65,
            Self::NotOnTransport => 66,
            Self::NotReady => 67,
            Self::NotShapeshift => 68,
            Self::NotStanding => 69,
            Self::NotTradeable => 70,
            Self::NotTrading => 71,
            Self::NotUnsheathed => 72,
            Self::NotWhileGhost => 73,
            Self::NotWhileLooting => 74,
            Self::NoAmmo => 75,
            Self::NoChargesRemain => 76,
            Self::NoChampion => 77,
            Self::NoComboPoints => 78,
            Self::NoDueling => 79,
            Self::NoEndurance => 80,
            Self::NoFish => 81,
            Self::NoItemsWhileShapeshifted => 82,
            Self::NoMountsAllowed => 83,
            Self::NoPet => 84,
            Self::NoPower => 85,
            Self::NothingToDispel => 86,
            Self::NothingToSteal => 87,
            Self::OnlyAbovewater => 88,
            Self::OnlyDaytime => 89,
            Self::OnlyIndoors => 90,
            Self::OnlyMounted => 91,
            Self::OnlyNighttime => 92,
            Self::OnlyOutdoors => 93,
            Self::OnlyShapeshift => 94,
            Self::OnlyStealthed => 95,
            Self::OnlyUnderwater => 96,
            Self::OutOfRange => 97,
            Self::Pacified => 98,
            Self::Possessed => 99,
            Self::Reagents { .. } => 100,
            Self::RequiresArea { .. } => 101,
            Self::RequiresSpellFocus { .. } => 102,
            Self::Rooted => 103,
            Self::Silenced => 104,
            Self::SpellInProgress => 105,
            Self::SpellLearned => 106,
            Self::SpellUnavailable => 107,
            Self::Stunned => 108,
            Self::TargetsDead => 109,
            Self::TargetAffectingCombat => 110,
            Self::TargetAurastate => 111,
            Self::TargetDueling => 112,
            Self::TargetEnemy => 113,
            Self::TargetEnraged => 114,
            Self::TargetFriendly => 115,
            Self::TargetInCombat => 116,
            Self::TargetIsPlayer => 117,
            Self::TargetIsPlayerControlled => 118,
            Self::TargetNotDead => 119,
            Self::TargetNotInParty => 120,
            Self::TargetNotLooted => 121,
            Self::TargetNotPlayer => 122,
            Self::TargetNoPockets => 123,
            Self::TargetNoWeapons => 124,
            Self::TargetNoRangedWeapons => 125,
            Self::TargetUnskinnable => 126,
            Self::ThirstSatiated => 127,
            Self::TooClose => 128,
            Self::TooManyOfItem { .. } => 129,
            Self::TotemCategory { .. } => 130,
            Self::Totems { .. } => 131,
            Self::TryAgain => 132,
            Self::UnitNotBehind => 133,
            Self::UnitNotInfront => 134,
            Self::WrongPetFood => 135,
            Self::NotWhileFatigued => 136,
            Self::TargetNotInInstance => 137,
            Self::NotWhileTrading => 138,
            Self::TargetNotInRaid => 139,
            Self::TargetFreeforall => 140,
            Self::NoEdibleCorpses => 141,
            Self::OnlyBattlegrounds => 142,
            Self::TargetNotGhost => 143,
            Self::TransformUnusable => 144,
            Self::WrongWeather => 145,
            Self::DamageImmune => 146,
            Self::PreventedByMechanic { .. } => 147,
            Self::PlayTime => 148,
            Self::Reputation => 149,
            Self::MinSkill { .. } => 150,
            Self::NotInArena => 151,
            Self::NotOnShapeshift => 152,
            Self::NotOnStealthed => 153,
            Self::NotOnDamageImmune => 154,
            Self::NotOnMounted => 155,
            Self::TooShallow => 156,
            Self::TargetNotInSanctuary => 157,
            Self::TargetIsTrivial => 158,
            Self::BmOrInvisgod => 159,
            Self::ExpertRidingRequirement => 160,
            Self::ArtisanRidingRequirement => 161,
            Self::NotIdle => 162,
            Self::NotInactive => 163,
            Self::PartialPlaytime => 164,
            Self::NoPlaytime => 165,
            Self::NotInBattleground => 166,
            Self::NotInRaidInstance => 167,
            Self::OnlyInArena => 168,
            Self::TargetLockedToRaidInstance => 169,
            Self::OnUseEnchant => 170,
            Self::NotOnGround => 171,
            Self::CustomError { .. } => 172,
            Self::CantDoThatRightNow => 173,
            Self::TooManySockets => 174,
            Self::InvalidGlyph => 175,
            Self::UniqueGlyph => 176,
            Self::GlyphSocketLocked => 177,
            Self::NoValidTargets => 178,
            Self::ItemAtMaxCharges => 179,
            Self::NotInBarbershop => 180,
            Self::FishingTooLow { .. } => 181,
            Self::ItemEnchantTradeWindow => 182,
            Self::SummonPending => 183,
            Self::MaxSockets => 184,
            Self::PetCanRename => 185,
            Self::TargetCannotBeResurrected => 186,
            Self::Unknown => 187,
        }
    }

}

impl std::fmt::Display for SMSG_PET_CAST_FAILED_SpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
            Self::AffectingCombat => f.write_str("AffectingCombat"),
            Self::AlreadyAtFullHealth => f.write_str("AlreadyAtFullHealth"),
            Self::AlreadyAtFullMana => f.write_str("AlreadyAtFullMana"),
            Self::AlreadyAtFullPower => f.write_str("AlreadyAtFullPower"),
            Self::AlreadyBeingTamed => f.write_str("AlreadyBeingTamed"),
            Self::AlreadyHaveCharm => f.write_str("AlreadyHaveCharm"),
            Self::AlreadyHaveSummon => f.write_str("AlreadyHaveSummon"),
            Self::AlreadyOpen => f.write_str("AlreadyOpen"),
            Self::AuraBounced => f.write_str("AuraBounced"),
            Self::AutotrackInterrupted => f.write_str("AutotrackInterrupted"),
            Self::BadImplicitTargets => f.write_str("BadImplicitTargets"),
            Self::BadTargets => f.write_str("BadTargets"),
            Self::CantBeCharmed => f.write_str("CantBeCharmed"),
            Self::CantBeDisenchanted => f.write_str("CantBeDisenchanted"),
            Self::CantBeDisenchantedSkill => f.write_str("CantBeDisenchantedSkill"),
            Self::CantBeMilled => f.write_str("CantBeMilled"),
            Self::CantBeProspected => f.write_str("CantBeProspected"),
            Self::CantCastOnTapped => f.write_str("CantCastOnTapped"),
            Self::CantDuelWhileInvisible => f.write_str("CantDuelWhileInvisible"),
            Self::CantDuelWhileStealthed => f.write_str("CantDuelWhileStealthed"),
            Self::CantStealth => f.write_str("CantStealth"),
            Self::CasterAurastate => f.write_str("CasterAurastate"),
            Self::CasterDead => f.write_str("CasterDead"),
            Self::Charmed => f.write_str("Charmed"),
            Self::ChestInUse => f.write_str("ChestInUse"),
            Self::Confused => f.write_str("Confused"),
            Self::DontReport => f.write_str("DontReport"),
            Self::EquippedItem => f.write_str("EquippedItem"),
            Self::EquippedItemClass{ .. } => f.write_str("EquippedItemClass"),
            Self::EquippedItemClassMainhand{ .. } => f.write_str("EquippedItemClassMainhand"),
            Self::EquippedItemClassOffhand{ .. } => f.write_str("EquippedItemClassOffhand"),
            Self::ErrorX => f.write_str("ErrorX"),
            Self::Fizzle => f.write_str("Fizzle"),
            Self::Fleeing => f.write_str("Fleeing"),
            Self::FoodLowlevel => f.write_str("FoodLowlevel"),
            Self::Highlevel => f.write_str("Highlevel"),
            Self::HungerSatiated => f.write_str("HungerSatiated"),
            Self::Immune => f.write_str("Immune"),
            Self::IncorrectArea => f.write_str("IncorrectArea"),
            Self::Interrupted => f.write_str("Interrupted"),
            Self::InterruptedCombat => f.write_str("InterruptedCombat"),
            Self::ItemAlreadyEnchanted => f.write_str("ItemAlreadyEnchanted"),
            Self::ItemGone => f.write_str("ItemGone"),
            Self::ItemNotFound => f.write_str("ItemNotFound"),
            Self::ItemNotReady => f.write_str("ItemNotReady"),
            Self::LevelRequirement => f.write_str("LevelRequirement"),
            Self::LineOfSight => f.write_str("LineOfSight"),
            Self::Lowlevel => f.write_str("Lowlevel"),
            Self::LowCastlevel => f.write_str("LowCastlevel"),
            Self::MainhandEmpty => f.write_str("MainhandEmpty"),
            Self::Moving => f.write_str("Moving"),
            Self::NeedAmmo => f.write_str("NeedAmmo"),
            Self::NeedAmmoPouch => f.write_str("NeedAmmoPouch"),
            Self::NeedExoticAmmo{ .. } => f.write_str("NeedExoticAmmo"),
            Self::NeedMoreItems{ .. } => f.write_str("NeedMoreItems"),
            Self::Nopath => f.write_str("Nopath"),
            Self::NotBehind => f.write_str("NotBehind"),
            Self::NotFishable => f.write_str("NotFishable"),
            Self::NotFlying => f.write_str("NotFlying"),
            Self::NotHere => f.write_str("NotHere"),
            Self::NotInfront => f.write_str("NotInfront"),
            Self::NotInControl => f.write_str("NotInControl"),
            Self::NotKnown => f.write_str("NotKnown"),
            Self::NotMounted => f.write_str("NotMounted"),
            Self::NotOnTaxi => f.write_str("NotOnTaxi"),
            Self::NotOnTransport => f.write_str("NotOnTransport"),
            Self::NotReady => f.write_str("NotReady"),
            Self::NotShapeshift => f.write_str("NotShapeshift"),
            Self::NotStanding => f.write_str("NotStanding"),
            Self::NotTradeable => f.write_str("NotTradeable"),
            Self::NotTrading => f.write_str("NotTrading"),
            Self::NotUnsheathed => f.write_str("NotUnsheathed"),
            Self::NotWhileGhost => f.write_str("NotWhileGhost"),
            Self::NotWhileLooting => f.write_str("NotWhileLooting"),
            Self::NoAmmo => f.write_str("NoAmmo"),
            Self::NoChargesRemain => f.write_str("NoChargesRemain"),
            Self::NoChampion => f.write_str("NoChampion"),
            Self::NoComboPoints => f.write_str("NoComboPoints"),
            Self::NoDueling => f.write_str("NoDueling"),
            Self::NoEndurance => f.write_str("NoEndurance"),
            Self::NoFish => f.write_str("NoFish"),
            Self::NoItemsWhileShapeshifted => f.write_str("NoItemsWhileShapeshifted"),
            Self::NoMountsAllowed => f.write_str("NoMountsAllowed"),
            Self::NoPet => f.write_str("NoPet"),
            Self::NoPower => f.write_str("NoPower"),
            Self::NothingToDispel => f.write_str("NothingToDispel"),
            Self::NothingToSteal => f.write_str("NothingToSteal"),
            Self::OnlyAbovewater => f.write_str("OnlyAbovewater"),
            Self::OnlyDaytime => f.write_str("OnlyDaytime"),
            Self::OnlyIndoors => f.write_str("OnlyIndoors"),
            Self::OnlyMounted => f.write_str("OnlyMounted"),
            Self::OnlyNighttime => f.write_str("OnlyNighttime"),
            Self::OnlyOutdoors => f.write_str("OnlyOutdoors"),
            Self::OnlyShapeshift => f.write_str("OnlyShapeshift"),
            Self::OnlyStealthed => f.write_str("OnlyStealthed"),
            Self::OnlyUnderwater => f.write_str("OnlyUnderwater"),
            Self::OutOfRange => f.write_str("OutOfRange"),
            Self::Pacified => f.write_str("Pacified"),
            Self::Possessed => f.write_str("Possessed"),
            Self::Reagents{ .. } => f.write_str("Reagents"),
            Self::RequiresArea{ .. } => f.write_str("RequiresArea"),
            Self::RequiresSpellFocus{ .. } => f.write_str("RequiresSpellFocus"),
            Self::Rooted => f.write_str("Rooted"),
            Self::Silenced => f.write_str("Silenced"),
            Self::SpellInProgress => f.write_str("SpellInProgress"),
            Self::SpellLearned => f.write_str("SpellLearned"),
            Self::SpellUnavailable => f.write_str("SpellUnavailable"),
            Self::Stunned => f.write_str("Stunned"),
            Self::TargetsDead => f.write_str("TargetsDead"),
            Self::TargetAffectingCombat => f.write_str("TargetAffectingCombat"),
            Self::TargetAurastate => f.write_str("TargetAurastate"),
            Self::TargetDueling => f.write_str("TargetDueling"),
            Self::TargetEnemy => f.write_str("TargetEnemy"),
            Self::TargetEnraged => f.write_str("TargetEnraged"),
            Self::TargetFriendly => f.write_str("TargetFriendly"),
            Self::TargetInCombat => f.write_str("TargetInCombat"),
            Self::TargetIsPlayer => f.write_str("TargetIsPlayer"),
            Self::TargetIsPlayerControlled => f.write_str("TargetIsPlayerControlled"),
            Self::TargetNotDead => f.write_str("TargetNotDead"),
            Self::TargetNotInParty => f.write_str("TargetNotInParty"),
            Self::TargetNotLooted => f.write_str("TargetNotLooted"),
            Self::TargetNotPlayer => f.write_str("TargetNotPlayer"),
            Self::TargetNoPockets => f.write_str("TargetNoPockets"),
            Self::TargetNoWeapons => f.write_str("TargetNoWeapons"),
            Self::TargetNoRangedWeapons => f.write_str("TargetNoRangedWeapons"),
            Self::TargetUnskinnable => f.write_str("TargetUnskinnable"),
            Self::ThirstSatiated => f.write_str("ThirstSatiated"),
            Self::TooClose => f.write_str("TooClose"),
            Self::TooManyOfItem{ .. } => f.write_str("TooManyOfItem"),
            Self::TotemCategory{ .. } => f.write_str("TotemCategory"),
            Self::Totems{ .. } => f.write_str("Totems"),
            Self::TryAgain => f.write_str("TryAgain"),
            Self::UnitNotBehind => f.write_str("UnitNotBehind"),
            Self::UnitNotInfront => f.write_str("UnitNotInfront"),
            Self::WrongPetFood => f.write_str("WrongPetFood"),
            Self::NotWhileFatigued => f.write_str("NotWhileFatigued"),
            Self::TargetNotInInstance => f.write_str("TargetNotInInstance"),
            Self::NotWhileTrading => f.write_str("NotWhileTrading"),
            Self::TargetNotInRaid => f.write_str("TargetNotInRaid"),
            Self::TargetFreeforall => f.write_str("TargetFreeforall"),
            Self::NoEdibleCorpses => f.write_str("NoEdibleCorpses"),
            Self::OnlyBattlegrounds => f.write_str("OnlyBattlegrounds"),
            Self::TargetNotGhost => f.write_str("TargetNotGhost"),
            Self::TransformUnusable => f.write_str("TransformUnusable"),
            Self::WrongWeather => f.write_str("WrongWeather"),
            Self::DamageImmune => f.write_str("DamageImmune"),
            Self::PreventedByMechanic{ .. } => f.write_str("PreventedByMechanic"),
            Self::PlayTime => f.write_str("PlayTime"),
            Self::Reputation => f.write_str("Reputation"),
            Self::MinSkill{ .. } => f.write_str("MinSkill"),
            Self::NotInArena => f.write_str("NotInArena"),
            Self::NotOnShapeshift => f.write_str("NotOnShapeshift"),
            Self::NotOnStealthed => f.write_str("NotOnStealthed"),
            Self::NotOnDamageImmune => f.write_str("NotOnDamageImmune"),
            Self::NotOnMounted => f.write_str("NotOnMounted"),
            Self::TooShallow => f.write_str("TooShallow"),
            Self::TargetNotInSanctuary => f.write_str("TargetNotInSanctuary"),
            Self::TargetIsTrivial => f.write_str("TargetIsTrivial"),
            Self::BmOrInvisgod => f.write_str("BmOrInvisgod"),
            Self::ExpertRidingRequirement => f.write_str("ExpertRidingRequirement"),
            Self::ArtisanRidingRequirement => f.write_str("ArtisanRidingRequirement"),
            Self::NotIdle => f.write_str("NotIdle"),
            Self::NotInactive => f.write_str("NotInactive"),
            Self::PartialPlaytime => f.write_str("PartialPlaytime"),
            Self::NoPlaytime => f.write_str("NoPlaytime"),
            Self::NotInBattleground => f.write_str("NotInBattleground"),
            Self::NotInRaidInstance => f.write_str("NotInRaidInstance"),
            Self::OnlyInArena => f.write_str("OnlyInArena"),
            Self::TargetLockedToRaidInstance => f.write_str("TargetLockedToRaidInstance"),
            Self::OnUseEnchant => f.write_str("OnUseEnchant"),
            Self::NotOnGround => f.write_str("NotOnGround"),
            Self::CustomError{ .. } => f.write_str("CustomError"),
            Self::CantDoThatRightNow => f.write_str("CantDoThatRightNow"),
            Self::TooManySockets => f.write_str("TooManySockets"),
            Self::InvalidGlyph => f.write_str("InvalidGlyph"),
            Self::UniqueGlyph => f.write_str("UniqueGlyph"),
            Self::GlyphSocketLocked => f.write_str("GlyphSocketLocked"),
            Self::NoValidTargets => f.write_str("NoValidTargets"),
            Self::ItemAtMaxCharges => f.write_str("ItemAtMaxCharges"),
            Self::NotInBarbershop => f.write_str("NotInBarbershop"),
            Self::FishingTooLow{ .. } => f.write_str("FishingTooLow"),
            Self::ItemEnchantTradeWindow => f.write_str("ItemEnchantTradeWindow"),
            Self::SummonPending => f.write_str("SummonPending"),
            Self::MaxSockets => f.write_str("MaxSockets"),
            Self::PetCanRename => f.write_str("PetCanRename"),
            Self::TargetCannotBeResurrected => f.write_str("TargetCannotBeResurrected"),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl SMSG_PET_CAST_FAILED_SpellCastResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::EquippedItemClass {
                ..
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_sub_class: u32
            }
            Self::EquippedItemClassMainhand {
                ..
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_sub_class: u32
            }
            Self::EquippedItemClassOffhand {
                ..
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_sub_class: u32
            }
            Self::NeedExoticAmmo {
                ..
            } => {
                1
                + 4 // equipped_item_sub_class: u32
            }
            Self::NeedMoreItems {
                ..
            } => {
                1
                + 4 // count: u32
                + 4 // item: Item
            }
            Self::Reagents {
                ..
            } => {
                1
                + 4 // missing_item: u32
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
            Self::TooManyOfItem {
                ..
            } => {
                1
                + 4 // item_limit_category: u32
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
            Self::PreventedByMechanic {
                ..
            } => {
                1
                + 4 // mechanic: u32
            }
            Self::MinSkill {
                ..
            } => {
                1
                + 4 // skill: Skill
                + 4 // skill_required: u32
            }
            Self::CustomError {
                ..
            } => {
                1
                + 4 // custom_error: u32
            }
            Self::FishingTooLow {
                ..
            } => {
                1
                + 4 // fishing_skill_required: u32
            }
            _ => 1,
        }
    }
}

