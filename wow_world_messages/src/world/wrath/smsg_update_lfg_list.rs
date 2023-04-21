use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    LfgListGroup, LfgListPlayer, LfgListUpdateType, LfgType,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm:111`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm#L111):
/// ```text
/// smsg SMSG_UPDATE_LFG_LIST = 0x0360 {
///     (u32)LfgType lfg_type;
///     u32 dungeon_id;
///     LfgListUpdateType update_type;
///     if (update_type == PARTIAL) {
///         u32 amount_of_deleted_guids;
///         Guid[amount_of_deleted_guids] deleted_guids;
///     }
///     u32 amount_of_groups;
///     u32 unknown1;
///     LfgListGroup[amount_of_groups] groups;
///     u32 amount_of_players;
///     u32 unknown2;
///     LfgListPlayer[amount_of_players] players;
/// }
/// ```
pub struct SMSG_UPDATE_LFG_LIST {
    pub lfg_type: LfgType,
    pub dungeon_id: u32,
    pub update_type: SMSG_UPDATE_LFG_LIST_LfgListUpdateType,
    /// emus set to 0.
    ///
    pub unknown1: u32,
    pub groups: Vec<LfgListGroup>,
    /// emus set to 0.
    ///
    pub unknown2: u32,
    pub players: Vec<LfgListPlayer>,
}

impl crate::private::Sealed for SMSG_UPDATE_LFG_LIST {}
impl crate::Message for SMSG_UPDATE_LFG_LIST {
    const OPCODE: u32 = 0x0360;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // lfg_type: LfgType
        w.write_all(&u32::from(self.lfg_type.as_int()).to_le_bytes())?;

        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // update_type: LfgListUpdateType
        w.write_all(&u8::from(self.update_type.as_int()).to_le_bytes())?;

        match &self.update_type {
            SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Partial {
                deleted_guids,
            } => {
                // amount_of_deleted_guids: u32
                w.write_all(&(deleted_guids.len() as u32).to_le_bytes())?;

                // deleted_guids: Guid[amount_of_deleted_guids]
                for i in deleted_guids.iter() {
                    w.write_all(&i.guid().to_le_bytes())?;
                }

            }
            _ => {}
        }

        // amount_of_groups: u32
        w.write_all(&(self.groups.len() as u32).to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // groups: LfgListGroup[amount_of_groups]
        for i in self.groups.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // players: LfgListPlayer[amount_of_players]
        for i in self.players.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(25..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0360, size: body_size as u32 });
        }

        // lfg_type: LfgType
        let lfg_type: LfgType = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // update_type: LfgListUpdateType
        let update_type: LfgListUpdateType = crate::util::read_u8_le(&mut r)?.try_into()?;

        let update_type_if = match update_type {
            LfgListUpdateType::Partial => {
                // amount_of_deleted_guids: u32
                let amount_of_deleted_guids = crate::util::read_u32_le(&mut r)?;

                // deleted_guids: Guid[amount_of_deleted_guids]
                let deleted_guids = {
                    let mut deleted_guids = Vec::with_capacity(amount_of_deleted_guids as usize);
                    for i in 0..amount_of_deleted_guids {
                        deleted_guids.push(Guid::read(&mut r)?);
                    }
                    deleted_guids
                };

                SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Partial {
                    deleted_guids,
                }
            }
            LfgListUpdateType::Full => SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Full,
        };

        // amount_of_groups: u32
        let amount_of_groups = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // groups: LfgListGroup[amount_of_groups]
        let groups = {
            let mut groups = Vec::with_capacity(amount_of_groups as usize);
            for i in 0..amount_of_groups {
                groups.push(LfgListGroup::read(&mut r)?);
            }
            groups
        };

        // amount_of_players: u32
        let amount_of_players = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // players: LfgListPlayer[amount_of_players]
        let players = {
            let mut players = Vec::with_capacity(amount_of_players as usize);
            for i in 0..amount_of_players {
                players.push(LfgListPlayer::read(&mut r)?);
            }
            players
        };

        Ok(Self {
            lfg_type,
            dungeon_id,
            update_type: update_type_if,
            unknown1,
            groups,
            unknown2,
            players,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_LFG_LIST {}

impl SMSG_UPDATE_LFG_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // lfg_type: LfgType
        + 4 // dungeon_id: u32
        + self.update_type.size() // update_type: SMSG_UPDATE_LFG_LIST_LfgListUpdateType
        + 4 // amount_of_groups: u32
        + 4 // unknown1: u32
        + self.groups.iter().fold(0, |acc, x| acc + x.size()) // groups: LfgListGroup[amount_of_groups]
        + 4 // amount_of_players: u32
        + 4 // unknown2: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: LfgListPlayer[amount_of_players]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    Partial {
        deleted_guids: Vec<Guid>,
    },
    Full,
}

impl Default for SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Full
    }
}

impl SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Partial { .. } => 0,
            Self::Full => 1,
        }
    }

}

impl SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Partial {
                deleted_guids,
            } => {
                1
                + 4 // amount_of_deleted_guids: u32
                + deleted_guids.len() *  8 // deleted_guids: Guid[amount_of_deleted_guids]
            }
            Self::Full => {
                1
            }
        }
    }
}

