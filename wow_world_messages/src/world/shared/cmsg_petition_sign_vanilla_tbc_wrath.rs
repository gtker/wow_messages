use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_sign.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_sign.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_SIGN = 0x01C0 {
///     Guid petition;
///     u8 unknown1;
/// }
/// ```
pub struct CMSG_PETITION_SIGN {
    pub petition: Guid,
    pub unknown1: u8,
}

impl crate::private::Sealed for CMSG_PETITION_SIGN {}
impl CMSG_PETITION_SIGN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x01C0, size: body_size });
        }

        // petition: Guid
        let petition = crate::util::read_guid(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            petition,
            unknown1,
        })
    }

}

impl crate::Message for CMSG_PETITION_SIGN {
    const OPCODE: u32 = 0x01c0;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PETITION_SIGN {{").unwrap();
        // Members
        writeln!(s, "    petition = {};", self.petition.guid()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 448_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "petition", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PETITION_SIGN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PETITION_SIGN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PETITION_SIGN {}

