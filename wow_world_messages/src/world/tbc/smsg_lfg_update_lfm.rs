use std::io::{Read, Write};

use crate::tbc::{
    LfgData,
    LfgUpdateLookingForMore,
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

impl crate::Message for SMSG_LFG_UPDATE_LFM {
    const OPCODE: u32 = 0x036d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // looking_for_more: LfgUpdateLookingForMore
        w.write_all(&u8::from(self.looking_for_more.as_int()).to_le_bytes())?;

        match &self.looking_for_more {
            SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::NotLookingForMore => {}
            SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::LookingForMore {
                data,
            } => {
                // data: LfgData
                data.write_into_vec(&mut w)?;

            }
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=5).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036D, size: body_size as u32 });
        }

        // looking_for_more: LfgUpdateLookingForMore
        let looking_for_more: LfgUpdateLookingForMore = crate::util::read_u8_le(&mut r)?.try_into()?;

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
    pub(crate) fn size(&self) -> usize {
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

impl SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotLookingForMore => {
                1
            }
            Self::LookingForMore {
                data,
            } => {
                1
                + 4 // data: LfgData
            }
        }
    }
}

