use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    AuraLog, AuraType, SpellSchool,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L1):
/// ```text
/// smsg SMSG_PERIODICAURALOG = 0x024E {
///     PackedGuid target;
///     PackedGuid caster;
///     u32 spell;
///     u32 amount_of_auras;
///     AuraLog[amount_of_auras] auras;
/// }
/// ```
pub struct SMSG_PERIODICAURALOG {
    pub target: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub auras: Vec<AuraLog>,
}

impl crate::private::Sealed for SMSG_PERIODICAURALOG {}
impl SMSG_PERIODICAURALOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024E, size: body_size });
        }

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // amount_of_auras: u32
        let amount_of_auras = crate::util::read_u32_le(&mut r)?;

        // auras: AuraLog[amount_of_auras]
        let auras = {
            let mut auras = Vec::with_capacity(amount_of_auras as usize);
            for _ in 0..amount_of_auras {
                auras.push(AuraLog::read(&mut r)?);
            }
            auras
        };

        Ok(Self {
            target,
            caster,
            spell,
            auras,
        })
    }

}

impl crate::Message for SMSG_PERIODICAURALOG {
    const OPCODE: u32 = 0x024e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PERIODICAURALOG {{").unwrap();
        // Members
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    amount_of_auras = {};", self.auras.len()).unwrap();
        write!(s, "    auras = [").unwrap();
        for v in self.auras.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        aura_type = {};", AuraType::try_from(v.aura_type.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.aura_type {
                crate::tbc::AuraLog_AuraType::PeriodicDamage {
                    absorbed,
                    damage1,
                    resisted,
                    school,
                } => {
                    writeln!(s, "        damage1 = {};", damage1).unwrap();
                    writeln!(s, "        school = {};", school.as_test_case_value()).unwrap();
                    writeln!(s, "        absorbed = {};", absorbed).unwrap();
                    writeln!(s, "        resisted = {};", resisted).unwrap();
                }
                crate::tbc::AuraLog_AuraType::PeriodicHeal {
                    damage2,
                } => {
                    writeln!(s, "        damage2 = {};", damage2).unwrap();
                }
                crate::tbc::AuraLog_AuraType::ObsModHealth {
                    damage2,
                } => {
                    writeln!(s, "        damage2 = {};", damage2).unwrap();
                }
                crate::tbc::AuraLog_AuraType::ObsModMana {
                    damage3,
                    misc_value1,
                } => {
                    writeln!(s, "        misc_value1 = {};", misc_value1).unwrap();
                    writeln!(s, "        damage3 = {};", damage3).unwrap();
                }
                crate::tbc::AuraLog_AuraType::PeriodicEnergize {
                    damage3,
                    misc_value1,
                } => {
                    writeln!(s, "        misc_value1 = {};", misc_value1).unwrap();
                    writeln!(s, "        damage3 = {};", damage3).unwrap();
                }
                crate::tbc::AuraLog_AuraType::PeriodicManaLeech {
                    damage,
                    gain_multiplier,
                    misc_value2,
                } => {
                    writeln!(s, "        misc_value2 = {};", misc_value2).unwrap();
                    writeln!(s, "        damage = {};", damage).unwrap();
                    writeln!(s, "    {}", if gain_multiplier.to_string().contains('.') { gain_multiplier.to_string() } else { format!("{}.0", gain_multiplier) }).unwrap();
                }
                crate::tbc::AuraLog_AuraType::PeriodicDamagePercent {
                    absorbed,
                    damage1,
                    resisted,
                    school,
                } => {
                    writeln!(s, "        damage1 = {};", damage1).unwrap();
                    writeln!(s, "        school = {};", school.as_test_case_value()).unwrap();
                    writeln!(s, "        absorbed = {};", absorbed).unwrap();
                    writeln!(s, "        resisted = {};", resisted).unwrap();
                }
                _ => {}
            }


            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 590_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.target), "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_auras", "    ");
        if !self.auras.is_empty() {
            writeln!(s, "    /* auras: AuraLog[amount_of_auras] start */").unwrap();
            for (i, v) in self.auras.iter().enumerate() {
                writeln!(s, "    /* auras: AuraLog[amount_of_auras] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "aura_type", "        ");
                match &v.aura_type {
                    crate::tbc::AuraLog_AuraType::PeriodicDamage {
                        absorbed,
                        damage1,
                        resisted,
                        school,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage1", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "school", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "absorbed", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "resisted", "        ");
                    }
                    crate::tbc::AuraLog_AuraType::PeriodicHeal {
                        damage2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage2", "        ");
                    }
                    crate::tbc::AuraLog_AuraType::ObsModHealth {
                        damage2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage2", "        ");
                    }
                    crate::tbc::AuraLog_AuraType::ObsModMana {
                        damage3,
                        misc_value1,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "misc_value1", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage3", "        ");
                    }
                    crate::tbc::AuraLog_AuraType::PeriodicEnergize {
                        damage3,
                        misc_value1,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "misc_value1", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage3", "        ");
                    }
                    crate::tbc::AuraLog_AuraType::PeriodicManaLeech {
                        damage,
                        gain_multiplier,
                        misc_value2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "misc_value2", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "gain_multiplier", "        ");
                    }
                    crate::tbc::AuraLog_AuraType::PeriodicDamagePercent {
                        absorbed,
                        damage1,
                        resisted,
                        school,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage1", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "school", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "absorbed", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "resisted", "        ");
                    }
                    _ => {}
                }

                writeln!(s, "    /* auras: AuraLog[amount_of_auras] {i} end */").unwrap();
            }
            writeln!(s, "    /* auras: AuraLog[amount_of_auras] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_auras: u32
        w.write_all(&(self.auras.len() as u32).to_le_bytes())?;

        // auras: AuraLog[amount_of_auras]
        for i in self.auras.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PERIODICAURALOG {}

impl SMSG_PERIODICAURALOG {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + self.auras.iter().fold(0, |acc, x| acc + x.size()) // auras: AuraLog[amount_of_auras]
    }
}

