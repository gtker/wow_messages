use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_stable_pet.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_stable_pet.wowm#L3):
/// ```text
/// cmsg CMSG_STABLE_PET = 0x0270 {
///     Guid stable_master;
/// }
/// ```
pub struct CMSG_STABLE_PET {
    pub stable_master: Guid,
}

impl crate::private::Sealed for CMSG_STABLE_PET {}
impl CMSG_STABLE_PET {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0270, size: body_size });
        }

        // stable_master: Guid
        let stable_master = crate::util::read_guid(&mut r)?;

        Ok(Self {
            stable_master,
        })
    }

}

impl crate::Message for CMSG_STABLE_PET {
    const OPCODE: u32 = 0x0270;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_STABLE_PET {{").unwrap();
        // Members
        writeln!(s, "    stable_master = {};", self.stable_master.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 624_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "stable_master", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // stable_master: Guid
        w.write_all(&self.stable_master.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_STABLE_PET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_STABLE_PET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_STABLE_PET {}

