use crate::tbc::Area;
use crate::tbc::SpellCastResult;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_CAST_FAILED {
    const OPCODE: u32 = 0x0130;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        // multiple_casts: Bool
        w.write_all(u8::from(self.multiple_casts).to_le_bytes().as_slice())?;

        match &self.result {
            SMSG_CAST_FAILED_SpellCastResult::AffectingCombat => {}
            SMSG_CAST_FAILED_SpellCastResult::AlreadyAtFullHealth => {}
            SMSG_CAST_FAILED_SpellCastResult::AlreadyAtFullMana => {}
            SMSG_CAST_FAILED_SpellCastResult::AlreadyAtFullPower => {}
            SMSG_CAST_FAILED_SpellCastResult::AlreadyBeingTamed => {}
            SMSG_CAST_FAILED_SpellCastResult::AlreadyHaveCharm => {}
            SMSG_CAST_FAILED_SpellCastResult::AlreadyHaveSummon => {}
            SMSG_CAST_FAILED_SpellCastResult::AlreadyOpen => {}
            SMSG_CAST_FAILED_SpellCastResult::AuraBounced => {}
            SMSG_CAST_FAILED_SpellCastResult::AutotrackInterrupted => {}
            SMSG_CAST_FAILED_SpellCastResult::BadImplicitTargets => {}
            SMSG_CAST_FAILED_SpellCastResult::BadTargets => {}
            SMSG_CAST_FAILED_SpellCastResult::CantBeCharmed => {}
            SMSG_CAST_FAILED_SpellCastResult::CantBeDisenchanted => {}
            SMSG_CAST_FAILED_SpellCastResult::CantBeDisenchantedSkill => {}
            SMSG_CAST_FAILED_SpellCastResult::CantBeProspected => {}
            SMSG_CAST_FAILED_SpellCastResult::CantCastOnTapped => {}
            SMSG_CAST_FAILED_SpellCastResult::CantDuelWhileInvisible => {}
            SMSG_CAST_FAILED_SpellCastResult::CantDuelWhileStealthed => {}
            SMSG_CAST_FAILED_SpellCastResult::CantStealth => {}
            SMSG_CAST_FAILED_SpellCastResult::CasterAurastate => {}
            SMSG_CAST_FAILED_SpellCastResult::CasterDead => {}
            SMSG_CAST_FAILED_SpellCastResult::Charmed => {}
            SMSG_CAST_FAILED_SpellCastResult::ChestInUse => {}
            SMSG_CAST_FAILED_SpellCastResult::Confused => {}
            SMSG_CAST_FAILED_SpellCastResult::DontReport => {}
            SMSG_CAST_FAILED_SpellCastResult::EquippedItem => {}
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
            SMSG_CAST_FAILED_SpellCastResult::EquippedItemClassMainhand => {}
            SMSG_CAST_FAILED_SpellCastResult::EquippedItemClassOffhand => {}
            SMSG_CAST_FAILED_SpellCastResult::ErrorX => {}
            SMSG_CAST_FAILED_SpellCastResult::Fizzle => {}
            SMSG_CAST_FAILED_SpellCastResult::Fleeing => {}
            SMSG_CAST_FAILED_SpellCastResult::FoodLowlevel => {}
            SMSG_CAST_FAILED_SpellCastResult::Highlevel => {}
            SMSG_CAST_FAILED_SpellCastResult::HungerSatiated => {}
            SMSG_CAST_FAILED_SpellCastResult::Immune => {}
            SMSG_CAST_FAILED_SpellCastResult::Interrupted => {}
            SMSG_CAST_FAILED_SpellCastResult::InterruptedCombat => {}
            SMSG_CAST_FAILED_SpellCastResult::ItemAlreadyEnchanted => {}
            SMSG_CAST_FAILED_SpellCastResult::ItemGone => {}
            SMSG_CAST_FAILED_SpellCastResult::ItemNotFound => {}
            SMSG_CAST_FAILED_SpellCastResult::ItemNotReady => {}
            SMSG_CAST_FAILED_SpellCastResult::LevelRequirement => {}
            SMSG_CAST_FAILED_SpellCastResult::LineOfSight => {}
            SMSG_CAST_FAILED_SpellCastResult::Lowlevel => {}
            SMSG_CAST_FAILED_SpellCastResult::LowCastlevel => {}
            SMSG_CAST_FAILED_SpellCastResult::MainhandEmpty => {}
            SMSG_CAST_FAILED_SpellCastResult::Moving => {}
            SMSG_CAST_FAILED_SpellCastResult::NeedAmmo => {}
            SMSG_CAST_FAILED_SpellCastResult::NeedAmmoPouch => {}
            SMSG_CAST_FAILED_SpellCastResult::NeedExoticAmmo => {}
            SMSG_CAST_FAILED_SpellCastResult::Nopath => {}
            SMSG_CAST_FAILED_SpellCastResult::NotBehind => {}
            SMSG_CAST_FAILED_SpellCastResult::NotFishable => {}
            SMSG_CAST_FAILED_SpellCastResult::NotFlying => {}
            SMSG_CAST_FAILED_SpellCastResult::NotHere => {}
            SMSG_CAST_FAILED_SpellCastResult::NotInfront => {}
            SMSG_CAST_FAILED_SpellCastResult::NotInControl => {}
            SMSG_CAST_FAILED_SpellCastResult::NotKnown => {}
            SMSG_CAST_FAILED_SpellCastResult::NotMounted => {}
            SMSG_CAST_FAILED_SpellCastResult::NotOnTaxi => {}
            SMSG_CAST_FAILED_SpellCastResult::NotOnTransport => {}
            SMSG_CAST_FAILED_SpellCastResult::NotReady => {}
            SMSG_CAST_FAILED_SpellCastResult::NotShapeshift => {}
            SMSG_CAST_FAILED_SpellCastResult::NotStanding => {}
            SMSG_CAST_FAILED_SpellCastResult::NotTradeable => {}
            SMSG_CAST_FAILED_SpellCastResult::NotTrading => {}
            SMSG_CAST_FAILED_SpellCastResult::NotUnsheathed => {}
            SMSG_CAST_FAILED_SpellCastResult::NotWhileGhost => {}
            SMSG_CAST_FAILED_SpellCastResult::NoAmmo => {}
            SMSG_CAST_FAILED_SpellCastResult::NoChargesRemain => {}
            SMSG_CAST_FAILED_SpellCastResult::NoChampion => {}
            SMSG_CAST_FAILED_SpellCastResult::NoComboPoints => {}
            SMSG_CAST_FAILED_SpellCastResult::NoDueling => {}
            SMSG_CAST_FAILED_SpellCastResult::NoEndurance => {}
            SMSG_CAST_FAILED_SpellCastResult::NoFish => {}
            SMSG_CAST_FAILED_SpellCastResult::NoItemsWhileShapeshifted => {}
            SMSG_CAST_FAILED_SpellCastResult::NoMountsAllowed => {}
            SMSG_CAST_FAILED_SpellCastResult::NoPet => {}
            SMSG_CAST_FAILED_SpellCastResult::NoPower => {}
            SMSG_CAST_FAILED_SpellCastResult::NothingToDispel => {}
            SMSG_CAST_FAILED_SpellCastResult::NothingToSteal => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyAbovewater => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyDaytime => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyIndoors => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyMounted => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyNighttime => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyOutdoors => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyShapeshift => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyStealthed => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyUnderwater => {}
            SMSG_CAST_FAILED_SpellCastResult::OutOfRange => {}
            SMSG_CAST_FAILED_SpellCastResult::Pacified => {}
            SMSG_CAST_FAILED_SpellCastResult::Possessed => {}
            SMSG_CAST_FAILED_SpellCastResult::Reagents => {}
            SMSG_CAST_FAILED_SpellCastResult::RequiresArea {
                area,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes())?;

            }
            SMSG_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                spell_focus,
            } => {
                // spell_focus: u32
                w.write_all(&spell_focus.to_le_bytes())?;

            }
            SMSG_CAST_FAILED_SpellCastResult::Rooted => {}
            SMSG_CAST_FAILED_SpellCastResult::Silenced => {}
            SMSG_CAST_FAILED_SpellCastResult::SpellInProgress => {}
            SMSG_CAST_FAILED_SpellCastResult::SpellLearned => {}
            SMSG_CAST_FAILED_SpellCastResult::SpellUnavailable => {}
            SMSG_CAST_FAILED_SpellCastResult::Stunned => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetsDead => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetAffectingCombat => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetAurastate => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetDueling => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetEnemy => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetEnraged => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetFriendly => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetInCombat => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetIsPlayer => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetIsPlayerControlled => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotDead => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotInParty => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotLooted => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotPlayer => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNoPockets => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNoWeapons => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetUnskinnable => {}
            SMSG_CAST_FAILED_SpellCastResult::ThirstSatiated => {}
            SMSG_CAST_FAILED_SpellCastResult::TooClose => {}
            SMSG_CAST_FAILED_SpellCastResult::TooManyOfItem => {}
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
            SMSG_CAST_FAILED_SpellCastResult::TrainingPoints => {}
            SMSG_CAST_FAILED_SpellCastResult::TryAgain => {}
            SMSG_CAST_FAILED_SpellCastResult::UnitNotBehind => {}
            SMSG_CAST_FAILED_SpellCastResult::UnitNotInfront => {}
            SMSG_CAST_FAILED_SpellCastResult::WrongPetFood => {}
            SMSG_CAST_FAILED_SpellCastResult::NotWhileFatigued => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotInInstance => {}
            SMSG_CAST_FAILED_SpellCastResult::NotWhileTrading => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotInRaid => {}
            SMSG_CAST_FAILED_SpellCastResult::DisenchantWhileLooting => {}
            SMSG_CAST_FAILED_SpellCastResult::ProspectWhileLooting => {}
            SMSG_CAST_FAILED_SpellCastResult::ProspectNeedMore => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetFreeforall => {}
            SMSG_CAST_FAILED_SpellCastResult::NoEdibleCorpses => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyBattlegrounds => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotGhost => {}
            SMSG_CAST_FAILED_SpellCastResult::TooManySkills => {}
            SMSG_CAST_FAILED_SpellCastResult::TransformUnusable => {}
            SMSG_CAST_FAILED_SpellCastResult::WrongWeather => {}
            SMSG_CAST_FAILED_SpellCastResult::DamageImmune => {}
            SMSG_CAST_FAILED_SpellCastResult::PreventedByMechanic => {}
            SMSG_CAST_FAILED_SpellCastResult::PlayTime => {}
            SMSG_CAST_FAILED_SpellCastResult::Reputation => {}
            SMSG_CAST_FAILED_SpellCastResult::MinSkill => {}
            SMSG_CAST_FAILED_SpellCastResult::NotInArena => {}
            SMSG_CAST_FAILED_SpellCastResult::NotOnShapeshift => {}
            SMSG_CAST_FAILED_SpellCastResult::NotOnStealthed => {}
            SMSG_CAST_FAILED_SpellCastResult::NotOnDamageImmune => {}
            SMSG_CAST_FAILED_SpellCastResult::NotOnMounted => {}
            SMSG_CAST_FAILED_SpellCastResult::TooShallow => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetNotInSanctuary => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetIsTrivial => {}
            SMSG_CAST_FAILED_SpellCastResult::BmOrInvisgod => {}
            SMSG_CAST_FAILED_SpellCastResult::ExpertRidingRequirement => {}
            SMSG_CAST_FAILED_SpellCastResult::ArtisanRidingRequirement => {}
            SMSG_CAST_FAILED_SpellCastResult::NotIdle => {}
            SMSG_CAST_FAILED_SpellCastResult::NotInactive => {}
            SMSG_CAST_FAILED_SpellCastResult::PartialPlaytime => {}
            SMSG_CAST_FAILED_SpellCastResult::NoPlaytime => {}
            SMSG_CAST_FAILED_SpellCastResult::NotInBattleground => {}
            SMSG_CAST_FAILED_SpellCastResult::OnlyInArena => {}
            SMSG_CAST_FAILED_SpellCastResult::TargetLockedToRaidInstance => {}
            SMSG_CAST_FAILED_SpellCastResult::Unknown => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=18).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0130, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        // multiple_casts: Bool
        let multiple_casts = crate::util::read_u8_le(r)? != 0;

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
                let item_class = crate::util::read_u32_le(r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(r)?;

                // item_inventory_type: u32
                let item_inventory_type = crate::util::read_u32_le(r)?;

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
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                SMSG_CAST_FAILED_SpellCastResult::RequiresArea {
                    area,
                }
            }
            SpellCastResult::RequiresSpellFocus => {
                // spell_focus: u32
                let spell_focus = crate::util::read_u32_le(r)?;

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
                        *i = crate::util::read_u32_le(r)?;
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
                        *i = crate::util::read_u32_le(r)?;
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
    pub(crate) fn size(&self) -> usize {
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
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::AffectingCombat => {
                1
            }
            Self::AlreadyAtFullHealth => {
                1
            }
            Self::AlreadyAtFullMana => {
                1
            }
            Self::AlreadyAtFullPower => {
                1
            }
            Self::AlreadyBeingTamed => {
                1
            }
            Self::AlreadyHaveCharm => {
                1
            }
            Self::AlreadyHaveSummon => {
                1
            }
            Self::AlreadyOpen => {
                1
            }
            Self::AuraBounced => {
                1
            }
            Self::AutotrackInterrupted => {
                1
            }
            Self::BadImplicitTargets => {
                1
            }
            Self::BadTargets => {
                1
            }
            Self::CantBeCharmed => {
                1
            }
            Self::CantBeDisenchanted => {
                1
            }
            Self::CantBeDisenchantedSkill => {
                1
            }
            Self::CantBeProspected => {
                1
            }
            Self::CantCastOnTapped => {
                1
            }
            Self::CantDuelWhileInvisible => {
                1
            }
            Self::CantDuelWhileStealthed => {
                1
            }
            Self::CantStealth => {
                1
            }
            Self::CasterAurastate => {
                1
            }
            Self::CasterDead => {
                1
            }
            Self::Charmed => {
                1
            }
            Self::ChestInUse => {
                1
            }
            Self::Confused => {
                1
            }
            Self::DontReport => {
                1
            }
            Self::EquippedItem => {
                1
            }
            Self::EquippedItemClass {
                item_class,
                item_inventory_type,
                item_sub_class,
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_inventory_type: u32
                + 4 // item_sub_class: u32
            }
            Self::EquippedItemClassMainhand => {
                1
            }
            Self::EquippedItemClassOffhand => {
                1
            }
            Self::ErrorX => {
                1
            }
            Self::Fizzle => {
                1
            }
            Self::Fleeing => {
                1
            }
            Self::FoodLowlevel => {
                1
            }
            Self::Highlevel => {
                1
            }
            Self::HungerSatiated => {
                1
            }
            Self::Immune => {
                1
            }
            Self::Interrupted => {
                1
            }
            Self::InterruptedCombat => {
                1
            }
            Self::ItemAlreadyEnchanted => {
                1
            }
            Self::ItemGone => {
                1
            }
            Self::ItemNotFound => {
                1
            }
            Self::ItemNotReady => {
                1
            }
            Self::LevelRequirement => {
                1
            }
            Self::LineOfSight => {
                1
            }
            Self::Lowlevel => {
                1
            }
            Self::LowCastlevel => {
                1
            }
            Self::MainhandEmpty => {
                1
            }
            Self::Moving => {
                1
            }
            Self::NeedAmmo => {
                1
            }
            Self::NeedAmmoPouch => {
                1
            }
            Self::NeedExoticAmmo => {
                1
            }
            Self::Nopath => {
                1
            }
            Self::NotBehind => {
                1
            }
            Self::NotFishable => {
                1
            }
            Self::NotFlying => {
                1
            }
            Self::NotHere => {
                1
            }
            Self::NotInfront => {
                1
            }
            Self::NotInControl => {
                1
            }
            Self::NotKnown => {
                1
            }
            Self::NotMounted => {
                1
            }
            Self::NotOnTaxi => {
                1
            }
            Self::NotOnTransport => {
                1
            }
            Self::NotReady => {
                1
            }
            Self::NotShapeshift => {
                1
            }
            Self::NotStanding => {
                1
            }
            Self::NotTradeable => {
                1
            }
            Self::NotTrading => {
                1
            }
            Self::NotUnsheathed => {
                1
            }
            Self::NotWhileGhost => {
                1
            }
            Self::NoAmmo => {
                1
            }
            Self::NoChargesRemain => {
                1
            }
            Self::NoChampion => {
                1
            }
            Self::NoComboPoints => {
                1
            }
            Self::NoDueling => {
                1
            }
            Self::NoEndurance => {
                1
            }
            Self::NoFish => {
                1
            }
            Self::NoItemsWhileShapeshifted => {
                1
            }
            Self::NoMountsAllowed => {
                1
            }
            Self::NoPet => {
                1
            }
            Self::NoPower => {
                1
            }
            Self::NothingToDispel => {
                1
            }
            Self::NothingToSteal => {
                1
            }
            Self::OnlyAbovewater => {
                1
            }
            Self::OnlyDaytime => {
                1
            }
            Self::OnlyIndoors => {
                1
            }
            Self::OnlyMounted => {
                1
            }
            Self::OnlyNighttime => {
                1
            }
            Self::OnlyOutdoors => {
                1
            }
            Self::OnlyShapeshift => {
                1
            }
            Self::OnlyStealthed => {
                1
            }
            Self::OnlyUnderwater => {
                1
            }
            Self::OutOfRange => {
                1
            }
            Self::Pacified => {
                1
            }
            Self::Possessed => {
                1
            }
            Self::Reagents => {
                1
            }
            Self::RequiresArea {
                area,
            } => {
                1
                + 4 // area: Area
            }
            Self::RequiresSpellFocus {
                spell_focus,
            } => {
                1
                + 4 // spell_focus: u32
            }
            Self::Rooted => {
                1
            }
            Self::Silenced => {
                1
            }
            Self::SpellInProgress => {
                1
            }
            Self::SpellLearned => {
                1
            }
            Self::SpellUnavailable => {
                1
            }
            Self::Stunned => {
                1
            }
            Self::TargetsDead => {
                1
            }
            Self::TargetAffectingCombat => {
                1
            }
            Self::TargetAurastate => {
                1
            }
            Self::TargetDueling => {
                1
            }
            Self::TargetEnemy => {
                1
            }
            Self::TargetEnraged => {
                1
            }
            Self::TargetFriendly => {
                1
            }
            Self::TargetInCombat => {
                1
            }
            Self::TargetIsPlayer => {
                1
            }
            Self::TargetIsPlayerControlled => {
                1
            }
            Self::TargetNotDead => {
                1
            }
            Self::TargetNotInParty => {
                1
            }
            Self::TargetNotLooted => {
                1
            }
            Self::TargetNotPlayer => {
                1
            }
            Self::TargetNoPockets => {
                1
            }
            Self::TargetNoWeapons => {
                1
            }
            Self::TargetUnskinnable => {
                1
            }
            Self::ThirstSatiated => {
                1
            }
            Self::TooClose => {
                1
            }
            Self::TooManyOfItem => {
                1
            }
            Self::TotemCategory {
                totem_categories,
            } => {
                1
                + 2 * core::mem::size_of::<u32>() // totem_categories: u32[2]
            }
            Self::Totems {
                totems,
            } => {
                1
                + 2 * core::mem::size_of::<u32>() // totems: u32[2]
            }
            Self::TrainingPoints => {
                1
            }
            Self::TryAgain => {
                1
            }
            Self::UnitNotBehind => {
                1
            }
            Self::UnitNotInfront => {
                1
            }
            Self::WrongPetFood => {
                1
            }
            Self::NotWhileFatigued => {
                1
            }
            Self::TargetNotInInstance => {
                1
            }
            Self::NotWhileTrading => {
                1
            }
            Self::TargetNotInRaid => {
                1
            }
            Self::DisenchantWhileLooting => {
                1
            }
            Self::ProspectWhileLooting => {
                1
            }
            Self::ProspectNeedMore => {
                1
            }
            Self::TargetFreeforall => {
                1
            }
            Self::NoEdibleCorpses => {
                1
            }
            Self::OnlyBattlegrounds => {
                1
            }
            Self::TargetNotGhost => {
                1
            }
            Self::TooManySkills => {
                1
            }
            Self::TransformUnusable => {
                1
            }
            Self::WrongWeather => {
                1
            }
            Self::DamageImmune => {
                1
            }
            Self::PreventedByMechanic => {
                1
            }
            Self::PlayTime => {
                1
            }
            Self::Reputation => {
                1
            }
            Self::MinSkill => {
                1
            }
            Self::NotInArena => {
                1
            }
            Self::NotOnShapeshift => {
                1
            }
            Self::NotOnStealthed => {
                1
            }
            Self::NotOnDamageImmune => {
                1
            }
            Self::NotOnMounted => {
                1
            }
            Self::TooShallow => {
                1
            }
            Self::TargetNotInSanctuary => {
                1
            }
            Self::TargetIsTrivial => {
                1
            }
            Self::BmOrInvisgod => {
                1
            }
            Self::ExpertRidingRequirement => {
                1
            }
            Self::ArtisanRidingRequirement => {
                1
            }
            Self::NotIdle => {
                1
            }
            Self::NotInactive => {
                1
            }
            Self::PartialPlaytime => {
                1
            }
            Self::NoPlaytime => {
                1
            }
            Self::NotInBattleground => {
                1
            }
            Self::OnlyInArena => {
                1
            }
            Self::TargetLockedToRaidInstance => {
                1
            }
            Self::Unknown => {
                1
            }
        }
    }
}

