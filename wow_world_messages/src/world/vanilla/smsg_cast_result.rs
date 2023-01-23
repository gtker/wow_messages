use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Area;
use crate::world::vanilla::CastFailureReason;
use crate::world::vanilla::SimpleSpellCastResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cast_result.wowm:449`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cast_result.wowm#L449):
/// ```text
/// smsg SMSG_CAST_RESULT = 0x0130 {
///     u32 spell;
///     SimpleSpellCastResult result;
///     if (result != FAILURE) {
///         CastFailureReason reason;
///         if (reason == REQUIRES_SPELL_FOCUS) {
///             u32 required_spell_focus;
///         }
///         else if (reason == REQUIRES_AREA) {
///             Area area;
///         }
///         else if (reason == EQUIPPED_ITEM_CLASS) {
///             u32 equipped_item_class;
///             u32 equipped_item_subclass_mask;
///             u32 equipped_item_inventory_type_mask;
///         }
///     }
/// }
/// ```
pub struct SMSG_CAST_RESULT {
    pub spell: u32,
    pub result: SMSG_CAST_RESULT_SimpleSpellCastResult,
}

impl crate::Message for SMSG_CAST_RESULT {
    const OPCODE: u32 = 0x0130;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // result: SimpleSpellCastResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_CAST_RESULT_SimpleSpellCastResult::Success {
                reason,
            } => {
                // reason: CastFailureReason
                w.write_all(&(reason.as_int() as u8).to_le_bytes())?;

                match &reason {
                    SMSG_CAST_RESULT_CastFailureReason::AffectingCombat => {}
                    SMSG_CAST_RESULT_CastFailureReason::AlreadyAtFullHealth => {}
                    SMSG_CAST_RESULT_CastFailureReason::AlreadyAtFullPower => {}
                    SMSG_CAST_RESULT_CastFailureReason::AlreadyBeingTamed => {}
                    SMSG_CAST_RESULT_CastFailureReason::AlreadyHaveCharm => {}
                    SMSG_CAST_RESULT_CastFailureReason::AlreadyHaveSummon => {}
                    SMSG_CAST_RESULT_CastFailureReason::AlreadyOpen => {}
                    SMSG_CAST_RESULT_CastFailureReason::AuraBounced => {}
                    SMSG_CAST_RESULT_CastFailureReason::AutotrackInterrupted => {}
                    SMSG_CAST_RESULT_CastFailureReason::BadImplicitTargets => {}
                    SMSG_CAST_RESULT_CastFailureReason::BadTargets => {}
                    SMSG_CAST_RESULT_CastFailureReason::CantBeCharmed => {}
                    SMSG_CAST_RESULT_CastFailureReason::CantBeDisenchanted => {}
                    SMSG_CAST_RESULT_CastFailureReason::CantBeProspected => {}
                    SMSG_CAST_RESULT_CastFailureReason::CantCastOnTapped => {}
                    SMSG_CAST_RESULT_CastFailureReason::CantDuelWhileInvisible => {}
                    SMSG_CAST_RESULT_CastFailureReason::CantDuelWhileStealthed => {}
                    SMSG_CAST_RESULT_CastFailureReason::CantStealth => {}
                    SMSG_CAST_RESULT_CastFailureReason::CasterAurastate => {}
                    SMSG_CAST_RESULT_CastFailureReason::CasterDead => {}
                    SMSG_CAST_RESULT_CastFailureReason::Charmed => {}
                    SMSG_CAST_RESULT_CastFailureReason::ChestInUse => {}
                    SMSG_CAST_RESULT_CastFailureReason::Confused => {}
                    SMSG_CAST_RESULT_CastFailureReason::DontReport => {}
                    SMSG_CAST_RESULT_CastFailureReason::EquippedItem => {}
                    SMSG_CAST_RESULT_CastFailureReason::EquippedItemClass {
                        equipped_item_class,
                        equipped_item_inventory_type_mask,
                        equipped_item_subclass_mask,
                    } => {
                        // equipped_item_class: u32
                        w.write_all(&equipped_item_class.to_le_bytes())?;

                        // equipped_item_subclass_mask: u32
                        w.write_all(&equipped_item_subclass_mask.to_le_bytes())?;

                        // equipped_item_inventory_type_mask: u32
                        w.write_all(&equipped_item_inventory_type_mask.to_le_bytes())?;

                    }
                    SMSG_CAST_RESULT_CastFailureReason::EquippedItemClassMainhand => {}
                    SMSG_CAST_RESULT_CastFailureReason::EquippedItemClassOffhand => {}
                    SMSG_CAST_RESULT_CastFailureReason::ErrorX => {}
                    SMSG_CAST_RESULT_CastFailureReason::Fizzle => {}
                    SMSG_CAST_RESULT_CastFailureReason::Fleeing => {}
                    SMSG_CAST_RESULT_CastFailureReason::FoodLowlevel => {}
                    SMSG_CAST_RESULT_CastFailureReason::Highlevel => {}
                    SMSG_CAST_RESULT_CastFailureReason::HungerSatiated => {}
                    SMSG_CAST_RESULT_CastFailureReason::Immune => {}
                    SMSG_CAST_RESULT_CastFailureReason::Interrupted => {}
                    SMSG_CAST_RESULT_CastFailureReason::InterruptedCombat => {}
                    SMSG_CAST_RESULT_CastFailureReason::ItemAlreadyEnchanted => {}
                    SMSG_CAST_RESULT_CastFailureReason::ItemGone => {}
                    SMSG_CAST_RESULT_CastFailureReason::ItemNotFound => {}
                    SMSG_CAST_RESULT_CastFailureReason::ItemNotReady => {}
                    SMSG_CAST_RESULT_CastFailureReason::LevelRequirement => {}
                    SMSG_CAST_RESULT_CastFailureReason::LineOfSight => {}
                    SMSG_CAST_RESULT_CastFailureReason::Lowlevel => {}
                    SMSG_CAST_RESULT_CastFailureReason::LowCastlevel => {}
                    SMSG_CAST_RESULT_CastFailureReason::MainhandEmpty => {}
                    SMSG_CAST_RESULT_CastFailureReason::Moving => {}
                    SMSG_CAST_RESULT_CastFailureReason::NeedAmmo => {}
                    SMSG_CAST_RESULT_CastFailureReason::NeedAmmoPouch => {}
                    SMSG_CAST_RESULT_CastFailureReason::NeedExoticAmmo => {}
                    SMSG_CAST_RESULT_CastFailureReason::Nopath => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotBehind => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotFishable => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotHere => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotInfront => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotInControl => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotKnown => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotMounted => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotOnTaxi => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotOnTransport => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotReady => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotShapeshift => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotStanding => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotTradeable => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotTrading => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotUnsheathed => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotWhileGhost => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoAmmo => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoChargesRemain => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoChampion => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoComboPoints => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoDueling => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoEndurance => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoFish => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoItemsWhileShapeshifted => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoMountsAllowed => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoPet => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoPower => {}
                    SMSG_CAST_RESULT_CastFailureReason::NothingToDispel => {}
                    SMSG_CAST_RESULT_CastFailureReason::NothingToSteal => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyAbovewater => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyDaytime => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyIndoors => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyMounted => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyNighttime => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyOutdoors => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyShapeshift => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyStealthed => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyUnderwater => {}
                    SMSG_CAST_RESULT_CastFailureReason::OutOfRange => {}
                    SMSG_CAST_RESULT_CastFailureReason::Pacified => {}
                    SMSG_CAST_RESULT_CastFailureReason::Possessed => {}
                    SMSG_CAST_RESULT_CastFailureReason::Reagents => {}
                    SMSG_CAST_RESULT_CastFailureReason::RequiresArea {
                        area,
                    } => {
                        // area: Area
                        w.write_all(&(area.as_int() as u32).to_le_bytes())?;

                    }
                    SMSG_CAST_RESULT_CastFailureReason::RequiresSpellFocus {
                        required_spell_focus,
                    } => {
                        // required_spell_focus: u32
                        w.write_all(&required_spell_focus.to_le_bytes())?;

                    }
                    SMSG_CAST_RESULT_CastFailureReason::Rooted => {}
                    SMSG_CAST_RESULT_CastFailureReason::Silenced => {}
                    SMSG_CAST_RESULT_CastFailureReason::SpellInProgress => {}
                    SMSG_CAST_RESULT_CastFailureReason::SpellLearned => {}
                    SMSG_CAST_RESULT_CastFailureReason::SpellUnavailable => {}
                    SMSG_CAST_RESULT_CastFailureReason::Stunned => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetsDead => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetAffectingCombat => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetAurastate => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetDueling => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetEnemy => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetEnraged => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetFriendly => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetInCombat => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetIsPlayer => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNotDead => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNotInParty => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNotLooted => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNotPlayer => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNoPockets => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNoWeapons => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetUnskinnable => {}
                    SMSG_CAST_RESULT_CastFailureReason::ThirstSatiated => {}
                    SMSG_CAST_RESULT_CastFailureReason::TooClose => {}
                    SMSG_CAST_RESULT_CastFailureReason::TooManyOfItem => {}
                    SMSG_CAST_RESULT_CastFailureReason::Totems => {}
                    SMSG_CAST_RESULT_CastFailureReason::TrainingPoints => {}
                    SMSG_CAST_RESULT_CastFailureReason::TryAgain => {}
                    SMSG_CAST_RESULT_CastFailureReason::UnitNotBehind => {}
                    SMSG_CAST_RESULT_CastFailureReason::UnitNotInfront => {}
                    SMSG_CAST_RESULT_CastFailureReason::WrongPetFood => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotWhileFatigued => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNotInInstance => {}
                    SMSG_CAST_RESULT_CastFailureReason::NotWhileTrading => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNotInRaid => {}
                    SMSG_CAST_RESULT_CastFailureReason::DisenchantWhileLooting => {}
                    SMSG_CAST_RESULT_CastFailureReason::ProspectWhileLooting => {}
                    SMSG_CAST_RESULT_CastFailureReason::ProspectNeedMore => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetFreeforall => {}
                    SMSG_CAST_RESULT_CastFailureReason::NoEdibleCorpses => {}
                    SMSG_CAST_RESULT_CastFailureReason::OnlyBattlegrounds => {}
                    SMSG_CAST_RESULT_CastFailureReason::TargetNotGhost => {}
                    SMSG_CAST_RESULT_CastFailureReason::TooManySkills => {}
                    SMSG_CAST_RESULT_CastFailureReason::TransformUnusable => {}
                    SMSG_CAST_RESULT_CastFailureReason::WrongWeather => {}
                    SMSG_CAST_RESULT_CastFailureReason::DamageImmune => {}
                    SMSG_CAST_RESULT_CastFailureReason::PreventedByMechanic => {}
                    SMSG_CAST_RESULT_CastFailureReason::PlayTime => {}
                    SMSG_CAST_RESULT_CastFailureReason::Reputation => {}
                    SMSG_CAST_RESULT_CastFailureReason::MinSkill => {}
                    SMSG_CAST_RESULT_CastFailureReason::Unknown => {}
                }

            }
            SMSG_CAST_RESULT_SimpleSpellCastResult::Failure => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=18).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0130, size: body_size as u32 });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // result: SimpleSpellCastResult
        let result: SimpleSpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            SimpleSpellCastResult::Success => {
                // reason: CastFailureReason
                let reason: CastFailureReason = crate::util::read_u8_le(r)?.try_into()?;

                let reason_if = match reason {
                    CastFailureReason::AffectingCombat => SMSG_CAST_RESULT_CastFailureReason::AffectingCombat,
                    CastFailureReason::AlreadyAtFullHealth => SMSG_CAST_RESULT_CastFailureReason::AlreadyAtFullHealth,
                    CastFailureReason::AlreadyAtFullPower => SMSG_CAST_RESULT_CastFailureReason::AlreadyAtFullPower,
                    CastFailureReason::AlreadyBeingTamed => SMSG_CAST_RESULT_CastFailureReason::AlreadyBeingTamed,
                    CastFailureReason::AlreadyHaveCharm => SMSG_CAST_RESULT_CastFailureReason::AlreadyHaveCharm,
                    CastFailureReason::AlreadyHaveSummon => SMSG_CAST_RESULT_CastFailureReason::AlreadyHaveSummon,
                    CastFailureReason::AlreadyOpen => SMSG_CAST_RESULT_CastFailureReason::AlreadyOpen,
                    CastFailureReason::AuraBounced => SMSG_CAST_RESULT_CastFailureReason::AuraBounced,
                    CastFailureReason::AutotrackInterrupted => SMSG_CAST_RESULT_CastFailureReason::AutotrackInterrupted,
                    CastFailureReason::BadImplicitTargets => SMSG_CAST_RESULT_CastFailureReason::BadImplicitTargets,
                    CastFailureReason::BadTargets => SMSG_CAST_RESULT_CastFailureReason::BadTargets,
                    CastFailureReason::CantBeCharmed => SMSG_CAST_RESULT_CastFailureReason::CantBeCharmed,
                    CastFailureReason::CantBeDisenchanted => SMSG_CAST_RESULT_CastFailureReason::CantBeDisenchanted,
                    CastFailureReason::CantBeProspected => SMSG_CAST_RESULT_CastFailureReason::CantBeProspected,
                    CastFailureReason::CantCastOnTapped => SMSG_CAST_RESULT_CastFailureReason::CantCastOnTapped,
                    CastFailureReason::CantDuelWhileInvisible => SMSG_CAST_RESULT_CastFailureReason::CantDuelWhileInvisible,
                    CastFailureReason::CantDuelWhileStealthed => SMSG_CAST_RESULT_CastFailureReason::CantDuelWhileStealthed,
                    CastFailureReason::CantStealth => SMSG_CAST_RESULT_CastFailureReason::CantStealth,
                    CastFailureReason::CasterAurastate => SMSG_CAST_RESULT_CastFailureReason::CasterAurastate,
                    CastFailureReason::CasterDead => SMSG_CAST_RESULT_CastFailureReason::CasterDead,
                    CastFailureReason::Charmed => SMSG_CAST_RESULT_CastFailureReason::Charmed,
                    CastFailureReason::ChestInUse => SMSG_CAST_RESULT_CastFailureReason::ChestInUse,
                    CastFailureReason::Confused => SMSG_CAST_RESULT_CastFailureReason::Confused,
                    CastFailureReason::DontReport => SMSG_CAST_RESULT_CastFailureReason::DontReport,
                    CastFailureReason::EquippedItem => SMSG_CAST_RESULT_CastFailureReason::EquippedItem,
                    CastFailureReason::EquippedItemClass => {
                        // equipped_item_class: u32
                        let equipped_item_class = crate::util::read_u32_le(r)?;

                        // equipped_item_subclass_mask: u32
                        let equipped_item_subclass_mask = crate::util::read_u32_le(r)?;

                        // equipped_item_inventory_type_mask: u32
                        let equipped_item_inventory_type_mask = crate::util::read_u32_le(r)?;

                        SMSG_CAST_RESULT_CastFailureReason::EquippedItemClass {
                            equipped_item_class,
                            equipped_item_inventory_type_mask,
                            equipped_item_subclass_mask,
                        }
                    }
                    CastFailureReason::EquippedItemClassMainhand => SMSG_CAST_RESULT_CastFailureReason::EquippedItemClassMainhand,
                    CastFailureReason::EquippedItemClassOffhand => SMSG_CAST_RESULT_CastFailureReason::EquippedItemClassOffhand,
                    CastFailureReason::ErrorX => SMSG_CAST_RESULT_CastFailureReason::ErrorX,
                    CastFailureReason::Fizzle => SMSG_CAST_RESULT_CastFailureReason::Fizzle,
                    CastFailureReason::Fleeing => SMSG_CAST_RESULT_CastFailureReason::Fleeing,
                    CastFailureReason::FoodLowlevel => SMSG_CAST_RESULT_CastFailureReason::FoodLowlevel,
                    CastFailureReason::Highlevel => SMSG_CAST_RESULT_CastFailureReason::Highlevel,
                    CastFailureReason::HungerSatiated => SMSG_CAST_RESULT_CastFailureReason::HungerSatiated,
                    CastFailureReason::Immune => SMSG_CAST_RESULT_CastFailureReason::Immune,
                    CastFailureReason::Interrupted => SMSG_CAST_RESULT_CastFailureReason::Interrupted,
                    CastFailureReason::InterruptedCombat => SMSG_CAST_RESULT_CastFailureReason::InterruptedCombat,
                    CastFailureReason::ItemAlreadyEnchanted => SMSG_CAST_RESULT_CastFailureReason::ItemAlreadyEnchanted,
                    CastFailureReason::ItemGone => SMSG_CAST_RESULT_CastFailureReason::ItemGone,
                    CastFailureReason::ItemNotFound => SMSG_CAST_RESULT_CastFailureReason::ItemNotFound,
                    CastFailureReason::ItemNotReady => SMSG_CAST_RESULT_CastFailureReason::ItemNotReady,
                    CastFailureReason::LevelRequirement => SMSG_CAST_RESULT_CastFailureReason::LevelRequirement,
                    CastFailureReason::LineOfSight => SMSG_CAST_RESULT_CastFailureReason::LineOfSight,
                    CastFailureReason::Lowlevel => SMSG_CAST_RESULT_CastFailureReason::Lowlevel,
                    CastFailureReason::LowCastlevel => SMSG_CAST_RESULT_CastFailureReason::LowCastlevel,
                    CastFailureReason::MainhandEmpty => SMSG_CAST_RESULT_CastFailureReason::MainhandEmpty,
                    CastFailureReason::Moving => SMSG_CAST_RESULT_CastFailureReason::Moving,
                    CastFailureReason::NeedAmmo => SMSG_CAST_RESULT_CastFailureReason::NeedAmmo,
                    CastFailureReason::NeedAmmoPouch => SMSG_CAST_RESULT_CastFailureReason::NeedAmmoPouch,
                    CastFailureReason::NeedExoticAmmo => SMSG_CAST_RESULT_CastFailureReason::NeedExoticAmmo,
                    CastFailureReason::Nopath => SMSG_CAST_RESULT_CastFailureReason::Nopath,
                    CastFailureReason::NotBehind => SMSG_CAST_RESULT_CastFailureReason::NotBehind,
                    CastFailureReason::NotFishable => SMSG_CAST_RESULT_CastFailureReason::NotFishable,
                    CastFailureReason::NotHere => SMSG_CAST_RESULT_CastFailureReason::NotHere,
                    CastFailureReason::NotInfront => SMSG_CAST_RESULT_CastFailureReason::NotInfront,
                    CastFailureReason::NotInControl => SMSG_CAST_RESULT_CastFailureReason::NotInControl,
                    CastFailureReason::NotKnown => SMSG_CAST_RESULT_CastFailureReason::NotKnown,
                    CastFailureReason::NotMounted => SMSG_CAST_RESULT_CastFailureReason::NotMounted,
                    CastFailureReason::NotOnTaxi => SMSG_CAST_RESULT_CastFailureReason::NotOnTaxi,
                    CastFailureReason::NotOnTransport => SMSG_CAST_RESULT_CastFailureReason::NotOnTransport,
                    CastFailureReason::NotReady => SMSG_CAST_RESULT_CastFailureReason::NotReady,
                    CastFailureReason::NotShapeshift => SMSG_CAST_RESULT_CastFailureReason::NotShapeshift,
                    CastFailureReason::NotStanding => SMSG_CAST_RESULT_CastFailureReason::NotStanding,
                    CastFailureReason::NotTradeable => SMSG_CAST_RESULT_CastFailureReason::NotTradeable,
                    CastFailureReason::NotTrading => SMSG_CAST_RESULT_CastFailureReason::NotTrading,
                    CastFailureReason::NotUnsheathed => SMSG_CAST_RESULT_CastFailureReason::NotUnsheathed,
                    CastFailureReason::NotWhileGhost => SMSG_CAST_RESULT_CastFailureReason::NotWhileGhost,
                    CastFailureReason::NoAmmo => SMSG_CAST_RESULT_CastFailureReason::NoAmmo,
                    CastFailureReason::NoChargesRemain => SMSG_CAST_RESULT_CastFailureReason::NoChargesRemain,
                    CastFailureReason::NoChampion => SMSG_CAST_RESULT_CastFailureReason::NoChampion,
                    CastFailureReason::NoComboPoints => SMSG_CAST_RESULT_CastFailureReason::NoComboPoints,
                    CastFailureReason::NoDueling => SMSG_CAST_RESULT_CastFailureReason::NoDueling,
                    CastFailureReason::NoEndurance => SMSG_CAST_RESULT_CastFailureReason::NoEndurance,
                    CastFailureReason::NoFish => SMSG_CAST_RESULT_CastFailureReason::NoFish,
                    CastFailureReason::NoItemsWhileShapeshifted => SMSG_CAST_RESULT_CastFailureReason::NoItemsWhileShapeshifted,
                    CastFailureReason::NoMountsAllowed => SMSG_CAST_RESULT_CastFailureReason::NoMountsAllowed,
                    CastFailureReason::NoPet => SMSG_CAST_RESULT_CastFailureReason::NoPet,
                    CastFailureReason::NoPower => SMSG_CAST_RESULT_CastFailureReason::NoPower,
                    CastFailureReason::NothingToDispel => SMSG_CAST_RESULT_CastFailureReason::NothingToDispel,
                    CastFailureReason::NothingToSteal => SMSG_CAST_RESULT_CastFailureReason::NothingToSteal,
                    CastFailureReason::OnlyAbovewater => SMSG_CAST_RESULT_CastFailureReason::OnlyAbovewater,
                    CastFailureReason::OnlyDaytime => SMSG_CAST_RESULT_CastFailureReason::OnlyDaytime,
                    CastFailureReason::OnlyIndoors => SMSG_CAST_RESULT_CastFailureReason::OnlyIndoors,
                    CastFailureReason::OnlyMounted => SMSG_CAST_RESULT_CastFailureReason::OnlyMounted,
                    CastFailureReason::OnlyNighttime => SMSG_CAST_RESULT_CastFailureReason::OnlyNighttime,
                    CastFailureReason::OnlyOutdoors => SMSG_CAST_RESULT_CastFailureReason::OnlyOutdoors,
                    CastFailureReason::OnlyShapeshift => SMSG_CAST_RESULT_CastFailureReason::OnlyShapeshift,
                    CastFailureReason::OnlyStealthed => SMSG_CAST_RESULT_CastFailureReason::OnlyStealthed,
                    CastFailureReason::OnlyUnderwater => SMSG_CAST_RESULT_CastFailureReason::OnlyUnderwater,
                    CastFailureReason::OutOfRange => SMSG_CAST_RESULT_CastFailureReason::OutOfRange,
                    CastFailureReason::Pacified => SMSG_CAST_RESULT_CastFailureReason::Pacified,
                    CastFailureReason::Possessed => SMSG_CAST_RESULT_CastFailureReason::Possessed,
                    CastFailureReason::Reagents => SMSG_CAST_RESULT_CastFailureReason::Reagents,
                    CastFailureReason::RequiresArea => {
                        // area: Area
                        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                        SMSG_CAST_RESULT_CastFailureReason::RequiresArea {
                            area,
                        }
                    }
                    CastFailureReason::RequiresSpellFocus => {
                        // required_spell_focus: u32
                        let required_spell_focus = crate::util::read_u32_le(r)?;

                        SMSG_CAST_RESULT_CastFailureReason::RequiresSpellFocus {
                            required_spell_focus,
                        }
                    }
                    CastFailureReason::Rooted => SMSG_CAST_RESULT_CastFailureReason::Rooted,
                    CastFailureReason::Silenced => SMSG_CAST_RESULT_CastFailureReason::Silenced,
                    CastFailureReason::SpellInProgress => SMSG_CAST_RESULT_CastFailureReason::SpellInProgress,
                    CastFailureReason::SpellLearned => SMSG_CAST_RESULT_CastFailureReason::SpellLearned,
                    CastFailureReason::SpellUnavailable => SMSG_CAST_RESULT_CastFailureReason::SpellUnavailable,
                    CastFailureReason::Stunned => SMSG_CAST_RESULT_CastFailureReason::Stunned,
                    CastFailureReason::TargetsDead => SMSG_CAST_RESULT_CastFailureReason::TargetsDead,
                    CastFailureReason::TargetAffectingCombat => SMSG_CAST_RESULT_CastFailureReason::TargetAffectingCombat,
                    CastFailureReason::TargetAurastate => SMSG_CAST_RESULT_CastFailureReason::TargetAurastate,
                    CastFailureReason::TargetDueling => SMSG_CAST_RESULT_CastFailureReason::TargetDueling,
                    CastFailureReason::TargetEnemy => SMSG_CAST_RESULT_CastFailureReason::TargetEnemy,
                    CastFailureReason::TargetEnraged => SMSG_CAST_RESULT_CastFailureReason::TargetEnraged,
                    CastFailureReason::TargetFriendly => SMSG_CAST_RESULT_CastFailureReason::TargetFriendly,
                    CastFailureReason::TargetInCombat => SMSG_CAST_RESULT_CastFailureReason::TargetInCombat,
                    CastFailureReason::TargetIsPlayer => SMSG_CAST_RESULT_CastFailureReason::TargetIsPlayer,
                    CastFailureReason::TargetNotDead => SMSG_CAST_RESULT_CastFailureReason::TargetNotDead,
                    CastFailureReason::TargetNotInParty => SMSG_CAST_RESULT_CastFailureReason::TargetNotInParty,
                    CastFailureReason::TargetNotLooted => SMSG_CAST_RESULT_CastFailureReason::TargetNotLooted,
                    CastFailureReason::TargetNotPlayer => SMSG_CAST_RESULT_CastFailureReason::TargetNotPlayer,
                    CastFailureReason::TargetNoPockets => SMSG_CAST_RESULT_CastFailureReason::TargetNoPockets,
                    CastFailureReason::TargetNoWeapons => SMSG_CAST_RESULT_CastFailureReason::TargetNoWeapons,
                    CastFailureReason::TargetUnskinnable => SMSG_CAST_RESULT_CastFailureReason::TargetUnskinnable,
                    CastFailureReason::ThirstSatiated => SMSG_CAST_RESULT_CastFailureReason::ThirstSatiated,
                    CastFailureReason::TooClose => SMSG_CAST_RESULT_CastFailureReason::TooClose,
                    CastFailureReason::TooManyOfItem => SMSG_CAST_RESULT_CastFailureReason::TooManyOfItem,
                    CastFailureReason::Totems => SMSG_CAST_RESULT_CastFailureReason::Totems,
                    CastFailureReason::TrainingPoints => SMSG_CAST_RESULT_CastFailureReason::TrainingPoints,
                    CastFailureReason::TryAgain => SMSG_CAST_RESULT_CastFailureReason::TryAgain,
                    CastFailureReason::UnitNotBehind => SMSG_CAST_RESULT_CastFailureReason::UnitNotBehind,
                    CastFailureReason::UnitNotInfront => SMSG_CAST_RESULT_CastFailureReason::UnitNotInfront,
                    CastFailureReason::WrongPetFood => SMSG_CAST_RESULT_CastFailureReason::WrongPetFood,
                    CastFailureReason::NotWhileFatigued => SMSG_CAST_RESULT_CastFailureReason::NotWhileFatigued,
                    CastFailureReason::TargetNotInInstance => SMSG_CAST_RESULT_CastFailureReason::TargetNotInInstance,
                    CastFailureReason::NotWhileTrading => SMSG_CAST_RESULT_CastFailureReason::NotWhileTrading,
                    CastFailureReason::TargetNotInRaid => SMSG_CAST_RESULT_CastFailureReason::TargetNotInRaid,
                    CastFailureReason::DisenchantWhileLooting => SMSG_CAST_RESULT_CastFailureReason::DisenchantWhileLooting,
                    CastFailureReason::ProspectWhileLooting => SMSG_CAST_RESULT_CastFailureReason::ProspectWhileLooting,
                    CastFailureReason::ProspectNeedMore => SMSG_CAST_RESULT_CastFailureReason::ProspectNeedMore,
                    CastFailureReason::TargetFreeforall => SMSG_CAST_RESULT_CastFailureReason::TargetFreeforall,
                    CastFailureReason::NoEdibleCorpses => SMSG_CAST_RESULT_CastFailureReason::NoEdibleCorpses,
                    CastFailureReason::OnlyBattlegrounds => SMSG_CAST_RESULT_CastFailureReason::OnlyBattlegrounds,
                    CastFailureReason::TargetNotGhost => SMSG_CAST_RESULT_CastFailureReason::TargetNotGhost,
                    CastFailureReason::TooManySkills => SMSG_CAST_RESULT_CastFailureReason::TooManySkills,
                    CastFailureReason::TransformUnusable => SMSG_CAST_RESULT_CastFailureReason::TransformUnusable,
                    CastFailureReason::WrongWeather => SMSG_CAST_RESULT_CastFailureReason::WrongWeather,
                    CastFailureReason::DamageImmune => SMSG_CAST_RESULT_CastFailureReason::DamageImmune,
                    CastFailureReason::PreventedByMechanic => SMSG_CAST_RESULT_CastFailureReason::PreventedByMechanic,
                    CastFailureReason::PlayTime => SMSG_CAST_RESULT_CastFailureReason::PlayTime,
                    CastFailureReason::Reputation => SMSG_CAST_RESULT_CastFailureReason::Reputation,
                    CastFailureReason::MinSkill => SMSG_CAST_RESULT_CastFailureReason::MinSkill,
                    CastFailureReason::Unknown => SMSG_CAST_RESULT_CastFailureReason::Unknown,
                };

                SMSG_CAST_RESULT_SimpleSpellCastResult::Success {
                    reason: reason_if,
                }
            }
            SimpleSpellCastResult::Failure => SMSG_CAST_RESULT_SimpleSpellCastResult::Failure,
        };

        Ok(Self {
            spell,
            result: result_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_CAST_RESULT {}

impl SMSG_CAST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // spell: u32
        + self.result.size() // result: SMSG_CAST_RESULT_SimpleSpellCastResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_CAST_RESULT_CastFailureReason {
    AffectingCombat,
    AlreadyAtFullHealth,
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
        equipped_item_class: u32,
        equipped_item_inventory_type_mask: u32,
        equipped_item_subclass_mask: u32,
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
        required_spell_focus: u32,
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
    Totems,
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
    Unknown,
}

impl Default for SMSG_CAST_RESULT_CastFailureReason {
    fn default() -> Self {
        // First enumerator without any fields
        Self::AffectingCombat
    }
}

impl SMSG_CAST_RESULT_CastFailureReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::AffectingCombat => 0,
            Self::AlreadyAtFullHealth => 1,
            Self::AlreadyAtFullPower => 2,
            Self::AlreadyBeingTamed => 3,
            Self::AlreadyHaveCharm => 4,
            Self::AlreadyHaveSummon => 5,
            Self::AlreadyOpen => 6,
            Self::AuraBounced => 7,
            Self::AutotrackInterrupted => 8,
            Self::BadImplicitTargets => 9,
            Self::BadTargets => 10,
            Self::CantBeCharmed => 11,
            Self::CantBeDisenchanted => 12,
            Self::CantBeProspected => 13,
            Self::CantCastOnTapped => 14,
            Self::CantDuelWhileInvisible => 15,
            Self::CantDuelWhileStealthed => 16,
            Self::CantStealth => 17,
            Self::CasterAurastate => 18,
            Self::CasterDead => 19,
            Self::Charmed => 20,
            Self::ChestInUse => 21,
            Self::Confused => 22,
            Self::DontReport => 23,
            Self::EquippedItem => 24,
            Self::EquippedItemClass { .. } => 25,
            Self::EquippedItemClassMainhand => 26,
            Self::EquippedItemClassOffhand => 27,
            Self::ErrorX => 28,
            Self::Fizzle => 29,
            Self::Fleeing => 30,
            Self::FoodLowlevel => 31,
            Self::Highlevel => 32,
            Self::HungerSatiated => 33,
            Self::Immune => 34,
            Self::Interrupted => 35,
            Self::InterruptedCombat => 36,
            Self::ItemAlreadyEnchanted => 37,
            Self::ItemGone => 38,
            Self::ItemNotFound => 39,
            Self::ItemNotReady => 40,
            Self::LevelRequirement => 41,
            Self::LineOfSight => 42,
            Self::Lowlevel => 43,
            Self::LowCastlevel => 44,
            Self::MainhandEmpty => 45,
            Self::Moving => 46,
            Self::NeedAmmo => 47,
            Self::NeedAmmoPouch => 48,
            Self::NeedExoticAmmo => 49,
            Self::Nopath => 50,
            Self::NotBehind => 51,
            Self::NotFishable => 52,
            Self::NotHere => 53,
            Self::NotInfront => 54,
            Self::NotInControl => 55,
            Self::NotKnown => 56,
            Self::NotMounted => 57,
            Self::NotOnTaxi => 58,
            Self::NotOnTransport => 59,
            Self::NotReady => 60,
            Self::NotShapeshift => 61,
            Self::NotStanding => 62,
            Self::NotTradeable => 63,
            Self::NotTrading => 64,
            Self::NotUnsheathed => 65,
            Self::NotWhileGhost => 66,
            Self::NoAmmo => 67,
            Self::NoChargesRemain => 68,
            Self::NoChampion => 69,
            Self::NoComboPoints => 70,
            Self::NoDueling => 71,
            Self::NoEndurance => 72,
            Self::NoFish => 73,
            Self::NoItemsWhileShapeshifted => 74,
            Self::NoMountsAllowed => 75,
            Self::NoPet => 76,
            Self::NoPower => 77,
            Self::NothingToDispel => 78,
            Self::NothingToSteal => 79,
            Self::OnlyAbovewater => 80,
            Self::OnlyDaytime => 81,
            Self::OnlyIndoors => 82,
            Self::OnlyMounted => 83,
            Self::OnlyNighttime => 84,
            Self::OnlyOutdoors => 85,
            Self::OnlyShapeshift => 86,
            Self::OnlyStealthed => 87,
            Self::OnlyUnderwater => 88,
            Self::OutOfRange => 89,
            Self::Pacified => 90,
            Self::Possessed => 91,
            Self::Reagents => 92,
            Self::RequiresArea { .. } => 93,
            Self::RequiresSpellFocus { .. } => 94,
            Self::Rooted => 95,
            Self::Silenced => 96,
            Self::SpellInProgress => 97,
            Self::SpellLearned => 98,
            Self::SpellUnavailable => 99,
            Self::Stunned => 100,
            Self::TargetsDead => 101,
            Self::TargetAffectingCombat => 102,
            Self::TargetAurastate => 103,
            Self::TargetDueling => 104,
            Self::TargetEnemy => 105,
            Self::TargetEnraged => 106,
            Self::TargetFriendly => 107,
            Self::TargetInCombat => 108,
            Self::TargetIsPlayer => 109,
            Self::TargetNotDead => 110,
            Self::TargetNotInParty => 111,
            Self::TargetNotLooted => 112,
            Self::TargetNotPlayer => 113,
            Self::TargetNoPockets => 114,
            Self::TargetNoWeapons => 115,
            Self::TargetUnskinnable => 116,
            Self::ThirstSatiated => 117,
            Self::TooClose => 118,
            Self::TooManyOfItem => 119,
            Self::Totems => 120,
            Self::TrainingPoints => 121,
            Self::TryAgain => 122,
            Self::UnitNotBehind => 123,
            Self::UnitNotInfront => 124,
            Self::WrongPetFood => 125,
            Self::NotWhileFatigued => 126,
            Self::TargetNotInInstance => 127,
            Self::NotWhileTrading => 128,
            Self::TargetNotInRaid => 129,
            Self::DisenchantWhileLooting => 130,
            Self::ProspectWhileLooting => 131,
            Self::ProspectNeedMore => 132,
            Self::TargetFreeforall => 133,
            Self::NoEdibleCorpses => 134,
            Self::OnlyBattlegrounds => 135,
            Self::TargetNotGhost => 136,
            Self::TooManySkills => 137,
            Self::TransformUnusable => 138,
            Self::WrongWeather => 139,
            Self::DamageImmune => 140,
            Self::PreventedByMechanic => 141,
            Self::PlayTime => 142,
            Self::Reputation => 143,
            Self::MinSkill => 144,
            Self::Unknown => 145,
        }
    }

}

