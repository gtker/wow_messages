use std::io::{Read, Write};

use crate::shared::level_vanilla_tbc_wrath::Level;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm#L18):
/// ```text
/// smsg SMSG_LEVELUP_INFO = 0x01D4 {
///     Level32 new_level;
///     u32 health;
///     u32 mana;
///     u32 rage;
///     u32 focus;
///     u32 energy;
///     u32 happiness;
///     u32 rune;
///     u32 runic_power;
///     u32 strength;
///     u32 agility;
///     u32 stamina;
///     u32 intellect;
///     u32 spirit;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_LEVELUP_INFO {
    pub new_level: Level,
    pub health: u32,
    pub mana: u32,
    pub rage: u32,
    pub focus: u32,
    pub energy: u32,
    pub happiness: u32,
    pub rune: u32,
    pub runic_power: u32,
    pub strength: u32,
    pub agility: u32,
    pub stamina: u32,
    pub intellect: u32,
    pub spirit: u32,
}

impl crate::private::Sealed for SMSG_LEVELUP_INFO {}
impl SMSG_LEVELUP_INFO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 56 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // new_level: Level32
        let new_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // health: u32
        let health = crate::util::read_u32_le(&mut r)?;

        // mana: u32
        let mana = crate::util::read_u32_le(&mut r)?;

        // rage: u32
        let rage = crate::util::read_u32_le(&mut r)?;

        // focus: u32
        let focus = crate::util::read_u32_le(&mut r)?;

        // energy: u32
        let energy = crate::util::read_u32_le(&mut r)?;

        // happiness: u32
        let happiness = crate::util::read_u32_le(&mut r)?;

        // rune: u32
        let rune = crate::util::read_u32_le(&mut r)?;

        // runic_power: u32
        let runic_power = crate::util::read_u32_le(&mut r)?;

        // strength: u32
        let strength = crate::util::read_u32_le(&mut r)?;

        // agility: u32
        let agility = crate::util::read_u32_le(&mut r)?;

        // stamina: u32
        let stamina = crate::util::read_u32_le(&mut r)?;

        // intellect: u32
        let intellect = crate::util::read_u32_le(&mut r)?;

        // spirit: u32
        let spirit = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            new_level,
            health,
            mana,
            rage,
            focus,
            energy,
            happiness,
            rune,
            runic_power,
            strength,
            agility,
            stamina,
            intellect,
            spirit,
        })
    }

}

impl crate::Message for SMSG_LEVELUP_INFO {
    const OPCODE: u32 = 0x01d4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LEVELUP_INFO"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LEVELUP_INFO {{").unwrap();
        // Members
        writeln!(s, "    new_level = {};", self.new_level.as_int()).unwrap();
        writeln!(s, "    health = {};", self.health).unwrap();
        writeln!(s, "    mana = {};", self.mana).unwrap();
        writeln!(s, "    rage = {};", self.rage).unwrap();
        writeln!(s, "    focus = {};", self.focus).unwrap();
        writeln!(s, "    energy = {};", self.energy).unwrap();
        writeln!(s, "    happiness = {};", self.happiness).unwrap();
        writeln!(s, "    rune = {};", self.rune).unwrap();
        writeln!(s, "    runic_power = {};", self.runic_power).unwrap();
        writeln!(s, "    strength = {};", self.strength).unwrap();
        writeln!(s, "    agility = {};", self.agility).unwrap();
        writeln!(s, "    stamina = {};", self.stamina).unwrap();
        writeln!(s, "    intellect = {};", self.intellect).unwrap();
        writeln!(s, "    spirit = {};", self.spirit).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 58_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 468_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "new_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "health", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "mana", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "focus", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "energy", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "happiness", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rune", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "runic_power", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "strength", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "agility", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "stamina", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "intellect", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spirit", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        56
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // new_level: Level32
        w.write_all(&u32::from(self.new_level.as_int()).to_le_bytes())?;

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

        // rune: u32
        w.write_all(&self.rune.to_le_bytes())?;

        // runic_power: u32
        w.write_all(&self.runic_power.to_le_bytes())?;

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(468, "SMSG_LEVELUP_INFO", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LEVELUP_INFO {}

