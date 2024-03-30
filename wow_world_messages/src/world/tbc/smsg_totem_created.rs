use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_totem_created.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_totem_created.wowm#L1):
/// ```text
/// smsg SMSG_TOTEM_CREATED = 0x0412 {
///     u8 slot;
///     Guid totem;
///     u32 duration;
///     Spell spell;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_TOTEM_CREATED {
    pub slot: u8,
    pub totem: Guid,
    pub duration: u32,
    pub spell: u32,
}

impl crate::private::Sealed for SMSG_TOTEM_CREATED {}
impl SMSG_TOTEM_CREATED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 17 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // totem: Guid
        let totem = crate::util::read_guid(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            slot,
            totem,
            duration,
            spell,
        })
    }

}

impl crate::Message for SMSG_TOTEM_CREATED {
    const OPCODE: u32 = 0x0412;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_TOTEM_CREATED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TOTEM_CREATED {{").unwrap();
        // Members
        writeln!(s, "    slot = {};", self.slot).unwrap();
        writeln!(s, "    totem = {};", self.totem.guid()).unwrap();
        writeln!(s, "    duration = {};", self.duration).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 19_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1042_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "totem", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // totem: Guid
        w.write_all(&self.totem.guid().to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1042, "SMSG_TOTEM_CREATED", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TOTEM_CREATED {}

