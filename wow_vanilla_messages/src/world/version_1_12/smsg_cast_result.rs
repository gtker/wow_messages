use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Area;
use crate::world::version_1_12::CastFailureReason;
use crate::world::version_1_12::SimpleSpellCastResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
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

impl ServerMessage for SMSG_CAST_RESULT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // result: SimpleSpellCastResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_CAST_RESULT_SimpleSpellCastResult::SUCCESS {
                reason,
            } => {
                // reason: CastFailureReason
                w.write_all(&(reason.as_int() as u8).to_le_bytes())?;

                match &reason {
                    SMSG_CAST_RESULT_CastFailureReason::AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULT_CastFailureReason::ALREADY_AT_FULL_HEALTH => {}
                    SMSG_CAST_RESULT_CastFailureReason::ALREADY_AT_FULL_POWER => {}
                    SMSG_CAST_RESULT_CastFailureReason::ALREADY_BEING_TAMED => {}
                    SMSG_CAST_RESULT_CastFailureReason::ALREADY_HAVE_CHARM => {}
                    SMSG_CAST_RESULT_CastFailureReason::ALREADY_HAVE_SUMMON => {}
                    SMSG_CAST_RESULT_CastFailureReason::ALREADY_OPEN => {}
                    SMSG_CAST_RESULT_CastFailureReason::AURA_BOUNCED => {}
                    SMSG_CAST_RESULT_CastFailureReason::AUTOTRACK_INTERRUPTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::BAD_IMPLICIT_TARGETS => {}
                    SMSG_CAST_RESULT_CastFailureReason::BAD_TARGETS => {}
                    SMSG_CAST_RESULT_CastFailureReason::CANT_BE_CHARMED => {}
                    SMSG_CAST_RESULT_CastFailureReason::CANT_BE_DISENCHANTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::CANT_BE_PROSPECTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::CANT_CAST_ON_TAPPED => {}
                    SMSG_CAST_RESULT_CastFailureReason::CANT_DUEL_WHILE_INVISIBLE => {}
                    SMSG_CAST_RESULT_CastFailureReason::CANT_DUEL_WHILE_STEALTHED => {}
                    SMSG_CAST_RESULT_CastFailureReason::CANT_STEALTH => {}
                    SMSG_CAST_RESULT_CastFailureReason::CASTER_AURASTATE => {}
                    SMSG_CAST_RESULT_CastFailureReason::CASTER_DEAD => {}
                    SMSG_CAST_RESULT_CastFailureReason::CHARMED => {}
                    SMSG_CAST_RESULT_CastFailureReason::CHEST_IN_USE => {}
                    SMSG_CAST_RESULT_CastFailureReason::CONFUSED => {}
                    SMSG_CAST_RESULT_CastFailureReason::DONT_REPORT => {}
                    SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM => {}
                    SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM_CLASS {
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
                    SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => {}
                    SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => {}
                    SMSG_CAST_RESULT_CastFailureReason::ERROR => {}
                    SMSG_CAST_RESULT_CastFailureReason::FIZZLE => {}
                    SMSG_CAST_RESULT_CastFailureReason::FLEEING => {}
                    SMSG_CAST_RESULT_CastFailureReason::FOOD_LOWLEVEL => {}
                    SMSG_CAST_RESULT_CastFailureReason::HIGHLEVEL => {}
                    SMSG_CAST_RESULT_CastFailureReason::HUNGER_SATIATED => {}
                    SMSG_CAST_RESULT_CastFailureReason::IMMUNE => {}
                    SMSG_CAST_RESULT_CastFailureReason::INTERRUPTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::INTERRUPTED_COMBAT => {}
                    SMSG_CAST_RESULT_CastFailureReason::ITEM_ALREADY_ENCHANTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::ITEM_GONE => {}
                    SMSG_CAST_RESULT_CastFailureReason::ITEM_NOT_FOUND => {}
                    SMSG_CAST_RESULT_CastFailureReason::ITEM_NOT_READY => {}
                    SMSG_CAST_RESULT_CastFailureReason::LEVEL_REQUIREMENT => {}
                    SMSG_CAST_RESULT_CastFailureReason::LINE_OF_SIGHT => {}
                    SMSG_CAST_RESULT_CastFailureReason::LOWLEVEL => {}
                    SMSG_CAST_RESULT_CastFailureReason::LOW_CASTLEVEL => {}
                    SMSG_CAST_RESULT_CastFailureReason::MAINHAND_EMPTY => {}
                    SMSG_CAST_RESULT_CastFailureReason::MOVING => {}
                    SMSG_CAST_RESULT_CastFailureReason::NEED_AMMO => {}
                    SMSG_CAST_RESULT_CastFailureReason::NEED_AMMO_POUCH => {}
                    SMSG_CAST_RESULT_CastFailureReason::NEED_EXOTIC_AMMO => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOPATH => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_BEHIND => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_FISHABLE => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_HERE => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_INFRONT => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_IN_CONTROL => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_KNOWN => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_MOUNTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_ON_TAXI => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_ON_TRANSPORT => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_READY => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_SHAPESHIFT => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_STANDING => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_TRADEABLE => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_TRADING => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_UNSHEATHED => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_WHILE_GHOST => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_AMMO => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_CHARGES_REMAIN => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_CHAMPION => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_COMBO_POINTS => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_DUELING => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_ENDURANCE => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_FISH => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_MOUNTS_ALLOWED => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_PET => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_POWER => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOTHING_TO_DISPEL => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOTHING_TO_STEAL => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_ABOVEWATER => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_DAYTIME => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_INDOORS => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_MOUNTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_NIGHTTIME => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_OUTDOORS => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_SHAPESHIFT => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_STEALTHED => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_UNDERWATER => {}
                    SMSG_CAST_RESULT_CastFailureReason::OUT_OF_RANGE => {}
                    SMSG_CAST_RESULT_CastFailureReason::PACIFIED => {}
                    SMSG_CAST_RESULT_CastFailureReason::POSSESSED => {}
                    SMSG_CAST_RESULT_CastFailureReason::REAGENTS => {}
                    SMSG_CAST_RESULT_CastFailureReason::REQUIRES_AREA {
                        area,
                    } => {
                        // area: Area
                        w.write_all(&(area.as_int() as u32).to_le_bytes())?;

                    }
                    SMSG_CAST_RESULT_CastFailureReason::REQUIRES_SPELL_FOCUS {
                        required_spell_focus,
                    } => {
                        // required_spell_focus: u32
                        w.write_all(&required_spell_focus.to_le_bytes())?;

                    }
                    SMSG_CAST_RESULT_CastFailureReason::ROOTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::SILENCED => {}
                    SMSG_CAST_RESULT_CastFailureReason::SPELL_IN_PROGRESS => {}
                    SMSG_CAST_RESULT_CastFailureReason::SPELL_LEARNED => {}
                    SMSG_CAST_RESULT_CastFailureReason::SPELL_UNAVAILABLE => {}
                    SMSG_CAST_RESULT_CastFailureReason::STUNNED => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGETS_DEAD => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_AFFECTING_COMBAT => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_AURASTATE => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_DUELING => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_ENEMY => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_ENRAGED => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_FRIENDLY => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_IN_COMBAT => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_IS_PLAYER => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_DEAD => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_IN_PARTY => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_LOOTED => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_PLAYER => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NO_POCKETS => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NO_WEAPONS => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_UNSKINNABLE => {}
                    SMSG_CAST_RESULT_CastFailureReason::THIRST_SATIATED => {}
                    SMSG_CAST_RESULT_CastFailureReason::TOO_CLOSE => {}
                    SMSG_CAST_RESULT_CastFailureReason::TOO_MANY_OF_ITEM => {}
                    SMSG_CAST_RESULT_CastFailureReason::TOTEMS => {}
                    SMSG_CAST_RESULT_CastFailureReason::TRAINING_POINTS => {}
                    SMSG_CAST_RESULT_CastFailureReason::TRY_AGAIN => {}
                    SMSG_CAST_RESULT_CastFailureReason::UNIT_NOT_BEHIND => {}
                    SMSG_CAST_RESULT_CastFailureReason::UNIT_NOT_INFRONT => {}
                    SMSG_CAST_RESULT_CastFailureReason::WRONG_PET_FOOD => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_WHILE_FATIGUED => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_IN_INSTANCE => {}
                    SMSG_CAST_RESULT_CastFailureReason::NOT_WHILE_TRADING => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_IN_RAID => {}
                    SMSG_CAST_RESULT_CastFailureReason::DISENCHANT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULT_CastFailureReason::PROSPECT_WHILE_LOOTING => {}
                    SMSG_CAST_RESULT_CastFailureReason::PROSPECT_NEED_MORE => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_FREEFORALL => {}
                    SMSG_CAST_RESULT_CastFailureReason::NO_EDIBLE_CORPSES => {}
                    SMSG_CAST_RESULT_CastFailureReason::ONLY_BATTLEGROUNDS => {}
                    SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_GHOST => {}
                    SMSG_CAST_RESULT_CastFailureReason::TOO_MANY_SKILLS => {}
                    SMSG_CAST_RESULT_CastFailureReason::TRANSFORM_UNUSABLE => {}
                    SMSG_CAST_RESULT_CastFailureReason::WRONG_WEATHER => {}
                    SMSG_CAST_RESULT_CastFailureReason::DAMAGE_IMMUNE => {}
                    SMSG_CAST_RESULT_CastFailureReason::PREVENTED_BY_MECHANIC => {}
                    SMSG_CAST_RESULT_CastFailureReason::PLAY_TIME => {}
                    SMSG_CAST_RESULT_CastFailureReason::REPUTATION => {}
                    SMSG_CAST_RESULT_CastFailureReason::MIN_SKILL => {}
                    SMSG_CAST_RESULT_CastFailureReason::UNKNOWN => {}
                }

            }
            SMSG_CAST_RESULT_SimpleSpellCastResult::FAILURE => {}
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0130;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // result: SimpleSpellCastResult
        let result: SimpleSpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            SimpleSpellCastResult::SUCCESS => {
                // reason: CastFailureReason
                let reason: CastFailureReason = crate::util::read_u8_le(r)?.try_into()?;

                let reason_if = match reason {
                    CastFailureReason::AFFECTING_COMBAT => SMSG_CAST_RESULT_CastFailureReason::AFFECTING_COMBAT,
                    CastFailureReason::ALREADY_AT_FULL_HEALTH => SMSG_CAST_RESULT_CastFailureReason::ALREADY_AT_FULL_HEALTH,
                    CastFailureReason::ALREADY_AT_FULL_POWER => SMSG_CAST_RESULT_CastFailureReason::ALREADY_AT_FULL_POWER,
                    CastFailureReason::ALREADY_BEING_TAMED => SMSG_CAST_RESULT_CastFailureReason::ALREADY_BEING_TAMED,
                    CastFailureReason::ALREADY_HAVE_CHARM => SMSG_CAST_RESULT_CastFailureReason::ALREADY_HAVE_CHARM,
                    CastFailureReason::ALREADY_HAVE_SUMMON => SMSG_CAST_RESULT_CastFailureReason::ALREADY_HAVE_SUMMON,
                    CastFailureReason::ALREADY_OPEN => SMSG_CAST_RESULT_CastFailureReason::ALREADY_OPEN,
                    CastFailureReason::AURA_BOUNCED => SMSG_CAST_RESULT_CastFailureReason::AURA_BOUNCED,
                    CastFailureReason::AUTOTRACK_INTERRUPTED => SMSG_CAST_RESULT_CastFailureReason::AUTOTRACK_INTERRUPTED,
                    CastFailureReason::BAD_IMPLICIT_TARGETS => SMSG_CAST_RESULT_CastFailureReason::BAD_IMPLICIT_TARGETS,
                    CastFailureReason::BAD_TARGETS => SMSG_CAST_RESULT_CastFailureReason::BAD_TARGETS,
                    CastFailureReason::CANT_BE_CHARMED => SMSG_CAST_RESULT_CastFailureReason::CANT_BE_CHARMED,
                    CastFailureReason::CANT_BE_DISENCHANTED => SMSG_CAST_RESULT_CastFailureReason::CANT_BE_DISENCHANTED,
                    CastFailureReason::CANT_BE_PROSPECTED => SMSG_CAST_RESULT_CastFailureReason::CANT_BE_PROSPECTED,
                    CastFailureReason::CANT_CAST_ON_TAPPED => SMSG_CAST_RESULT_CastFailureReason::CANT_CAST_ON_TAPPED,
                    CastFailureReason::CANT_DUEL_WHILE_INVISIBLE => SMSG_CAST_RESULT_CastFailureReason::CANT_DUEL_WHILE_INVISIBLE,
                    CastFailureReason::CANT_DUEL_WHILE_STEALTHED => SMSG_CAST_RESULT_CastFailureReason::CANT_DUEL_WHILE_STEALTHED,
                    CastFailureReason::CANT_STEALTH => SMSG_CAST_RESULT_CastFailureReason::CANT_STEALTH,
                    CastFailureReason::CASTER_AURASTATE => SMSG_CAST_RESULT_CastFailureReason::CASTER_AURASTATE,
                    CastFailureReason::CASTER_DEAD => SMSG_CAST_RESULT_CastFailureReason::CASTER_DEAD,
                    CastFailureReason::CHARMED => SMSG_CAST_RESULT_CastFailureReason::CHARMED,
                    CastFailureReason::CHEST_IN_USE => SMSG_CAST_RESULT_CastFailureReason::CHEST_IN_USE,
                    CastFailureReason::CONFUSED => SMSG_CAST_RESULT_CastFailureReason::CONFUSED,
                    CastFailureReason::DONT_REPORT => SMSG_CAST_RESULT_CastFailureReason::DONT_REPORT,
                    CastFailureReason::EQUIPPED_ITEM => SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM,
                    CastFailureReason::EQUIPPED_ITEM_CLASS => {
                        // equipped_item_class: u32
                        let equipped_item_class = crate::util::read_u32_le(r)?;

                        // equipped_item_subclass_mask: u32
                        let equipped_item_subclass_mask = crate::util::read_u32_le(r)?;

                        // equipped_item_inventory_type_mask: u32
                        let equipped_item_inventory_type_mask = crate::util::read_u32_le(r)?;

                        SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM_CLASS {
                            equipped_item_class,
                            equipped_item_inventory_type_mask,
                            equipped_item_subclass_mask,
                        }
                    }
                    CastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND => SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM_CLASS_MAINHAND,
                    CastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND => SMSG_CAST_RESULT_CastFailureReason::EQUIPPED_ITEM_CLASS_OFFHAND,
                    CastFailureReason::ERROR => SMSG_CAST_RESULT_CastFailureReason::ERROR,
                    CastFailureReason::FIZZLE => SMSG_CAST_RESULT_CastFailureReason::FIZZLE,
                    CastFailureReason::FLEEING => SMSG_CAST_RESULT_CastFailureReason::FLEEING,
                    CastFailureReason::FOOD_LOWLEVEL => SMSG_CAST_RESULT_CastFailureReason::FOOD_LOWLEVEL,
                    CastFailureReason::HIGHLEVEL => SMSG_CAST_RESULT_CastFailureReason::HIGHLEVEL,
                    CastFailureReason::HUNGER_SATIATED => SMSG_CAST_RESULT_CastFailureReason::HUNGER_SATIATED,
                    CastFailureReason::IMMUNE => SMSG_CAST_RESULT_CastFailureReason::IMMUNE,
                    CastFailureReason::INTERRUPTED => SMSG_CAST_RESULT_CastFailureReason::INTERRUPTED,
                    CastFailureReason::INTERRUPTED_COMBAT => SMSG_CAST_RESULT_CastFailureReason::INTERRUPTED_COMBAT,
                    CastFailureReason::ITEM_ALREADY_ENCHANTED => SMSG_CAST_RESULT_CastFailureReason::ITEM_ALREADY_ENCHANTED,
                    CastFailureReason::ITEM_GONE => SMSG_CAST_RESULT_CastFailureReason::ITEM_GONE,
                    CastFailureReason::ITEM_NOT_FOUND => SMSG_CAST_RESULT_CastFailureReason::ITEM_NOT_FOUND,
                    CastFailureReason::ITEM_NOT_READY => SMSG_CAST_RESULT_CastFailureReason::ITEM_NOT_READY,
                    CastFailureReason::LEVEL_REQUIREMENT => SMSG_CAST_RESULT_CastFailureReason::LEVEL_REQUIREMENT,
                    CastFailureReason::LINE_OF_SIGHT => SMSG_CAST_RESULT_CastFailureReason::LINE_OF_SIGHT,
                    CastFailureReason::LOWLEVEL => SMSG_CAST_RESULT_CastFailureReason::LOWLEVEL,
                    CastFailureReason::LOW_CASTLEVEL => SMSG_CAST_RESULT_CastFailureReason::LOW_CASTLEVEL,
                    CastFailureReason::MAINHAND_EMPTY => SMSG_CAST_RESULT_CastFailureReason::MAINHAND_EMPTY,
                    CastFailureReason::MOVING => SMSG_CAST_RESULT_CastFailureReason::MOVING,
                    CastFailureReason::NEED_AMMO => SMSG_CAST_RESULT_CastFailureReason::NEED_AMMO,
                    CastFailureReason::NEED_AMMO_POUCH => SMSG_CAST_RESULT_CastFailureReason::NEED_AMMO_POUCH,
                    CastFailureReason::NEED_EXOTIC_AMMO => SMSG_CAST_RESULT_CastFailureReason::NEED_EXOTIC_AMMO,
                    CastFailureReason::NOPATH => SMSG_CAST_RESULT_CastFailureReason::NOPATH,
                    CastFailureReason::NOT_BEHIND => SMSG_CAST_RESULT_CastFailureReason::NOT_BEHIND,
                    CastFailureReason::NOT_FISHABLE => SMSG_CAST_RESULT_CastFailureReason::NOT_FISHABLE,
                    CastFailureReason::NOT_HERE => SMSG_CAST_RESULT_CastFailureReason::NOT_HERE,
                    CastFailureReason::NOT_INFRONT => SMSG_CAST_RESULT_CastFailureReason::NOT_INFRONT,
                    CastFailureReason::NOT_IN_CONTROL => SMSG_CAST_RESULT_CastFailureReason::NOT_IN_CONTROL,
                    CastFailureReason::NOT_KNOWN => SMSG_CAST_RESULT_CastFailureReason::NOT_KNOWN,
                    CastFailureReason::NOT_MOUNTED => SMSG_CAST_RESULT_CastFailureReason::NOT_MOUNTED,
                    CastFailureReason::NOT_ON_TAXI => SMSG_CAST_RESULT_CastFailureReason::NOT_ON_TAXI,
                    CastFailureReason::NOT_ON_TRANSPORT => SMSG_CAST_RESULT_CastFailureReason::NOT_ON_TRANSPORT,
                    CastFailureReason::NOT_READY => SMSG_CAST_RESULT_CastFailureReason::NOT_READY,
                    CastFailureReason::NOT_SHAPESHIFT => SMSG_CAST_RESULT_CastFailureReason::NOT_SHAPESHIFT,
                    CastFailureReason::NOT_STANDING => SMSG_CAST_RESULT_CastFailureReason::NOT_STANDING,
                    CastFailureReason::NOT_TRADEABLE => SMSG_CAST_RESULT_CastFailureReason::NOT_TRADEABLE,
                    CastFailureReason::NOT_TRADING => SMSG_CAST_RESULT_CastFailureReason::NOT_TRADING,
                    CastFailureReason::NOT_UNSHEATHED => SMSG_CAST_RESULT_CastFailureReason::NOT_UNSHEATHED,
                    CastFailureReason::NOT_WHILE_GHOST => SMSG_CAST_RESULT_CastFailureReason::NOT_WHILE_GHOST,
                    CastFailureReason::NO_AMMO => SMSG_CAST_RESULT_CastFailureReason::NO_AMMO,
                    CastFailureReason::NO_CHARGES_REMAIN => SMSG_CAST_RESULT_CastFailureReason::NO_CHARGES_REMAIN,
                    CastFailureReason::NO_CHAMPION => SMSG_CAST_RESULT_CastFailureReason::NO_CHAMPION,
                    CastFailureReason::NO_COMBO_POINTS => SMSG_CAST_RESULT_CastFailureReason::NO_COMBO_POINTS,
                    CastFailureReason::NO_DUELING => SMSG_CAST_RESULT_CastFailureReason::NO_DUELING,
                    CastFailureReason::NO_ENDURANCE => SMSG_CAST_RESULT_CastFailureReason::NO_ENDURANCE,
                    CastFailureReason::NO_FISH => SMSG_CAST_RESULT_CastFailureReason::NO_FISH,
                    CastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED => SMSG_CAST_RESULT_CastFailureReason::NO_ITEMS_WHILE_SHAPESHIFTED,
                    CastFailureReason::NO_MOUNTS_ALLOWED => SMSG_CAST_RESULT_CastFailureReason::NO_MOUNTS_ALLOWED,
                    CastFailureReason::NO_PET => SMSG_CAST_RESULT_CastFailureReason::NO_PET,
                    CastFailureReason::NO_POWER => SMSG_CAST_RESULT_CastFailureReason::NO_POWER,
                    CastFailureReason::NOTHING_TO_DISPEL => SMSG_CAST_RESULT_CastFailureReason::NOTHING_TO_DISPEL,
                    CastFailureReason::NOTHING_TO_STEAL => SMSG_CAST_RESULT_CastFailureReason::NOTHING_TO_STEAL,
                    CastFailureReason::ONLY_ABOVEWATER => SMSG_CAST_RESULT_CastFailureReason::ONLY_ABOVEWATER,
                    CastFailureReason::ONLY_DAYTIME => SMSG_CAST_RESULT_CastFailureReason::ONLY_DAYTIME,
                    CastFailureReason::ONLY_INDOORS => SMSG_CAST_RESULT_CastFailureReason::ONLY_INDOORS,
                    CastFailureReason::ONLY_MOUNTED => SMSG_CAST_RESULT_CastFailureReason::ONLY_MOUNTED,
                    CastFailureReason::ONLY_NIGHTTIME => SMSG_CAST_RESULT_CastFailureReason::ONLY_NIGHTTIME,
                    CastFailureReason::ONLY_OUTDOORS => SMSG_CAST_RESULT_CastFailureReason::ONLY_OUTDOORS,
                    CastFailureReason::ONLY_SHAPESHIFT => SMSG_CAST_RESULT_CastFailureReason::ONLY_SHAPESHIFT,
                    CastFailureReason::ONLY_STEALTHED => SMSG_CAST_RESULT_CastFailureReason::ONLY_STEALTHED,
                    CastFailureReason::ONLY_UNDERWATER => SMSG_CAST_RESULT_CastFailureReason::ONLY_UNDERWATER,
                    CastFailureReason::OUT_OF_RANGE => SMSG_CAST_RESULT_CastFailureReason::OUT_OF_RANGE,
                    CastFailureReason::PACIFIED => SMSG_CAST_RESULT_CastFailureReason::PACIFIED,
                    CastFailureReason::POSSESSED => SMSG_CAST_RESULT_CastFailureReason::POSSESSED,
                    CastFailureReason::REAGENTS => SMSG_CAST_RESULT_CastFailureReason::REAGENTS,
                    CastFailureReason::REQUIRES_AREA => {
                        // area: Area
                        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                        SMSG_CAST_RESULT_CastFailureReason::REQUIRES_AREA {
                            area,
                        }
                    }
                    CastFailureReason::REQUIRES_SPELL_FOCUS => {
                        // required_spell_focus: u32
                        let required_spell_focus = crate::util::read_u32_le(r)?;

                        SMSG_CAST_RESULT_CastFailureReason::REQUIRES_SPELL_FOCUS {
                            required_spell_focus,
                        }
                    }
                    CastFailureReason::ROOTED => SMSG_CAST_RESULT_CastFailureReason::ROOTED,
                    CastFailureReason::SILENCED => SMSG_CAST_RESULT_CastFailureReason::SILENCED,
                    CastFailureReason::SPELL_IN_PROGRESS => SMSG_CAST_RESULT_CastFailureReason::SPELL_IN_PROGRESS,
                    CastFailureReason::SPELL_LEARNED => SMSG_CAST_RESULT_CastFailureReason::SPELL_LEARNED,
                    CastFailureReason::SPELL_UNAVAILABLE => SMSG_CAST_RESULT_CastFailureReason::SPELL_UNAVAILABLE,
                    CastFailureReason::STUNNED => SMSG_CAST_RESULT_CastFailureReason::STUNNED,
                    CastFailureReason::TARGETS_DEAD => SMSG_CAST_RESULT_CastFailureReason::TARGETS_DEAD,
                    CastFailureReason::TARGET_AFFECTING_COMBAT => SMSG_CAST_RESULT_CastFailureReason::TARGET_AFFECTING_COMBAT,
                    CastFailureReason::TARGET_AURASTATE => SMSG_CAST_RESULT_CastFailureReason::TARGET_AURASTATE,
                    CastFailureReason::TARGET_DUELING => SMSG_CAST_RESULT_CastFailureReason::TARGET_DUELING,
                    CastFailureReason::TARGET_ENEMY => SMSG_CAST_RESULT_CastFailureReason::TARGET_ENEMY,
                    CastFailureReason::TARGET_ENRAGED => SMSG_CAST_RESULT_CastFailureReason::TARGET_ENRAGED,
                    CastFailureReason::TARGET_FRIENDLY => SMSG_CAST_RESULT_CastFailureReason::TARGET_FRIENDLY,
                    CastFailureReason::TARGET_IN_COMBAT => SMSG_CAST_RESULT_CastFailureReason::TARGET_IN_COMBAT,
                    CastFailureReason::TARGET_IS_PLAYER => SMSG_CAST_RESULT_CastFailureReason::TARGET_IS_PLAYER,
                    CastFailureReason::TARGET_NOT_DEAD => SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_DEAD,
                    CastFailureReason::TARGET_NOT_IN_PARTY => SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_IN_PARTY,
                    CastFailureReason::TARGET_NOT_LOOTED => SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_LOOTED,
                    CastFailureReason::TARGET_NOT_PLAYER => SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_PLAYER,
                    CastFailureReason::TARGET_NO_POCKETS => SMSG_CAST_RESULT_CastFailureReason::TARGET_NO_POCKETS,
                    CastFailureReason::TARGET_NO_WEAPONS => SMSG_CAST_RESULT_CastFailureReason::TARGET_NO_WEAPONS,
                    CastFailureReason::TARGET_UNSKINNABLE => SMSG_CAST_RESULT_CastFailureReason::TARGET_UNSKINNABLE,
                    CastFailureReason::THIRST_SATIATED => SMSG_CAST_RESULT_CastFailureReason::THIRST_SATIATED,
                    CastFailureReason::TOO_CLOSE => SMSG_CAST_RESULT_CastFailureReason::TOO_CLOSE,
                    CastFailureReason::TOO_MANY_OF_ITEM => SMSG_CAST_RESULT_CastFailureReason::TOO_MANY_OF_ITEM,
                    CastFailureReason::TOTEMS => SMSG_CAST_RESULT_CastFailureReason::TOTEMS,
                    CastFailureReason::TRAINING_POINTS => SMSG_CAST_RESULT_CastFailureReason::TRAINING_POINTS,
                    CastFailureReason::TRY_AGAIN => SMSG_CAST_RESULT_CastFailureReason::TRY_AGAIN,
                    CastFailureReason::UNIT_NOT_BEHIND => SMSG_CAST_RESULT_CastFailureReason::UNIT_NOT_BEHIND,
                    CastFailureReason::UNIT_NOT_INFRONT => SMSG_CAST_RESULT_CastFailureReason::UNIT_NOT_INFRONT,
                    CastFailureReason::WRONG_PET_FOOD => SMSG_CAST_RESULT_CastFailureReason::WRONG_PET_FOOD,
                    CastFailureReason::NOT_WHILE_FATIGUED => SMSG_CAST_RESULT_CastFailureReason::NOT_WHILE_FATIGUED,
                    CastFailureReason::TARGET_NOT_IN_INSTANCE => SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_IN_INSTANCE,
                    CastFailureReason::NOT_WHILE_TRADING => SMSG_CAST_RESULT_CastFailureReason::NOT_WHILE_TRADING,
                    CastFailureReason::TARGET_NOT_IN_RAID => SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_IN_RAID,
                    CastFailureReason::DISENCHANT_WHILE_LOOTING => SMSG_CAST_RESULT_CastFailureReason::DISENCHANT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_WHILE_LOOTING => SMSG_CAST_RESULT_CastFailureReason::PROSPECT_WHILE_LOOTING,
                    CastFailureReason::PROSPECT_NEED_MORE => SMSG_CAST_RESULT_CastFailureReason::PROSPECT_NEED_MORE,
                    CastFailureReason::TARGET_FREEFORALL => SMSG_CAST_RESULT_CastFailureReason::TARGET_FREEFORALL,
                    CastFailureReason::NO_EDIBLE_CORPSES => SMSG_CAST_RESULT_CastFailureReason::NO_EDIBLE_CORPSES,
                    CastFailureReason::ONLY_BATTLEGROUNDS => SMSG_CAST_RESULT_CastFailureReason::ONLY_BATTLEGROUNDS,
                    CastFailureReason::TARGET_NOT_GHOST => SMSG_CAST_RESULT_CastFailureReason::TARGET_NOT_GHOST,
                    CastFailureReason::TOO_MANY_SKILLS => SMSG_CAST_RESULT_CastFailureReason::TOO_MANY_SKILLS,
                    CastFailureReason::TRANSFORM_UNUSABLE => SMSG_CAST_RESULT_CastFailureReason::TRANSFORM_UNUSABLE,
                    CastFailureReason::WRONG_WEATHER => SMSG_CAST_RESULT_CastFailureReason::WRONG_WEATHER,
                    CastFailureReason::DAMAGE_IMMUNE => SMSG_CAST_RESULT_CastFailureReason::DAMAGE_IMMUNE,
                    CastFailureReason::PREVENTED_BY_MECHANIC => SMSG_CAST_RESULT_CastFailureReason::PREVENTED_BY_MECHANIC,
                    CastFailureReason::PLAY_TIME => SMSG_CAST_RESULT_CastFailureReason::PLAY_TIME,
                    CastFailureReason::REPUTATION => SMSG_CAST_RESULT_CastFailureReason::REPUTATION,
                    CastFailureReason::MIN_SKILL => SMSG_CAST_RESULT_CastFailureReason::MIN_SKILL,
                    CastFailureReason::UNKNOWN => SMSG_CAST_RESULT_CastFailureReason::UNKNOWN,
                };

                SMSG_CAST_RESULT_SimpleSpellCastResult::SUCCESS {
                    reason: reason_if,
                }
            }
            SimpleSpellCastResult::FAILURE => SMSG_CAST_RESULT_SimpleSpellCastResult::FAILURE,
        };

        Ok(Self {
            spell,
            result: result_if,
        })
    }

}

