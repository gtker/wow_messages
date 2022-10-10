use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::PetitionShowlist;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm#L23):
/// ```text
/// smsg SMSG_PETITION_SHOWLIST = 0x01BC {
///     Guid npc;
///     u8 amount_of_petitions;
///     PetitionShowlist[amount_of_petitions] petitions;
/// }
/// ```
pub struct SMSG_PETITION_SHOWLIST {
    pub npc: Guid,
    pub petitions: Vec<PetitionShowlist>,
}

impl crate::Message for SMSG_PETITION_SHOWLIST {
    const OPCODE: u32 = 0x01bc;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_petitions: u8
        w.write_all(&(self.petitions.len() as u8).to_le_bytes())?;

        // petitions: PetitionShowlist[amount_of_petitions]
        for i in self.petitions.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // amount_of_petitions: u8
        let amount_of_petitions = crate::util::read_u8_le(r)?;

        // petitions: PetitionShowlist[amount_of_petitions]
        let mut petitions = Vec::with_capacity(amount_of_petitions as usize);
        for i in 0..amount_of_petitions {
            let o = PetitionShowlist::read(r)?;
            petitions.push(o);
        }

        Ok(Self {
            npc,
            petitions,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PETITION_SHOWLIST {}

impl SMSG_PETITION_SHOWLIST {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 1 // amount_of_petitions: u8
        + self.petitions.len() * 24 // petitions: PetitionShowlist[amount_of_petitions]
    }
}

