use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{CastFailureReason, CastFailureReasonError};
use crate::world::v1::v12::{SimpleSpellCastResult, SimpleSpellCastResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CAST_RESULT {
    pub spell: u32,
    pub result: SMSG_CAST_RESULTSimpleSpellCastResult,
}

impl ServerMessageWrite for SMSG_CAST_RESULT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_CAST_RESULT {
    const OPCODE: u16 = 0x0130;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_CAST_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // result: SimpleSpellCastResult
        let result = SimpleSpellCastResult::read(r)?;

        let result_if = match result {
            SimpleSpellCastResult::SUCCESS => {
                // reason: CastFailureReason
                let reason = CastFailureReason::read(r)?;

                let reason_if = match reason {
                    CastFailureReason::AFFECTING_COMBAT => SMSG_CAST_RESULTCastFailureReason::AFFECTING_COMBAT,
                    CastFailureReason::ALREADY_AT_FULL_HEALTH => SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_HEALTH,
                    CastFailureReason::ALREADY_AT_FULL_POWER => SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_POWER,
                    CastFailureReason::ALREADY_BEING_TAMED => SMSG_CAST_RESULTCastFailureReason::ALREADY_BEING_TAMED,
                    CastFailureReason::ALREADY_HAVE_CHARM => SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_CHARM,
                    CastFailureReason::ALREADY_HAVE_SUMMON => SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_SUMMON,
                    CastFailureReason::ALREADY_OPEN => SMSG_CAST_RESULTCastFailureReason::ALREADY_OPEN,
                    CastFailureReason::AURA_BOUNCED => SMSG_CAST_RESULTCastFailureReason::AURA_BOUNCED,
                    CastFailureReason::AUTOTRACK_INTERRUPTED => SMSG_CAST_RESULTCastFailureReason::AUTOTRACK_INTERRUPTED,
                    CastFailureReason::BAD_IMPLICIT_TARGETS => SMSG_CAST_RESULTCastFailureReason::BAD_IMPLICIT_TARGETS,
                    CastFailureReason::BAD_TARGETS => SMSG_CAST_RESULTCastFailureReason::BAD_TARGETS,
                    CastFailureReason::CANT_BE_CHARMED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_CHARMED,
                    CastFailureReason::CANT_BE_DISENCHANTED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_DISENCHANTED,
                    CastFailureReason::CANT_BE_PROSPECTED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_PROSPECTED,
                    CastFailureReason::CANT_CAST_ON_TAPPED => SMSG_CAST_RESULTCastFailureReason::CANT_CAST_ON_TAPPED,
                    CastFailureReason::CANT_DUEL_WHILE_INVISIBLE => SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_INVISIBLE,
                    CastFailureReason::CANT_DUEL_WHILE_STEALTHED => SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_STEALTHED,
                    CastFailureReason::CANT_STEALTH => SMSG_CAST_RESULTCastFailureReason::CANT_STEALTH,
                    CastFailureReason::CASTER_AURASTATE => SMSG_CAST_RESULTCastFailureReason::CASTER_AURASTATE,
                    CastFailureReason::CASTER_DEAD => SMSG_CAST_RESULTCastFailureReason::CASTER_DEAD,
                    CastFailureReason::CHARMED => SMSG_CAST_RESULTCastFailureReason::CHARMED,
                    CastFailureReason::CHEST_IN_USE => SMSG_CAST_RESULTCastFailureReason::CHEST_IN_USE,
                    CastFailureReason::CONFUSED => SMSG_CAST_RESULTCastFailureReason::CONFUSED,
                    CastFailureReason::DONT_REPORT => SMSG_CAST_RESULTCastFailureReason::DONT_REPORT,
                    CastFailureReason::EQUIPPED_ITEM => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM,
                    CastFailureReason::EQUIPPED_ITEM_CLASS => {
                        // equipped_item_class: u32
                        let equipped_item_class = crate::util::read_u32_le(r)?;

                        // equipped_item_subclass_mask: u32
                        let equipped_item_subclass_mask = crate::util::read_u32_le(r)?;

                        // equipped_item_inventory_type_mask: u32
                        let equipped_item_inventory_type_mask = crate::util::read_u32_le(r)?;

                        SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS {
                            equipped_item_class,
                            equipped_item_subclass_mask,
                            equipped_item_inventory_type_mask,
                        }
                    }
                    CastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND,
                    CastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND,
                    CastFailureReason::ERROR => SMSG_CAST_RESULTCastFailureReason::ERROR,
                    CastFailureReason::FIZZLE => SMSG_CAST_RESULTCastFailureReason::FIZZLE,
                    CastFailureReason::FLEEING => SMSG_CAST_RESULTCastFailureReason::FLEEING,
                    CastFailureReason::FOOD_LOWLEVEL => SMSG_CAST_RESULTCastFailureReason::FOOD_LOWLEVEL,
                    CastFailureReason::HIGHLEVEL => SMSG_CAST_RESULTCastFailureReason::HIGHLEVEL,
                    CastFailureReason::HUNGER_SATIATED => SMSG_CAST_RESULTCastFailureReason::HUNGER_SATIATED,
                    CastFailureReason::IMMUNE => SMSG_CAST_RESULTCastFailureReason::IMMUNE,
                    CastFailureReason::INTERRUPTED => SMSG_CAST_RESULTCastFailureReason::INTERRUPTED,
                    CastFailureReason::INTERRUPTED_COMBAT => SMSG_CAST_RESULTCastFailureReason::INTERRUPTED_COMBAT,
                    CastFailureReason::ITEM_ALREADY_ENCHANTED => SMSG_CAST_RESULTCastFailureReason::ITEM_ALREADY_ENCHANTED,
                    CastFailureReason::ITEM_GONE => SMSG_CAST_RESULTCastFailureReason::ITEM_GONE,
                    CastFailureReason::ITEM_NOT_FOUND => SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_FOUND,
                    CastFailureReason::ITEM_NOT_READY => SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_READY,
                    CastFailureReason::LEVEL_REQUIREMENT => SMSG_CAST_RESULTCastFailureReason::LEVEL_REQUIREMENT,
                    CastFailureReason::LINE_OF_SIGHT => SMSG_CAST_RESULTCastFailureReason::LINE_OF_SIGHT,
                    CastFailureReason::LOWLEVEL => SMSG_CAST_RESULTCastFailureReason::LOWLEVEL,
                    CastFailureReason::LOW_CASTLEVEL => SMSG_CAST_RESULTCastFailureReason::LOW_CASTLEVEL,
                    CastFailureReason::MAINHAND_EMPTY => SMSG_CAST_RESULTCastFailureReason::MAINHAND_EMPTY,
                    CastFailureReason::MOVING => SMSG_CAST_RESULTCastFailureReason::MOVING,
                    CastFailureReason::NEED_AMMO => SMSG_CAST_RESULTCastFailureReason::NEED_AMMO,
                    CastFailureReason::NEED_AMMO_POUCH => SMSG_CAST_RESULTCastFailureReason::NEED_AMMO_POUCH,
                    CastFailureReason::NEED_EXOTIC_AMMO => SMSG_CAST_RESULTCastFailureReason::NEED_EXOTIC_AMMO,
                    CastFailureReason::NOPATH => SMSG_CAST_RESULTCastFailureReason::NOPATH,
                    CastFailureReason::NOT_BEHIND => SMSG_CAST_RESULTCastFailureReason::NOT_BEHIND,
                    CastFailureReason::NOT_FISHABLE => SMSG_CAST_RESULTCastFailureReason::NOT_FISHABLE,
                    CastFailureReason::NOT_HERE => SMSG_CAST_RESULTCastFailureReason::NOT_HERE,
                    CastFailureReason::NOT_INFRONT => SMSG_CAST_RESULTCastFailureReason::NOT_INFRONT,
                    CastFailureReason::NOT_IN_CONTROL => SMSG_CAST_RESULTCastFailureReason::NOT_IN_CONTROL,
                    CastFailureReason::NOT_KNOWN => SMSG_CAST_RESULTCastFailureReason::NOT_KNOWN,
                    CastFailureReason::NOT_MOUNTED => SMSG_CAST_RESULTCastFailureReason::NOT_MOUNTED,
                    CastFailureReason::NOT_ON_TAXI => SMSG_CAST_RESULTCastFailureReason::NOT_ON_TAXI,
                    CastFailureReason::NOT_ON_TRANSPORT => SMSG_CAST_RESULTCastFailureReason::NOT_ON_TRANSPORT,
                    CastFailureReason::NOT_READY => SMSG_CAST_RESULTCastFailureReason::NOT_READY,
                    CastFailureReason::NOT_SHAPESHIFT => SMSG_CAST_RESULTCastFailureReason::NOT_SHAPESHIFT,
                    CastFailureReason::NOT_STANDING => SMSG_CAST_RESULTCastFailureReason::NOT_STANDING,
                    CastFailureReason::NOT_TRADEABLE => SMSG_CAST_RESULTCastFailureReason::NOT_TRADEABLE,
                    CastFailureReason::NOT_TRADING => SMSG_CAST_RESULTCastFailureReason::NOT_TRADING,
                    CastFailureReason::NOT_UNSHEATHED => SMSG_CAST_RESULTCastFailureReason::NOT_UNSHEATHED,
                    CastFailureReason::NOT_WHILE_GHOST => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_GHOST,
                    CastFailureReason::NO_AMMO => SMSG_CAST_RESULTCastFailureReason::NO_AMMO,
                    CastFailureReason::NO_CHARGES_REMAIN => SMSG_CAST_RESULTCastFailureReason::NO_CHARGES_REMAIN,
                    CastFailureReason::NO_CHAMPION => SMSG_CAST_RESULTCastFailureReason::NO_CHAMPION,
                    CastFailureReason::NO_COMBO_POINTS => SMSG_CAST_RESULTCastFailureReason::NO_COMBO_POINTS,
                    CastFailureReason::NO_DUELING => SMSG_CAST_RESULTCastFailureReason::NO_DUELING,
                    CastFailureReason::NO_ENDURANCE => SMSG_CAST_RESULTCastFailureReason::NO_ENDURANCE,
                    CastFailureReason::NO_FISH => SMSG_CAST_RESULTCastFailureReason::NO_FISH,
                    CastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => SMSG_CAST_RESULTCastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED,
                    CastFailureReason::NO_MOUNTS_ALLOWED => SMSG_CAST_RESULTCastFailureReason::NO_MOUNTS_ALLOWED,
                    CastFailureReason::NO_PET => SMSG_CAST_RESULTCastFailureReason::NO_PET,
                    CastFailureReason::NO_POWER => SMSG_CAST_RESULTCastFailureReason::NO_POWER,
                    CastFailureReason::NOTHING_TO_DISPEL => SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_DISPEL,
                    CastFailureReason::NOTHING_TO_STEAL => SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_STEAL,
                    CastFailureReason::ONLY_ABOVEWATER => SMSG_CAST_RESULTCastFailureReason::ONLY_ABOVEWATER,
                    CastFailureReason::ONLY_DAYTIME => SMSG_CAST_RESULTCastFailureReason::ONLY_DAYTIME,
                    CastFailureReason::ONLY_INDOORS => SMSG_CAST_RESULTCastFailureReason::ONLY_INDOORS,
                    CastFailureReason::ONLY_MOUNTED => SMSG_CAST_RESULTCastFailureReason::ONLY_MOUNTED,
                    CastFailureReason::ONLY_NIGHTTIME => SMSG_CAST_RESULTCastFailureReason::ONLY_NIGHTTIME,
                    CastFailureReason::ONLY_OUTDOORS => SMSG_CAST_RESULTCastFailureReason::ONLY_OUTDOORS,
                    CastFailureReason::ONLY_SHAPESHIFT => SMSG_CAST_RESULTCastFailureReason::ONLY_SHAPESHIFT,
                    CastFailureReason::ONLY_STEALTHED => SMSG_CAST_RESULTCastFailureReason::ONLY_STEALTHED,
                    CastFailureReason::ONLY_UNDERWATER => SMSG_CAST_RESULTCastFailureReason::ONLY_UNDERWATER,
                    CastFailureReason::OUT_OF_RANGE => SMSG_CAST_RESULTCastFailureReason::OUT_OF_RANGE,
                    CastFailureReason::PACIFIED => SMSG_CAST_RESULTCastFailureReason::PACIFIED,
                    CastFailureReason::POSSESSED => SMSG_CAST_RESULTCastFailureReason::POSSESSED,
                    CastFailureReason::REAGENTS => SMSG_CAST_RESULTCastFailureReason::REAGENTS,
                    CastFailureReason::REQUIRES_AREA => {
                        // area: Area
                        let area = Area::read(r)?;

                        SMSG_CAST_RESULTCastFailureReason::REQUIRES_AREA {
                            area,
                        }
                    }
                    CastFailureReason::REQUIRES_SPELL_FOCUS => {
                        // required_spell_focus: u32
                        let required_spell_focus = crate::util::read_u32_le(r)?;

                        SMSG_CAST_RESULTCastFailureReason::REQUIRES_SPELL_FOCUS {
                            required_spell_focus,
                        }
                    }
                    CastFailureReason::ROOTED => SMSG_CAST_RESULTCastFailureReason::ROOTED,
                    CastFailureReason::SILENCED => SMSG_CAST_RESULTCastFailureReason::SILENCED,
                    CastFailureReason::SPELL_IN_PROGRESS => SMSG_CAST_RESULTCastFailureReason::SPELL_IN_PROGRESS,
                    CastFailureReason::SPELL_LEARNED => SMSG_CAST_RESULTCastFailureReason::SPELL_LEARNED,
                    CastFailureReason::SPELL_UNAVAILABLE => SMSG_CAST_RESULTCastFailureReason::SPELL_UNAVAILABLE,
                    CastFailureReason::STUNNED => SMSG_CAST_RESULTCastFailureReason::STUNNED,
                    CastFailureReason::TARGETS_DEAD => SMSG_CAST_RESULTCastFailureReason::TARGETS_DEAD,
                    CastFailureReason::TARGET_AFFECTING_COMBAT => SMSG_CAST_RESULTCastFailureReason::TARGET_AFFECTING_COMBAT,
                    CastFailureReason::TARGET_AURASTATE => SMSG_CAST_RESULTCastFailureReason::TARGET_AURASTATE,
                    CastFailureReason::TARGET_DUELING => SMSG_CAST_RESULTCastFailureReason::TARGET_DUELING,
                    CastFailureReason::TARGET_ENEMY => SMSG_CAST_RESULTCastFailureReason::TARGET_ENEMY,
                    CastFailureReason::TARGET_ENRAGED => SMSG_CAST_RESULTCastFailureReason::TARGET_ENRAGED,
                    CastFailureReason::TARGET_FRIENDLY => SMSG_CAST_RESULTCastFailureReason::TARGET_FRIENDLY,
                    CastFailureReason::TARGET_IN_COMBAT => SMSG_CAST_RESULTCastFailureReason::TARGET_IN_COMBAT,
                    CastFailureReason::TARGET_IS_PLAYER => SMSG_CAST_RESULTCastFailureReason::TARGET_IS_PLAYER,
                    CastFailureReason::TARGET_NOT_DEAD => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_DEAD,
                    CastFailureReason::TARGET_NOT_IN_PARTY => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_PARTY,
                    CastFailureReason::TARGET_NOT_LOOTED => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_LOOTED,
                    CastFailureReason::TARGET_NOT_PLAYER => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_PLAYER,
                    CastFailureReason::TARGET_NO_POCKETS => SMSG_CAST_RESULTCastFailureReason::TARGET_NO_POCKETS,
                    CastFailureReason::TARGET_NO_WEAPONS => SMSG_CAST_RESULTCastFailureReason::TARGET_NO_WEAPONS,
                    CastFailureReason::TARGET_UNSKINNABLE => SMSG_CAST_RESULTCastFailureReason::TARGET_UNSKINNABLE,
                    CastFailureReason::THIRST_SATIATED => SMSG_CAST_RESULTCastFailureReason::THIRST_SATIATED,
                    CastFailureReason::TOO_CLOSE => SMSG_CAST_RESULTCastFailureReason::TOO_CLOSE,
                    CastFailureReason::TOO_MANY_OF_ITEM => SMSG_CAST_RESULTCastFailureReason::TOO_MANY_OF_ITEM,
                    CastFailureReason::TOTEMS => SMSG_CAST_RESULTCastFailureReason::TOTEMS,
                    CastFailureReason::TRAINING_POINTS => SMSG_CAST_RESULTCastFailureReason::TRAINING_POINTS,
                    CastFailureReason::TRY_AGAIN => SMSG_CAST_RESULTCastFailureReason::TRY_AGAIN,
                    CastFailureReason::UNIT_NOT_BEHIND => SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_BEHIND,
                    CastFailureReason::UNIT_NOT_INFRONT => SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_INFRONT,
                    CastFailureReason::WRONG_PET_FOOD => SMSG_CAST_RESULTCastFailureReason::WRONG_PET_FOOD,
                    CastFailureReason::NOT_WHILE_FATIGUED => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_FATIGUED,
                    CastFailureReason::TARGET_NOT_IN_INSTANCE => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_INSTANCE,
                    CastFailureReason::NOT_WHILE_TRADING => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_TRADING,
                    CastFailureReason::TARGET_NOT_IN_RAID => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_RAID,
                    CastFailureReason::DISENCHANT_WHILE_LOOTING => SMSG_CAST_RESULTCastFailureReason::DISENCHANT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_WHILE_LOOTING => SMSG_CAST_RESULTCastFailureReason::PROSPECT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_NEED_MORE => SMSG_CAST_RESULTCastFailureReason::PROSPECT_NEED_MORE,
                    CastFailureReason::TARGET_FREEFORALL => SMSG_CAST_RESULTCastFailureReason::TARGET_FREEFORALL,
                    CastFailureReason::NO_EDIBLE_CORPSES => SMSG_CAST_RESULTCastFailureReason::NO_EDIBLE_CORPSES,
                    CastFailureReason::ONLY_BATTLEGROUNDS => SMSG_CAST_RESULTCastFailureReason::ONLY_BATTLEGROUNDS,
                    CastFailureReason::TARGET_NOT_GHOST => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_GHOST,
                    CastFailureReason::TOO_MANY_SKILLS => SMSG_CAST_RESULTCastFailureReason::TOO_MANY_SKILLS,
                    CastFailureReason::TRANSFORM_UNUSABLE => SMSG_CAST_RESULTCastFailureReason::TRANSFORM_UNUSABLE,
                    CastFailureReason::WRONG_WEATHER => SMSG_CAST_RESULTCastFailureReason::WRONG_WEATHER,
                    CastFailureReason::DAMAGE_IMMUNE => SMSG_CAST_RESULTCastFailureReason::DAMAGE_IMMUNE,
                    CastFailureReason::PREVENTED_BY_MECHANIC => SMSG_CAST_RESULTCastFailureReason::PREVENTED_BY_MECHANIC,
                    CastFailureReason::PLAY_TIME => SMSG_CAST_RESULTCastFailureReason::PLAY_TIME,
                    CastFailureReason::REPUTATION => SMSG_CAST_RESULTCastFailureReason::REPUTATION,
                    CastFailureReason::MIN_SKILL => SMSG_CAST_RESULTCastFailureReason::MIN_SKILL,
                    CastFailureReason::UNKNOWN => SMSG_CAST_RESULTCastFailureReason::UNKNOWN,
                };

                SMSG_CAST_RESULTSimpleSpellCastResult::SUCCESS {
                    reason: reason_if,
                }
            }
            SimpleSpellCastResult::FAILURE => SMSG_CAST_RESULTSimpleSpellCastResult::FAILURE,
        };

        Ok(Self {
            spell,
            result: result_if,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // result: SimpleSpellCastResult
        self.result.write(w)?;

        match &self.result {
            SMSG_CAST_RESULTSimpleSpellCastResult::SUCCESS {
                reason,
            } => {
                // reason: CastFailureReason
                reason.write(w)?;

                match &reason {
                    SMSG_CAST_RESULTCastFailureReason::AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_HEALTH => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_POWER => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_BEING_TAMED => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_CHARM => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_SUMMON => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_OPEN => {}
                    SMSG_CAST_RESULTCastFailureReason::AURA_BOUNCED => {}
                    SMSG_CAST_RESULTCastFailureReason::AUTOTRACK_INTERRUPTED => {}
                    SMSG_CAST_RESULTCastFailureReason::BAD_IMPLICIT_TARGETS => {}
                    SMSG_CAST_RESULTCastFailureReason::BAD_TARGETS => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_CHARMED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_DISENCHANTED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_PROSPECTED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_CAST_ON_TAPPED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_INVISIBLE => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_STEALTHED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_STEALTH => {}
                    SMSG_CAST_RESULTCastFailureReason::CASTER_AURASTATE => {}
                    SMSG_CAST_RESULTCastFailureReason::CASTER_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::CHARMED => {}
                    SMSG_CAST_RESULTCastFailureReason::CHEST_IN_USE => {}
                    SMSG_CAST_RESULTCastFailureReason::CONFUSED => {}
                    SMSG_CAST_RESULTCastFailureReason::DONT_REPORT => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS {
                        equipped_item_class,
                        equipped_item_subclass_mask,
                        equipped_item_inventory_type_mask,
                    } => {
                        // equipped_item_class: u32
                        w.write_all(&equipped_item_class.to_le_bytes())?;

                        // equipped_item_subclass_mask: u32
                        w.write_all(&equipped_item_subclass_mask.to_le_bytes())?;

                        // equipped_item_inventory_type_mask: u32
                        w.write_all(&equipped_item_inventory_type_mask.to_le_bytes())?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => {}
                    SMSG_CAST_RESULTCastFailureReason::ERROR => {}
                    SMSG_CAST_RESULTCastFailureReason::FIZZLE => {}
                    SMSG_CAST_RESULTCastFailureReason::FLEEING => {}
                    SMSG_CAST_RESULTCastFailureReason::FOOD_LOWLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::HIGHLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::HUNGER_SATIATED => {}
                    SMSG_CAST_RESULTCastFailureReason::IMMUNE => {}
                    SMSG_CAST_RESULTCastFailureReason::INTERRUPTED => {}
                    SMSG_CAST_RESULTCastFailureReason::INTERRUPTED_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_ALREADY_ENCHANTED => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_GONE => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_FOUND => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_READY => {}
                    SMSG_CAST_RESULTCastFailureReason::LEVEL_REQUIREMENT => {}
                    SMSG_CAST_RESULTCastFailureReason::LINE_OF_SIGHT => {}
                    SMSG_CAST_RESULTCastFailureReason::LOWLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::LOW_CASTLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::MAINHAND_EMPTY => {}
                    SMSG_CAST_RESULTCastFailureReason::MOVING => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_AMMO_POUCH => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_EXOTIC_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NOPATH => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_BEHIND => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_FISHABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_HERE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_INFRONT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_IN_CONTROL => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_KNOWN => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_MOUNTED => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_ON_TAXI => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_ON_TRANSPORT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_READY => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_SHAPESHIFT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_STANDING => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_TRADEABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_TRADING => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_UNSHEATHED => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_GHOST => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_CHARGES_REMAIN => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_CHAMPION => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_COMBO_POINTS => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_DUELING => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_ENDURANCE => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_FISH => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_MOUNTS_ALLOWED => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_PET => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_POWER => {}
                    SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_DISPEL => {}
                    SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_STEAL => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_ABOVEWATER => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_DAYTIME => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_INDOORS => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_MOUNTED => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_NIGHTTIME => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_OUTDOORS => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_SHAPESHIFT => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_STEALTHED => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_UNDERWATER => {}
                    SMSG_CAST_RESULTCastFailureReason::OUT_OF_RANGE => {}
                    SMSG_CAST_RESULTCastFailureReason::PACIFIED => {}
                    SMSG_CAST_RESULTCastFailureReason::POSSESSED => {}
                    SMSG_CAST_RESULTCastFailureReason::REAGENTS => {}
                    SMSG_CAST_RESULTCastFailureReason::REQUIRES_AREA {
                        area,
                    } => {
                        // area: Area
                        area.write(w)?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::REQUIRES_SPELL_FOCUS {
                        required_spell_focus,
                    } => {
                        // required_spell_focus: u32
                        w.write_all(&required_spell_focus.to_le_bytes())?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::ROOTED => {}
                    SMSG_CAST_RESULTCastFailureReason::SILENCED => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_IN_PROGRESS => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_LEARNED => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_UNAVAILABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::STUNNED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGETS_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_AURASTATE => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_DUELING => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_ENEMY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_ENRAGED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_FRIENDLY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_IN_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_IS_PLAYER => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_PARTY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_LOOTED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_PLAYER => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NO_POCKETS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NO_WEAPONS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_UNSKINNABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::THIRST_SATIATED => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_CLOSE => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_MANY_OF_ITEM => {}
                    SMSG_CAST_RESULTCastFailureReason::TOTEMS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRAINING_POINTS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRY_AGAIN => {}
                    SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_BEHIND => {}
                    SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_INFRONT => {}
                    SMSG_CAST_RESULTCastFailureReason::WRONG_PET_FOOD => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_FATIGUED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_INSTANCE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_TRADING => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_RAID => {}
                    SMSG_CAST_RESULTCastFailureReason::DISENCHANT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULTCastFailureReason::PROSPECT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULTCastFailureReason::PROSPECT_NEED_MORE => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_FREEFORALL => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_EDIBLE_CORPSES => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_BATTLEGROUNDS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_GHOST => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_MANY_SKILLS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRANSFORM_UNUSABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::WRONG_WEATHER => {}
                    SMSG_CAST_RESULTCastFailureReason::DAMAGE_IMMUNE => {}
                    SMSG_CAST_RESULTCastFailureReason::PREVENTED_BY_MECHANIC => {}
                    SMSG_CAST_RESULTCastFailureReason::PLAY_TIME => {}
                    SMSG_CAST_RESULTCastFailureReason::REPUTATION => {}
                    SMSG_CAST_RESULTCastFailureReason::MIN_SKILL => {}
                    SMSG_CAST_RESULTCastFailureReason::UNKNOWN => {}
                }

            }
            SMSG_CAST_RESULTSimpleSpellCastResult::FAILURE => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::tokio_read_u32_le(r).await?;

        // result: SimpleSpellCastResult
        let result = SimpleSpellCastResult::tokio_read(r).await?;

        let result_if = match result {
            SimpleSpellCastResult::SUCCESS => {
                // reason: CastFailureReason
                let reason = CastFailureReason::tokio_read(r).await?;

                let reason_if = match reason {
                    CastFailureReason::AFFECTING_COMBAT => SMSG_CAST_RESULTCastFailureReason::AFFECTING_COMBAT,
                    CastFailureReason::ALREADY_AT_FULL_HEALTH => SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_HEALTH,
                    CastFailureReason::ALREADY_AT_FULL_POWER => SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_POWER,
                    CastFailureReason::ALREADY_BEING_TAMED => SMSG_CAST_RESULTCastFailureReason::ALREADY_BEING_TAMED,
                    CastFailureReason::ALREADY_HAVE_CHARM => SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_CHARM,
                    CastFailureReason::ALREADY_HAVE_SUMMON => SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_SUMMON,
                    CastFailureReason::ALREADY_OPEN => SMSG_CAST_RESULTCastFailureReason::ALREADY_OPEN,
                    CastFailureReason::AURA_BOUNCED => SMSG_CAST_RESULTCastFailureReason::AURA_BOUNCED,
                    CastFailureReason::AUTOTRACK_INTERRUPTED => SMSG_CAST_RESULTCastFailureReason::AUTOTRACK_INTERRUPTED,
                    CastFailureReason::BAD_IMPLICIT_TARGETS => SMSG_CAST_RESULTCastFailureReason::BAD_IMPLICIT_TARGETS,
                    CastFailureReason::BAD_TARGETS => SMSG_CAST_RESULTCastFailureReason::BAD_TARGETS,
                    CastFailureReason::CANT_BE_CHARMED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_CHARMED,
                    CastFailureReason::CANT_BE_DISENCHANTED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_DISENCHANTED,
                    CastFailureReason::CANT_BE_PROSPECTED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_PROSPECTED,
                    CastFailureReason::CANT_CAST_ON_TAPPED => SMSG_CAST_RESULTCastFailureReason::CANT_CAST_ON_TAPPED,
                    CastFailureReason::CANT_DUEL_WHILE_INVISIBLE => SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_INVISIBLE,
                    CastFailureReason::CANT_DUEL_WHILE_STEALTHED => SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_STEALTHED,
                    CastFailureReason::CANT_STEALTH => SMSG_CAST_RESULTCastFailureReason::CANT_STEALTH,
                    CastFailureReason::CASTER_AURASTATE => SMSG_CAST_RESULTCastFailureReason::CASTER_AURASTATE,
                    CastFailureReason::CASTER_DEAD => SMSG_CAST_RESULTCastFailureReason::CASTER_DEAD,
                    CastFailureReason::CHARMED => SMSG_CAST_RESULTCastFailureReason::CHARMED,
                    CastFailureReason::CHEST_IN_USE => SMSG_CAST_RESULTCastFailureReason::CHEST_IN_USE,
                    CastFailureReason::CONFUSED => SMSG_CAST_RESULTCastFailureReason::CONFUSED,
                    CastFailureReason::DONT_REPORT => SMSG_CAST_RESULTCastFailureReason::DONT_REPORT,
                    CastFailureReason::EQUIPPED_ITEM => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM,
                    CastFailureReason::EQUIPPED_ITEM_CLASS => {
                        // equipped_item_class: u32
                        let equipped_item_class = crate::util::tokio_read_u32_le(r).await?;

                        // equipped_item_subclass_mask: u32
                        let equipped_item_subclass_mask = crate::util::tokio_read_u32_le(r).await?;

                        // equipped_item_inventory_type_mask: u32
                        let equipped_item_inventory_type_mask = crate::util::tokio_read_u32_le(r).await?;

                        SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS {
                            equipped_item_class,
                            equipped_item_subclass_mask,
                            equipped_item_inventory_type_mask,
                        }
                    }
                    CastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND,
                    CastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND,
                    CastFailureReason::ERROR => SMSG_CAST_RESULTCastFailureReason::ERROR,
                    CastFailureReason::FIZZLE => SMSG_CAST_RESULTCastFailureReason::FIZZLE,
                    CastFailureReason::FLEEING => SMSG_CAST_RESULTCastFailureReason::FLEEING,
                    CastFailureReason::FOOD_LOWLEVEL => SMSG_CAST_RESULTCastFailureReason::FOOD_LOWLEVEL,
                    CastFailureReason::HIGHLEVEL => SMSG_CAST_RESULTCastFailureReason::HIGHLEVEL,
                    CastFailureReason::HUNGER_SATIATED => SMSG_CAST_RESULTCastFailureReason::HUNGER_SATIATED,
                    CastFailureReason::IMMUNE => SMSG_CAST_RESULTCastFailureReason::IMMUNE,
                    CastFailureReason::INTERRUPTED => SMSG_CAST_RESULTCastFailureReason::INTERRUPTED,
                    CastFailureReason::INTERRUPTED_COMBAT => SMSG_CAST_RESULTCastFailureReason::INTERRUPTED_COMBAT,
                    CastFailureReason::ITEM_ALREADY_ENCHANTED => SMSG_CAST_RESULTCastFailureReason::ITEM_ALREADY_ENCHANTED,
                    CastFailureReason::ITEM_GONE => SMSG_CAST_RESULTCastFailureReason::ITEM_GONE,
                    CastFailureReason::ITEM_NOT_FOUND => SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_FOUND,
                    CastFailureReason::ITEM_NOT_READY => SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_READY,
                    CastFailureReason::LEVEL_REQUIREMENT => SMSG_CAST_RESULTCastFailureReason::LEVEL_REQUIREMENT,
                    CastFailureReason::LINE_OF_SIGHT => SMSG_CAST_RESULTCastFailureReason::LINE_OF_SIGHT,
                    CastFailureReason::LOWLEVEL => SMSG_CAST_RESULTCastFailureReason::LOWLEVEL,
                    CastFailureReason::LOW_CASTLEVEL => SMSG_CAST_RESULTCastFailureReason::LOW_CASTLEVEL,
                    CastFailureReason::MAINHAND_EMPTY => SMSG_CAST_RESULTCastFailureReason::MAINHAND_EMPTY,
                    CastFailureReason::MOVING => SMSG_CAST_RESULTCastFailureReason::MOVING,
                    CastFailureReason::NEED_AMMO => SMSG_CAST_RESULTCastFailureReason::NEED_AMMO,
                    CastFailureReason::NEED_AMMO_POUCH => SMSG_CAST_RESULTCastFailureReason::NEED_AMMO_POUCH,
                    CastFailureReason::NEED_EXOTIC_AMMO => SMSG_CAST_RESULTCastFailureReason::NEED_EXOTIC_AMMO,
                    CastFailureReason::NOPATH => SMSG_CAST_RESULTCastFailureReason::NOPATH,
                    CastFailureReason::NOT_BEHIND => SMSG_CAST_RESULTCastFailureReason::NOT_BEHIND,
                    CastFailureReason::NOT_FISHABLE => SMSG_CAST_RESULTCastFailureReason::NOT_FISHABLE,
                    CastFailureReason::NOT_HERE => SMSG_CAST_RESULTCastFailureReason::NOT_HERE,
                    CastFailureReason::NOT_INFRONT => SMSG_CAST_RESULTCastFailureReason::NOT_INFRONT,
                    CastFailureReason::NOT_IN_CONTROL => SMSG_CAST_RESULTCastFailureReason::NOT_IN_CONTROL,
                    CastFailureReason::NOT_KNOWN => SMSG_CAST_RESULTCastFailureReason::NOT_KNOWN,
                    CastFailureReason::NOT_MOUNTED => SMSG_CAST_RESULTCastFailureReason::NOT_MOUNTED,
                    CastFailureReason::NOT_ON_TAXI => SMSG_CAST_RESULTCastFailureReason::NOT_ON_TAXI,
                    CastFailureReason::NOT_ON_TRANSPORT => SMSG_CAST_RESULTCastFailureReason::NOT_ON_TRANSPORT,
                    CastFailureReason::NOT_READY => SMSG_CAST_RESULTCastFailureReason::NOT_READY,
                    CastFailureReason::NOT_SHAPESHIFT => SMSG_CAST_RESULTCastFailureReason::NOT_SHAPESHIFT,
                    CastFailureReason::NOT_STANDING => SMSG_CAST_RESULTCastFailureReason::NOT_STANDING,
                    CastFailureReason::NOT_TRADEABLE => SMSG_CAST_RESULTCastFailureReason::NOT_TRADEABLE,
                    CastFailureReason::NOT_TRADING => SMSG_CAST_RESULTCastFailureReason::NOT_TRADING,
                    CastFailureReason::NOT_UNSHEATHED => SMSG_CAST_RESULTCastFailureReason::NOT_UNSHEATHED,
                    CastFailureReason::NOT_WHILE_GHOST => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_GHOST,
                    CastFailureReason::NO_AMMO => SMSG_CAST_RESULTCastFailureReason::NO_AMMO,
                    CastFailureReason::NO_CHARGES_REMAIN => SMSG_CAST_RESULTCastFailureReason::NO_CHARGES_REMAIN,
                    CastFailureReason::NO_CHAMPION => SMSG_CAST_RESULTCastFailureReason::NO_CHAMPION,
                    CastFailureReason::NO_COMBO_POINTS => SMSG_CAST_RESULTCastFailureReason::NO_COMBO_POINTS,
                    CastFailureReason::NO_DUELING => SMSG_CAST_RESULTCastFailureReason::NO_DUELING,
                    CastFailureReason::NO_ENDURANCE => SMSG_CAST_RESULTCastFailureReason::NO_ENDURANCE,
                    CastFailureReason::NO_FISH => SMSG_CAST_RESULTCastFailureReason::NO_FISH,
                    CastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => SMSG_CAST_RESULTCastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED,
                    CastFailureReason::NO_MOUNTS_ALLOWED => SMSG_CAST_RESULTCastFailureReason::NO_MOUNTS_ALLOWED,
                    CastFailureReason::NO_PET => SMSG_CAST_RESULTCastFailureReason::NO_PET,
                    CastFailureReason::NO_POWER => SMSG_CAST_RESULTCastFailureReason::NO_POWER,
                    CastFailureReason::NOTHING_TO_DISPEL => SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_DISPEL,
                    CastFailureReason::NOTHING_TO_STEAL => SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_STEAL,
                    CastFailureReason::ONLY_ABOVEWATER => SMSG_CAST_RESULTCastFailureReason::ONLY_ABOVEWATER,
                    CastFailureReason::ONLY_DAYTIME => SMSG_CAST_RESULTCastFailureReason::ONLY_DAYTIME,
                    CastFailureReason::ONLY_INDOORS => SMSG_CAST_RESULTCastFailureReason::ONLY_INDOORS,
                    CastFailureReason::ONLY_MOUNTED => SMSG_CAST_RESULTCastFailureReason::ONLY_MOUNTED,
                    CastFailureReason::ONLY_NIGHTTIME => SMSG_CAST_RESULTCastFailureReason::ONLY_NIGHTTIME,
                    CastFailureReason::ONLY_OUTDOORS => SMSG_CAST_RESULTCastFailureReason::ONLY_OUTDOORS,
                    CastFailureReason::ONLY_SHAPESHIFT => SMSG_CAST_RESULTCastFailureReason::ONLY_SHAPESHIFT,
                    CastFailureReason::ONLY_STEALTHED => SMSG_CAST_RESULTCastFailureReason::ONLY_STEALTHED,
                    CastFailureReason::ONLY_UNDERWATER => SMSG_CAST_RESULTCastFailureReason::ONLY_UNDERWATER,
                    CastFailureReason::OUT_OF_RANGE => SMSG_CAST_RESULTCastFailureReason::OUT_OF_RANGE,
                    CastFailureReason::PACIFIED => SMSG_CAST_RESULTCastFailureReason::PACIFIED,
                    CastFailureReason::POSSESSED => SMSG_CAST_RESULTCastFailureReason::POSSESSED,
                    CastFailureReason::REAGENTS => SMSG_CAST_RESULTCastFailureReason::REAGENTS,
                    CastFailureReason::REQUIRES_AREA => {
                        // area: Area
                        let area = Area::tokio_read(r).await?;

                        SMSG_CAST_RESULTCastFailureReason::REQUIRES_AREA {
                            area,
                        }
                    }
                    CastFailureReason::REQUIRES_SPELL_FOCUS => {
                        // required_spell_focus: u32
                        let required_spell_focus = crate::util::tokio_read_u32_le(r).await?;

                        SMSG_CAST_RESULTCastFailureReason::REQUIRES_SPELL_FOCUS {
                            required_spell_focus,
                        }
                    }
                    CastFailureReason::ROOTED => SMSG_CAST_RESULTCastFailureReason::ROOTED,
                    CastFailureReason::SILENCED => SMSG_CAST_RESULTCastFailureReason::SILENCED,
                    CastFailureReason::SPELL_IN_PROGRESS => SMSG_CAST_RESULTCastFailureReason::SPELL_IN_PROGRESS,
                    CastFailureReason::SPELL_LEARNED => SMSG_CAST_RESULTCastFailureReason::SPELL_LEARNED,
                    CastFailureReason::SPELL_UNAVAILABLE => SMSG_CAST_RESULTCastFailureReason::SPELL_UNAVAILABLE,
                    CastFailureReason::STUNNED => SMSG_CAST_RESULTCastFailureReason::STUNNED,
                    CastFailureReason::TARGETS_DEAD => SMSG_CAST_RESULTCastFailureReason::TARGETS_DEAD,
                    CastFailureReason::TARGET_AFFECTING_COMBAT => SMSG_CAST_RESULTCastFailureReason::TARGET_AFFECTING_COMBAT,
                    CastFailureReason::TARGET_AURASTATE => SMSG_CAST_RESULTCastFailureReason::TARGET_AURASTATE,
                    CastFailureReason::TARGET_DUELING => SMSG_CAST_RESULTCastFailureReason::TARGET_DUELING,
                    CastFailureReason::TARGET_ENEMY => SMSG_CAST_RESULTCastFailureReason::TARGET_ENEMY,
                    CastFailureReason::TARGET_ENRAGED => SMSG_CAST_RESULTCastFailureReason::TARGET_ENRAGED,
                    CastFailureReason::TARGET_FRIENDLY => SMSG_CAST_RESULTCastFailureReason::TARGET_FRIENDLY,
                    CastFailureReason::TARGET_IN_COMBAT => SMSG_CAST_RESULTCastFailureReason::TARGET_IN_COMBAT,
                    CastFailureReason::TARGET_IS_PLAYER => SMSG_CAST_RESULTCastFailureReason::TARGET_IS_PLAYER,
                    CastFailureReason::TARGET_NOT_DEAD => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_DEAD,
                    CastFailureReason::TARGET_NOT_IN_PARTY => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_PARTY,
                    CastFailureReason::TARGET_NOT_LOOTED => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_LOOTED,
                    CastFailureReason::TARGET_NOT_PLAYER => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_PLAYER,
                    CastFailureReason::TARGET_NO_POCKETS => SMSG_CAST_RESULTCastFailureReason::TARGET_NO_POCKETS,
                    CastFailureReason::TARGET_NO_WEAPONS => SMSG_CAST_RESULTCastFailureReason::TARGET_NO_WEAPONS,
                    CastFailureReason::TARGET_UNSKINNABLE => SMSG_CAST_RESULTCastFailureReason::TARGET_UNSKINNABLE,
                    CastFailureReason::THIRST_SATIATED => SMSG_CAST_RESULTCastFailureReason::THIRST_SATIATED,
                    CastFailureReason::TOO_CLOSE => SMSG_CAST_RESULTCastFailureReason::TOO_CLOSE,
                    CastFailureReason::TOO_MANY_OF_ITEM => SMSG_CAST_RESULTCastFailureReason::TOO_MANY_OF_ITEM,
                    CastFailureReason::TOTEMS => SMSG_CAST_RESULTCastFailureReason::TOTEMS,
                    CastFailureReason::TRAINING_POINTS => SMSG_CAST_RESULTCastFailureReason::TRAINING_POINTS,
                    CastFailureReason::TRY_AGAIN => SMSG_CAST_RESULTCastFailureReason::TRY_AGAIN,
                    CastFailureReason::UNIT_NOT_BEHIND => SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_BEHIND,
                    CastFailureReason::UNIT_NOT_INFRONT => SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_INFRONT,
                    CastFailureReason::WRONG_PET_FOOD => SMSG_CAST_RESULTCastFailureReason::WRONG_PET_FOOD,
                    CastFailureReason::NOT_WHILE_FATIGUED => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_FATIGUED,
                    CastFailureReason::TARGET_NOT_IN_INSTANCE => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_INSTANCE,
                    CastFailureReason::NOT_WHILE_TRADING => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_TRADING,
                    CastFailureReason::TARGET_NOT_IN_RAID => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_RAID,
                    CastFailureReason::DISENCHANT_WHILE_LOOTING => SMSG_CAST_RESULTCastFailureReason::DISENCHANT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_WHILE_LOOTING => SMSG_CAST_RESULTCastFailureReason::PROSPECT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_NEED_MORE => SMSG_CAST_RESULTCastFailureReason::PROSPECT_NEED_MORE,
                    CastFailureReason::TARGET_FREEFORALL => SMSG_CAST_RESULTCastFailureReason::TARGET_FREEFORALL,
                    CastFailureReason::NO_EDIBLE_CORPSES => SMSG_CAST_RESULTCastFailureReason::NO_EDIBLE_CORPSES,
                    CastFailureReason::ONLY_BATTLEGROUNDS => SMSG_CAST_RESULTCastFailureReason::ONLY_BATTLEGROUNDS,
                    CastFailureReason::TARGET_NOT_GHOST => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_GHOST,
                    CastFailureReason::TOO_MANY_SKILLS => SMSG_CAST_RESULTCastFailureReason::TOO_MANY_SKILLS,
                    CastFailureReason::TRANSFORM_UNUSABLE => SMSG_CAST_RESULTCastFailureReason::TRANSFORM_UNUSABLE,
                    CastFailureReason::WRONG_WEATHER => SMSG_CAST_RESULTCastFailureReason::WRONG_WEATHER,
                    CastFailureReason::DAMAGE_IMMUNE => SMSG_CAST_RESULTCastFailureReason::DAMAGE_IMMUNE,
                    CastFailureReason::PREVENTED_BY_MECHANIC => SMSG_CAST_RESULTCastFailureReason::PREVENTED_BY_MECHANIC,
                    CastFailureReason::PLAY_TIME => SMSG_CAST_RESULTCastFailureReason::PLAY_TIME,
                    CastFailureReason::REPUTATION => SMSG_CAST_RESULTCastFailureReason::REPUTATION,
                    CastFailureReason::MIN_SKILL => SMSG_CAST_RESULTCastFailureReason::MIN_SKILL,
                    CastFailureReason::UNKNOWN => SMSG_CAST_RESULTCastFailureReason::UNKNOWN,
                };

                SMSG_CAST_RESULTSimpleSpellCastResult::SUCCESS {
                    reason: reason_if,
                }
            }
            SimpleSpellCastResult::FAILURE => SMSG_CAST_RESULTSimpleSpellCastResult::FAILURE,
        };

        Ok(Self {
            spell,
            result: result_if,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes()).await?;

        // result: SimpleSpellCastResult
        self.result.tokio_write(w).await?;

        match &self.result {
            SMSG_CAST_RESULTSimpleSpellCastResult::SUCCESS {
                reason,
            } => {
                // reason: CastFailureReason
                reason.tokio_write(w).await?;

                match &reason {
                    SMSG_CAST_RESULTCastFailureReason::AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_HEALTH => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_POWER => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_BEING_TAMED => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_CHARM => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_SUMMON => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_OPEN => {}
                    SMSG_CAST_RESULTCastFailureReason::AURA_BOUNCED => {}
                    SMSG_CAST_RESULTCastFailureReason::AUTOTRACK_INTERRUPTED => {}
                    SMSG_CAST_RESULTCastFailureReason::BAD_IMPLICIT_TARGETS => {}
                    SMSG_CAST_RESULTCastFailureReason::BAD_TARGETS => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_CHARMED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_DISENCHANTED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_PROSPECTED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_CAST_ON_TAPPED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_INVISIBLE => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_STEALTHED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_STEALTH => {}
                    SMSG_CAST_RESULTCastFailureReason::CASTER_AURASTATE => {}
                    SMSG_CAST_RESULTCastFailureReason::CASTER_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::CHARMED => {}
                    SMSG_CAST_RESULTCastFailureReason::CHEST_IN_USE => {}
                    SMSG_CAST_RESULTCastFailureReason::CONFUSED => {}
                    SMSG_CAST_RESULTCastFailureReason::DONT_REPORT => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS {
                        equipped_item_class,
                        equipped_item_subclass_mask,
                        equipped_item_inventory_type_mask,
                    } => {
                        // equipped_item_class: u32
                        w.write_all(&equipped_item_class.to_le_bytes()).await?;

                        // equipped_item_subclass_mask: u32
                        w.write_all(&equipped_item_subclass_mask.to_le_bytes()).await?;

                        // equipped_item_inventory_type_mask: u32
                        w.write_all(&equipped_item_inventory_type_mask.to_le_bytes()).await?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => {}
                    SMSG_CAST_RESULTCastFailureReason::ERROR => {}
                    SMSG_CAST_RESULTCastFailureReason::FIZZLE => {}
                    SMSG_CAST_RESULTCastFailureReason::FLEEING => {}
                    SMSG_CAST_RESULTCastFailureReason::FOOD_LOWLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::HIGHLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::HUNGER_SATIATED => {}
                    SMSG_CAST_RESULTCastFailureReason::IMMUNE => {}
                    SMSG_CAST_RESULTCastFailureReason::INTERRUPTED => {}
                    SMSG_CAST_RESULTCastFailureReason::INTERRUPTED_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_ALREADY_ENCHANTED => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_GONE => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_FOUND => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_READY => {}
                    SMSG_CAST_RESULTCastFailureReason::LEVEL_REQUIREMENT => {}
                    SMSG_CAST_RESULTCastFailureReason::LINE_OF_SIGHT => {}
                    SMSG_CAST_RESULTCastFailureReason::LOWLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::LOW_CASTLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::MAINHAND_EMPTY => {}
                    SMSG_CAST_RESULTCastFailureReason::MOVING => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_AMMO_POUCH => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_EXOTIC_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NOPATH => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_BEHIND => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_FISHABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_HERE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_INFRONT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_IN_CONTROL => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_KNOWN => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_MOUNTED => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_ON_TAXI => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_ON_TRANSPORT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_READY => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_SHAPESHIFT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_STANDING => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_TRADEABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_TRADING => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_UNSHEATHED => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_GHOST => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_CHARGES_REMAIN => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_CHAMPION => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_COMBO_POINTS => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_DUELING => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_ENDURANCE => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_FISH => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_MOUNTS_ALLOWED => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_PET => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_POWER => {}
                    SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_DISPEL => {}
                    SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_STEAL => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_ABOVEWATER => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_DAYTIME => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_INDOORS => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_MOUNTED => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_NIGHTTIME => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_OUTDOORS => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_SHAPESHIFT => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_STEALTHED => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_UNDERWATER => {}
                    SMSG_CAST_RESULTCastFailureReason::OUT_OF_RANGE => {}
                    SMSG_CAST_RESULTCastFailureReason::PACIFIED => {}
                    SMSG_CAST_RESULTCastFailureReason::POSSESSED => {}
                    SMSG_CAST_RESULTCastFailureReason::REAGENTS => {}
                    SMSG_CAST_RESULTCastFailureReason::REQUIRES_AREA {
                        area,
                    } => {
                        // area: Area
                        area.tokio_write(w).await?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::REQUIRES_SPELL_FOCUS {
                        required_spell_focus,
                    } => {
                        // required_spell_focus: u32
                        w.write_all(&required_spell_focus.to_le_bytes()).await?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::ROOTED => {}
                    SMSG_CAST_RESULTCastFailureReason::SILENCED => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_IN_PROGRESS => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_LEARNED => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_UNAVAILABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::STUNNED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGETS_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_AURASTATE => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_DUELING => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_ENEMY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_ENRAGED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_FRIENDLY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_IN_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_IS_PLAYER => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_PARTY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_LOOTED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_PLAYER => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NO_POCKETS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NO_WEAPONS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_UNSKINNABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::THIRST_SATIATED => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_CLOSE => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_MANY_OF_ITEM => {}
                    SMSG_CAST_RESULTCastFailureReason::TOTEMS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRAINING_POINTS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRY_AGAIN => {}
                    SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_BEHIND => {}
                    SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_INFRONT => {}
                    SMSG_CAST_RESULTCastFailureReason::WRONG_PET_FOOD => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_FATIGUED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_INSTANCE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_TRADING => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_RAID => {}
                    SMSG_CAST_RESULTCastFailureReason::DISENCHANT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULTCastFailureReason::PROSPECT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULTCastFailureReason::PROSPECT_NEED_MORE => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_FREEFORALL => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_EDIBLE_CORPSES => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_BATTLEGROUNDS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_GHOST => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_MANY_SKILLS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRANSFORM_UNUSABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::WRONG_WEATHER => {}
                    SMSG_CAST_RESULTCastFailureReason::DAMAGE_IMMUNE => {}
                    SMSG_CAST_RESULTCastFailureReason::PREVENTED_BY_MECHANIC => {}
                    SMSG_CAST_RESULTCastFailureReason::PLAY_TIME => {}
                    SMSG_CAST_RESULTCastFailureReason::REPUTATION => {}
                    SMSG_CAST_RESULTCastFailureReason::MIN_SKILL => {}
                    SMSG_CAST_RESULTCastFailureReason::UNKNOWN => {}
                }

            }
            SMSG_CAST_RESULTSimpleSpellCastResult::FAILURE => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::astd_read_u32_le(r).await?;

        // result: SimpleSpellCastResult
        let result = SimpleSpellCastResult::astd_read(r).await?;

        let result_if = match result {
            SimpleSpellCastResult::SUCCESS => {
                // reason: CastFailureReason
                let reason = CastFailureReason::astd_read(r).await?;

                let reason_if = match reason {
                    CastFailureReason::AFFECTING_COMBAT => SMSG_CAST_RESULTCastFailureReason::AFFECTING_COMBAT,
                    CastFailureReason::ALREADY_AT_FULL_HEALTH => SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_HEALTH,
                    CastFailureReason::ALREADY_AT_FULL_POWER => SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_POWER,
                    CastFailureReason::ALREADY_BEING_TAMED => SMSG_CAST_RESULTCastFailureReason::ALREADY_BEING_TAMED,
                    CastFailureReason::ALREADY_HAVE_CHARM => SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_CHARM,
                    CastFailureReason::ALREADY_HAVE_SUMMON => SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_SUMMON,
                    CastFailureReason::ALREADY_OPEN => SMSG_CAST_RESULTCastFailureReason::ALREADY_OPEN,
                    CastFailureReason::AURA_BOUNCED => SMSG_CAST_RESULTCastFailureReason::AURA_BOUNCED,
                    CastFailureReason::AUTOTRACK_INTERRUPTED => SMSG_CAST_RESULTCastFailureReason::AUTOTRACK_INTERRUPTED,
                    CastFailureReason::BAD_IMPLICIT_TARGETS => SMSG_CAST_RESULTCastFailureReason::BAD_IMPLICIT_TARGETS,
                    CastFailureReason::BAD_TARGETS => SMSG_CAST_RESULTCastFailureReason::BAD_TARGETS,
                    CastFailureReason::CANT_BE_CHARMED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_CHARMED,
                    CastFailureReason::CANT_BE_DISENCHANTED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_DISENCHANTED,
                    CastFailureReason::CANT_BE_PROSPECTED => SMSG_CAST_RESULTCastFailureReason::CANT_BE_PROSPECTED,
                    CastFailureReason::CANT_CAST_ON_TAPPED => SMSG_CAST_RESULTCastFailureReason::CANT_CAST_ON_TAPPED,
                    CastFailureReason::CANT_DUEL_WHILE_INVISIBLE => SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_INVISIBLE,
                    CastFailureReason::CANT_DUEL_WHILE_STEALTHED => SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_STEALTHED,
                    CastFailureReason::CANT_STEALTH => SMSG_CAST_RESULTCastFailureReason::CANT_STEALTH,
                    CastFailureReason::CASTER_AURASTATE => SMSG_CAST_RESULTCastFailureReason::CASTER_AURASTATE,
                    CastFailureReason::CASTER_DEAD => SMSG_CAST_RESULTCastFailureReason::CASTER_DEAD,
                    CastFailureReason::CHARMED => SMSG_CAST_RESULTCastFailureReason::CHARMED,
                    CastFailureReason::CHEST_IN_USE => SMSG_CAST_RESULTCastFailureReason::CHEST_IN_USE,
                    CastFailureReason::CONFUSED => SMSG_CAST_RESULTCastFailureReason::CONFUSED,
                    CastFailureReason::DONT_REPORT => SMSG_CAST_RESULTCastFailureReason::DONT_REPORT,
                    CastFailureReason::EQUIPPED_ITEM => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM,
                    CastFailureReason::EQUIPPED_ITEM_CLASS => {
                        // equipped_item_class: u32
                        let equipped_item_class = crate::util::astd_read_u32_le(r).await?;

                        // equipped_item_subclass_mask: u32
                        let equipped_item_subclass_mask = crate::util::astd_read_u32_le(r).await?;

                        // equipped_item_inventory_type_mask: u32
                        let equipped_item_inventory_type_mask = crate::util::astd_read_u32_le(r).await?;

                        SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS {
                            equipped_item_class,
                            equipped_item_subclass_mask,
                            equipped_item_inventory_type_mask,
                        }
                    }
                    CastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND,
                    CastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND,
                    CastFailureReason::ERROR => SMSG_CAST_RESULTCastFailureReason::ERROR,
                    CastFailureReason::FIZZLE => SMSG_CAST_RESULTCastFailureReason::FIZZLE,
                    CastFailureReason::FLEEING => SMSG_CAST_RESULTCastFailureReason::FLEEING,
                    CastFailureReason::FOOD_LOWLEVEL => SMSG_CAST_RESULTCastFailureReason::FOOD_LOWLEVEL,
                    CastFailureReason::HIGHLEVEL => SMSG_CAST_RESULTCastFailureReason::HIGHLEVEL,
                    CastFailureReason::HUNGER_SATIATED => SMSG_CAST_RESULTCastFailureReason::HUNGER_SATIATED,
                    CastFailureReason::IMMUNE => SMSG_CAST_RESULTCastFailureReason::IMMUNE,
                    CastFailureReason::INTERRUPTED => SMSG_CAST_RESULTCastFailureReason::INTERRUPTED,
                    CastFailureReason::INTERRUPTED_COMBAT => SMSG_CAST_RESULTCastFailureReason::INTERRUPTED_COMBAT,
                    CastFailureReason::ITEM_ALREADY_ENCHANTED => SMSG_CAST_RESULTCastFailureReason::ITEM_ALREADY_ENCHANTED,
                    CastFailureReason::ITEM_GONE => SMSG_CAST_RESULTCastFailureReason::ITEM_GONE,
                    CastFailureReason::ITEM_NOT_FOUND => SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_FOUND,
                    CastFailureReason::ITEM_NOT_READY => SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_READY,
                    CastFailureReason::LEVEL_REQUIREMENT => SMSG_CAST_RESULTCastFailureReason::LEVEL_REQUIREMENT,
                    CastFailureReason::LINE_OF_SIGHT => SMSG_CAST_RESULTCastFailureReason::LINE_OF_SIGHT,
                    CastFailureReason::LOWLEVEL => SMSG_CAST_RESULTCastFailureReason::LOWLEVEL,
                    CastFailureReason::LOW_CASTLEVEL => SMSG_CAST_RESULTCastFailureReason::LOW_CASTLEVEL,
                    CastFailureReason::MAINHAND_EMPTY => SMSG_CAST_RESULTCastFailureReason::MAINHAND_EMPTY,
                    CastFailureReason::MOVING => SMSG_CAST_RESULTCastFailureReason::MOVING,
                    CastFailureReason::NEED_AMMO => SMSG_CAST_RESULTCastFailureReason::NEED_AMMO,
                    CastFailureReason::NEED_AMMO_POUCH => SMSG_CAST_RESULTCastFailureReason::NEED_AMMO_POUCH,
                    CastFailureReason::NEED_EXOTIC_AMMO => SMSG_CAST_RESULTCastFailureReason::NEED_EXOTIC_AMMO,
                    CastFailureReason::NOPATH => SMSG_CAST_RESULTCastFailureReason::NOPATH,
                    CastFailureReason::NOT_BEHIND => SMSG_CAST_RESULTCastFailureReason::NOT_BEHIND,
                    CastFailureReason::NOT_FISHABLE => SMSG_CAST_RESULTCastFailureReason::NOT_FISHABLE,
                    CastFailureReason::NOT_HERE => SMSG_CAST_RESULTCastFailureReason::NOT_HERE,
                    CastFailureReason::NOT_INFRONT => SMSG_CAST_RESULTCastFailureReason::NOT_INFRONT,
                    CastFailureReason::NOT_IN_CONTROL => SMSG_CAST_RESULTCastFailureReason::NOT_IN_CONTROL,
                    CastFailureReason::NOT_KNOWN => SMSG_CAST_RESULTCastFailureReason::NOT_KNOWN,
                    CastFailureReason::NOT_MOUNTED => SMSG_CAST_RESULTCastFailureReason::NOT_MOUNTED,
                    CastFailureReason::NOT_ON_TAXI => SMSG_CAST_RESULTCastFailureReason::NOT_ON_TAXI,
                    CastFailureReason::NOT_ON_TRANSPORT => SMSG_CAST_RESULTCastFailureReason::NOT_ON_TRANSPORT,
                    CastFailureReason::NOT_READY => SMSG_CAST_RESULTCastFailureReason::NOT_READY,
                    CastFailureReason::NOT_SHAPESHIFT => SMSG_CAST_RESULTCastFailureReason::NOT_SHAPESHIFT,
                    CastFailureReason::NOT_STANDING => SMSG_CAST_RESULTCastFailureReason::NOT_STANDING,
                    CastFailureReason::NOT_TRADEABLE => SMSG_CAST_RESULTCastFailureReason::NOT_TRADEABLE,
                    CastFailureReason::NOT_TRADING => SMSG_CAST_RESULTCastFailureReason::NOT_TRADING,
                    CastFailureReason::NOT_UNSHEATHED => SMSG_CAST_RESULTCastFailureReason::NOT_UNSHEATHED,
                    CastFailureReason::NOT_WHILE_GHOST => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_GHOST,
                    CastFailureReason::NO_AMMO => SMSG_CAST_RESULTCastFailureReason::NO_AMMO,
                    CastFailureReason::NO_CHARGES_REMAIN => SMSG_CAST_RESULTCastFailureReason::NO_CHARGES_REMAIN,
                    CastFailureReason::NO_CHAMPION => SMSG_CAST_RESULTCastFailureReason::NO_CHAMPION,
                    CastFailureReason::NO_COMBO_POINTS => SMSG_CAST_RESULTCastFailureReason::NO_COMBO_POINTS,
                    CastFailureReason::NO_DUELING => SMSG_CAST_RESULTCastFailureReason::NO_DUELING,
                    CastFailureReason::NO_ENDURANCE => SMSG_CAST_RESULTCastFailureReason::NO_ENDURANCE,
                    CastFailureReason::NO_FISH => SMSG_CAST_RESULTCastFailureReason::NO_FISH,
                    CastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => SMSG_CAST_RESULTCastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED,
                    CastFailureReason::NO_MOUNTS_ALLOWED => SMSG_CAST_RESULTCastFailureReason::NO_MOUNTS_ALLOWED,
                    CastFailureReason::NO_PET => SMSG_CAST_RESULTCastFailureReason::NO_PET,
                    CastFailureReason::NO_POWER => SMSG_CAST_RESULTCastFailureReason::NO_POWER,
                    CastFailureReason::NOTHING_TO_DISPEL => SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_DISPEL,
                    CastFailureReason::NOTHING_TO_STEAL => SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_STEAL,
                    CastFailureReason::ONLY_ABOVEWATER => SMSG_CAST_RESULTCastFailureReason::ONLY_ABOVEWATER,
                    CastFailureReason::ONLY_DAYTIME => SMSG_CAST_RESULTCastFailureReason::ONLY_DAYTIME,
                    CastFailureReason::ONLY_INDOORS => SMSG_CAST_RESULTCastFailureReason::ONLY_INDOORS,
                    CastFailureReason::ONLY_MOUNTED => SMSG_CAST_RESULTCastFailureReason::ONLY_MOUNTED,
                    CastFailureReason::ONLY_NIGHTTIME => SMSG_CAST_RESULTCastFailureReason::ONLY_NIGHTTIME,
                    CastFailureReason::ONLY_OUTDOORS => SMSG_CAST_RESULTCastFailureReason::ONLY_OUTDOORS,
                    CastFailureReason::ONLY_SHAPESHIFT => SMSG_CAST_RESULTCastFailureReason::ONLY_SHAPESHIFT,
                    CastFailureReason::ONLY_STEALTHED => SMSG_CAST_RESULTCastFailureReason::ONLY_STEALTHED,
                    CastFailureReason::ONLY_UNDERWATER => SMSG_CAST_RESULTCastFailureReason::ONLY_UNDERWATER,
                    CastFailureReason::OUT_OF_RANGE => SMSG_CAST_RESULTCastFailureReason::OUT_OF_RANGE,
                    CastFailureReason::PACIFIED => SMSG_CAST_RESULTCastFailureReason::PACIFIED,
                    CastFailureReason::POSSESSED => SMSG_CAST_RESULTCastFailureReason::POSSESSED,
                    CastFailureReason::REAGENTS => SMSG_CAST_RESULTCastFailureReason::REAGENTS,
                    CastFailureReason::REQUIRES_AREA => {
                        // area: Area
                        let area = Area::astd_read(r).await?;

                        SMSG_CAST_RESULTCastFailureReason::REQUIRES_AREA {
                            area,
                        }
                    }
                    CastFailureReason::REQUIRES_SPELL_FOCUS => {
                        // required_spell_focus: u32
                        let required_spell_focus = crate::util::astd_read_u32_le(r).await?;

                        SMSG_CAST_RESULTCastFailureReason::REQUIRES_SPELL_FOCUS {
                            required_spell_focus,
                        }
                    }
                    CastFailureReason::ROOTED => SMSG_CAST_RESULTCastFailureReason::ROOTED,
                    CastFailureReason::SILENCED => SMSG_CAST_RESULTCastFailureReason::SILENCED,
                    CastFailureReason::SPELL_IN_PROGRESS => SMSG_CAST_RESULTCastFailureReason::SPELL_IN_PROGRESS,
                    CastFailureReason::SPELL_LEARNED => SMSG_CAST_RESULTCastFailureReason::SPELL_LEARNED,
                    CastFailureReason::SPELL_UNAVAILABLE => SMSG_CAST_RESULTCastFailureReason::SPELL_UNAVAILABLE,
                    CastFailureReason::STUNNED => SMSG_CAST_RESULTCastFailureReason::STUNNED,
                    CastFailureReason::TARGETS_DEAD => SMSG_CAST_RESULTCastFailureReason::TARGETS_DEAD,
                    CastFailureReason::TARGET_AFFECTING_COMBAT => SMSG_CAST_RESULTCastFailureReason::TARGET_AFFECTING_COMBAT,
                    CastFailureReason::TARGET_AURASTATE => SMSG_CAST_RESULTCastFailureReason::TARGET_AURASTATE,
                    CastFailureReason::TARGET_DUELING => SMSG_CAST_RESULTCastFailureReason::TARGET_DUELING,
                    CastFailureReason::TARGET_ENEMY => SMSG_CAST_RESULTCastFailureReason::TARGET_ENEMY,
                    CastFailureReason::TARGET_ENRAGED => SMSG_CAST_RESULTCastFailureReason::TARGET_ENRAGED,
                    CastFailureReason::TARGET_FRIENDLY => SMSG_CAST_RESULTCastFailureReason::TARGET_FRIENDLY,
                    CastFailureReason::TARGET_IN_COMBAT => SMSG_CAST_RESULTCastFailureReason::TARGET_IN_COMBAT,
                    CastFailureReason::TARGET_IS_PLAYER => SMSG_CAST_RESULTCastFailureReason::TARGET_IS_PLAYER,
                    CastFailureReason::TARGET_NOT_DEAD => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_DEAD,
                    CastFailureReason::TARGET_NOT_IN_PARTY => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_PARTY,
                    CastFailureReason::TARGET_NOT_LOOTED => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_LOOTED,
                    CastFailureReason::TARGET_NOT_PLAYER => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_PLAYER,
                    CastFailureReason::TARGET_NO_POCKETS => SMSG_CAST_RESULTCastFailureReason::TARGET_NO_POCKETS,
                    CastFailureReason::TARGET_NO_WEAPONS => SMSG_CAST_RESULTCastFailureReason::TARGET_NO_WEAPONS,
                    CastFailureReason::TARGET_UNSKINNABLE => SMSG_CAST_RESULTCastFailureReason::TARGET_UNSKINNABLE,
                    CastFailureReason::THIRST_SATIATED => SMSG_CAST_RESULTCastFailureReason::THIRST_SATIATED,
                    CastFailureReason::TOO_CLOSE => SMSG_CAST_RESULTCastFailureReason::TOO_CLOSE,
                    CastFailureReason::TOO_MANY_OF_ITEM => SMSG_CAST_RESULTCastFailureReason::TOO_MANY_OF_ITEM,
                    CastFailureReason::TOTEMS => SMSG_CAST_RESULTCastFailureReason::TOTEMS,
                    CastFailureReason::TRAINING_POINTS => SMSG_CAST_RESULTCastFailureReason::TRAINING_POINTS,
                    CastFailureReason::TRY_AGAIN => SMSG_CAST_RESULTCastFailureReason::TRY_AGAIN,
                    CastFailureReason::UNIT_NOT_BEHIND => SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_BEHIND,
                    CastFailureReason::UNIT_NOT_INFRONT => SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_INFRONT,
                    CastFailureReason::WRONG_PET_FOOD => SMSG_CAST_RESULTCastFailureReason::WRONG_PET_FOOD,
                    CastFailureReason::NOT_WHILE_FATIGUED => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_FATIGUED,
                    CastFailureReason::TARGET_NOT_IN_INSTANCE => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_INSTANCE,
                    CastFailureReason::NOT_WHILE_TRADING => SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_TRADING,
                    CastFailureReason::TARGET_NOT_IN_RAID => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_RAID,
                    CastFailureReason::DISENCHANT_WHILE_LOOTING => SMSG_CAST_RESULTCastFailureReason::DISENCHANT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_WHILE_LOOTING => SMSG_CAST_RESULTCastFailureReason::PROSPECT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_NEED_MORE => SMSG_CAST_RESULTCastFailureReason::PROSPECT_NEED_MORE,
                    CastFailureReason::TARGET_FREEFORALL => SMSG_CAST_RESULTCastFailureReason::TARGET_FREEFORALL,
                    CastFailureReason::NO_EDIBLE_CORPSES => SMSG_CAST_RESULTCastFailureReason::NO_EDIBLE_CORPSES,
                    CastFailureReason::ONLY_BATTLEGROUNDS => SMSG_CAST_RESULTCastFailureReason::ONLY_BATTLEGROUNDS,
                    CastFailureReason::TARGET_NOT_GHOST => SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_GHOST,
                    CastFailureReason::TOO_MANY_SKILLS => SMSG_CAST_RESULTCastFailureReason::TOO_MANY_SKILLS,
                    CastFailureReason::TRANSFORM_UNUSABLE => SMSG_CAST_RESULTCastFailureReason::TRANSFORM_UNUSABLE,
                    CastFailureReason::WRONG_WEATHER => SMSG_CAST_RESULTCastFailureReason::WRONG_WEATHER,
                    CastFailureReason::DAMAGE_IMMUNE => SMSG_CAST_RESULTCastFailureReason::DAMAGE_IMMUNE,
                    CastFailureReason::PREVENTED_BY_MECHANIC => SMSG_CAST_RESULTCastFailureReason::PREVENTED_BY_MECHANIC,
                    CastFailureReason::PLAY_TIME => SMSG_CAST_RESULTCastFailureReason::PLAY_TIME,
                    CastFailureReason::REPUTATION => SMSG_CAST_RESULTCastFailureReason::REPUTATION,
                    CastFailureReason::MIN_SKILL => SMSG_CAST_RESULTCastFailureReason::MIN_SKILL,
                    CastFailureReason::UNKNOWN => SMSG_CAST_RESULTCastFailureReason::UNKNOWN,
                };

                SMSG_CAST_RESULTSimpleSpellCastResult::SUCCESS {
                    reason: reason_if,
                }
            }
            SimpleSpellCastResult::FAILURE => SMSG_CAST_RESULTSimpleSpellCastResult::FAILURE,
        };

        Ok(Self {
            spell,
            result: result_if,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes()).await?;

        // result: SimpleSpellCastResult
        self.result.astd_write(w).await?;

        match &self.result {
            SMSG_CAST_RESULTSimpleSpellCastResult::SUCCESS {
                reason,
            } => {
                // reason: CastFailureReason
                reason.astd_write(w).await?;

                match &reason {
                    SMSG_CAST_RESULTCastFailureReason::AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_HEALTH => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_POWER => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_BEING_TAMED => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_CHARM => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_SUMMON => {}
                    SMSG_CAST_RESULTCastFailureReason::ALREADY_OPEN => {}
                    SMSG_CAST_RESULTCastFailureReason::AURA_BOUNCED => {}
                    SMSG_CAST_RESULTCastFailureReason::AUTOTRACK_INTERRUPTED => {}
                    SMSG_CAST_RESULTCastFailureReason::BAD_IMPLICIT_TARGETS => {}
                    SMSG_CAST_RESULTCastFailureReason::BAD_TARGETS => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_CHARMED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_DISENCHANTED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_BE_PROSPECTED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_CAST_ON_TAPPED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_INVISIBLE => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_STEALTHED => {}
                    SMSG_CAST_RESULTCastFailureReason::CANT_STEALTH => {}
                    SMSG_CAST_RESULTCastFailureReason::CASTER_AURASTATE => {}
                    SMSG_CAST_RESULTCastFailureReason::CASTER_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::CHARMED => {}
                    SMSG_CAST_RESULTCastFailureReason::CHEST_IN_USE => {}
                    SMSG_CAST_RESULTCastFailureReason::CONFUSED => {}
                    SMSG_CAST_RESULTCastFailureReason::DONT_REPORT => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS {
                        equipped_item_class,
                        equipped_item_subclass_mask,
                        equipped_item_inventory_type_mask,
                    } => {
                        // equipped_item_class: u32
                        w.write_all(&equipped_item_class.to_le_bytes()).await?;

                        // equipped_item_subclass_mask: u32
                        w.write_all(&equipped_item_subclass_mask.to_le_bytes()).await?;

                        // equipped_item_inventory_type_mask: u32
                        w.write_all(&equipped_item_inventory_type_mask.to_le_bytes()).await?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => {}
                    SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => {}
                    SMSG_CAST_RESULTCastFailureReason::ERROR => {}
                    SMSG_CAST_RESULTCastFailureReason::FIZZLE => {}
                    SMSG_CAST_RESULTCastFailureReason::FLEEING => {}
                    SMSG_CAST_RESULTCastFailureReason::FOOD_LOWLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::HIGHLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::HUNGER_SATIATED => {}
                    SMSG_CAST_RESULTCastFailureReason::IMMUNE => {}
                    SMSG_CAST_RESULTCastFailureReason::INTERRUPTED => {}
                    SMSG_CAST_RESULTCastFailureReason::INTERRUPTED_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_ALREADY_ENCHANTED => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_GONE => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_FOUND => {}
                    SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_READY => {}
                    SMSG_CAST_RESULTCastFailureReason::LEVEL_REQUIREMENT => {}
                    SMSG_CAST_RESULTCastFailureReason::LINE_OF_SIGHT => {}
                    SMSG_CAST_RESULTCastFailureReason::LOWLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::LOW_CASTLEVEL => {}
                    SMSG_CAST_RESULTCastFailureReason::MAINHAND_EMPTY => {}
                    SMSG_CAST_RESULTCastFailureReason::MOVING => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_AMMO_POUCH => {}
                    SMSG_CAST_RESULTCastFailureReason::NEED_EXOTIC_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NOPATH => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_BEHIND => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_FISHABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_HERE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_INFRONT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_IN_CONTROL => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_KNOWN => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_MOUNTED => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_ON_TAXI => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_ON_TRANSPORT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_READY => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_SHAPESHIFT => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_STANDING => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_TRADEABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_TRADING => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_UNSHEATHED => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_GHOST => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_AMMO => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_CHARGES_REMAIN => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_CHAMPION => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_COMBO_POINTS => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_DUELING => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_ENDURANCE => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_FISH => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_MOUNTS_ALLOWED => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_PET => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_POWER => {}
                    SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_DISPEL => {}
                    SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_STEAL => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_ABOVEWATER => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_DAYTIME => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_INDOORS => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_MOUNTED => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_NIGHTTIME => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_OUTDOORS => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_SHAPESHIFT => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_STEALTHED => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_UNDERWATER => {}
                    SMSG_CAST_RESULTCastFailureReason::OUT_OF_RANGE => {}
                    SMSG_CAST_RESULTCastFailureReason::PACIFIED => {}
                    SMSG_CAST_RESULTCastFailureReason::POSSESSED => {}
                    SMSG_CAST_RESULTCastFailureReason::REAGENTS => {}
                    SMSG_CAST_RESULTCastFailureReason::REQUIRES_AREA {
                        area,
                    } => {
                        // area: Area
                        area.astd_write(w).await?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::REQUIRES_SPELL_FOCUS {
                        required_spell_focus,
                    } => {
                        // required_spell_focus: u32
                        w.write_all(&required_spell_focus.to_le_bytes()).await?;

                    }
                    SMSG_CAST_RESULTCastFailureReason::ROOTED => {}
                    SMSG_CAST_RESULTCastFailureReason::SILENCED => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_IN_PROGRESS => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_LEARNED => {}
                    SMSG_CAST_RESULTCastFailureReason::SPELL_UNAVAILABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::STUNNED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGETS_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_AURASTATE => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_DUELING => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_ENEMY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_ENRAGED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_FRIENDLY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_IN_COMBAT => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_IS_PLAYER => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_DEAD => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_PARTY => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_LOOTED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_PLAYER => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NO_POCKETS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NO_WEAPONS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_UNSKINNABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::THIRST_SATIATED => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_CLOSE => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_MANY_OF_ITEM => {}
                    SMSG_CAST_RESULTCastFailureReason::TOTEMS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRAINING_POINTS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRY_AGAIN => {}
                    SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_BEHIND => {}
                    SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_INFRONT => {}
                    SMSG_CAST_RESULTCastFailureReason::WRONG_PET_FOOD => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_FATIGUED => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_INSTANCE => {}
                    SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_TRADING => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_RAID => {}
                    SMSG_CAST_RESULTCastFailureReason::DISENCHANT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULTCastFailureReason::PROSPECT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULTCastFailureReason::PROSPECT_NEED_MORE => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_FREEFORALL => {}
                    SMSG_CAST_RESULTCastFailureReason::NO_EDIBLE_CORPSES => {}
                    SMSG_CAST_RESULTCastFailureReason::ONLY_BATTLEGROUNDS => {}
                    SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_GHOST => {}
                    SMSG_CAST_RESULTCastFailureReason::TOO_MANY_SKILLS => {}
                    SMSG_CAST_RESULTCastFailureReason::TRANSFORM_UNUSABLE => {}
                    SMSG_CAST_RESULTCastFailureReason::WRONG_WEATHER => {}
                    SMSG_CAST_RESULTCastFailureReason::DAMAGE_IMMUNE => {}
                    SMSG_CAST_RESULTCastFailureReason::PREVENTED_BY_MECHANIC => {}
                    SMSG_CAST_RESULTCastFailureReason::PLAY_TIME => {}
                    SMSG_CAST_RESULTCastFailureReason::REPUTATION => {}
                    SMSG_CAST_RESULTCastFailureReason::MIN_SKILL => {}
                    SMSG_CAST_RESULTCastFailureReason::UNKNOWN => {}
                }

            }
            SMSG_CAST_RESULTSimpleSpellCastResult::FAILURE => {}
        }

        Ok(())
    }
}

impl VariableSized for SMSG_CAST_RESULT {
    fn size(&self) -> usize {
        4 // spell: u32
        + self.result.size() // result: SimpleSpellCastResult and subfields
    }
}

impl MaximumPossibleSized for SMSG_CAST_RESULT {
    fn maximum_possible_size() -> usize {
        4 // spell: u32
        + SimpleSpellCastResult::maximum_possible_size() // result: SimpleSpellCastResult
    }
}

#[derive(Debug)]
pub enum SMSG_CAST_RESULTError {
    Io(std::io::Error),
    Area(AreaError),
    CastFailureReason(CastFailureReasonError),
    SimpleSpellCastResult(SimpleSpellCastResultError),
}

impl std::error::Error for SMSG_CAST_RESULTError {}
impl std::fmt::Display for SMSG_CAST_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::CastFailureReason(i) => i.fmt(f),
            Self::SimpleSpellCastResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CAST_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_CAST_RESULTError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<CastFailureReasonError> for SMSG_CAST_RESULTError {
    fn from(e: CastFailureReasonError) -> Self {
        Self::CastFailureReason(e)
    }
}

impl From<SimpleSpellCastResultError> for SMSG_CAST_RESULTError {
    fn from(e: SimpleSpellCastResultError) -> Self {
        Self::SimpleSpellCastResult(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_CAST_RESULTCastFailureReason {
    AFFECTING_COMBAT,
    ALREADY_AT_FULL_HEALTH,
    ALREADY_AT_FULL_POWER,
    ALREADY_BEING_TAMED,
    ALREADY_HAVE_CHARM,
    ALREADY_HAVE_SUMMON,
    ALREADY_OPEN,
    AURA_BOUNCED,
    AUTOTRACK_INTERRUPTED,
    BAD_IMPLICIT_TARGETS,
    BAD_TARGETS,
    CANT_BE_CHARMED,
    CANT_BE_DISENCHANTED,
    CANT_BE_PROSPECTED,
    CANT_CAST_ON_TAPPED,
    CANT_DUEL_WHILE_INVISIBLE,
    CANT_DUEL_WHILE_STEALTHED,
    CANT_STEALTH,
    CASTER_AURASTATE,
    CASTER_DEAD,
    CHARMED,
    CHEST_IN_USE,
    CONFUSED,
    DONT_REPORT,
    EQUIPPED_ITEM,
    EQUIPPED_ITEM_CLASS {
        equipped_item_class: u32,
        equipped_item_subclass_mask: u32,
        equipped_item_inventory_type_mask: u32,
    },
    EQUIPPED_ITEM_CLASS_MAINHAND,
    EQUIPPED_ITEM_CLASS_OFFHAND,
    ERROR,
    FIZZLE,
    FLEEING,
    FOOD_LOWLEVEL,
    HIGHLEVEL,
    HUNGER_SATIATED,
    IMMUNE,
    INTERRUPTED,
    INTERRUPTED_COMBAT,
    ITEM_ALREADY_ENCHANTED,
    ITEM_GONE,
    ITEM_NOT_FOUND,
    ITEM_NOT_READY,
    LEVEL_REQUIREMENT,
    LINE_OF_SIGHT,
    LOWLEVEL,
    LOW_CASTLEVEL,
    MAINHAND_EMPTY,
    MOVING,
    NEED_AMMO,
    NEED_AMMO_POUCH,
    NEED_EXOTIC_AMMO,
    NOPATH,
    NOT_BEHIND,
    NOT_FISHABLE,
    NOT_HERE,
    NOT_INFRONT,
    NOT_IN_CONTROL,
    NOT_KNOWN,
    NOT_MOUNTED,
    NOT_ON_TAXI,
    NOT_ON_TRANSPORT,
    NOT_READY,
    NOT_SHAPESHIFT,
    NOT_STANDING,
    NOT_TRADEABLE,
    NOT_TRADING,
    NOT_UNSHEATHED,
    NOT_WHILE_GHOST,
    NO_AMMO,
    NO_CHARGES_REMAIN,
    NO_CHAMPION,
    NO_COMBO_POINTS,
    NO_DUELING,
    NO_ENDURANCE,
    NO_FISH,
    NO_ITEMS_WHILE_SHAPESHIFTED,
    NO_MOUNTS_ALLOWED,
    NO_PET,
    NO_POWER,
    NOTHING_TO_DISPEL,
    NOTHING_TO_STEAL,
    ONLY_ABOVEWATER,
    ONLY_DAYTIME,
    ONLY_INDOORS,
    ONLY_MOUNTED,
    ONLY_NIGHTTIME,
    ONLY_OUTDOORS,
    ONLY_SHAPESHIFT,
    ONLY_STEALTHED,
    ONLY_UNDERWATER,
    OUT_OF_RANGE,
    PACIFIED,
    POSSESSED,
    REAGENTS,
    REQUIRES_AREA {
        area: Area,
    },
    REQUIRES_SPELL_FOCUS {
        required_spell_focus: u32,
    },
    ROOTED,
    SILENCED,
    SPELL_IN_PROGRESS,
    SPELL_LEARNED,
    SPELL_UNAVAILABLE,
    STUNNED,
    TARGETS_DEAD,
    TARGET_AFFECTING_COMBAT,
    TARGET_AURASTATE,
    TARGET_DUELING,
    TARGET_ENEMY,
    TARGET_ENRAGED,
    TARGET_FRIENDLY,
    TARGET_IN_COMBAT,
    TARGET_IS_PLAYER,
    TARGET_NOT_DEAD,
    TARGET_NOT_IN_PARTY,
    TARGET_NOT_LOOTED,
    TARGET_NOT_PLAYER,
    TARGET_NO_POCKETS,
    TARGET_NO_WEAPONS,
    TARGET_UNSKINNABLE,
    THIRST_SATIATED,
    TOO_CLOSE,
    TOO_MANY_OF_ITEM,
    TOTEMS,
    TRAINING_POINTS,
    TRY_AGAIN,
    UNIT_NOT_BEHIND,
    UNIT_NOT_INFRONT,
    WRONG_PET_FOOD,
    NOT_WHILE_FATIGUED,
    TARGET_NOT_IN_INSTANCE,
    NOT_WHILE_TRADING,
    TARGET_NOT_IN_RAID,
    DISENCHANT_WHILE_LOOTING,
    PROSPECT_WHILE_LOOTING,
    PROSPECT_NEED_MORE,
    TARGET_FREEFORALL,
    NO_EDIBLE_CORPSES,
    ONLY_BATTLEGROUNDS,
    TARGET_NOT_GHOST,
    TOO_MANY_SKILLS,
    TRANSFORM_UNUSABLE,
    WRONG_WEATHER,
    DAMAGE_IMMUNE,
    PREVENTED_BY_MECHANIC,
    PLAY_TIME,
    REPUTATION,
    MIN_SKILL,
    UNKNOWN,
}

impl From<&CastFailureReason> for SMSG_CAST_RESULTCastFailureReason {
    fn from(e: &CastFailureReason) -> Self {
        match &e {
            CastFailureReason::AFFECTING_COMBAT => Self::AFFECTING_COMBAT,
            CastFailureReason::ALREADY_AT_FULL_HEALTH => Self::ALREADY_AT_FULL_HEALTH,
            CastFailureReason::ALREADY_AT_FULL_POWER => Self::ALREADY_AT_FULL_POWER,
            CastFailureReason::ALREADY_BEING_TAMED => Self::ALREADY_BEING_TAMED,
            CastFailureReason::ALREADY_HAVE_CHARM => Self::ALREADY_HAVE_CHARM,
            CastFailureReason::ALREADY_HAVE_SUMMON => Self::ALREADY_HAVE_SUMMON,
            CastFailureReason::ALREADY_OPEN => Self::ALREADY_OPEN,
            CastFailureReason::AURA_BOUNCED => Self::AURA_BOUNCED,
            CastFailureReason::AUTOTRACK_INTERRUPTED => Self::AUTOTRACK_INTERRUPTED,
            CastFailureReason::BAD_IMPLICIT_TARGETS => Self::BAD_IMPLICIT_TARGETS,
            CastFailureReason::BAD_TARGETS => Self::BAD_TARGETS,
            CastFailureReason::CANT_BE_CHARMED => Self::CANT_BE_CHARMED,
            CastFailureReason::CANT_BE_DISENCHANTED => Self::CANT_BE_DISENCHANTED,
            CastFailureReason::CANT_BE_PROSPECTED => Self::CANT_BE_PROSPECTED,
            CastFailureReason::CANT_CAST_ON_TAPPED => Self::CANT_CAST_ON_TAPPED,
            CastFailureReason::CANT_DUEL_WHILE_INVISIBLE => Self::CANT_DUEL_WHILE_INVISIBLE,
            CastFailureReason::CANT_DUEL_WHILE_STEALTHED => Self::CANT_DUEL_WHILE_STEALTHED,
            CastFailureReason::CANT_STEALTH => Self::CANT_STEALTH,
            CastFailureReason::CASTER_AURASTATE => Self::CASTER_AURASTATE,
            CastFailureReason::CASTER_DEAD => Self::CASTER_DEAD,
            CastFailureReason::CHARMED => Self::CHARMED,
            CastFailureReason::CHEST_IN_USE => Self::CHEST_IN_USE,
            CastFailureReason::CONFUSED => Self::CONFUSED,
            CastFailureReason::DONT_REPORT => Self::DONT_REPORT,
            CastFailureReason::EQUIPPED_ITEM => Self::EQUIPPED_ITEM,
            CastFailureReason::EQUIPPED_ITEM_CLASS => Self::EQUIPPED_ITEM_CLASS {
                equipped_item_class: Default::default(),
                equipped_item_subclass_mask: Default::default(),
                equipped_item_inventory_type_mask: Default::default(),
            },
            CastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => Self::EQUIPPED_ITEM_CLASS_MAINHAND,
            CastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => Self::EQUIPPED_ITEM_CLASS_OFFHAND,
            CastFailureReason::ERROR => Self::ERROR,
            CastFailureReason::FIZZLE => Self::FIZZLE,
            CastFailureReason::FLEEING => Self::FLEEING,
            CastFailureReason::FOOD_LOWLEVEL => Self::FOOD_LOWLEVEL,
            CastFailureReason::HIGHLEVEL => Self::HIGHLEVEL,
            CastFailureReason::HUNGER_SATIATED => Self::HUNGER_SATIATED,
            CastFailureReason::IMMUNE => Self::IMMUNE,
            CastFailureReason::INTERRUPTED => Self::INTERRUPTED,
            CastFailureReason::INTERRUPTED_COMBAT => Self::INTERRUPTED_COMBAT,
            CastFailureReason::ITEM_ALREADY_ENCHANTED => Self::ITEM_ALREADY_ENCHANTED,
            CastFailureReason::ITEM_GONE => Self::ITEM_GONE,
            CastFailureReason::ITEM_NOT_FOUND => Self::ITEM_NOT_FOUND,
            CastFailureReason::ITEM_NOT_READY => Self::ITEM_NOT_READY,
            CastFailureReason::LEVEL_REQUIREMENT => Self::LEVEL_REQUIREMENT,
            CastFailureReason::LINE_OF_SIGHT => Self::LINE_OF_SIGHT,
            CastFailureReason::LOWLEVEL => Self::LOWLEVEL,
            CastFailureReason::LOW_CASTLEVEL => Self::LOW_CASTLEVEL,
            CastFailureReason::MAINHAND_EMPTY => Self::MAINHAND_EMPTY,
            CastFailureReason::MOVING => Self::MOVING,
            CastFailureReason::NEED_AMMO => Self::NEED_AMMO,
            CastFailureReason::NEED_AMMO_POUCH => Self::NEED_AMMO_POUCH,
            CastFailureReason::NEED_EXOTIC_AMMO => Self::NEED_EXOTIC_AMMO,
            CastFailureReason::NOPATH => Self::NOPATH,
            CastFailureReason::NOT_BEHIND => Self::NOT_BEHIND,
            CastFailureReason::NOT_FISHABLE => Self::NOT_FISHABLE,
            CastFailureReason::NOT_HERE => Self::NOT_HERE,
            CastFailureReason::NOT_INFRONT => Self::NOT_INFRONT,
            CastFailureReason::NOT_IN_CONTROL => Self::NOT_IN_CONTROL,
            CastFailureReason::NOT_KNOWN => Self::NOT_KNOWN,
            CastFailureReason::NOT_MOUNTED => Self::NOT_MOUNTED,
            CastFailureReason::NOT_ON_TAXI => Self::NOT_ON_TAXI,
            CastFailureReason::NOT_ON_TRANSPORT => Self::NOT_ON_TRANSPORT,
            CastFailureReason::NOT_READY => Self::NOT_READY,
            CastFailureReason::NOT_SHAPESHIFT => Self::NOT_SHAPESHIFT,
            CastFailureReason::NOT_STANDING => Self::NOT_STANDING,
            CastFailureReason::NOT_TRADEABLE => Self::NOT_TRADEABLE,
            CastFailureReason::NOT_TRADING => Self::NOT_TRADING,
            CastFailureReason::NOT_UNSHEATHED => Self::NOT_UNSHEATHED,
            CastFailureReason::NOT_WHILE_GHOST => Self::NOT_WHILE_GHOST,
            CastFailureReason::NO_AMMO => Self::NO_AMMO,
            CastFailureReason::NO_CHARGES_REMAIN => Self::NO_CHARGES_REMAIN,
            CastFailureReason::NO_CHAMPION => Self::NO_CHAMPION,
            CastFailureReason::NO_COMBO_POINTS => Self::NO_COMBO_POINTS,
            CastFailureReason::NO_DUELING => Self::NO_DUELING,
            CastFailureReason::NO_ENDURANCE => Self::NO_ENDURANCE,
            CastFailureReason::NO_FISH => Self::NO_FISH,
            CastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => Self::NO_ITEMS_WHILE_SHAPESHIFTED,
            CastFailureReason::NO_MOUNTS_ALLOWED => Self::NO_MOUNTS_ALLOWED,
            CastFailureReason::NO_PET => Self::NO_PET,
            CastFailureReason::NO_POWER => Self::NO_POWER,
            CastFailureReason::NOTHING_TO_DISPEL => Self::NOTHING_TO_DISPEL,
            CastFailureReason::NOTHING_TO_STEAL => Self::NOTHING_TO_STEAL,
            CastFailureReason::ONLY_ABOVEWATER => Self::ONLY_ABOVEWATER,
            CastFailureReason::ONLY_DAYTIME => Self::ONLY_DAYTIME,
            CastFailureReason::ONLY_INDOORS => Self::ONLY_INDOORS,
            CastFailureReason::ONLY_MOUNTED => Self::ONLY_MOUNTED,
            CastFailureReason::ONLY_NIGHTTIME => Self::ONLY_NIGHTTIME,
            CastFailureReason::ONLY_OUTDOORS => Self::ONLY_OUTDOORS,
            CastFailureReason::ONLY_SHAPESHIFT => Self::ONLY_SHAPESHIFT,
            CastFailureReason::ONLY_STEALTHED => Self::ONLY_STEALTHED,
            CastFailureReason::ONLY_UNDERWATER => Self::ONLY_UNDERWATER,
            CastFailureReason::OUT_OF_RANGE => Self::OUT_OF_RANGE,
            CastFailureReason::PACIFIED => Self::PACIFIED,
            CastFailureReason::POSSESSED => Self::POSSESSED,
            CastFailureReason::REAGENTS => Self::REAGENTS,
            CastFailureReason::REQUIRES_AREA => Self::REQUIRES_AREA {
                area: Default::default(),
            },
            CastFailureReason::REQUIRES_SPELL_FOCUS => Self::REQUIRES_SPELL_FOCUS {
                required_spell_focus: Default::default(),
            },
            CastFailureReason::ROOTED => Self::ROOTED,
            CastFailureReason::SILENCED => Self::SILENCED,
            CastFailureReason::SPELL_IN_PROGRESS => Self::SPELL_IN_PROGRESS,
            CastFailureReason::SPELL_LEARNED => Self::SPELL_LEARNED,
            CastFailureReason::SPELL_UNAVAILABLE => Self::SPELL_UNAVAILABLE,
            CastFailureReason::STUNNED => Self::STUNNED,
            CastFailureReason::TARGETS_DEAD => Self::TARGETS_DEAD,
            CastFailureReason::TARGET_AFFECTING_COMBAT => Self::TARGET_AFFECTING_COMBAT,
            CastFailureReason::TARGET_AURASTATE => Self::TARGET_AURASTATE,
            CastFailureReason::TARGET_DUELING => Self::TARGET_DUELING,
            CastFailureReason::TARGET_ENEMY => Self::TARGET_ENEMY,
            CastFailureReason::TARGET_ENRAGED => Self::TARGET_ENRAGED,
            CastFailureReason::TARGET_FRIENDLY => Self::TARGET_FRIENDLY,
            CastFailureReason::TARGET_IN_COMBAT => Self::TARGET_IN_COMBAT,
            CastFailureReason::TARGET_IS_PLAYER => Self::TARGET_IS_PLAYER,
            CastFailureReason::TARGET_NOT_DEAD => Self::TARGET_NOT_DEAD,
            CastFailureReason::TARGET_NOT_IN_PARTY => Self::TARGET_NOT_IN_PARTY,
            CastFailureReason::TARGET_NOT_LOOTED => Self::TARGET_NOT_LOOTED,
            CastFailureReason::TARGET_NOT_PLAYER => Self::TARGET_NOT_PLAYER,
            CastFailureReason::TARGET_NO_POCKETS => Self::TARGET_NO_POCKETS,
            CastFailureReason::TARGET_NO_WEAPONS => Self::TARGET_NO_WEAPONS,
            CastFailureReason::TARGET_UNSKINNABLE => Self::TARGET_UNSKINNABLE,
            CastFailureReason::THIRST_SATIATED => Self::THIRST_SATIATED,
            CastFailureReason::TOO_CLOSE => Self::TOO_CLOSE,
            CastFailureReason::TOO_MANY_OF_ITEM => Self::TOO_MANY_OF_ITEM,
            CastFailureReason::TOTEMS => Self::TOTEMS,
            CastFailureReason::TRAINING_POINTS => Self::TRAINING_POINTS,
            CastFailureReason::TRY_AGAIN => Self::TRY_AGAIN,
            CastFailureReason::UNIT_NOT_BEHIND => Self::UNIT_NOT_BEHIND,
            CastFailureReason::UNIT_NOT_INFRONT => Self::UNIT_NOT_INFRONT,
            CastFailureReason::WRONG_PET_FOOD => Self::WRONG_PET_FOOD,
            CastFailureReason::NOT_WHILE_FATIGUED => Self::NOT_WHILE_FATIGUED,
            CastFailureReason::TARGET_NOT_IN_INSTANCE => Self::TARGET_NOT_IN_INSTANCE,
            CastFailureReason::NOT_WHILE_TRADING => Self::NOT_WHILE_TRADING,
            CastFailureReason::TARGET_NOT_IN_RAID => Self::TARGET_NOT_IN_RAID,
            CastFailureReason::DISENCHANT_WHILE_LOOTING => Self::DISENCHANT_WHILE_LOOTING,
            CastFailureReason::PROSPECT_WHILE_LOOTING => Self::PROSPECT_WHILE_LOOTING,
            CastFailureReason::PROSPECT_NEED_MORE => Self::PROSPECT_NEED_MORE,
            CastFailureReason::TARGET_FREEFORALL => Self::TARGET_FREEFORALL,
            CastFailureReason::NO_EDIBLE_CORPSES => Self::NO_EDIBLE_CORPSES,
            CastFailureReason::ONLY_BATTLEGROUNDS => Self::ONLY_BATTLEGROUNDS,
            CastFailureReason::TARGET_NOT_GHOST => Self::TARGET_NOT_GHOST,
            CastFailureReason::TOO_MANY_SKILLS => Self::TOO_MANY_SKILLS,
            CastFailureReason::TRANSFORM_UNUSABLE => Self::TRANSFORM_UNUSABLE,
            CastFailureReason::WRONG_WEATHER => Self::WRONG_WEATHER,
            CastFailureReason::DAMAGE_IMMUNE => Self::DAMAGE_IMMUNE,
            CastFailureReason::PREVENTED_BY_MECHANIC => Self::PREVENTED_BY_MECHANIC,
            CastFailureReason::PLAY_TIME => Self::PLAY_TIME,
            CastFailureReason::REPUTATION => Self::REPUTATION,
            CastFailureReason::MIN_SKILL => Self::MIN_SKILL,
            CastFailureReason::UNKNOWN => Self::UNKNOWN,
        }
    }
}

impl From<&SMSG_CAST_RESULTCastFailureReason> for CastFailureReason {
    fn from(v: &SMSG_CAST_RESULTCastFailureReason) -> Self {
        match &v {
            SMSG_CAST_RESULTCastFailureReason::AFFECTING_COMBAT => Self::AFFECTING_COMBAT,
            SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_HEALTH => Self::ALREADY_AT_FULL_HEALTH,
            SMSG_CAST_RESULTCastFailureReason::ALREADY_AT_FULL_POWER => Self::ALREADY_AT_FULL_POWER,
            SMSG_CAST_RESULTCastFailureReason::ALREADY_BEING_TAMED => Self::ALREADY_BEING_TAMED,
            SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_CHARM => Self::ALREADY_HAVE_CHARM,
            SMSG_CAST_RESULTCastFailureReason::ALREADY_HAVE_SUMMON => Self::ALREADY_HAVE_SUMMON,
            SMSG_CAST_RESULTCastFailureReason::ALREADY_OPEN => Self::ALREADY_OPEN,
            SMSG_CAST_RESULTCastFailureReason::AURA_BOUNCED => Self::AURA_BOUNCED,
            SMSG_CAST_RESULTCastFailureReason::AUTOTRACK_INTERRUPTED => Self::AUTOTRACK_INTERRUPTED,
            SMSG_CAST_RESULTCastFailureReason::BAD_IMPLICIT_TARGETS => Self::BAD_IMPLICIT_TARGETS,
            SMSG_CAST_RESULTCastFailureReason::BAD_TARGETS => Self::BAD_TARGETS,
            SMSG_CAST_RESULTCastFailureReason::CANT_BE_CHARMED => Self::CANT_BE_CHARMED,
            SMSG_CAST_RESULTCastFailureReason::CANT_BE_DISENCHANTED => Self::CANT_BE_DISENCHANTED,
            SMSG_CAST_RESULTCastFailureReason::CANT_BE_PROSPECTED => Self::CANT_BE_PROSPECTED,
            SMSG_CAST_RESULTCastFailureReason::CANT_CAST_ON_TAPPED => Self::CANT_CAST_ON_TAPPED,
            SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_INVISIBLE => Self::CANT_DUEL_WHILE_INVISIBLE,
            SMSG_CAST_RESULTCastFailureReason::CANT_DUEL_WHILE_STEALTHED => Self::CANT_DUEL_WHILE_STEALTHED,
            SMSG_CAST_RESULTCastFailureReason::CANT_STEALTH => Self::CANT_STEALTH,
            SMSG_CAST_RESULTCastFailureReason::CASTER_AURASTATE => Self::CASTER_AURASTATE,
            SMSG_CAST_RESULTCastFailureReason::CASTER_DEAD => Self::CASTER_DEAD,
            SMSG_CAST_RESULTCastFailureReason::CHARMED => Self::CHARMED,
            SMSG_CAST_RESULTCastFailureReason::CHEST_IN_USE => Self::CHEST_IN_USE,
            SMSG_CAST_RESULTCastFailureReason::CONFUSED => Self::CONFUSED,
            SMSG_CAST_RESULTCastFailureReason::DONT_REPORT => Self::DONT_REPORT,
            SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM => Self::EQUIPPED_ITEM,
            SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS { .. } => Self::EQUIPPED_ITEM_CLASS,
            SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => Self::EQUIPPED_ITEM_CLASS_MAINHAND,
            SMSG_CAST_RESULTCastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => Self::EQUIPPED_ITEM_CLASS_OFFHAND,
            SMSG_CAST_RESULTCastFailureReason::ERROR => Self::ERROR,
            SMSG_CAST_RESULTCastFailureReason::FIZZLE => Self::FIZZLE,
            SMSG_CAST_RESULTCastFailureReason::FLEEING => Self::FLEEING,
            SMSG_CAST_RESULTCastFailureReason::FOOD_LOWLEVEL => Self::FOOD_LOWLEVEL,
            SMSG_CAST_RESULTCastFailureReason::HIGHLEVEL => Self::HIGHLEVEL,
            SMSG_CAST_RESULTCastFailureReason::HUNGER_SATIATED => Self::HUNGER_SATIATED,
            SMSG_CAST_RESULTCastFailureReason::IMMUNE => Self::IMMUNE,
            SMSG_CAST_RESULTCastFailureReason::INTERRUPTED => Self::INTERRUPTED,
            SMSG_CAST_RESULTCastFailureReason::INTERRUPTED_COMBAT => Self::INTERRUPTED_COMBAT,
            SMSG_CAST_RESULTCastFailureReason::ITEM_ALREADY_ENCHANTED => Self::ITEM_ALREADY_ENCHANTED,
            SMSG_CAST_RESULTCastFailureReason::ITEM_GONE => Self::ITEM_GONE,
            SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_FOUND => Self::ITEM_NOT_FOUND,
            SMSG_CAST_RESULTCastFailureReason::ITEM_NOT_READY => Self::ITEM_NOT_READY,
            SMSG_CAST_RESULTCastFailureReason::LEVEL_REQUIREMENT => Self::LEVEL_REQUIREMENT,
            SMSG_CAST_RESULTCastFailureReason::LINE_OF_SIGHT => Self::LINE_OF_SIGHT,
            SMSG_CAST_RESULTCastFailureReason::LOWLEVEL => Self::LOWLEVEL,
            SMSG_CAST_RESULTCastFailureReason::LOW_CASTLEVEL => Self::LOW_CASTLEVEL,
            SMSG_CAST_RESULTCastFailureReason::MAINHAND_EMPTY => Self::MAINHAND_EMPTY,
            SMSG_CAST_RESULTCastFailureReason::MOVING => Self::MOVING,
            SMSG_CAST_RESULTCastFailureReason::NEED_AMMO => Self::NEED_AMMO,
            SMSG_CAST_RESULTCastFailureReason::NEED_AMMO_POUCH => Self::NEED_AMMO_POUCH,
            SMSG_CAST_RESULTCastFailureReason::NEED_EXOTIC_AMMO => Self::NEED_EXOTIC_AMMO,
            SMSG_CAST_RESULTCastFailureReason::NOPATH => Self::NOPATH,
            SMSG_CAST_RESULTCastFailureReason::NOT_BEHIND => Self::NOT_BEHIND,
            SMSG_CAST_RESULTCastFailureReason::NOT_FISHABLE => Self::NOT_FISHABLE,
            SMSG_CAST_RESULTCastFailureReason::NOT_HERE => Self::NOT_HERE,
            SMSG_CAST_RESULTCastFailureReason::NOT_INFRONT => Self::NOT_INFRONT,
            SMSG_CAST_RESULTCastFailureReason::NOT_IN_CONTROL => Self::NOT_IN_CONTROL,
            SMSG_CAST_RESULTCastFailureReason::NOT_KNOWN => Self::NOT_KNOWN,
            SMSG_CAST_RESULTCastFailureReason::NOT_MOUNTED => Self::NOT_MOUNTED,
            SMSG_CAST_RESULTCastFailureReason::NOT_ON_TAXI => Self::NOT_ON_TAXI,
            SMSG_CAST_RESULTCastFailureReason::NOT_ON_TRANSPORT => Self::NOT_ON_TRANSPORT,
            SMSG_CAST_RESULTCastFailureReason::NOT_READY => Self::NOT_READY,
            SMSG_CAST_RESULTCastFailureReason::NOT_SHAPESHIFT => Self::NOT_SHAPESHIFT,
            SMSG_CAST_RESULTCastFailureReason::NOT_STANDING => Self::NOT_STANDING,
            SMSG_CAST_RESULTCastFailureReason::NOT_TRADEABLE => Self::NOT_TRADEABLE,
            SMSG_CAST_RESULTCastFailureReason::NOT_TRADING => Self::NOT_TRADING,
            SMSG_CAST_RESULTCastFailureReason::NOT_UNSHEATHED => Self::NOT_UNSHEATHED,
            SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_GHOST => Self::NOT_WHILE_GHOST,
            SMSG_CAST_RESULTCastFailureReason::NO_AMMO => Self::NO_AMMO,
            SMSG_CAST_RESULTCastFailureReason::NO_CHARGES_REMAIN => Self::NO_CHARGES_REMAIN,
            SMSG_CAST_RESULTCastFailureReason::NO_CHAMPION => Self::NO_CHAMPION,
            SMSG_CAST_RESULTCastFailureReason::NO_COMBO_POINTS => Self::NO_COMBO_POINTS,
            SMSG_CAST_RESULTCastFailureReason::NO_DUELING => Self::NO_DUELING,
            SMSG_CAST_RESULTCastFailureReason::NO_ENDURANCE => Self::NO_ENDURANCE,
            SMSG_CAST_RESULTCastFailureReason::NO_FISH => Self::NO_FISH,
            SMSG_CAST_RESULTCastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => Self::NO_ITEMS_WHILE_SHAPESHIFTED,
            SMSG_CAST_RESULTCastFailureReason::NO_MOUNTS_ALLOWED => Self::NO_MOUNTS_ALLOWED,
            SMSG_CAST_RESULTCastFailureReason::NO_PET => Self::NO_PET,
            SMSG_CAST_RESULTCastFailureReason::NO_POWER => Self::NO_POWER,
            SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_DISPEL => Self::NOTHING_TO_DISPEL,
            SMSG_CAST_RESULTCastFailureReason::NOTHING_TO_STEAL => Self::NOTHING_TO_STEAL,
            SMSG_CAST_RESULTCastFailureReason::ONLY_ABOVEWATER => Self::ONLY_ABOVEWATER,
            SMSG_CAST_RESULTCastFailureReason::ONLY_DAYTIME => Self::ONLY_DAYTIME,
            SMSG_CAST_RESULTCastFailureReason::ONLY_INDOORS => Self::ONLY_INDOORS,
            SMSG_CAST_RESULTCastFailureReason::ONLY_MOUNTED => Self::ONLY_MOUNTED,
            SMSG_CAST_RESULTCastFailureReason::ONLY_NIGHTTIME => Self::ONLY_NIGHTTIME,
            SMSG_CAST_RESULTCastFailureReason::ONLY_OUTDOORS => Self::ONLY_OUTDOORS,
            SMSG_CAST_RESULTCastFailureReason::ONLY_SHAPESHIFT => Self::ONLY_SHAPESHIFT,
            SMSG_CAST_RESULTCastFailureReason::ONLY_STEALTHED => Self::ONLY_STEALTHED,
            SMSG_CAST_RESULTCastFailureReason::ONLY_UNDERWATER => Self::ONLY_UNDERWATER,
            SMSG_CAST_RESULTCastFailureReason::OUT_OF_RANGE => Self::OUT_OF_RANGE,
            SMSG_CAST_RESULTCastFailureReason::PACIFIED => Self::PACIFIED,
            SMSG_CAST_RESULTCastFailureReason::POSSESSED => Self::POSSESSED,
            SMSG_CAST_RESULTCastFailureReason::REAGENTS => Self::REAGENTS,
            SMSG_CAST_RESULTCastFailureReason::REQUIRES_AREA { .. } => Self::REQUIRES_AREA,
            SMSG_CAST_RESULTCastFailureReason::REQUIRES_SPELL_FOCUS { .. } => Self::REQUIRES_SPELL_FOCUS,
            SMSG_CAST_RESULTCastFailureReason::ROOTED => Self::ROOTED,
            SMSG_CAST_RESULTCastFailureReason::SILENCED => Self::SILENCED,
            SMSG_CAST_RESULTCastFailureReason::SPELL_IN_PROGRESS => Self::SPELL_IN_PROGRESS,
            SMSG_CAST_RESULTCastFailureReason::SPELL_LEARNED => Self::SPELL_LEARNED,
            SMSG_CAST_RESULTCastFailureReason::SPELL_UNAVAILABLE => Self::SPELL_UNAVAILABLE,
            SMSG_CAST_RESULTCastFailureReason::STUNNED => Self::STUNNED,
            SMSG_CAST_RESULTCastFailureReason::TARGETS_DEAD => Self::TARGETS_DEAD,
            SMSG_CAST_RESULTCastFailureReason::TARGET_AFFECTING_COMBAT => Self::TARGET_AFFECTING_COMBAT,
            SMSG_CAST_RESULTCastFailureReason::TARGET_AURASTATE => Self::TARGET_AURASTATE,
            SMSG_CAST_RESULTCastFailureReason::TARGET_DUELING => Self::TARGET_DUELING,
            SMSG_CAST_RESULTCastFailureReason::TARGET_ENEMY => Self::TARGET_ENEMY,
            SMSG_CAST_RESULTCastFailureReason::TARGET_ENRAGED => Self::TARGET_ENRAGED,
            SMSG_CAST_RESULTCastFailureReason::TARGET_FRIENDLY => Self::TARGET_FRIENDLY,
            SMSG_CAST_RESULTCastFailureReason::TARGET_IN_COMBAT => Self::TARGET_IN_COMBAT,
            SMSG_CAST_RESULTCastFailureReason::TARGET_IS_PLAYER => Self::TARGET_IS_PLAYER,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_DEAD => Self::TARGET_NOT_DEAD,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_PARTY => Self::TARGET_NOT_IN_PARTY,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_LOOTED => Self::TARGET_NOT_LOOTED,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_PLAYER => Self::TARGET_NOT_PLAYER,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NO_POCKETS => Self::TARGET_NO_POCKETS,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NO_WEAPONS => Self::TARGET_NO_WEAPONS,
            SMSG_CAST_RESULTCastFailureReason::TARGET_UNSKINNABLE => Self::TARGET_UNSKINNABLE,
            SMSG_CAST_RESULTCastFailureReason::THIRST_SATIATED => Self::THIRST_SATIATED,
            SMSG_CAST_RESULTCastFailureReason::TOO_CLOSE => Self::TOO_CLOSE,
            SMSG_CAST_RESULTCastFailureReason::TOO_MANY_OF_ITEM => Self::TOO_MANY_OF_ITEM,
            SMSG_CAST_RESULTCastFailureReason::TOTEMS => Self::TOTEMS,
            SMSG_CAST_RESULTCastFailureReason::TRAINING_POINTS => Self::TRAINING_POINTS,
            SMSG_CAST_RESULTCastFailureReason::TRY_AGAIN => Self::TRY_AGAIN,
            SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_BEHIND => Self::UNIT_NOT_BEHIND,
            SMSG_CAST_RESULTCastFailureReason::UNIT_NOT_INFRONT => Self::UNIT_NOT_INFRONT,
            SMSG_CAST_RESULTCastFailureReason::WRONG_PET_FOOD => Self::WRONG_PET_FOOD,
            SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_FATIGUED => Self::NOT_WHILE_FATIGUED,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_INSTANCE => Self::TARGET_NOT_IN_INSTANCE,
            SMSG_CAST_RESULTCastFailureReason::NOT_WHILE_TRADING => Self::NOT_WHILE_TRADING,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_IN_RAID => Self::TARGET_NOT_IN_RAID,
            SMSG_CAST_RESULTCastFailureReason::DISENCHANT_WHILE_LOOTING => Self::DISENCHANT_WHILE_LOOTING,
            SMSG_CAST_RESULTCastFailureReason::PROSPECT_WHILE_LOOTING => Self::PROSPECT_WHILE_LOOTING,
            SMSG_CAST_RESULTCastFailureReason::PROSPECT_NEED_MORE => Self::PROSPECT_NEED_MORE,
            SMSG_CAST_RESULTCastFailureReason::TARGET_FREEFORALL => Self::TARGET_FREEFORALL,
            SMSG_CAST_RESULTCastFailureReason::NO_EDIBLE_CORPSES => Self::NO_EDIBLE_CORPSES,
            SMSG_CAST_RESULTCastFailureReason::ONLY_BATTLEGROUNDS => Self::ONLY_BATTLEGROUNDS,
            SMSG_CAST_RESULTCastFailureReason::TARGET_NOT_GHOST => Self::TARGET_NOT_GHOST,
            SMSG_CAST_RESULTCastFailureReason::TOO_MANY_SKILLS => Self::TOO_MANY_SKILLS,
            SMSG_CAST_RESULTCastFailureReason::TRANSFORM_UNUSABLE => Self::TRANSFORM_UNUSABLE,
            SMSG_CAST_RESULTCastFailureReason::WRONG_WEATHER => Self::WRONG_WEATHER,
            SMSG_CAST_RESULTCastFailureReason::DAMAGE_IMMUNE => Self::DAMAGE_IMMUNE,
            SMSG_CAST_RESULTCastFailureReason::PREVENTED_BY_MECHANIC => Self::PREVENTED_BY_MECHANIC,
            SMSG_CAST_RESULTCastFailureReason::PLAY_TIME => Self::PLAY_TIME,
            SMSG_CAST_RESULTCastFailureReason::REPUTATION => Self::REPUTATION,
            SMSG_CAST_RESULTCastFailureReason::MIN_SKILL => Self::MIN_SKILL,
            SMSG_CAST_RESULTCastFailureReason::UNKNOWN => Self::UNKNOWN,
        }
    }
}

impl Default for SMSG_CAST_RESULTCastFailureReason {
    fn default() -> Self {
        // First enumerator without any fields
        Self::AFFECTING_COMBAT
    }
}

impl SMSG_CAST_RESULTCastFailureReason {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.astd_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.astd_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.astd_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.astd_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.astd_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFailureReason = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_CAST_RESULTCastFailureReason {
    fn size(&self) -> usize {
        match self {
            Self::AFFECTING_COMBAT =>  {
                1
            }
            Self::ALREADY_AT_FULL_HEALTH =>  {
                1
            }
            Self::ALREADY_AT_FULL_POWER =>  {
                1
            }
            Self::ALREADY_BEING_TAMED =>  {
                1
            }
            Self::ALREADY_HAVE_CHARM =>  {
                1
            }
            Self::ALREADY_HAVE_SUMMON =>  {
                1
            }
            Self::ALREADY_OPEN =>  {
                1
            }
            Self::AURA_BOUNCED =>  {
                1
            }
            Self::AUTOTRACK_INTERRUPTED =>  {
                1
            }
            Self::BAD_IMPLICIT_TARGETS =>  {
                1
            }
            Self::BAD_TARGETS =>  {
                1
            }
            Self::CANT_BE_CHARMED =>  {
                1
            }
            Self::CANT_BE_DISENCHANTED =>  {
                1
            }
            Self::CANT_BE_PROSPECTED =>  {
                1
            }
            Self::CANT_CAST_ON_TAPPED =>  {
                1
            }
            Self::CANT_DUEL_WHILE_INVISIBLE =>  {
                1
            }
            Self::CANT_DUEL_WHILE_STEALTHED =>  {
                1
            }
            Self::CANT_STEALTH =>  {
                1
            }
            Self::CASTER_AURASTATE =>  {
                1
            }
            Self::CASTER_DEAD =>  {
                1
            }
            Self::CHARMED =>  {
                1
            }
            Self::CHEST_IN_USE =>  {
                1
            }
            Self::CONFUSED =>  {
                1
            }
            Self::DONT_REPORT =>  {
                1
            }
            Self::EQUIPPED_ITEM =>  {
                1
            }
            Self::EQUIPPED_ITEM_CLASS  {
                equipped_item_class,
                equipped_item_subclass_mask,
                equipped_item_inventory_type_mask,
            } => {
                1
                + 4 // equipped_item_class: u32
                + 4 // equipped_item_subclass_mask: u32
                + 4 // equipped_item_inventory_type_mask: u32
            }
            Self::EQUIPPED_ITEM_CLASS_MAINHAND =>  {
                1
            }
            Self::EQUIPPED_ITEM_CLASS_OFFHAND =>  {
                1
            }
            Self::ERROR =>  {
                1
            }
            Self::FIZZLE =>  {
                1
            }
            Self::FLEEING =>  {
                1
            }
            Self::FOOD_LOWLEVEL =>  {
                1
            }
            Self::HIGHLEVEL =>  {
                1
            }
            Self::HUNGER_SATIATED =>  {
                1
            }
            Self::IMMUNE =>  {
                1
            }
            Self::INTERRUPTED =>  {
                1
            }
            Self::INTERRUPTED_COMBAT =>  {
                1
            }
            Self::ITEM_ALREADY_ENCHANTED =>  {
                1
            }
            Self::ITEM_GONE =>  {
                1
            }
            Self::ITEM_NOT_FOUND =>  {
                1
            }
            Self::ITEM_NOT_READY =>  {
                1
            }
            Self::LEVEL_REQUIREMENT =>  {
                1
            }
            Self::LINE_OF_SIGHT =>  {
                1
            }
            Self::LOWLEVEL =>  {
                1
            }
            Self::LOW_CASTLEVEL =>  {
                1
            }
            Self::MAINHAND_EMPTY =>  {
                1
            }
            Self::MOVING =>  {
                1
            }
            Self::NEED_AMMO =>  {
                1
            }
            Self::NEED_AMMO_POUCH =>  {
                1
            }
            Self::NEED_EXOTIC_AMMO =>  {
                1
            }
            Self::NOPATH =>  {
                1
            }
            Self::NOT_BEHIND =>  {
                1
            }
            Self::NOT_FISHABLE =>  {
                1
            }
            Self::NOT_HERE =>  {
                1
            }
            Self::NOT_INFRONT =>  {
                1
            }
            Self::NOT_IN_CONTROL =>  {
                1
            }
            Self::NOT_KNOWN =>  {
                1
            }
            Self::NOT_MOUNTED =>  {
                1
            }
            Self::NOT_ON_TAXI =>  {
                1
            }
            Self::NOT_ON_TRANSPORT =>  {
                1
            }
            Self::NOT_READY =>  {
                1
            }
            Self::NOT_SHAPESHIFT =>  {
                1
            }
            Self::NOT_STANDING =>  {
                1
            }
            Self::NOT_TRADEABLE =>  {
                1
            }
            Self::NOT_TRADING =>  {
                1
            }
            Self::NOT_UNSHEATHED =>  {
                1
            }
            Self::NOT_WHILE_GHOST =>  {
                1
            }
            Self::NO_AMMO =>  {
                1
            }
            Self::NO_CHARGES_REMAIN =>  {
                1
            }
            Self::NO_CHAMPION =>  {
                1
            }
            Self::NO_COMBO_POINTS =>  {
                1
            }
            Self::NO_DUELING =>  {
                1
            }
            Self::NO_ENDURANCE =>  {
                1
            }
            Self::NO_FISH =>  {
                1
            }
            Self::NO_ITEMS_WHILE_SHAPESHIFTED =>  {
                1
            }
            Self::NO_MOUNTS_ALLOWED =>  {
                1
            }
            Self::NO_PET =>  {
                1
            }
            Self::NO_POWER =>  {
                1
            }
            Self::NOTHING_TO_DISPEL =>  {
                1
            }
            Self::NOTHING_TO_STEAL =>  {
                1
            }
            Self::ONLY_ABOVEWATER =>  {
                1
            }
            Self::ONLY_DAYTIME =>  {
                1
            }
            Self::ONLY_INDOORS =>  {
                1
            }
            Self::ONLY_MOUNTED =>  {
                1
            }
            Self::ONLY_NIGHTTIME =>  {
                1
            }
            Self::ONLY_OUTDOORS =>  {
                1
            }
            Self::ONLY_SHAPESHIFT =>  {
                1
            }
            Self::ONLY_STEALTHED =>  {
                1
            }
            Self::ONLY_UNDERWATER =>  {
                1
            }
            Self::OUT_OF_RANGE =>  {
                1
            }
            Self::PACIFIED =>  {
                1
            }
            Self::POSSESSED =>  {
                1
            }
            Self::REAGENTS =>  {
                1
            }
            Self::REQUIRES_AREA  {
                area,
            } => {
                1
                + Area::size() // area: Area
            }
            Self::REQUIRES_SPELL_FOCUS  {
                required_spell_focus,
            } => {
                1
                + 4 // required_spell_focus: u32
            }
            Self::ROOTED =>  {
                1
            }
            Self::SILENCED =>  {
                1
            }
            Self::SPELL_IN_PROGRESS =>  {
                1
            }
            Self::SPELL_LEARNED =>  {
                1
            }
            Self::SPELL_UNAVAILABLE =>  {
                1
            }
            Self::STUNNED =>  {
                1
            }
            Self::TARGETS_DEAD =>  {
                1
            }
            Self::TARGET_AFFECTING_COMBAT =>  {
                1
            }
            Self::TARGET_AURASTATE =>  {
                1
            }
            Self::TARGET_DUELING =>  {
                1
            }
            Self::TARGET_ENEMY =>  {
                1
            }
            Self::TARGET_ENRAGED =>  {
                1
            }
            Self::TARGET_FRIENDLY =>  {
                1
            }
            Self::TARGET_IN_COMBAT =>  {
                1
            }
            Self::TARGET_IS_PLAYER =>  {
                1
            }
            Self::TARGET_NOT_DEAD =>  {
                1
            }
            Self::TARGET_NOT_IN_PARTY =>  {
                1
            }
            Self::TARGET_NOT_LOOTED =>  {
                1
            }
            Self::TARGET_NOT_PLAYER =>  {
                1
            }
            Self::TARGET_NO_POCKETS =>  {
                1
            }
            Self::TARGET_NO_WEAPONS =>  {
                1
            }
            Self::TARGET_UNSKINNABLE =>  {
                1
            }
            Self::THIRST_SATIATED =>  {
                1
            }
            Self::TOO_CLOSE =>  {
                1
            }
            Self::TOO_MANY_OF_ITEM =>  {
                1
            }
            Self::TOTEMS =>  {
                1
            }
            Self::TRAINING_POINTS =>  {
                1
            }
            Self::TRY_AGAIN =>  {
                1
            }
            Self::UNIT_NOT_BEHIND =>  {
                1
            }
            Self::UNIT_NOT_INFRONT =>  {
                1
            }
            Self::WRONG_PET_FOOD =>  {
                1
            }
            Self::NOT_WHILE_FATIGUED =>  {
                1
            }
            Self::TARGET_NOT_IN_INSTANCE =>  {
                1
            }
            Self::NOT_WHILE_TRADING =>  {
                1
            }
            Self::TARGET_NOT_IN_RAID =>  {
                1
            }
            Self::DISENCHANT_WHILE_LOOTING =>  {
                1
            }
            Self::PROSPECT_WHILE_LOOTING =>  {
                1
            }
            Self::PROSPECT_NEED_MORE =>  {
                1
            }
            Self::TARGET_FREEFORALL =>  {
                1
            }
            Self::NO_EDIBLE_CORPSES =>  {
                1
            }
            Self::ONLY_BATTLEGROUNDS =>  {
                1
            }
            Self::TARGET_NOT_GHOST =>  {
                1
            }
            Self::TOO_MANY_SKILLS =>  {
                1
            }
            Self::TRANSFORM_UNUSABLE =>  {
                1
            }
            Self::WRONG_WEATHER =>  {
                1
            }
            Self::DAMAGE_IMMUNE =>  {
                1
            }
            Self::PREVENTED_BY_MECHANIC =>  {
                1
            }
            Self::PLAY_TIME =>  {
                1
            }
            Self::REPUTATION =>  {
                1
            }
            Self::MIN_SKILL =>  {
                1
            }
            Self::UNKNOWN =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_CAST_RESULTCastFailureReason {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_CAST_RESULTSimpleSpellCastResult {
    SUCCESS {
        reason: SMSG_CAST_RESULTCastFailureReason,
    },
    FAILURE,
}

impl From<&SimpleSpellCastResult> for SMSG_CAST_RESULTSimpleSpellCastResult {
    fn from(e: &SimpleSpellCastResult) -> Self {
        match &e {
            SimpleSpellCastResult::SUCCESS => Self::SUCCESS {
                reason: Default::default(),
            },
            SimpleSpellCastResult::FAILURE => Self::FAILURE,
        }
    }
}

impl From<&SMSG_CAST_RESULTSimpleSpellCastResult> for SimpleSpellCastResult {
    fn from(v: &SMSG_CAST_RESULTSimpleSpellCastResult) -> Self {
        match &v {
            SMSG_CAST_RESULTSimpleSpellCastResult::SUCCESS { .. } => Self::SUCCESS,
            SMSG_CAST_RESULTSimpleSpellCastResult::FAILURE => Self::FAILURE,
        }
    }
}

impl Default for SMSG_CAST_RESULTSimpleSpellCastResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            reason: Default::default(),
        }
    }
}

impl SMSG_CAST_RESULTSimpleSpellCastResult {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.astd_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.astd_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.astd_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.astd_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.astd_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SimpleSpellCastResult = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_CAST_RESULTSimpleSpellCastResult {
    fn size(&self) -> usize {
        match self {
            Self::SUCCESS  {
                reason,
            } => {
                1
                + reason.size() // reason: SMSG_CAST_RESULTCastFailureReason and subfields
            }
            Self::FAILURE =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_CAST_RESULTSimpleSpellCastResult {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

