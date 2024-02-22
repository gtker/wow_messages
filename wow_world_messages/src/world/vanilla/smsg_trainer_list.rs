use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Skill, TrainerSpell, TrainerSpellState,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm#L28):
/// ```text
/// smsg SMSG_TRAINER_LIST = 0x01B1 {
///     Guid guid;
///     u32 trainer_type;
///     u32 amount_of_spells;
///     TrainerSpell[amount_of_spells] spells;
///     CString greeting;
/// }
/// ```
pub struct SMSG_TRAINER_LIST {
    pub guid: Guid,
    pub trainer_type: u32,
    pub spells: Vec<TrainerSpell>,
    pub greeting: String,
}

impl crate::private::Sealed for SMSG_TRAINER_LIST {}
impl SMSG_TRAINER_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(17..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // trainer_type: u32
        let trainer_type = crate::util::read_u32_le(&mut r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(&mut r)?;

        // spells: TrainerSpell[amount_of_spells]
        let spells = {
            let mut spells = Vec::with_capacity(amount_of_spells as usize);

            let allocation_size = u64::from(amount_of_spells) * 38;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_spells {
                spells.push(TrainerSpell::read(&mut r)?);
            }
            spells
        };

        // greeting: CString
        let greeting = {
            let greeting = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(greeting)?
        };

        Ok(Self {
            guid,
            trainer_type,
            spells,
            greeting,
        })
    }

}

impl crate::Message for SMSG_TRAINER_LIST {
    const OPCODE: u32 = 0x01b1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_TRAINER_LIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRAINER_LIST {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    trainer_type = {};", self.trainer_type).unwrap();
        writeln!(s, "    amount_of_spells = {};", self.spells.len()).unwrap();
        writeln!(s, "    spells = [").unwrap();
        for v in self.spells.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            spell = {};", v.spell).unwrap();
            writeln!(s, "            state = {};", v.state.as_test_case_value()).unwrap();
            writeln!(s, "            spell_cost = {};", v.spell_cost).unwrap();
            writeln!(s, "            talent_point_cost = {};", v.talent_point_cost).unwrap();
            writeln!(s, "            first_rank = {};", v.first_rank).unwrap();
            writeln!(s, "            required_level = {};", v.required_level).unwrap();
            writeln!(s, "            required_skill = {};", v.required_skill.as_test_case_value()).unwrap();
            writeln!(s, "            required_skill_value = {};", v.required_skill_value).unwrap();
            writeln!(s, "            required_spells = [").unwrap();
            for v in v.required_spells.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "            ];").unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    greeting = \"{}\";", self.greeting).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 433_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "trainer_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_spells", "    ");
        if !self.spells.is_empty() {
            writeln!(s, "    /* spells: TrainerSpell[amount_of_spells] start */").unwrap();
            for (i, v) in self.spells.iter().enumerate() {
                writeln!(s, "    /* spells: TrainerSpell[amount_of_spells] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "state", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_cost", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "talent_point_cost", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "first_rank", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "required_level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "required_skill", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "required_skill_value", "        ");
                writeln!(s, "    /* required_spells: u32[3] start */").unwrap();
                for (i, v) in v.required_spells.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("required_spells {i}"), "        ");
                }
                writeln!(s, "    /* required_spells: u32[3] end */").unwrap();
                writeln!(s, "    /* spells: TrainerSpell[amount_of_spells] {i} end */").unwrap();
            }
            writeln!(s, "    /* spells: TrainerSpell[amount_of_spells] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, self.greeting.len() + 1, "greeting", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // trainer_type: u32
        w.write_all(&self.trainer_type.to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: TrainerSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.write_into_vec(&mut w)?;
        }

        // greeting: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.greeting.as_bytes().iter().next_back(), Some(&0_u8), "String `greeting` must not be null-terminated.");
        w.write_all(self.greeting.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(433, "SMSG_TRAINER_LIST", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TRAINER_LIST {}

impl SMSG_TRAINER_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // trainer_type: u32
        + 4 // amount_of_spells: u32
        + self.spells.len() * 38 // spells: TrainerSpell[amount_of_spells]
        + self.greeting.len() + 1 // greeting: CString
    }
}

