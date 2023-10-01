use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm#L3):
/// ```text
/// smsg SMSG_DISPEL_FAILED = 0x0262 {
///     Guid caster;
///     Guid target;
///     Spell[-] spells;
/// }
/// ```
pub struct SMSG_DISPEL_FAILED {
    pub caster: Guid,
    pub target: Guid,
    pub spells: Vec<u32>,
}

impl crate::private::Sealed for SMSG_DISPEL_FAILED {}
impl SMSG_DISPEL_FAILED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=65551).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // spells: Spell[-]
        let spells = {
            let mut current_size = {
                8 // caster: Guid
                + 8 // target: Guid
            };
            let mut spells = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                spells.push(crate::util::read_u32_le(&mut r)?);
                current_size += 4;
            }
            spells
        };

        Ok(Self {
            caster,
            target,
            spells,
        })
    }

}

impl crate::Message for SMSG_DISPEL_FAILED {
    const OPCODE: u32 = 0x0262;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_DISPEL_FAILED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DISPEL_FAILED {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    spells = [").unwrap();
        for v in self.spells.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 610_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
        if !self.spells.is_empty() {
            writeln!(s, "    /* spells: Spell[-] start */").unwrap();
            for (i, v) in self.spells.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("spells {i}"), "    ");
            }
            writeln!(s, "    /* spells: Spell[-] end */").unwrap();
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
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // spells: Spell[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(610, "SMSG_DISPEL_FAILED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DISPEL_FAILED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DISPEL_FAILED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DISPEL_FAILED {}

impl SMSG_DISPEL_FAILED {
    pub(crate) fn size(&self) -> usize {
        8 // caster: Guid
        + 8 // target: Guid
        + self.spells.len() * 4 // spells: Spell[-]
    }
}

