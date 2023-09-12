use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_pet_learn_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_pet_learn_talent.wowm#L1):
/// ```text
/// cmsg CMSG_PET_LEARN_TALENT = 0x047A {
///     Guid pet;
///     u32 talent;
///     u32 rank;
/// }
/// ```
pub struct CMSG_PET_LEARN_TALENT {
    pub pet: Guid,
    pub talent: u32,
    pub rank: u32,
}

impl crate::private::Sealed for CMSG_PET_LEARN_TALENT {}
impl CMSG_PET_LEARN_TALENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // pet: Guid
        let pet = crate::util::read_guid(&mut r)?;

        // talent: u32
        let talent = crate::util::read_u32_le(&mut r)?;

        // rank: u32
        let rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            pet,
            talent,
            rank,
        })
    }

}

impl crate::Message for CMSG_PET_LEARN_TALENT {
    const OPCODE: u32 = 0x047a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_PET_LEARN_TALENT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PET_LEARN_TALENT {{").unwrap();
        // Members
        writeln!(s, "    pet = {};", self.pet.guid()).unwrap();
        writeln!(s, "    talent = {};", self.talent).unwrap();
        writeln!(s, "    rank = {};", self.rank).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1146_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "pet", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "talent", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rank", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // talent: u32
        w.write_all(&self.talent.to_le_bytes())?;

        // rank: u32
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1146, "CMSG_PET_LEARN_TALENT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_LEARN_TALENT {}

