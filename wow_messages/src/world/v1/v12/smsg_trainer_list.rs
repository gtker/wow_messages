use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{TrainerSpell, TrainerSpellError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:704`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L704):
/// ```text
/// smsg SMSG_TRAINER_LIST = 0x1B1 {
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
    pub amount_of_spells: u32,
    pub spells: Vec<TrainerSpell>,
    pub greeting: String,
}

impl WorldServerMessageWrite for SMSG_TRAINER_LIST {
    const OPCODE: u16 = 0x1b1;

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
impl WorldMessageBody for SMSG_TRAINER_LIST {
    type Error = SMSG_TRAINER_LISTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
            amount_of_spells,
            spells,
            greeting,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // trainer_type: u32
        w.write_all(&self.trainer_type.to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: TrainerSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.write(w)?;
        }

        // greeting: CString
        w.write_all(self.greeting.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_TRAINER_LIST {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // trainer_type: u32
        + 4 // amount_of_spells: u32
        + self.spells.iter().fold(0, |acc, x| acc + TrainerSpell::size()) // spells: TrainerSpell[amount_of_spells]
        + self.greeting.len() + 1 // greeting: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_TRAINER_LIST {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // trainer_type: u32
        + 4 // amount_of_spells: u32
        + 4294967295 * TrainerSpell::maximum_possible_size() // spells: TrainerSpell[amount_of_spells]
        + 256 // greeting: CString
    }
}

#[derive(Debug)]
pub enum SMSG_TRAINER_LISTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    TrainerSpell(TrainerSpellError),
}

impl std::error::Error for SMSG_TRAINER_LISTError {}
impl std::fmt::Display for SMSG_TRAINER_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::TrainerSpell(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRAINER_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_TRAINER_LISTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<TrainerSpellError> for SMSG_TRAINER_LISTError {
    fn from(e: TrainerSpellError) -> Self {
        Self::TrainerSpell(e)
    }
}

