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
impl SMSG_PVP_CREDIT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x028C, size: body_size });
        }

        // honor_points: u32
        let honor_points = crate::util::read_u32_le(&mut r)?;

        // victim: Guid
        let victim = crate::util::read_guid(&mut r)?;

        // rank: PvpRank
        let rank = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            honor_points,
            victim,
            rank,
        })
    }

}

impl crate::Message for SMSG_PVP_CREDIT {
    const OPCODE: u32 = 0x028c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PVP_CREDIT {{").unwrap();
        // Members
        writeln!(s, "    honor_points = {};", self.honor_points).unwrap();
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    rank = {};", self.rank.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 652_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_points", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rank", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // honor_points: u32
        w.write_all(&self.honor_points.to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        // rank: PvpRank
        w.write_all(&u32::from(self.rank.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PVP_CREDIT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PVP_CREDIT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PVP_CREDIT {}

