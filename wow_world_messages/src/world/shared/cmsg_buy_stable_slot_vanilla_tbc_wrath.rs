use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm#L3):
/// ```text
/// cmsg CMSG_BUY_STABLE_SLOT = 0x0272 {
///     Guid npc;
/// }
/// ```
pub struct CMSG_BUY_STABLE_SLOT {
    pub npc: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_BUY_STABLE_SLOT {
    pub fn to_test_case_string(&self) -> Option<String> {
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
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_BUY_STABLE_SLOT {}
impl crate::Message for CMSG_BUY_STABLE_SLOT {
    const OPCODE: u32 = 0x0272;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_BUY_STABLE_SLOT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0272, size: body_size });
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        Ok(Self {
            npc,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BUY_STABLE_SLOT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BUY_STABLE_SLOT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUY_STABLE_SLOT {}

