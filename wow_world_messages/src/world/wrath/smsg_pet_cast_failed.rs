use crate::wrath::Area;
use crate::wrath::Skill;
use crate::wrath::SpellCastResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm#L36):
/// ```text
/// smsg SMSG_PET_CAST_FAILED = 0x0138 {
///     u8 cast_count;
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
///         u32 item;
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
pub struct SMSG_PET_CAST_FAILED {
    pub cast_count: u8,
    pub id: u32,
    pub result: SMSG_PET_CAST_FAILED_SpellCastResult,
    pub multiple_casts: bool,
}

impl crate::Message for SMSG_PET_CAST_FAILED {
    const OPCODE: u32 = 0x0138;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        // multiple_casts: Bool
        w.write_all(u8::from(self.multiple_casts).to_le_bytes().as_slice())?;

        match &self.result {
            SMSG_PET_CAST_FAILED_SpellCastResult::Success => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AffectingCombat => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyAtFullHealth => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyAtFullMana => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyAtFullPower => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyBeingTamed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyHaveCharm => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyHaveSummon => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AlreadyOpen => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AuraBounced => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::AutotrackInterrupted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::BadImplicitTargets => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::BadTargets => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantBeCharmed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantBeDisenchanted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantBeDisenchantedSkill => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantBeMilled => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantBeProspected => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantCastOnTapped => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantDuelWhileInvisible => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantDuelWhileStealthed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CantStealth => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CasterAurastate => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CasterDead => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Charmed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ChestInUse => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Confused => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::DontReport => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItem => {}
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
            SMSG_PET_CAST_FAILED_SpellCastResult::ErrorX => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Fizzle => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Fleeing => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::FoodLowlevel => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Highlevel => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::HungerSatiated => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Immune => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::IncorrectArea => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Interrupted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::InterruptedCombat => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ItemAlreadyEnchanted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ItemGone => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ItemNotFound => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ItemNotReady => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::LevelRequirement => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::LineOfSight => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Lowlevel => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::LowCastlevel => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::MainhandEmpty => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Moving => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NeedAmmo => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NeedAmmoPouch => {}
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
                // item: u32
                w.write_all(&item.to_le_bytes())?;

                // count: u32
                w.write_all(&count.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::Nopath => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotBehind => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotFishable => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotFlying => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotHere => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotInfront => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotInControl => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotKnown => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotMounted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotOnTaxi => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotOnTransport => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotReady => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotShapeshift => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotStanding => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotTradeable => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotTrading => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotUnsheathed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileGhost => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileLooting => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoAmmo => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoChargesRemain => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoChampion => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoComboPoints => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoDueling => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoEndurance => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoFish => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoItemsWhileShapeshifted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoMountsAllowed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoPet => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoPower => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NothingToDispel => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NothingToSteal => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyAbovewater => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyDaytime => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyIndoors => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyMounted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyNighttime => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyOutdoors => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyShapeshift => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyStealthed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyUnderwater => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OutOfRange => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Pacified => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Possessed => {}
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
                w.write_all(&u32::from(area.as_int()).to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::RequiresSpellFocus {
                spell_focus,
            } => {
                // spell_focus: u32
                w.write_all(&spell_focus.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::Rooted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Silenced => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::SpellInProgress => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::SpellLearned => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::SpellUnavailable => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Stunned => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetsDead => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetAffectingCombat => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetAurastate => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetDueling => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetEnemy => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetEnraged => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetFriendly => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetInCombat => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetIsPlayer => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetIsPlayerControlled => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotDead => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInParty => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotLooted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotPlayer => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNoPockets => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNoWeapons => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNoRangedWeapons => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetUnskinnable => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ThirstSatiated => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TooClose => {}
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
            SMSG_PET_CAST_FAILED_SpellCastResult::TryAgain => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::UnitNotBehind => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::UnitNotInfront => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::WrongPetFood => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileFatigued => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInInstance => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotWhileTrading => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInRaid => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetFreeforall => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoEdibleCorpses => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyBattlegrounds => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotGhost => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TransformUnusable => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::WrongWeather => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::DamageImmune => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::PreventedByMechanic {
                mechanic,
            } => {
                // mechanic: u32
                w.write_all(&mechanic.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::PlayTime => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Reputation => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::MinSkill {
                skill,
                skill_required,
            } => {
                // skill: Skill
                w.write_all(&u32::from(skill.as_int()).to_le_bytes())?;

                // skill_required: u32
                w.write_all(&skill_required.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::NotInArena => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotOnShapeshift => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotOnStealthed => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotOnDamageImmune => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotOnMounted => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TooShallow => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetNotInSanctuary => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetIsTrivial => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::BmOrInvisgod => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ExpertRidingRequirement => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ArtisanRidingRequirement => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotIdle => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotInactive => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::PartialPlaytime => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoPlaytime => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotInBattleground => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotInRaidInstance => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnlyInArena => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetLockedToRaidInstance => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::OnUseEnchant => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotOnGround => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::CustomError {
                custom_error,
            } => {
                // custom_error: u32
                w.write_all(&custom_error.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::CantDoThatRightNow => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TooManySockets => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::InvalidGlyph => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::UniqueGlyph => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::GlyphSocketLocked => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NoValidTargets => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::ItemAtMaxCharges => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::NotInBarbershop => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::FishingTooLow {
                fishing_skill_required,
            } => {
                // fishing_skill_required: u32
                w.write_all(&fishing_skill_required.to_le_bytes())?;

            }
            SMSG_PET_CAST_FAILED_SpellCastResult::ItemEnchantTradeWindow => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::SummonPending => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::MaxSockets => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::PetCanRename => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::TargetCannotBeResurrected => {}
            SMSG_PET_CAST_FAILED_SpellCastResult::Unknown => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(7..=15).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0138, size: body_size as u32 });
        }

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        // multiple_casts: Bool
        let multiple_casts = crate::util::read_u8_le(r)? != 0;

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
                let item_class = crate::util::read_u32_le(r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClass {
                    item_class,
                    item_sub_class,
                }
            }
            SpellCastResult::EquippedItemClassMainhand => {
                // item_class: u32
                let item_class = crate::util::read_u32_le(r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::EquippedItemClassMainhand {
                    item_class,
                    item_sub_class,
                }
            }
            SpellCastResult::EquippedItemClassOffhand => {
                // item_class: u32
                let item_class = crate::util::read_u32_le(r)?;

                // item_sub_class: u32
                let item_sub_class = crate::util::read_u32_le(r)?;

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
                let equipped_item_sub_class = crate::util::read_u32_le(r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::NeedExoticAmmo {
                    equipped_item_sub_class,
                }
            }
            SpellCastResult::NeedMoreItems => {
                // item: u32
                let item = crate::util::read_u32_le(r)?;

                // count: u32
                let count = crate::util::read_u32_le(r)?;

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
                let missing_item = crate::util::read_u32_le(r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::Reagents {
                    missing_item,
                }
            }
            SpellCastResult::RequiresArea => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                SMSG_PET_CAST_FAILED_SpellCastResult::RequiresArea {
                    area,
                }
            }
            SpellCastResult::RequiresSpellFocus => {
                // spell_focus: u32
                let spell_focus = crate::util::read_u32_le(r)?;

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
                let item_limit_category = crate::util::read_u32_le(r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::TooManyOfItem {
                    item_limit_category,
                }
            }
            SpellCastResult::TotemCategory => {
                // totem_categories: u32[2]
                let totem_categories = {
                    let mut totem_categories = [u32::default(); 2];
                    for i in totem_categories.iter_mut() {
                        *i = crate::util::read_u32_le(r)?;
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
                        *i = crate::util::read_u32_le(r)?;
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
                let mechanic = crate::util::read_u32_le(r)?;

                SMSG_PET_CAST_FAILED_SpellCastResult::PreventedByMechanic {
                    mechanic,
                }
            }
            SpellCastResult::PlayTime => SMSG_PET_CAST_FAILED_SpellCastResult::PlayTime,
            SpellCastResult::Reputation => SMSG_PET_CAST_FAILED_SpellCastResult::Reputation,
            SpellCastResult::MinSkill => {
                // skill: Skill
                let skill: Skill = (crate::util::read_u32_le(r)? as u16).try_into()?;

                // skill_required: u32
                let skill_required = crate::util::read_u32_le(r)?;

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
                let custom_error = crate::util::read_u32_le(r)?;

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
                let fishing_skill_required = crate::util::read_u32_le(r)?;

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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_CAST_FAILED {}

impl SMSG_PET_CAST_FAILED {
    pub(crate) fn size(&self) -> usize {
        1 // cast_count: u8
        + 4 // id: u32
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

impl SMSG_PET_CAST_FAILED_SpellCastResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Success => {
                1
            }
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
            Self::CantBeMilled => {
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
                item_sub_class,
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_sub_class: u32
            }
            Self::EquippedItemClassMainhand {
                item_class,
                item_sub_class,
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_sub_class: u32
            }
            Self::EquippedItemClassOffhand {
                item_class,
                item_sub_class,
            } => {
                1
                + 4 // item_class: u32
                + 4 // item_sub_class: u32
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
            Self::IncorrectArea => {
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
            Self::NeedExoticAmmo {
                equipped_item_sub_class,
            } => {
                1
                + 4 // equipped_item_sub_class: u32
            }
            Self::NeedMoreItems {
                count,
                item,
            } => {
                1
                + 4 // count: u32
                + 4 // item: u32
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
            Self::NotWhileLooting => {
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
            Self::Reagents {
                missing_item,
            } => {
                1
                + 4 // missing_item: u32
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
            Self::TargetNoRangedWeapons => {
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
            Self::TooManyOfItem {
                item_limit_category,
            } => {
                1
                + 4 // item_limit_category: u32
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
            Self::TransformUnusable => {
                1
            }
            Self::WrongWeather => {
                1
            }
            Self::DamageImmune => {
                1
            }
            Self::PreventedByMechanic {
                mechanic,
            } => {
                1
                + 4 // mechanic: u32
            }
            Self::PlayTime => {
                1
            }
            Self::Reputation => {
                1
            }
            Self::MinSkill {
                skill,
                skill_required,
            } => {
                1
                + 4 // skill: Skill
                + 4 // skill_required: u32
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
            Self::NotInRaidInstance => {
                1
            }
            Self::OnlyInArena => {
                1
            }
            Self::TargetLockedToRaidInstance => {
                1
            }
            Self::OnUseEnchant => {
                1
            }
            Self::NotOnGround => {
                1
            }
            Self::CustomError {
                custom_error,
            } => {
                1
                + 4 // custom_error: u32
            }
            Self::CantDoThatRightNow => {
                1
            }
            Self::TooManySockets => {
                1
            }
            Self::InvalidGlyph => {
                1
            }
            Self::UniqueGlyph => {
                1
            }
            Self::GlyphSocketLocked => {
                1
            }
            Self::NoValidTargets => {
                1
            }
            Self::ItemAtMaxCharges => {
                1
            }
            Self::NotInBarbershop => {
                1
            }
            Self::FishingTooLow {
                fishing_skill_required,
            } => {
                1
                + 4 // fishing_skill_required: u32
            }
            Self::ItemEnchantTradeWindow => {
                1
            }
            Self::SummonPending => {
                1
            }
            Self::MaxSockets => {
                1
            }
            Self::PetCanRename => {
                1
            }
            Self::TargetCannotBeResurrected => {
                1
            }
            Self::Unknown => {
                1
            }
        }
    }
}

