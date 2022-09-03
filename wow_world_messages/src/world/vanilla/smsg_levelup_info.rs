use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm#L3):
/// ```text
/// smsg SMSG_LEVELUP_INFO = 0x01D4 {
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

impl crate::Message for SMSG_LEVELUP_INFO {
    const OPCODE: u32 = 0x01d4;

    fn size_without_header(&self) -> u32 {
        48
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 48 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

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

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_LEVELUP_INFO {}

