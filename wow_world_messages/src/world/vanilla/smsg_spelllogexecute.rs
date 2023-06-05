use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::SpellLog;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L1):
/// ```text
/// smsg SMSG_SPELLLOGEXECUTE = 0x024C {
///     PackedGuid caster;
///     u32 spell;
///     u32 amount_of_effects;
///     SpellLog[amount_of_effects] logs;
/// }
/// ```
pub struct SMSG_SPELLLOGEXECUTE {
    pub caster: Guid,
    pub spell: u32,
    pub logs: Vec<SpellLog>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SPELLLOGEXECUTE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLLOGEXECUTE {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    amount_of_effects = {};", self.logs.len()).unwrap();
        write!(s, "    logs = [").unwrap();
        for v in self.logs.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    effect = {};", crate::vanilla::SpellEffect::try_from(v.effect.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.effect {
                crate::vanilla::SpellLog_SpellEffect::Instakill {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::PowerDrain {
                    amount,
                    multiplier,
                    power,
                    target1,
                } => {
                    writeln!(s, "    target1 = {};", target1.guid()).unwrap();
                    writeln!(s, "    amount = {};", amount).unwrap();
                    writeln!(s, "    power = {};", power.as_test_case_value()).unwrap();
                    writeln!(s, "    {}", if multiplier.to_string().contains(".") { multiplier.to_string() } else { format!("{}.0", multiplier) }).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Heal {
                    heal_amount,
                    heal_critical,
                    target2,
                } => {
                    writeln!(s, "    target2 = {};", target2.guid()).unwrap();
                    writeln!(s, "    heal_amount = {};", heal_amount).unwrap();
                    writeln!(s, "    heal_critical = {};", heal_critical).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Resurrect {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::AddExtraAttacks {
                    extra_attacks,
                    target4,
                } => {
                    writeln!(s, "    target4 = {};", target4.guid()).unwrap();
                    writeln!(s, "    extra_attacks = {};", extra_attacks).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::CreateItem {
                    item,
                } => {
                    writeln!(s, "    item = {};", item).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Summon {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Energize {
                    energize_amount,
                    energize_power,
                    target3,
                } => {
                    writeln!(s, "    target3 = {};", target3.guid()).unwrap();
                    writeln!(s, "    energize_amount = {};", energize_amount).unwrap();
                    writeln!(s, "    energize_power = {};", energize_power).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::OpenLock {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Dispel {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonWild {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonGuardian {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::TransDoor {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonPet {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::OpenLockItem {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Threat {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::HealMaxHealth {
                    heal_amount,
                    heal_critical,
                    target2,
                } => {
                    writeln!(s, "    target2 = {};", target2.guid()).unwrap();
                    writeln!(s, "    heal_amount = {};", heal_amount).unwrap();
                    writeln!(s, "    heal_critical = {};", heal_critical).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::InterruptCast {
                    interrupted_spell,
                    target5,
                } => {
                    writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                    writeln!(s, "    interrupted_spell = {};", interrupted_spell).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Distract {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonPossessed {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonTotem {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonObjectWild {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Sanctuary {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonTotemSlot1 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonTotemSlot2 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonTotemSlot3 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonTotemSlot4 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::ThreatAll {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonCritter {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::FeedPet {
                    feed_pet_item,
                } => {
                    writeln!(s, "    feed_pet_item = {};", feed_pet_item).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::DismissPet {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonObjectSlot1 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonObjectSlot2 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonObjectSlot3 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonObjectSlot4 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::DispelMechanic {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::DurabilityDamage {
                    item_to_damage,
                    target6,
                    unknown5,
                } => {
                    writeln!(s, "    target6 = {};", target6.guid()).unwrap();
                    writeln!(s, "    item_to_damage = {};", item_to_damage).unwrap();
                    writeln!(s, "    unknown5 = {};", unknown5).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SummonDemon {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::ResurrectNew {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::AttackMe {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::SkinPlayerCorpse {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::ModifyThreatPercent {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog_SpellEffect::Unknown126 {
                    target7,
                } => {
                    writeln!(s, "    target7 = {};", target7.guid()).unwrap();
                }
                _ => {}
            }


            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 588_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_SPELLLOGEXECUTE {}
impl crate::Message for SMSG_SPELLLOGEXECUTE {
    const OPCODE: u32 = 0x024c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_effects: u32
        w.write_all(&(self.logs.len() as u32).to_le_bytes())?;

        // logs: SpellLog[amount_of_effects]
        for i in self.logs.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(10..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024C, size: body_size });
        }

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // amount_of_effects: u32
        let amount_of_effects = crate::util::read_u32_le(&mut r)?;

        // logs: SpellLog[amount_of_effects]
        let logs = {
            let mut logs = Vec::with_capacity(amount_of_effects as usize);
            for _ in 0..amount_of_effects {
                logs.push(SpellLog::read(&mut r)?);
            }
            logs
        };

        Ok(Self {
            caster,
            spell,
            logs,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLLOGEXECUTE {}

impl SMSG_SPELLLOGEXECUTE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_effects: u32
        + self.logs.iter().fold(0, |acc, x| acc + x.size()) // logs: SpellLog[amount_of_effects]
    }
}

