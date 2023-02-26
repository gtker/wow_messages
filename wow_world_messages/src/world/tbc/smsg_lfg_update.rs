use crate::tbc::LfgData;
use crate::tbc::LfgUpdateLookingForMore;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update.wowm#L15):
/// ```text
/// smsg SMSG_LFG_UPDATE = 0x036C {
///     Bool queued;
///     Bool is_looking_for_group;
///     LfgUpdateLookingForMore looking_for_more;
///     if (looking_for_more == LOOKING_FOR_MORE) {
///         LfgData data;
///     }
/// }
/// ```
pub struct SMSG_LFG_UPDATE {
    pub queued: bool,
    pub is_looking_for_group: bool,
    pub looking_for_more: SMSG_LFG_UPDATE_LfgUpdateLookingForMore,
}

impl crate::Message for SMSG_LFG_UPDATE {
    const OPCODE: u32 = 0x036c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // queued: Bool
        w.write_all(u8::from(self.queued).to_le_bytes().as_slice())?;

        // is_looking_for_group: Bool
        w.write_all(u8::from(self.is_looking_for_group).to_le_bytes().as_slice())?;

        // looking_for_more: LfgUpdateLookingForMore
        w.write_all(&(self.looking_for_more.as_int() as u8).to_le_bytes())?;

        match &self.looking_for_more {
            SMSG_LFG_UPDATE_LfgUpdateLookingForMore::NotLookingForMore => {}
            SMSG_LFG_UPDATE_LfgUpdateLookingForMore::LookingForMore {
                data,
            } => {
                // data: LfgData
                data.write_into_vec(w)?;

            }
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(3..=7).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036C, size: body_size as u32 });
        }

        // queued: Bool
        let queued = crate::util::read_u8_le(r)? != 0;
        // is_looking_for_group: Bool
        let is_looking_for_group = crate::util::read_u8_le(r)? != 0;
        // looking_for_more: LfgUpdateLookingForMore
        let looking_for_more: LfgUpdateLookingForMore = crate::util::read_u8_le(r)?.try_into()?;

        let looking_for_more_if = match looking_for_more {
            LfgUpdateLookingForMore::NotLookingForMore => SMSG_LFG_UPDATE_LfgUpdateLookingForMore::NotLookingForMore,
            LfgUpdateLookingForMore::LookingForMore => {
                // data: LfgData
                let data = LfgData::read(r)?;

                SMSG_LFG_UPDATE_LfgUpdateLookingForMore::LookingForMore {
                    data,
                }
            }
        };

        Ok(Self {
            queued,
            is_looking_for_group,
            looking_for_more: looking_for_more_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LFG_UPDATE {}

impl SMSG_LFG_UPDATE {
    pub(crate) fn size(&self) -> usize {
        1 // queued: Bool
        + 1 // is_looking_for_group: Bool
        + self.looking_for_more.size() // looking_for_more: SMSG_LFG_UPDATE_LfgUpdateLookingForMore
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_LFG_UPDATE_LfgUpdateLookingForMore {
    NotLookingForMore,
    LookingForMore {
        data: LfgData,
    },
}

impl Default for SMSG_LFG_UPDATE_LfgUpdateLookingForMore {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotLookingForMore
    }
}

impl SMSG_LFG_UPDATE_LfgUpdateLookingForMore {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotLookingForMore => 0,
            Self::LookingForMore { .. } => 1,
        }
    }

}

impl SMSG_LFG_UPDATE_LfgUpdateLookingForMore {
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

