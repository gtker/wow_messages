use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm#L3):
/// ```text
/// cmsg CMSG_BUY_STABLE_SLOT = 0x0272 {
///     Guid npc;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_BUY_STABLE_SLOT {
    pub npc: Guid,
}

impl crate::private::Sealed for CMSG_BUY_STABLE_SLOT {}
impl CMSG_BUY_STABLE_SLOT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        Ok(Self {
            npc,
        })
    }

}

impl crate::Message for CMSG_BUY_STABLE_SLOT {
    const OPCODE: u32 = 0x0272;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BUY_STABLE_SLOT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BUY_STABLE_SLOT {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 626_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(626, "CMSG_BUY_STABLE_SLOT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BUY_STABLE_SLOT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BUY_STABLE_SLOT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUY_STABLE_SLOT {}

