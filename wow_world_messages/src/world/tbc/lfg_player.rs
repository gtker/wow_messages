use crate::Guid;
use crate::tbc::LfgPlayerMember;
use crate::tbc::Area;
use crate::tbc::LfgMode;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:37`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L37):
/// ```text
/// struct LfgPlayer {
///     PackedGuid guid;
///     u32 level;
///     Area area;
///     LfgMode lfg_mode;
///     u32[3] lfg_slots;
///     CString comment;
///     u32 amount_of_members;
///     LfgPlayerMember[amount_of_members] members;
/// }
/// ```
pub struct LfgPlayer {
    pub guid: Guid,
    pub level: u32,
    pub area: Area,
    pub lfg_mode: LfgMode,
    pub lfg_slots: [u32; 3],
    pub comment: String,
    pub members: Vec<LfgPlayerMember>,
}

impl LfgPlayer {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // area: Area
        w.write_all(&u32::from(self.area.as_int()).to_le_bytes())?;

        // lfg_mode: LfgMode
        w.write_all(&u8::from(self.lfg_mode.as_int()).to_le_bytes())?;

        // lfg_slots: u32[3]
        for i in self.lfg_slots.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `comment` must not be null-terminated.");
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
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // level: u32
        let level = crate::util::read_u32_le(&mut r)?;

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // lfg_mode: LfgMode
        let lfg_mode: LfgMode = crate::util::read_u8_le(&mut r)?.try_into()?;

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
            for i in 0..amount_of_members {
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
        self.guid.size() // guid: Guid
        + 4 // level: u32
        + 4 // area: Area
        + 1 // lfg_mode: LfgMode
        + 3 * core::mem::size_of::<u32>() // lfg_slots: u32[3]
        + self.comment.len() + 1 // comment: CString
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: LfgPlayerMember[amount_of_members]
    }
}

