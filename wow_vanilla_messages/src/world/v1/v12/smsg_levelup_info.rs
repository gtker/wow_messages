use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm#L3):
/// ```text
/// smsg SMSG_LEVELUP_INFO = 0x1D4 {
///     u32 new_level;
///     u32 health;
///     u32 mana;
///     u32 rage;
///     u32 focus;
///     u32 energy;
///     u32 happiness;
///     u32 strength;
///     u32 agility;
///     u32 stamina;
///     u32 intellect;
///     u32 spirit;
/// }
/// ```
pub struct SMSG_LEVELUP_INFO {
    pub new_level: u32,
    pub health: u32,
    pub mana: u32,
    pub rage: u32,
    pub focus: u32,
    pub energy: u32,
    pub happiness: u32,
    pub strength: u32,
    pub agility: u32,
    pub stamina: u32,
    pub intellect: u32,
    pub spirit: u32,
}

impl WorldServerMessageWrite for SMSG_LEVELUP_INFO {
    const OPCODE: u16 = 0x1d4;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_LEVELUP_INFO {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // new_level: u32
        let new_level = crate::util::read_u32_le(r)?;

        // health: u32
        let health = crate::util::read_u32_le(r)?;

        // mana: u32
        let mana = crate::util::read_u32_le(r)?;

        // rage: u32
        let rage = crate::util::read_u32_le(r)?;

        // focus: u32
        let focus = crate::util::read_u32_le(r)?;

        // energy: u32
        let energy = crate::util::read_u32_le(r)?;

        // happiness: u32
        let happiness = crate::util::read_u32_le(r)?;

        // strength: u32
        let strength = crate::util::read_u32_le(r)?;

        // agility: u32
        let agility = crate::util::read_u32_le(r)?;

        // stamina: u32
        let stamina = crate::util::read_u32_le(r)?;

        // intellect: u32
        let intellect = crate::util::read_u32_le(r)?;

        // spirit: u32
        let spirit = crate::util::read_u32_le(r)?;

        Ok(Self {
            new_level,
            health,
            mana,
            rage,
            focus,
            energy,
            happiness,
            strength,
            agility,
            stamina,
            intellect,
            spirit,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // new_level: u32
        w.write_all(&self.new_level.to_le_bytes())?;

        // health: u32
        w.write_all(&self.health.to_le_bytes())?;

        // mana: u32
        w.write_all(&self.mana.to_le_bytes())?;

        // rage: u32
        w.write_all(&self.rage.to_le_bytes())?;

        // focus: u32
        w.write_all(&self.focus.to_le_bytes())?;

        // energy: u32
        w.write_all(&self.energy.to_le_bytes())?;

        // happiness: u32
        w.write_all(&self.happiness.to_le_bytes())?;

        // strength: u32
        w.write_all(&self.strength.to_le_bytes())?;

        // agility: u32
        w.write_all(&self.agility.to_le_bytes())?;

        // stamina: u32
        w.write_all(&self.stamina.to_le_bytes())?;

        // intellect: u32
        w.write_all(&self.intellect.to_le_bytes())?;

        // spirit: u32
        w.write_all(&self.spirit.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LEVELUP_INFO {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LEVELUP_INFO {
    fn maximum_possible_size() -> usize {
        4 // new_level: u32
        + 4 // health: u32
        + 4 // mana: u32
        + 4 // rage: u32
        + 4 // focus: u32
        + 4 // energy: u32
        + 4 // happiness: u32
        + 4 // strength: u32
        + 4 // agility: u32
        + 4 // stamina: u32
        + 4 // intellect: u32
        + 4 // spirit: u32
    }
}

