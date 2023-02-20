use crate::wrath::InspectTalent;
use crate::wrath::TalentInfoSpec;
use crate::wrath::TalentInfoType;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_TALENTS_INFO {
    const OPCODE: u32 = 0x04c0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // talent_type: TalentInfoType
        w.write_all(&(self.talent_type.as_int() as u8).to_le_bytes())?;

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
                    i.write_into_vec(w)?;
                }

            }
            SMSG_TALENTS_INFO_TalentInfoType::Pet {
                talents,
            } => {
                // amount_of_talents: u8
                w.write_all(&(talents.len() as u8).to_le_bytes())?;

                // talents: InspectTalent[amount_of_talents]
                for i in talents.iter() {
                    i.write_into_vec(w)?;
                }

            }
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=459271).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04C0, size: body_size as u32 });
        }

        // talent_type: TalentInfoType
        let talent_type: TalentInfoType = crate::util::read_u8_le(r)?.try_into()?;

        // points_left: u32
        let points_left = crate::util::read_u32_le(r)?;

        let talent_type_if = match talent_type {
            TalentInfoType::Player => {
                // amount_of_specs: u8
                let amount_of_specs = crate::util::read_u8_le(r)?;

                // active_spec: u8
                let active_spec = crate::util::read_u8_le(r)?;

                // specs: TalentInfoSpec[amount_of_specs]
                let specs = {
                    let mut specs = Vec::with_capacity(amount_of_specs as usize);
                    for i in 0..amount_of_specs {
                        specs.push(TalentInfoSpec::read(r)?);
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
                let amount_of_talents = crate::util::read_u8_le(r)?;

                // talents: InspectTalent[amount_of_talents]
                let talents = {
                    let mut talents = Vec::with_capacity(amount_of_talents as usize);
                    for i in 0..amount_of_talents {
                        talents.push(InspectTalent::read(r)?);
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

impl SMSG_TALENTS_INFO_TalentInfoType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Player {
                active_spec,
                specs,
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