impl SMSG_CAST_RESULT_CastFailureReason {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::AffectingCombat => {
                1
            }
            Self::AlreadyAtFullHealth => {
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
                equipped_item_class,
                equipped_item_inventory_type_mask,
                equipped_item_subclass_mask,
            } => {
                1
                + 4 // equipped_item_class: u32
                + 4 // equipped_item_inventory_type_mask: u32
                + 4 // equipped_item_subclass_mask: u32
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
                required_spell_focus,
            } => {
                1
                + 4 // required_spell_focus: u32
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
            Self::Totems => {
                1
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
            Self::Unknown => {
                1
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_CAST_RESULT_SimpleSpellCastResult {
    Success {
        reason: SMSG_CAST_RESULT_CastFailureReason,
    },
    Failure,
}

impl Default for SMSG_CAST_RESULT_SimpleSpellCastResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Failure
    }
}

impl SMSG_CAST_RESULT_SimpleSpellCastResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success { .. } => 0,
            Self::Failure => 2,
        }
    }

}

impl SMSG_CAST_RESULT_SimpleSpellCastResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Success {
                reason,
            } => {
                1
                + reason.size() // reason: SMSG_CAST_RESULT_CastFailureReason
            }
            Self::Failure => {
                1
            }
        }
    }
}

