use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_mgr_entry_invite_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_mgr_entry_invite_response.wowm#L1):
/// ```text
/// cmsg CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE = 0x04DF {
///     u32 battle_id;
///     Bool accepted;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE {
    pub battle_id: u32,
    pub accepted: bool,
}

impl crate::private::Sealed for CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE {}
impl CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // accepted: Bool
        let accepted = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            battle_id,
            accepted,
        })
    }

}

impl crate::Message for CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE {
    const OPCODE: u32 = 0x04df;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    battle_id = {};", self.battle_id).unwrap();
        writeln!(s, "    accepted = {};", if self.accepted { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 9_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1247_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battle_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "accepted", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // accepted: Bool
        w.write_all(u8::from(self.accepted).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1247, "CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BATTLEFIELD_MGR_ENTRY_INVITE_RESPONSE {}