impl SMSG_CAST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // spell: u32
        + self.result.size() // result: SMSG_CAST_RESULT_SimpleSpellCastResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_CAST_RESULT_CastFailureReason {
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
        equipped_item_inventory_type_mask: u32,
        equipped_item_subclass_mask: u32,
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

impl Default for SMSG_CAST_RESULT_CastFailureReason {
    fn default() -> Self {
        // First enumerator without any fields
        Self::AFFECTING_COMBAT
    }
}

impl SMSG_CAST_RESULT_CastFailureReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::AFFECTING_COMBAT => 0,
            Self::ALREADY_AT_FULL_HEALTH => 1,
            Self::ALREADY_AT_FULL_POWER => 2,
            Self::ALREADY_BEING_TAMED => 3,
            Self::ALREADY_HAVE_CHARM => 4,
            Self::ALREADY_HAVE_SUMMON => 5,
            Self::ALREADY_OPEN => 6,
            Self::AURA_BOUNCED => 7,
            Self::AUTOTRACK_INTERRUPTED => 8,
            Self::BAD_IMPLICIT_TARGETS => 9,
            Self::BAD_TARGETS => 10,
            Self::CANT_BE_CHARMED => 11,
            Self::CANT_BE_DISENCHANTED => 12,
            Self::CANT_BE_PROSPECTED => 13,
            Self::CANT_CAST_ON_TAPPED => 14,
            Self::CANT_DUEL_WHILE_INVISIBLE => 15,
            Self::CANT_DUEL_WHILE_STEALTHED => 16,
            Self::CANT_STEALTH => 17,
            Self::CASTER_AURASTATE => 18,
            Self::CASTER_DEAD => 19,
            Self::CHARMED => 20,
            Self::CHEST_IN_USE => 21,
            Self::CONFUSED => 22,
            Self::DONT_REPORT => 23,
            Self::EQUIPPED_ITEM => 24,
            Self::EQUIPPED_ITEM_CLASS { .. } => 25,
            Self::EQUIPPED_ITEM_CLASS_MAINHAND => 26,
            Self::EQUIPPED_ITEM_CLASS_OFFHAND => 27,
            Self::ERROR => 28,
            Self::FIZZLE => 29,
            Self::FLEEING => 30,
            Self::FOOD_LOWLEVEL => 31,
            Self::HIGHLEVEL => 32,
            Self::HUNGER_SATIATED => 33,
            Self::IMMUNE => 34,
            Self::INTERRUPTED => 35,
            Self::INTERRUPTED_COMBAT => 36,
            Self::ITEM_ALREADY_ENCHANTED => 37,
            Self::ITEM_GONE => 38,
            Self::ITEM_NOT_FOUND => 39,
            Self::ITEM_NOT_READY => 40,
            Self::LEVEL_REQUIREMENT => 41,
            Self::LINE_OF_SIGHT => 42,
            Self::LOWLEVEL => 43,
            Self::LOW_CASTLEVEL => 44,
            Self::MAINHAND_EMPTY => 45,
            Self::MOVING => 46,
            Self::NEED_AMMO => 47,
            Self::NEED_AMMO_POUCH => 48,
            Self::NEED_EXOTIC_AMMO => 49,
            Self::NOPATH => 50,
            Self::NOT_BEHIND => 51,
            Self::NOT_FISHABLE => 52,
            Self::NOT_HERE => 53,
            Self::NOT_INFRONT => 54,
            Self::NOT_IN_CONTROL => 55,
            Self::NOT_KNOWN => 56,
            Self::NOT_MOUNTED => 57,
            Self::NOT_ON_TAXI => 58,
            Self::NOT_ON_TRANSPORT => 59,
            Self::NOT_READY => 60,
            Self::NOT_SHAPESHIFT => 61,
            Self::NOT_STANDING => 62,
            Self::NOT_TRADEABLE => 63,
            Self::NOT_TRADING => 64,
            Self::NOT_UNSHEATHED => 65,
            Self::NOT_WHILE_GHOST => 66,
            Self::NO_AMMO => 67,
            Self::NO_CHARGES_REMAIN => 68,
            Self::NO_CHAMPION => 69,
            Self::NO_COMBO_POINTS => 70,
            Self::NO_DUELING => 71,
            Self::NO_ENDURANCE => 72,
            Self::NO_FISH => 73,
            Self::NO_ITEMS_WHILE_SHAPESHIFTED => 74,
            Self::NO_MOUNTS_ALLOWED => 75,
            Self::NO_PET => 76,
            Self::NO_POWER => 77,
            Self::NOTHING_TO_DISPEL => 78,
            Self::NOTHING_TO_STEAL => 79,
            Self::ONLY_ABOVEWATER => 80,
            Self::ONLY_DAYTIME => 81,
            Self::ONLY_INDOORS => 82,
            Self::ONLY_MOUNTED => 83,
            Self::ONLY_NIGHTTIME => 84,
            Self::ONLY_OUTDOORS => 85,
            Self::ONLY_SHAPESHIFT => 86,
            Self::ONLY_STEALTHED => 87,
            Self::ONLY_UNDERWATER => 88,
            Self::OUT_OF_RANGE => 89,
            Self::PACIFIED => 90,
            Self::POSSESSED => 91,
            Self::REAGENTS => 92,
            Self::REQUIRES_AREA { .. } => 93,
            Self::REQUIRES_SPELL_FOCUS { .. } => 94,
            Self::ROOTED => 95,
            Self::SILENCED => 96,
            Self::SPELL_IN_PROGRESS => 97,
            Self::SPELL_LEARNED => 98,
            Self::SPELL_UNAVAILABLE => 99,
            Self::STUNNED => 100,
            Self::TARGETS_DEAD => 101,
            Self::TARGET_AFFECTING_COMBAT => 102,
            Self::TARGET_AURASTATE => 103,
            Self::TARGET_DUELING => 104,
            Self::TARGET_ENEMY => 105,
            Self::TARGET_ENRAGED => 106,
            Self::TARGET_FRIENDLY => 107,
            Self::TARGET_IN_COMBAT => 108,
            Self::TARGET_IS_PLAYER => 109,
            Self::TARGET_NOT_DEAD => 110,
            Self::TARGET_NOT_IN_PARTY => 111,
            Self::TARGET_NOT_LOOTED => 112,
            Self::TARGET_NOT_PLAYER => 113,
            Self::TARGET_NO_POCKETS => 114,
            Self::TARGET_NO_WEAPONS => 115,
            Self::TARGET_UNSKINNABLE => 116,
            Self::THIRST_SATIATED => 117,
            Self::TOO_CLOSE => 118,
            Self::TOO_MANY_OF_ITEM => 119,
            Self::TOTEMS => 120,
            Self::TRAINING_POINTS => 121,
            Self::TRY_AGAIN => 122,
            Self::UNIT_NOT_BEHIND => 123,
            Self::UNIT_NOT_INFRONT => 124,
            Self::WRONG_PET_FOOD => 125,
            Self::NOT_WHILE_FATIGUED => 126,
            Self::TARGET_NOT_IN_INSTANCE => 127,
            Self::NOT_WHILE_TRADING => 128,
            Self::TARGET_NOT_IN_RAID => 129,
            Self::DISENCHANT_WHILE_LOOTING => 130,
            Self::PROSPECT_WHILE_LOOTING => 131,
            Self::PROSPECT_NEED_MORE => 132,
            Self::TARGET_FREEFORALL => 133,
            Self::NO_EDIBLE_CORPSES => 134,
            Self::ONLY_BATTLEGROUNDS => 135,
            Self::TARGET_NOT_GHOST => 136,
            Self::TOO_MANY_SKILLS => 137,
            Self::TRANSFORM_UNUSABLE => 138,
            Self::WRONG_WEATHER => 139,
            Self::DAMAGE_IMMUNE => 140,
            Self::PREVENTED_BY_MECHANIC => 141,
            Self::PLAY_TIME => 142,
            Self::REPUTATION => 143,
            Self::MIN_SKILL => 144,
            Self::UNKNOWN => 145,
        }
    }

}

