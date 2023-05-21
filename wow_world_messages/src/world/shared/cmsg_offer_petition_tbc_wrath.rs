use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_offer_petition.wowm#L8):
/// ```text
/// cmsg CMSG_OFFER_PETITION = 0x01C3 {
///     u32 unknown0;
///     Guid petition;
///     Guid target;
/// }
/// ```
pub struct CMSG_OFFER_PETITION {
    pub unknown0: u32,
    pub petition: Guid,
    pub target: Guid,
}

impl crate::private::Sealed for CMSG_OFFER_PETITION {}
impl CMSG_OFFER_PETITION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 20 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown0: u32
        let unknown0 = crate::util::read_u32_le(&mut r)?;

        // petition: Guid
        let petition = crate::util::read_guid(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        Ok(Self {
            unknown0,
            petition,
            target,
        })
    }

}

impl crate::Message for CMSG_OFFER_PETITION {
    const OPCODE: u32 = 0x01c3;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_OFFER_PETITION {{").unwrap();
        // Members
        writeln!(s, "    unknown0 = {};", self.unknown0).unwrap();
        writeln!(s, "    petition = {};", self.petition.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 24_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 451_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "petition", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown0: u32
        w.write_all(&self.unknown0.to_le_bytes())?;

        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(451, "CMSG_OFFER_PETITION", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_OFFER_PETITION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_OFFER_PETITION {}

