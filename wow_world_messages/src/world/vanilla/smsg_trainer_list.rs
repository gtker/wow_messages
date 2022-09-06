use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::TrainerSpell;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm#L33):
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

impl crate::Message for SMSG_TRAINER_LIST {
    const OPCODE: u32 = 0x01b1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // trainer_type: u32
        w.write_all(&self.trainer_type.to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: TrainerSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.write_into_vec(w)?;
        }

        // greeting: CString
        w.write_all(self.greeting.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // trainer_type: u32
        let trainer_type = crate::util::read_u32_le(r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(r)?;

        // spells: TrainerSpell[amount_of_spells]
        let mut spells = Vec::with_capacity(amount_of_spells as usize);
        for i in 0..amount_of_spells {
            spells.push(TrainerSpell::read(r)?);
        }

        // greeting: CString
        let greeting = crate::util::read_c_string_to_vec(r)?;
        let greeting = String::from_utf8(greeting)?;

        Ok(Self {
            guid,
            trainer_type,
            spells,
            greeting,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_TRAINER_LIST {}

impl SMSG_TRAINER_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // trainer_type: u32
        + 4 // amount_of_spells: u32
        + self.spells.len() * 38 // spells: TrainerSpell[amount_of_spells]
        + self.greeting.len() + 1 // greeting: CString
    }
}

