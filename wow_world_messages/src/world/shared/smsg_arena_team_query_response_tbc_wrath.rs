use std::convert::{TryFrom, TryInto};
use crate::world::shared::arena_type_tbc_wrath::ArenaType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_arena_team_query_response.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_TEAM_QUERY_RESPONSE = 0x034C {
///     u32 arena_team;
///     CString team_name;
///     ArenaType team_type;
///     u32 background_color;
///     u32 emblem_style;
///     u32 emblem_color;
///     u32 border_style;
///     u32 border_color;
/// }
/// ```
pub struct SMSG_ARENA_TEAM_QUERY_RESPONSE {
    pub arena_team: u32,
    pub team_name: String,
    pub team_type: ArenaType,
    pub background_color: u32,
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
}

impl crate::Message for SMSG_ARENA_TEAM_QUERY_RESPONSE {
    const OPCODE: u32 = 0x034c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        // team_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.team_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `team_name` must not be null-terminated.");
        w.write_all(self.team_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // team_type: ArenaType
        w.write_all(&(self.team_type.as_int() as u8).to_le_bytes())?;

        // background_color: u32
        w.write_all(&self.background_color.to_le_bytes())?;

        // emblem_style: u32
        w.write_all(&self.emblem_style.to_le_bytes())?;

        // emblem_color: u32
        w.write_all(&self.emblem_color.to_le_bytes())?;

        // border_style: u32
        w.write_all(&self.border_style.to_le_bytes())?;

        // border_color: u32
        w.write_all(&self.border_color.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(26..=281).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x034C, size: body_size as u32 });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(r)?;

        // team_name: CString
        let team_name = crate::util::read_c_string_to_vec(r)?;
        let team_name = String::from_utf8(team_name)?;

        // team_type: ArenaType
        let team_type: ArenaType = crate::util::read_u8_le(r)?.try_into()?;

        // background_color: u32
        let background_color = crate::util::read_u32_le(r)?;

        // emblem_style: u32
        let emblem_style = crate::util::read_u32_le(r)?;

        // emblem_color: u32
        let emblem_color = crate::util::read_u32_le(r)?;

        // border_style: u32
        let border_style = crate::util::read_u32_le(r)?;

        // border_color: u32
        let border_color = crate::util::read_u32_le(r)?;

        Ok(Self {
            arena_team,
            team_name,
            team_type,
            background_color,
            emblem_style,
            emblem_color,
            border_style,
            border_color,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ARENA_TEAM_QUERY_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ARENA_TEAM_QUERY_RESPONSE {}

impl SMSG_ARENA_TEAM_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // arena_team: u32
        + self.team_name.len() + 1 // team_name: CString
        + 1 // team_type: ArenaType
        + 4 // background_color: u32
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
    }
}

