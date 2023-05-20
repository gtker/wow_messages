use std::io::{Read, Write};

use crate::tbc::{
    ArenaTeamMember, ArenaType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_roster.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_roster.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_TEAM_ROSTER = 0x034E {
///     u32 arena_team;
///     u32 amount_of_members;
///     ArenaType arena_type;
///     ArenaTeamMember[amount_of_members] members;
/// }
/// ```
pub struct SMSG_ARENA_TEAM_ROSTER {
    pub arena_team: u32,
    pub arena_type: ArenaType,
    pub members: Vec<ArenaTeamMember>,
}

impl crate::private::Sealed for SMSG_ARENA_TEAM_ROSTER {}
impl crate::Message for SMSG_ARENA_TEAM_ROSTER {
    const OPCODE: u32 = 0x034e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // arena_type: ArenaType
        w.write_all(&(self.arena_type.as_int().to_le_bytes()))?;

        // members: ArenaTeamMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x034E, size: body_size });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // arena_type: ArenaType
        let arena_type: ArenaType = crate::util::read_u8_le(&mut r)?.try_into()?;

        // members: ArenaTeamMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(ArenaTeamMember::read(&mut r)?);
            }
            members
        };

        Ok(Self {
            arena_team,
            arena_type,
            members,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_ROSTER {}

impl SMSG_ARENA_TEAM_ROSTER {
    pub(crate) fn size(&self) -> usize {
        4 // arena_team: u32
        + 4 // amount_of_members: u32
        + 1 // arena_type: ArenaType
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: ArenaTeamMember[amount_of_members]
    }
}

