use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::environmental_damage_type_vanilla_tbc_wrath::EnvironmentalDamageType;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm#L12):
/// ```text
/// smsg SMSG_ENVIRONMENTAL_DAMAGE_LOG = 0x01FC {
///     Guid guid;
///     EnvironmentalDamageType damage_type;
///     u32 damage;
///     u32 absorb;
///     u32 resist;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ENVIRONMENTAL_DAMAGE_LOG {
    pub guid: Guid,
    pub damage_type: EnvironmentalDamageType,
    pub damage: u32,
    pub absorb: u32,
    pub resist: u32,
}

impl crate::private::Sealed for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}
impl SMSG_ENVIRONMENTAL_DAMAGE_LOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 21 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // damage_type: EnvironmentalDamageType
        let damage_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        // absorb: u32
        let absorb = crate::util::read_u32_le(&mut r)?;

        // resist: u32
        let resist = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            damage_type,
            damage,
            absorb,
            resist,
        })
    }

}

impl crate::Message for SMSG_ENVIRONMENTAL_DAMAGE_LOG {
    const OPCODE: u32 = 0x01fc;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ENVIRONMENTAL_DAMAGE_LOG"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ENVIRONMENTAL_DAMAGE_LOG {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    damage_type = {};", self.damage_type.as_test_case_value()).unwrap();
        writeln!(s, "    damage = {};", self.damage).unwrap();
        writeln!(s, "    absorb = {};", self.absorb).unwrap();
        writeln!(s, "    resist = {};", self.resist).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 23_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 508_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "damage_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "absorb", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "resist", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // damage_type: EnvironmentalDamageType
        w.write_all(&(self.damage_type.as_int().to_le_bytes()))?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(508, "SMSG_ENVIRONMENTAL_DAMAGE_LOG", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}