impl SMSG_CAST_RESULT_CastFailureReason {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::AFFECTING_COMBAT => {
                1
            }
            Self::ALREADY_AT_FULL_HEALTH => {
                1
            }
            Self::ALREADY_AT_FULL_POWER => {
                1
            }
            Self::ALREADY_BEING_TAMED => {
                1
            }
            Self::ALREADY_HAVE_CHARM => {
                1
            }
            Self::ALREADY_HAVE_SUMMON => {
                1
            }
            Self::ALREADY_OPEN => {
                1
            }
            Self::AURA_BOUNCED => {
                1
            }
            Self::AUTOTRACK_INTERRUPTED => {
                1
            }
            Self::BAD_IMPLICIT_TARGETS => {
                1
            }
            Self::BAD_TARGETS => {
                1
            }
            Self::CANT_BE_CHARMED => {
                1
            }
            Self::CANT_BE_DISENCHANTED => {
                1
            }
            Self::CANT_BE_PROSPECTED => {
                1
            }
            Self::CANT_CAST_ON_TAPPED => {
                1
            }
            Self::CANT_DUEL_WHILE_INVISIBLE => {
                1
            }
            Self::CANT_DUEL_WHILE_STEALTHED => {
                1
            }
            Self::CANT_STEALTH => {
                1
            }
            Self::CASTER_AURASTATE => {
                1
            }
            Self::CASTER_DEAD => {
                1
            }
            Self::CHARMED => {
                1
            }
            Self::CHEST_IN_USE => {
                1
            }
            Self::CONFUSED => {
                1
            }
            Self::DONT_REPORT => {
                1
            }
            Self::EQUIPPED_ITEM => {
                1
            }
            Self::EQUIPPED_ITEM_CLASS {
                equipped_item_class,
                equipped_item_inventory_type_mask,
                equipped_item_subclass_mask,
            } => {
                1
                + 4 // equipped_item_class: u32
                + 4 // equipped_item_inventory_type_mask: u32
                + 4 // equipped_item_subclass_mask: u32
            }
            Self::EQUIPPED_ITEM_CLASS_MAINHAND => {
                1
            }
            Self::EQUIPPED_ITEM_CLASS_OFFHAND => {
                1
            }
            Self::ERROR => {
                1
            }
            Self::FIZZLE => {
                1
            }
            Self::FLEEING => {
                1
            }
            Self::FOOD_LOWLEVEL => {
                1
            }
            Self::HIGHLEVEL => {
                1
            }
            Self::HUNGER_SATIATED => {
                1
            }
            Self::IMMUNE => {
                1
            }
            Self::INTERRUPTED => {
                1
            }
            Self::INTERRUPTED_COMBAT => {
                1
            }
            Self::ITEM_ALREADY_ENCHANTED => {
                1
            }
            Self::ITEM_GONE => {
                1
            }
            Self::ITEM_NOT_FOUND => {
                1
            }
            Self::ITEM_NOT_READY => {
                1
            }
            Self::LEVEL_REQUIREMENT => {
                1
            }
            Self::LINE_OF_SIGHT => {
                1
            }
            Self::LOWLEVEL => {
                1
            }
            Self::LOW_CASTLEVEL => {
                1
            }
            Self::MAINHAND_EMPTY => {
                1
            }
            Self::MOVING => {
                1
            }
            Self::NEED_AMMO => {
                1
            }
            Self::NEED_AMMO_POUCH => {
                1
            }
            Self::NEED_EXOTIC_AMMO => {
                1
            }
            Self::NOPATH => {
                1
            }
            Self::NOT_BEHIND => {
                1
            }
            Self::NOT_FISHABLE => {
                1
            }
            Self::NOT_HERE => {
                1
            }
            Self::NOT_INFRONT => {
                1
            }
            Self::NOT_IN_CONTROL => {
                1
            }
            Self::NOT_KNOWN => {
                1
            }
            Self::NOT_MOUNTED => {
                1
            }
            Self::NOT_ON_TAXI => {
                1
            }
            Self::NOT_ON_TRANSPORT => {
                1
            }
            Self::NOT_READY => {
                1
            }
            Self::NOT_SHAPESHIFT => {
                1
            }
            Self::NOT_STANDING => {
                1
            }
            Self::NOT_TRADEABLE => {
                1
            }
            Self::NOT_TRADING => {
                1
            }
            Self::NOT_UNSHEATHED => {
                1
            }
            Self::NOT_WHILE_GHOST => {
                1
            }
            Self::NO_AMMO => {
                1
            }
            Self::NO_CHARGES_REMAIN => {
                1
            }
            Self::NO_CHAMPION => {
                1
            }
            Self::NO_COMBO_POINTS => {
                1
            }
            Self::NO_DUELING => {
                1
            }
            Self::NO_ENDURANCE => {
                1
            }
            Self::NO_FISH => {
                1
            }
            Self::NO_ITEMS_WHILE_SHAPESHIFTED => {
                1
            }
            Self::NO_MOUNTS_ALLOWED => {
                1
            }
            Self::NO_PET => {
                1
            }
            Self::NO_POWER => {
                1
            }
            Self::NOTHING_TO_DISPEL => {
                1
            }
            Self::NOTHING_TO_STEAL => {
                1
            }
            Self::ONLY_ABOVEWATER => {
                1
            }
            Self::ONLY_DAYTIME => {
                1
            }
            Self::ONLY_INDOORS => {
                1
            }
            Self::ONLY_MOUNTED => {
                1
            }
            Self::ONLY_NIGHTTIME => {
                1
            }
            Self::ONLY_OUTDOORS => {
                1
            }
            Self::ONLY_SHAPESHIFT => {
                1
            }
            Self::ONLY_STEALTHED => {
                1
            }
            Self::ONLY_UNDERWATER => {
                1
            }
            Self::OUT_OF_RANGE => {
                1
            }
            Self::PACIFIED => {
                1
            }
            Self::POSSESSED => {
                1
            }
            Self::REAGENTS => {
                1
            }
            Self::REQUIRES_AREA {
                area,
            } => {
                1
                + 4 // area: Area
            }
            Self::REQUIRES_SPELL_FOCUS {
                required_spell_focus,
            } => {
                1
                + 4 // required_spell_focus: u32
            }
            Self::ROOTED => {
                1
            }
            Self::SILENCED => {
                1
            }
            Self::SPELL_IN_PROGRESS => {
                1
            }
            Self::SPELL_LEARNED => {
                1
            }
            Self::SPELL_UNAVAILABLE => {
                1
            }
            Self::STUNNED => {
                1
            }
            Self::TARGETS_DEAD => {
                1
            }
            Self::TARGET_AFFECTING_COMBAT => {
                1
            }
            Self::TARGET_AURASTATE => {
                1
            }
            Self::TARGET_DUELING => {
                1
            }
            Self::TARGET_ENEMY => {
                1
            }
            Self::TARGET_ENRAGED => {
                1
            }
            Self::TARGET_FRIENDLY => {
                1
            }
            Self::TARGET_IN_COMBAT => {
                1
            }
            Self::TARGET_IS_PLAYER => {
                1
            }
            Self::TARGET_NOT_DEAD => {
                1
            }
            Self::TARGET_NOT_IN_PARTY => {
                1
            }
            Self::TARGET_NOT_LOOTED => {
                1
            }
            Self::TARGET_NOT_PLAYER => {
                1
            }
            Self::TARGET_NO_POCKETS => {
                1
            }
            Self::TARGET_NO_WEAPONS => {
                1
            }
            Self::TARGET_UNSKINNABLE => {
                1
            }
            Self::THIRST_SATIATED => {
                1
            }
            Self::TOO_CLOSE => {
                1
            }
            Self::TOO_MANY_OF_ITEM => {
                1
            }
            Self::TOTEMS => {
                1
            }
            Self::TRAINING_POINTS => {
                1
            }
            Self::TRY_AGAIN => {
                1
            }
            Self::UNIT_NOT_BEHIND => {
                1
            }
            Self::UNIT_NOT_INFRONT => {
                1
            }
            Self::WRONG_PET_FOOD => {
                1
            }
            Self::NOT_WHILE_FATIGUED => {
                1
            }
            Self::TARGET_NOT_IN_INSTANCE => {
                1
            }
            Self::NOT_WHILE_TRADING => {
                1
            }
            Self::TARGET_NOT_IN_RAID => {
                1
            }
            Self::DISENCHANT_WHILE_LOOTING => {
                1
            }
            Self::PROSPECT_WHILE_LOOTING => {
                1
            }
            Self::PROSPECT_NEED_MORE => {
                1
            }
            Self::TARGET_FREEFORALL => {
                1
            }
            Self::NO_EDIBLE_CORPSES => {
                1
            }
            Self::ONLY_BATTLEGROUNDS => {
                1
            }
            Self::TARGET_NOT_GHOST => {
                1
            }
            Self::TOO_MANY_SKILLS => {
                1
            }
            Self::TRANSFORM_UNUSABLE => {
                1
            }
            Self::WRONG_WEATHER => {
                1
            }
            Self::DAMAGE_IMMUNE => {
                1
            }
            Self::PREVENTED_BY_MECHANIC => {
                1
            }
            Self::PLAY_TIME => {
                1
            }
            Self::REPUTATION => {
                1
            }
            Self::MIN_SKILL => {
                1
            }
            Self::UNKNOWN => {
                1
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_CAST_RESULT_SimpleSpellCastResult {
    SUCCESS {
        reason: SMSG_CAST_RESULT_CastFailureReason,
    },
    FAILURE,
}

impl Default for SMSG_CAST_RESULT_SimpleSpellCastResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            reason: Default::default(),
        }
    }
}

impl SMSG_CAST_RESULT_SimpleSpellCastResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SUCCESS { .. } => 0,
            Self::FAILURE => 2,
        }
    }

}

impl SMSG_CAST_RESULT_SimpleSpellCastResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::SUCCESS {
                reason,
            } => {
                1
                + reason.size() // reason: SMSG_CAST_RESULT_CastFailureReason
            }
            Self::FAILURE => {
                1
            }
        }
    }
}

