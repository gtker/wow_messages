use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::PetitionShowlist;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm#L39):
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

#[cfg(feature = "print-testcase")]
impl SMSG_PETITION_SHOWLIST {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PETITION_SHOWLIST {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    amount_of_petitions = {};", self.petitions.len()).unwrap();
        write!(s, "    petitions = [").unwrap();
        for v in self.petitions.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    index = {};", v.index).unwrap();
            writeln!(s, "    charter_entry = {};", v.charter_entry).unwrap();
            writeln!(s, "    charter_display_id = {};", v.charter_display_id).unwrap();
            writeln!(s, "    guild_charter_cost = {};", v.guild_charter_cost).unwrap();
            writeln!(s, "    unknown1 = {};", v.unknown1).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 444_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_PETITION_SHOWLIST {}
impl crate::Message for SMSG_PETITION_SHOWLIST {
    const OPCODE: u32 = 0x01bc;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_petitions: u8
        w.write_all(&(self.petitions.len() as u8).to_le_bytes())?;

        // petitions: PetitionShowlist[amount_of_petitions]
        for i in self.petitions.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=5129).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BC, size: body_size });
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // amount_of_petitions: u8
        let amount_of_petitions = crate::util::read_u8_le(&mut r)?;

        // petitions: PetitionShowlist[amount_of_petitions]
        let petitions = {
            let mut petitions = Vec::with_capacity(amount_of_petitions as usize);
            for _ in 0..amount_of_petitions {
                petitions.push(PetitionShowlist::read(&mut r)?);
            }
            petitions
        };

        Ok(Self {
            npc,
            petitions,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PETITION_SHOWLIST {}

impl SMSG_PETITION_SHOWLIST {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 1 // amount_of_petitions: u8
        + self.petitions.len() * 20 // petitions: PetitionShowlist[amount_of_petitions]
    }
}

