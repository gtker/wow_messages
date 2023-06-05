use std::io::{Read, Write};

use crate::wrath::{
    InspectTalent, TalentInfoSpec, TalentInfoType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_talents_info.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_talents_info.wowm#L17):
/// ```text
/// smsg SMSG_TALENTS_INFO = 0x04C0 {
///     TalentInfoType talent_type;
///     u32 points_left;
///     if (talent_type == PET) {
///         u8 amount_of_talents;
///         InspectTalent[amount_of_talents] talents;
///     }
///     else if (talent_type == PLAYER) {
///         u8 amount_of_specs;
///         u8 active_spec;
///         TalentInfoSpec[amount_of_specs] specs;
///     }
/// }
/// ```
pub struct SMSG_TALENTS_INFO {
    pub talent_type: SMSG_TALENTS_INFO_TalentInfoType,
    pub points_left: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_TALENTS_INFO {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TALENTS_INFO {{").unwrap();
        // Members
        writeln!(s, "    talent_type = {};", crate::wrath::TalentInfoType::try_from(self.talent_type.as_int()).unwrap().as_test_case_value()).unwrap();
        writeln!(s, "    points_left = {};", self.points_left).unwrap();
        match &self.talent_type {
            crate::wrath::SMSG_TALENTS_INFO_TalentInfoType::Player {
                active_spec,
                specs,
            } => {
                writeln!(s, "    amount_of_specs = {};", specs.len()).unwrap();
                writeln!(s, "    active_spec = {};", active_spec).unwrap();
                write!(s, "    specs = [").unwrap();
                for v in specs.as_slice() {
                    writeln!(s, "{{").unwrap();
                    // Members
                    writeln!(s, "        amount_of_talents = {};", v.talents.len()).unwrap();
                    write!(s, "        talents = [").unwrap();
                    for v in v.talents.as_slice() {
                        writeln!(s, "{{").unwrap();
                        // Members
                        writeln!(s, "            talent = {};", v.talent.as_test_case_value()).unwrap();
                        writeln!(s, "            max_rank = {};", v.max_rank).unwrap();

                        writeln!(s, "    }},").unwrap();
                    }
                    writeln!(s, "];").unwrap();
                    writeln!(s, "        amount_of_glyphs = {};", v.glyphs.len()).unwrap();
                    write!(s, "        glyphs = [").unwrap();
                    for v in v.glyphs.as_slice() {
                        write!(s, "{v:#04X}, ").unwrap();
                    }
                    writeln!(s, "];").unwrap();

                    writeln!(s, "    }},").unwrap();
                }
                writeln!(s, "];").unwrap();
            }
            crate::wrath::SMSG_TALENTS_INFO_TalentInfoType::Pet {
                talents,
            } => {
                writeln!(s, "    amount_of_talents = {};", talents.len()).unwrap();
                write!(s, "    talents = [").unwrap();
                for v in talents.as_slice() {
                    writeln!(s, "{{").unwrap();
                    // Members
                    writeln!(s, "        talent = {};", v.talent.as_test_case_value()).unwrap();
                    writeln!(s, "        max_rank = {};", v.max_rank).unwrap();

                    writeln!(s, "    }},").unwrap();
                }
                writeln!(s, "];").unwrap();
            }
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1216_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "talent_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "points_left", "    ");
        match &self.talent_type {
            crate::wrath::SMSG_TALENTS_INFO_TalentInfoType::Player {
                active_spec,
                specs,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_specs", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "active_spec", "    ");
                if !specs.is_empty() {
                    writeln!(s, "    /* specs: TalentInfoSpec[amount_of_specs] start */").unwrap();
                    for (i, v) in specs.iter().enumerate() {
                        writeln!(s, "    /* specs: TalentInfoSpec[amount_of_specs] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_talents", "        ");
                        if !v.talents.is_empty() {
                            writeln!(s, "    /* talents: InspectTalent[amount_of_talents] start */").unwrap();
                            for (i, v) in v.talents.iter().enumerate() {
                                writeln!(s, "    /* talents: InspectTalent[amount_of_talents] {i} start */").unwrap();
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "talent", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 1, "max_rank", "            ");
                                writeln!(s, "    /* talents: InspectTalent[amount_of_talents] {i} end */").unwrap();
                            }
                            writeln!(s, "    /* talents: InspectTalent[amount_of_talents] end */").unwrap();
                        }
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_glyphs", "        ");
                        if !v.glyphs.is_empty() {
                            writeln!(s, "    /* glyphs: u16[amount_of_glyphs] start */").unwrap();
                            for (i, v) in v.glyphs.iter().enumerate() {
                                crate::util::write_bytes(&mut s, &mut bytes, 2, &format!("glyphs {i}"), "        ");
                            }
                            writeln!(s, "    /* glyphs: u16[amount_of_glyphs] end */").unwrap();
                        }
                        writeln!(s, "    /* specs: TalentInfoSpec[amount_of_specs] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* specs: TalentInfoSpec[amount_of_specs] end */").unwrap();
                }
            }
            crate::wrath::SMSG_TALENTS_INFO_TalentInfoType::Pet {
                talents,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_talents", "    ");
                if !talents.is_empty() {
                    writeln!(s, "    /* talents: InspectTalent[amount_of_talents] start */").unwrap();
                    for (i, v) in talents.iter().enumerate() {
                        writeln!(s, "    /* talents: InspectTalent[amount_of_talents] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "talent", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "max_rank", "        ");
                        writeln!(s, "    /* talents: InspectTalent[amount_of_talents] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* talents: InspectTalent[amount_of_talents] end */").unwrap();
                }
            }
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_TALENTS_INFO {}
impl crate::Message for SMSG_TALENTS_INFO {
    const OPCODE: u32 = 0x04c0;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_TALENTS_INFO::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // talent_type: TalentInfoType
        w.write_all(&(self.talent_type.as_int().to_le_bytes()))?;

        // points_left: u32
        w.write_all(&self.points_left.to_le_bytes())?;

        match &self.talent_type {
            SMSG_TALENTS_INFO_TalentInfoType::Player {
                active_spec,
                specs,
            } => {
                // amount_of_specs: u8
                w.write_all(&(specs.len() as u8).to_le_bytes())?;

                // active_spec: u8
                w.write_all(&active_spec.to_le_bytes())?;

                // specs: TalentInfoSpec[amount_of_specs]
                for i in specs.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
            SMSG_TALENTS_INFO_TalentInfoType::Pet {
                talents,
            } => {
                // amount_of_talents: u8
                w.write_all(&(talents.len() as u8).to_le_bytes())?;

                // talents: InspectTalent[amount_of_talents]
                for i in talents.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=459271).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04C0, size: body_size });
        }

        // talent_type: TalentInfoType
        let talent_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // points_left: u32
        let points_left = crate::util::read_u32_le(&mut r)?;

        let talent_type_if = match talent_type {
            TalentInfoType::Player => {
                // amount_of_specs: u8
                let amount_of_specs = crate::util::read_u8_le(&mut r)?;

                // active_spec: u8
                let active_spec = crate::util::read_u8_le(&mut r)?;

                // specs: TalentInfoSpec[amount_of_specs]
                let specs = {
                    let mut specs = Vec::with_capacity(amount_of_specs as usize);
                    for _ in 0..amount_of_specs {
                        specs.push(TalentInfoSpec::read(&mut r)?);
                    }
                    specs
                };

                SMSG_TALENTS_INFO_TalentInfoType::Player {
                    active_spec,
                    specs,
                }
            }
            TalentInfoType::Pet => {
                // amount_of_talents: u8
                let amount_of_talents = crate::util::read_u8_le(&mut r)?;

                // talents: InspectTalent[amount_of_talents]
                let talents = {
                    let mut talents = Vec::with_capacity(amount_of_talents as usize);
                    for _ in 0..amount_of_talents {
                        talents.push(InspectTalent::read(&mut r)?);
                    }
                    talents
                };

                SMSG_TALENTS_INFO_TalentInfoType::Pet {
                    talents,
                }
            }
        };

        Ok(Self {
            talent_type: talent_type_if,
            points_left,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TALENTS_INFO {}

impl SMSG_TALENTS_INFO {
    pub(crate) fn size(&self) -> usize {
        self.talent_type.size() // talent_type: SMSG_TALENTS_INFO_TalentInfoType
        + 4 // points_left: u32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_TALENTS_INFO_TalentInfoType {
    Player {
        active_spec: u8,
        specs: Vec<TalentInfoSpec>,
    },
    Pet {
        talents: Vec<InspectTalent>,
    },
}

impl Default for SMSG_TALENTS_INFO_TalentInfoType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Player {
            active_spec: Default::default(),
            specs: Default::default(),
        }
    }
}

impl SMSG_TALENTS_INFO_TalentInfoType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Player { .. } => 0,
            Self::Pet { .. } => 1,
        }
    }

}

impl std::fmt::Display for SMSG_TALENTS_INFO_TalentInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player{ .. } => f.write_str("Player"),
            Self::Pet{ .. } => f.write_str("Pet"),
        }
    }
}

impl SMSG_TALENTS_INFO_TalentInfoType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Player {
                specs,
                ..
            } => {
                1
                + 1 // active_spec: u8
                + 1 // amount_of_specs: u8
                + specs.iter().fold(0, |acc, x| acc + x.size()) // specs: TalentInfoSpec[amount_of_specs]
            }
            Self::Pet {
                talents,
            } => {
                1
                + 1 // amount_of_talents: u8
                + talents.len() * 5 // talents: InspectTalent[amount_of_talents]
            }
        }
    }
}

