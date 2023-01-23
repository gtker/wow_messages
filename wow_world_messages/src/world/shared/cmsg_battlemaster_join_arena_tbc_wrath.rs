use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::join_arena_type_tbc_wrath::JoinArenaType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_battlemaster_join_arena.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_battlemaster_join_arena.wowm#L9):
/// ```text
/// cmsg CMSG_BATTLEMASTER_JOIN_ARENA = 0x0358 {
///     Guid battlemaster;
///     JoinArenaType arena_type;
///     Bool as_group;
///     Bool rated;
/// }
/// ```
pub struct CMSG_BATTLEMASTER_JOIN_ARENA {
    pub battlemaster: Guid,
    pub arena_type: JoinArenaType,
    pub as_group: bool,
    pub rated: bool,
}

impl crate::Message for CMSG_BATTLEMASTER_JOIN_ARENA {
    const OPCODE: u32 = 0x0358;

    fn size_without_header(&self) -> u32 {
        11
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // arena_type: JoinArenaType
        w.write_all(&(self.arena_type.as_int() as u8).to_le_bytes())?;

        // as_group: Bool
        w.write_all(u8::from(self.as_group).to_le_bytes().as_slice())?;

        // rated: Bool
        w.write_all(u8::from(self.rated).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 11 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0358, size: body_size as u32 });
        }

        // battlemaster: Guid
        let battlemaster = Guid::read(r)?;

        // arena_type: JoinArenaType
        let arena_type: JoinArenaType = crate::util::read_u8_le(r)?.try_into()?;

        // as_group: Bool
        let as_group = crate::util::read_u8_le(r)? != 0;
        // rated: Bool
        let rated = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            battlemaster,
            arena_type,
            as_group,
            rated,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_BATTLEMASTER_JOIN_ARENA {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_BATTLEMASTER_JOIN_ARENA {}

