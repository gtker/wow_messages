use std::io::{Read, Write};

use crate::vanilla::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_exploration_experience.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_exploration_experience.wowm#L1):
/// ```text
/// smsg SMSG_EXPLORATION_EXPERIENCE = 0x01F8 {
///     Area area;
///     u32 experience;
/// }
/// ```
pub struct SMSG_EXPLORATION_EXPERIENCE {
    pub area: Area,
    pub experience: u32,
}

impl crate::private::Sealed for SMSG_EXPLORATION_EXPERIENCE {}
impl SMSG_EXPLORATION_EXPERIENCE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // experience: u32
        let experience = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            area,
            experience,
        })
    }

}

impl crate::Message for SMSG_EXPLORATION_EXPERIENCE {
    const OPCODE: u32 = 0x01f8;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_EXPLORATION_EXPERIENCE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_EXPLORATION_EXPERIENCE {{").unwrap();
        // Members
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();
        writeln!(s, "    experience = {};", self.experience).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 504_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "experience", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // experience: u32
        w.write_all(&self.experience.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(504, "SMSG_EXPLORATION_EXPERIENCE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_EXPLORATION_EXPERIENCE {}

