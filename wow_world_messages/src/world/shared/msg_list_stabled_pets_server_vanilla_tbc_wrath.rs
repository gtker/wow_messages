use std::io::{Read, Write};

use crate::Guid;
use crate::shared::stabled_pet_vanilla_tbc_wrath::StabledPet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm#L14):
/// ```text
/// smsg MSG_LIST_STABLED_PETS_Server = 0x026F {
///     Guid npc;
///     u8 amount_of_pets;
///     u8 stable_slots;
///     StabledPet[amount_of_pets] pets;
/// }
/// ```
pub struct MSG_LIST_STABLED_PETS_Server {
    pub npc: Guid,
    pub stable_slots: u8,
    pub pets: Vec<StabledPet>,
}

#[cfg(feature = "print-testcase")]
impl MSG_LIST_STABLED_PETS_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_LIST_STABLED_PETS_Server {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    amount_of_pets = {};", self.pets.len()).unwrap();
        writeln!(s, "    stable_slots = {};", self.stable_slots).unwrap();
        write!(s, "    pets = [").unwrap();
        for v in self.pets.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    pet_number = {};", v.pet_number).unwrap();
            writeln!(s, "    entry = {};", v.entry).unwrap();
            writeln!(s, "    level = {};", v.level.as_int()).unwrap();
            writeln!(s, "    name = \"{}\";", v.name).unwrap();
            writeln!(s, "    loyalty = {};", v.loyalty).unwrap();
            writeln!(s, "    slot = {};", v.slot).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 623_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_LIST_STABLED_PETS_Server {}
impl crate::Message for MSG_LIST_STABLED_PETS_Server {
    const OPCODE: u32 = 0x026f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes())?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes())?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(10..=69898).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x026F, size: body_size });
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // amount_of_pets: u8
        let amount_of_pets = crate::util::read_u8_le(&mut r)?;

        // stable_slots: u8
        let stable_slots = crate::util::read_u8_le(&mut r)?;

        // pets: StabledPet[amount_of_pets]
        let pets = {
            let mut pets = Vec::with_capacity(amount_of_pets as usize);
            for _ in 0..amount_of_pets {
                pets.push(StabledPet::read(&mut r)?);
            }
            pets
        };

        Ok(Self {
            npc,
            stable_slots,
            pets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_LIST_STABLED_PETS_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_LIST_STABLED_PETS_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_LIST_STABLED_PETS_Server {}

impl MSG_LIST_STABLED_PETS_Server {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 1 // amount_of_pets: u8
        + 1 // stable_slots: u8
        + self.pets.iter().fold(0, |acc, x| acc + x.size()) // pets: StabledPet[amount_of_pets]
    }
}

