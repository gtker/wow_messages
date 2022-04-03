use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::PetitionShowlist;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new5.wowm:285`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new5.wowm#L285):
/// ```text
/// smsg SMSG_PETITION_SHOWLIST = 0x1BC {
///     Guid npc;
///     u8 amount_of_petitions;
///     PetitionShowlist[amount_of_petitions] petitions;
/// }
/// ```
pub struct SMSG_PETITION_SHOWLIST {
    pub npc: Guid,
    pub amount_of_petitions: u8,
    pub petitions: Vec<PetitionShowlist>,
}

impl WorldServerMessageWrite for SMSG_PETITION_SHOWLIST {
    const OPCODE: u16 = 0x1bc;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PETITION_SHOWLIST {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // amount_of_petitions: u8
        let amount_of_petitions = crate::util::read_u8_le(r)?;

        // petitions: PetitionShowlist[amount_of_petitions]
        let mut petitions = Vec::with_capacity(amount_of_petitions as usize);
        for i in 0..amount_of_petitions {
            petitions.push(PetitionShowlist::read(r)?);
        }

        Ok(Self {
            npc,
            amount_of_petitions,
            petitions,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.write(w)?;

        // amount_of_petitions: u8
        w.write_all(&(self.petitions.len() as u8).to_le_bytes())?;

        // petitions: PetitionShowlist[amount_of_petitions]
        for i in self.petitions.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_PETITION_SHOWLIST {
    fn size(&self) -> usize {
        8 // npc: Guid
        + 1 // amount_of_petitions: u8
        + self.petitions.iter().fold(0, |acc, x| acc + PetitionShowlist::size()) // petitions: PetitionShowlist[amount_of_petitions]
    }
}

impl MaximumPossibleSized for SMSG_PETITION_SHOWLIST {
    fn maximum_possible_size() -> usize {
        8 // npc: Guid
        + 1 // amount_of_petitions: u8
        + 255 * PetitionShowlist::maximum_possible_size() // petitions: PetitionShowlist[amount_of_petitions]
    }
}

