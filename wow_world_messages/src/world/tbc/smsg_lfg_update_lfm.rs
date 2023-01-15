use std::convert::{TryFrom, TryInto};
use crate::world::tbc::LfgType;
use crate::world::tbc::LfgUpdateLookingForMore;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update_lfm.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update_lfm.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_LFM = 0x036D {
///     LfgUpdateLookingForMore looking_for_more;
///     if (looking_for_more == LOOKING_FOR_MORE) {
///         u16 entry;
///         (u16)LfgType lfg_type;
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // looking_for_more: LfgUpdateLookingForMore
        w.write_all(&(self.looking_for_more.as_int() as u8).to_le_bytes())?;

        match &self.looking_for_more {
            SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::NotLookingForMore => {}
            SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::LookingForMore {
                entry,
                lfg_type,
            } => {
                // entry: u16
                w.write_all(&entry.to_le_bytes())?;

                // lfg_type: LfgType
                w.write_all(&(lfg_type.as_int() as u16).to_le_bytes())?;

            }
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=5).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036D, size: body_size as u32 });
        }

        // looking_for_more: LfgUpdateLookingForMore
        let looking_for_more: LfgUpdateLookingForMore = crate::util::read_u8_le(r)?.try_into()?;

        let looking_for_more_if = match looking_for_more {
            LfgUpdateLookingForMore::NotLookingForMore => SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::NotLookingForMore,
            LfgUpdateLookingForMore::LookingForMore => {
                // entry: u16
                let entry = crate::util::read_u16_le(r)?;

                // lfg_type: LfgType
                let lfg_type: LfgType = (crate::util::read_u16_le(r)? as u8).try_into()?;

                SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore::LookingForMore {
                    entry,
                    lfg_type,
                }
            }
        };

        Ok(Self {
            looking_for_more: looking_for_more_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_LFG_UPDATE_LFM {}

impl SMSG_LFG_UPDATE_LFM {
    pub(crate) fn size(&self) -> usize {
        self.looking_for_more.size() // looking_for_more: SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SMSG_LFG_UPDATE_LFM_LfgUpdateLookingForMore {
    NotLookingForMore,
    LookingForMore {
        entry: u16,
        lfg_type: LfgType,
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
                entry,
                lfg_type,
            } => {
                1
                + 2 // entry: u16
                + 2 // lfg_type: LfgType
            }
        }
    }
}

