use std::io::{Read, Write};

use crate::vanilla::{
    Area, CastFailureReason, SimpleSpellCastResult,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cast_result.wowm:303`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cast_result.wowm#L303):
/// ```text
/// smsg SMSG_CAST_RESULT = 0x0130 {
///     Spell spell;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CAST_RESULT {
    pub spell: u32,
    pub result: SMSG_CAST_RESULT_SimpleSpellCastResult,
}

impl crate::private::Sealed for SMSG_CAST_RESULT {}
impl SMSG_CAST_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=18).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        // result: SimpleSpellCastResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            SimpleSpellCastResult::Success => {
                // reason: CastFailureReason
                let reason = crate::util::read_u8_le(&mut r)?.try_into()?;

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
                        let equipped_item_class = crate::util::read_u32_le(&mut r)?;

                        // equipped_item_subclass_mask: u32
                        let equipped_item_subclass_mask = crate::util::read_u32_le(&mut r)?;

                        // equipped_item_inventory_type_mask: u32
                        let equipped_item_inventory_type_mask = crate::util::read_u32_le(&mut r)?;

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
                        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

                        SMSG_CAST_RESULT_CastFailureReason::RequiresArea {
                            area,
                        }
                    }
                    CastFailureReason::RequiresSpellFocus => {
                        // required_spell_focus: u32
                        let required_spell_focus = crate::util::read_u32_le(&mut r)?;

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

impl crate::Message for SMSG_CAST_RESULT {
    const OPCODE: u32 = 0x0130;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CAST_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CAST_RESULT {{").unwrap();
        // Members
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    result = {};", SimpleSpellCastResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::vanilla::SMSG_CAST_RESULT_SimpleSpellCastResult::Success {
                reason,
            } => {
                writeln!(s, "    reason = {};", CastFailureReason::try_from(reason.as_int()).unwrap().as_test_case_value()).unwrap();
                match &reason {
                    crate::vanilla::SMSG_CAST_RESULT_CastFailureReason::EquippedItemClass {
                        equipped_item_class,
                        equipped_item_inventory_type_mask,
                        equipped_item_subclass_mask,
                    } => {
                        writeln!(s, "    equipped_item_class = {};", equipped_item_class).unwrap();
                        writeln!(s, "    equipped_item_subclass_mask = {};", equipped_item_subclass_mask).unwrap();
                        writeln!(s, "    equipped_item_inventory_type_mask = {};", equipped_item_inventory_type_mask).unwrap();
                    }
                    crate::vanilla::SMSG_CAST_RESULT_CastFailureReason::RequiresArea {
                        area,
                    } => {
                        writeln!(s, "    area = {};", area.as_test_case_value()).unwrap();
                    }
                    crate::vanilla::SMSG_CAST_RESULT_CastFailureReason::RequiresSpellFocus {
                        required_spell_focus,
                    } => {
                        writeln!(s, "    required_spell_focus = {};", required_spell_focus).unwrap();
                    }
                    _ => {}
                }

            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 304_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self.result {
            crate::vanilla::SMSG_CAST_RESULT_SimpleSpellCastResult::Success {
                reason,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "reason", "    ");
                match &reason {
                    crate::vanilla::SMSG_CAST_RESULT_CastFailureReason::EquippedItemClass {
                        equipped_item_class,
                        equipped_item_inventory_type_mask,
                        equipped_item_subclass_mask,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equipped_item_class", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equipped_item_subclass_mask", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equipped_item_inventory_type_mask", "    ");
                    }
                    crate::vanilla::SMSG_CAST_RESULT_CastFailureReason::RequiresArea {
                        area,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");
                    }
                    crate::vanilla::SMSG_CAST_RESULT_CastFailureReason::RequiresSpellFocus {
                        required_spell_focus,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "required_spell_focus", "    ");
                    }
                    _ => {}
                }

            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        // result: SimpleSpellCastResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        match &self.result {
            SMSG_CAST_RESULT_SimpleSpellCastResult::Success {
                reason,
            } => {
                // reason: CastFailureReason
                w.write_all(&(reason.as_int().to_le_bytes()))?;

                match &reason {
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
                    SMSG_CAST_RESULT_CastFailureReason::RequiresArea {
                        area,
                    } => {
                        // area: Area
                        w.write_all(&(area.as_int().to_le_bytes()))?;

                    }
                    SMSG_CAST_RESULT_CastFailureReason::RequiresSpellFocus {
                        required_spell_focus,
                    } => {
                        // required_spell_focus: u32
                        w.write_all(&required_spell_focus.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(304, "SMSG_CAST_RESULT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CAST_RESULT {}

impl SMSG_CAST_RESULT {
    pub(crate) const fn size(&self) -> usize {
        4 // spell: Spell
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

impl std::fmt::Display for SMSG_CAST_RESULT_CastFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AffectingCombat => f.write_str("AffectingCombat"),
            Self::AlreadyAtFullHealth => f.write_str("AlreadyAtFullHealth"),
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
            Self::EquippedItemClassMainhand => f.write_str("EquippedItemClassMainhand"),
            Self::EquippedItemClassOffhand => f.write_str("EquippedItemClassOffhand"),
            Self::ErrorX => f.write_str("ErrorX"),
            Self::Fizzle => f.write_str("Fizzle"),
            Self::Fleeing => f.write_str("Fleeing"),
            Self::FoodLowlevel => f.write_str("FoodLowlevel"),
            Self::Highlevel => f.write_str("Highlevel"),
            Self::HungerSatiated => f.write_str("HungerSatiated"),
            Self::Immune => f.write_str("Immune"),
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
            Self::NeedExoticAmmo => f.write_str("NeedExoticAmmo"),
            Self::Nopath => f.write_str("Nopath"),
            Self::NotBehind => f.write_str("NotBehind"),
            Self::NotFishable => f.write_str("NotFishable"),
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
            Self::Reagents => f.write_str("Reagents"),
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
            Self::TargetNotDead => f.write_str("TargetNotDead"),
            Self::TargetNotInParty => f.write_str("TargetNotInParty"),
            Self::TargetNotLooted => f.write_str("TargetNotLooted"),
            Self::TargetNotPlayer => f.write_str("TargetNotPlayer"),
            Self::TargetNoPockets => f.write_str("TargetNoPockets"),
            Self::TargetNoWeapons => f.write_str("TargetNoWeapons"),
            Self::TargetUnskinnable => f.write_str("TargetUnskinnable"),
            Self::ThirstSatiated => f.write_str("ThirstSatiated"),
            Self::TooClose => f.write_str("TooClose"),
            Self::TooManyOfItem => f.write_str("TooManyOfItem"),
            Self::Totems => f.write_str("Totems"),
            Self::TrainingPoints => f.write_str("TrainingPoints"),
            Self::TryAgain => f.write_str("TryAgain"),
            Self::UnitNotBehind => f.write_str("UnitNotBehind"),
            Self::UnitNotInfront => f.write_str("UnitNotInfront"),
            Self::WrongPetFood => f.write_str("WrongPetFood"),
            Self::NotWhileFatigued => f.write_str("NotWhileFatigued"),
            Self::TargetNotInInstance => f.write_str("TargetNotInInstance"),
            Self::NotWhileTrading => f.write_str("NotWhileTrading"),
            Self::TargetNotInRaid => f.write_str("TargetNotInRaid"),
            Self::DisenchantWhileLooting => f.write_str("DisenchantWhileLooting"),
            Self::ProspectWhileLooting => f.write_str("ProspectWhileLooting"),
            Self::ProspectNeedMore => f.write_str("ProspectNeedMore"),
            Self::TargetFreeforall => f.write_str("TargetFreeforall"),
            Self::NoEdibleCorpses => f.write_str("NoEdibleCorpses"),
            Self::OnlyBattlegrounds => f.write_str("OnlyBattlegrounds"),
            Self::TargetNotGhost => f.write_str("TargetNotGhost"),
            Self::TooManySkills => f.write_str("TooManySkills"),
            Self::TransformUnusable => f.write_str("TransformUnusable"),
            Self::WrongWeather => f.write_str("WrongWeather"),
            Self::DamageImmune => f.write_str("DamageImmune"),
            Self::PreventedByMechanic => f.write_str("PreventedByMechanic"),
            Self::PlayTime => f.write_str("PlayTime"),
            Self::Reputation => f.write_str("Reputation"),
            Self::MinSkill => f.write_str("MinSkill"),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl SMSG_CAST_RESULT_CastFailureReason {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::EquippedItemClass {
                ..
            } => {
                1
                + 4 // equipped_item_class: u32
                + 4 // equipped_item_inventory_type_mask: u32
                + 4 // equipped_item_subclass_mask: u32
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
                + 4 // required_spell_focus: u32
            }
            _ => 1,
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

impl std::fmt::Display for SMSG_CAST_RESULT_SimpleSpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success{ .. } => f.write_str("Success"),
            Self::Failure => f.write_str("Failure"),
        }
    }
}

impl SMSG_CAST_RESULT_SimpleSpellCastResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Success {
                reason,
            } => {
                1
                + reason.size() // reason: SMSG_CAST_RESULT_CastFailureReason
            }
            _ => 1,
        }
    }
}

