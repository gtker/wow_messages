use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_unstable_pet.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_unstable_pet.wowm#L3):
/// ```text
/// cmsg CMSG_UNSTABLE_PET = 0x0271 {
///     Guid stable_master;
///     u32 pet_number;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_UNSTABLE_PET {
    pub stable_master: Guid,
    pub pet_number: u32,
}

impl crate::private::Sealed for CMSG_UNSTABLE_PET {}
impl CMSG_UNSTABLE_PET {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // stable_master: Guid
        let stable_master = crate::util::read_guid(&mut r)?;

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            stable_master,
            pet_number,
        })
    }

}

impl crate::Message for CMSG_UNSTABLE_PET {
    const OPCODE: u32 = 0x0271;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_UNSTABLE_PET"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_UNSTABLE_PET {{").unwrap();
        // Members
        writeln!(s, "    stable_master = {};", self.stable_master.guid()).unwrap();
        writeln!(s, "    pet_number = {};", self.pet_number).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 625_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "stable_master", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "pet_number", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // stable_master: Guid
        w.write_all(&self.stable_master.guid().to_le_bytes())?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(625, "CMSG_UNSTABLE_PET", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_UNSTABLE_PET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_UNSTABLE_PET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UNSTABLE_PET {}

