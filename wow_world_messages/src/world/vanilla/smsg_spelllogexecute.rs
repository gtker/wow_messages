use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Power, SpellEffect, SpellLog,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L1):
/// ```text
/// smsg SMSG_SPELLLOGEXECUTE = 0x024C {
///     PackedGuid caster;
///     Spell spell;
///     u32 amount_of_effects;
///     SpellLog[amount_of_effects] logs;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_SPELLLOGEXECUTE {
    pub caster: Guid,
    pub spell: u32,
    pub logs: Vec<SpellLog>,
}

impl crate::private::Sealed for SMSG_SPELLLOGEXECUTE {}
impl SMSG_SPELLLOGEXECUTE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        // amount_of_effects: u32
        let amount_of_effects = crate::util::read_u32_le(&mut r)?;

        // logs: SpellLog[amount_of_effects]
        let logs = {
            let mut logs = Vec::with_capacity(amount_of_effects as usize);

            let allocation_size = u64::from(amount_of_effects) * 8;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

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

impl crate::Message for SMSG_SPELLLOGEXECUTE {
    const OPCODE: u32 = 0x024c;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELLLOGEXECUTE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLLOGEXECUTE {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    amount_of_effects = {};", self.logs.len()).unwrap();
        writeln!(s, "    logs = [").unwrap();
        for v in self.logs.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            effect = {};", SpellEffect::try_from(v.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v {
                crate::vanilla::SpellLog::Instakill {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::PowerDrain {
                    amount,
                    multiplier,
                    power,
                    target1,
                } => {
                    writeln!(s, "            target1 = {};", target1.guid()).unwrap();
                    writeln!(s, "            amount = {};", amount).unwrap();
                    writeln!(s, "            power = {};", power.as_test_case_value()).unwrap();
                    writeln!(s, "            multiplier = {};", if multiplier.to_string().contains('.') { multiplier.to_string() } else { format!("{}.0", multiplier) }).unwrap();
                }
                crate::vanilla::SpellLog::Heal {
                    heal_amount,
                    heal_critical,
                    target2,
                } => {
                    writeln!(s, "            target2 = {};", target2.guid()).unwrap();
                    writeln!(s, "            heal_amount = {};", heal_amount).unwrap();
                    writeln!(s, "            heal_critical = {};", heal_critical).unwrap();
                }
                crate::vanilla::SpellLog::Resurrect {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::AddExtraAttacks {
                    extra_attacks,
                    target4,
                } => {
                    writeln!(s, "            target4 = {};", target4.guid()).unwrap();
                    writeln!(s, "            extra_attacks = {};", extra_attacks).unwrap();
                }
                crate::vanilla::SpellLog::CreateItem {
                    item,
                } => {
                    writeln!(s, "            item = {};", item).unwrap();
                }
                crate::vanilla::SpellLog::Summon {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::Energize {
                    energize_amount,
                    energize_power,
                    target3,
                } => {
                    writeln!(s, "            target3 = {};", target3.guid()).unwrap();
                    writeln!(s, "            energize_amount = {};", energize_amount).unwrap();
                    writeln!(s, "            energize_power = {};", energize_power).unwrap();
                }
                crate::vanilla::SpellLog::OpenLock {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::Dispel {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonWild {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonGuardian {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::TransDoor {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonPet {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::OpenLockItem {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::Threat {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::HealMaxHealth {
                    heal_amount,
                    heal_critical,
                    target2,
                } => {
                    writeln!(s, "            target2 = {};", target2.guid()).unwrap();
                    writeln!(s, "            heal_amount = {};", heal_amount).unwrap();
                    writeln!(s, "            heal_critical = {};", heal_critical).unwrap();
                }
                crate::vanilla::SpellLog::InterruptCast {
                    interrupted_spell,
                    target5,
                } => {
                    writeln!(s, "            target5 = {};", target5.guid()).unwrap();
                    writeln!(s, "            interrupted_spell = {};", interrupted_spell).unwrap();
                }
                crate::vanilla::SpellLog::Distract {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonPossessed {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonTotem {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonObjectWild {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::Sanctuary {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonTotemSlot1 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonTotemSlot2 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonTotemSlot3 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonTotemSlot4 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::ThreatAll {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonCritter {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::FeedPet {
                    feed_pet_item,
                } => {
                    writeln!(s, "            feed_pet_item = {};", feed_pet_item).unwrap();
                }
                crate::vanilla::SpellLog::DismissPet {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonObjectSlot1 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonObjectSlot2 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonObjectSlot3 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SummonObjectSlot4 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::DispelMechanic {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::DurabilityDamage {
                    item_to_damage,
                    target6,
                    unknown5,
                } => {
                    writeln!(s, "            target6 = {};", target6.guid()).unwrap();
                    writeln!(s, "            item_to_damage = {};", item_to_damage).unwrap();
                    writeln!(s, "            unknown5 = {};", unknown5).unwrap();
                }
                crate::vanilla::SpellLog::SummonDemon {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::ResurrectNew {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::AttackMe {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::SkinPlayerCorpse {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::ModifyThreatPercent {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                crate::vanilla::SpellLog::Unknown126 {
                    target7,
                } => {
                    writeln!(s, "            target7 = {};", target7.guid()).unwrap();
                }
                _ => {}
            }


            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 588_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_effects", "    ");
        if !self.logs.is_empty() {
            writeln!(s, "    /* logs: SpellLog[amount_of_effects] start */").unwrap();
            for (i, v) in self.logs.iter().enumerate() {
                writeln!(s, "    /* logs: SpellLog[amount_of_effects] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "effect", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_logs", "        ");
                match &v {
                    crate::vanilla::SpellLog::Instakill {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::PowerDrain {
                        amount,
                        multiplier,
                        power,
                        target1,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target1", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "power", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "multiplier", "        ");
                    }
                    crate::vanilla::SpellLog::Heal {
                        heal_amount,
                        heal_critical,
                        target2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target2", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "heal_amount", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "heal_critical", "        ");
                    }
                    crate::vanilla::SpellLog::Resurrect {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::AddExtraAttacks {
                        extra_attacks,
                        target4,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target4", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "extra_attacks", "        ");
                    }
                    crate::vanilla::SpellLog::CreateItem {
                        item,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                    }
                    crate::vanilla::SpellLog::Summon {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::Energize {
                        energize_amount,
                        energize_power,
                        target3,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target3", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "energize_amount", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "energize_power", "        ");
                    }
                    crate::vanilla::SpellLog::OpenLock {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::Dispel {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonWild {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonGuardian {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::TransDoor {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonPet {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::OpenLockItem {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::Threat {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::HealMaxHealth {
                        heal_amount,
                        heal_critical,
                        target2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target2", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "heal_amount", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "heal_critical", "        ");
                    }
                    crate::vanilla::SpellLog::InterruptCast {
                        interrupted_spell,
                        target5,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "interrupted_spell", "        ");
                    }
                    crate::vanilla::SpellLog::Distract {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonPossessed {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonTotem {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonObjectWild {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::Sanctuary {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonTotemSlot1 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonTotemSlot2 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonTotemSlot3 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonTotemSlot4 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::ThreatAll {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonCritter {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::FeedPet {
                        feed_pet_item,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "feed_pet_item", "        ");
                    }
                    crate::vanilla::SpellLog::DismissPet {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonObjectSlot1 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonObjectSlot2 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonObjectSlot3 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SummonObjectSlot4 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::DispelMechanic {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::DurabilityDamage {
                        item_to_damage,
                        target6,
                        unknown5,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target6", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_to_damage", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown5", "        ");
                    }
                    crate::vanilla::SpellLog::SummonDemon {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::ResurrectNew {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::AttackMe {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::SkinPlayerCorpse {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::ModifyThreatPercent {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    crate::vanilla::SpellLog::Unknown126 {
                        target7,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "target7", "        ");
                    }
                    _ => {}
                }

                writeln!(s, "    /* logs: SpellLog[amount_of_effects] {i} end */").unwrap();
            }
            writeln!(s, "    /* logs: SpellLog[amount_of_effects] end */").unwrap();
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
        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_effects: u32
        w.write_all(&(self.logs.len() as u32).to_le_bytes())?;

        // logs: SpellLog[amount_of_effects]
        for i in self.logs.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(588, "SMSG_SPELLLOGEXECUTE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLLOGEXECUTE {}

impl SMSG_SPELLLOGEXECUTE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // spell: Spell
        + 4 // amount_of_effects: u32
        + self.logs.iter().fold(0, |acc, x| acc + x.size()) // logs: SpellLog[amount_of_effects]
    }
}

