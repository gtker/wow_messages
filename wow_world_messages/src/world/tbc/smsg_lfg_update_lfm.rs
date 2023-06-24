use std::io::{Read, Write};

use crate::tbc::{
    LfgData, LfgType, LfgUpdateLookingForMore,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_lfm.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_lfm.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_LFM = 0x036D {
///     LfgUpdateLookingForMore looking_for_more;
///     if (looking_for_more == LOOKING_FOR_MORE) {
///         LfgData data;
///     }
/// }
/// ```
pub struct SMSG_LFG_UPDATE_LFM {
    pub looking_for_more: SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore,
}

impl crate::private::Sealed for SMSG_LFG_UPDATE_LFM {}
impl crate::Message for SMSG_LFG_UPDATE_LFM {
    const OPCODE: u32 = 0x036d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_UPDATE_LFM {{").unwrap();
        // Members
        writeln!(s, "    looking_for_more = {};", LfgUpdateLookingForMore::try_from(self.looking_for_more.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.looking_for_more {
            crate::tbc::SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::LookingForMore {
                data,
            } => {
                // data: LfgData
                writeln!(s, "    data = {{").unwrap();
                // Members
                writeln!(s, "        entry = {};", data.entry).unwrap();
                writeln!(s, "        lfg_type = {};", data.lfg_type.as_test_case_value()).unwrap();

                writeln!(s, "    }};").unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 877_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "looking_for_more", "    ");
        match &self.looking_for_more {
            crate::tbc::SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::LookingForMore {
                data,
            } => {
                writeln!(s, "    /* data: LfgData start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 2, "entry", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 2, "lfg_type", "        ");
                writeln!(s, "    /* data: LfgData end */").unwrap();
            }
            _ => {}
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
        // looking_for_more: LfgUpdateLookingForMore
        w.write_all(&(self.looking_for_more.as_int().to_le_bytes()))?;

        match &self.looking_for_more {
            SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::LookingForMore {
                data,
            } => {
                // data: LfgData
                data.write_into_vec(&mut w)?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=5).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036D, size: body_size });
        }

        // looking_for_more: LfgUpdateLookingForMore
        let looking_for_more = crate::util::read_u8_le(&mut r)?.try_into()?;

        let looking_for_more_if = match looking_for_more {
            LfgUpdateLookingForMore::NotLookingForMore => SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::NotLookingForMore,
            LfgUpdateLookingForMore::LookingForMore => {
                // data: LfgData
                let data = LfgData::read(&mut r)?;

                SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::LookingForMore {
                    data,
                }
            }
        };

        Ok(Self {
            looking_for_more: looking_for_more_if,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LFG_UPDATE_LFM {}

impl SMSG_LFG_UPDATE_LFM {
    pub(crate) const fn size(&self) -> usize {
        self.looking_for_more.size() // looking_for_more: SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore {
    NotLookingForMore,
    LookingForMore {
        data: LfgData,
    },
}

impl Default for SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotLookingForMore
    }
}

impl SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotLookingForMore => 0,
            Self::LookingForMore { .. } => 1,
        }
    }

}

impl std::fmt::Display for SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotLookingForMore => f.write_str("NotLookingForMore"),
            Self::LookingForMore{ .. } => f.write_str("LookingForMore"),
        }
    }
}

impl SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::LookingForMore {
                ..
            } => {
                1
                + 4 // data: LfgData
            }
            _ => 1,
        }
    }
}

