use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::pvp_rank_vanilla_tbc_wrath::PvpRank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/smsg_pvp_credit.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/smsg_pvp_credit.wowm#L3):
/// ```text
/// smsg SMSG_PVP_CREDIT = 0x028C {
///     u32 honor_points;
///     Guid victim;
///     (u32)PvpRank rank;
/// }
/// ```
pub struct SMSG_PVP_CREDIT {
    pub honor_points: u32,
    pub victim: Guid,
    pub rank: PvpRank,
}

impl crate::private::Sealed for SMSG_PVP_CREDIT {}
impl crate::Message for SMSG_PVP_CREDIT {
    const OPCODE: u32 = 0x028c;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // honor_points: u32
        w.write_all(&self.honor_points.to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        // rank: PvpRank
        w.write_all(&u32::from(self.rank.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x028C, size: body_size });
        }

        // honor_points: u32
        let honor_points = crate::util::read_u32_le(&mut r)?;

        // victim: Guid
        let victim = Guid::read(&mut r)?;

        // rank: PvpRank
        let rank: PvpRank = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            honor_points,
            victim,
            rank,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PVP_CREDIT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PVP_CREDIT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PVP_CREDIT {}

