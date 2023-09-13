use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
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

impl crate::private::Sealed for MSG_LIST_STABLED_PETS_Server {}
impl MSG_LIST_STABLED_PETS_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(10..=69898).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for MSG_LIST_STABLED_PETS_Server {
    const OPCODE: u32 = 0x026f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_LIST_STABLED_PETS_Server"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_LIST_STABLED_PETS_Server {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    amount_of_pets = {};", self.pets.len()).unwrap();
        writeln!(s, "    stable_slots = {};", self.stable_slots).unwrap();
        writeln!(s, "    pets = [").unwrap();
        for v in self.pets.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            pet_number = {};", v.pet_number).unwrap();
            writeln!(s, "            entry = {};", v.entry).unwrap();
            writeln!(s, "            level = {};", v.level.as_int()).unwrap();
            writeln!(s, "            name = \"{}\";", v.name).unwrap();
            writeln!(s, "            loyalty = {};", v.loyalty).unwrap();
            writeln!(s, "            slot = {};", v.slot).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 623_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_pets", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "stable_slots", "    ");
        if !self.pets.is_empty() {
            writeln!(s, "    /* pets: StabledPet[amount_of_pets] start */").unwrap();
            for (i, v) in self.pets.iter().enumerate() {
                writeln!(s, "    /* pets: StabledPet[amount_of_pets] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "pet_number", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "entry", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.name.len() + 1, "name", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "loyalty", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "        ");
                writeln!(s, "    /* pets: StabledPet[amount_of_pets] {i} end */").unwrap();
            }
            writeln!(s, "    /* pets: StabledPet[amount_of_pets] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(623, "MSG_LIST_STABLED_PETS_Server", body_size, a))
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

