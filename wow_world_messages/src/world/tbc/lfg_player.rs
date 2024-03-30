use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::tbc::{
    Area, LfgMode, LfgPlayerMember,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L36):
/// ```text
/// struct LfgPlayer {
///     PackedGuid guid;
///     Level32 level;
///     Area area;
///     LfgMode lfg_mode;
///     u32[3] lfg_slots;
///     CString comment;
///     u32 amount_of_members;
///     LfgPlayerMember[amount_of_members] members;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgPlayer {
    pub guid: Guid,
    pub level: Level,
    pub area: Area,
    pub lfg_mode: LfgMode,
    pub lfg_slots: [u32; 3],
    pub comment: String,
    pub members: Vec<LfgPlayerMember>,
}

impl LfgPlayer {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // level: Level32
        w.write_all(&u32::from(self.level.as_int()).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // lfg_mode: LfgMode
        w.write_all(&(self.lfg_mode.as_int().to_le_bytes()))?;

        // lfg_slots: u32[3]
        for i in self.lfg_slots.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.comment.as_bytes().iter().next_back(), Some(&0_u8), "String `comment` must not be null-terminated.");
        w.write_all(self.comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: LfgPlayerMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl LfgPlayer {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // level: Level32
        let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // lfg_mode: LfgMode
        let lfg_mode = crate::util::read_u8_le(&mut r)?.try_into()?;

        // lfg_slots: u32[3]
        let lfg_slots = {
            let mut lfg_slots = [u32::default(); 3];
            for i in lfg_slots.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            lfg_slots
        };

        // comment: CString
        let comment = {
            let comment = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(comment)?
        };

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // members: LfgPlayerMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);

            let allocation_size = u64::from(amount_of_members) * 5;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_members {
                members.push(LfgPlayerMember::read(&mut r)?);
            }
            members
        };

        Ok(Self {
            guid,
            level,
            area,
            lfg_mode,
            lfg_slots,
            comment,
            members,
        })
    }

}

impl LfgPlayer {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // level: Level32
        + 4 // area: Area
        + 1 // lfg_mode: LfgMode
        + 12 // lfg_slots: u32[3]
        + self.comment.len() + 1 // comment: CString
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: LfgPlayerMember[amount_of_members]
    }
}

