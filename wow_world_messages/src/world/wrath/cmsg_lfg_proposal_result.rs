use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_proposal_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_proposal_result.wowm#L1):
/// ```text
/// cmsg CMSG_LFG_PROPOSAL_RESULT = 0x0362 {
///     u32 proposal_id;
///     Bool accept_join;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_LFG_PROPOSAL_RESULT {
    pub proposal_id: u32,
    pub accept_join: bool,
}

impl crate::private::Sealed for CMSG_LFG_PROPOSAL_RESULT {}
impl CMSG_LFG_PROPOSAL_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // proposal_id: u32
        let proposal_id = crate::util::read_u32_le(&mut r)?;

        // accept_join: Bool
        let accept_join = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            proposal_id,
            accept_join,
        })
    }

}

impl crate::Message for CMSG_LFG_PROPOSAL_RESULT {
    const OPCODE: u32 = 0x0362;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_LFG_PROPOSAL_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LFG_PROPOSAL_RESULT {{").unwrap();
        // Members
        writeln!(s, "    proposal_id = {};", self.proposal_id).unwrap();
        writeln!(s, "    accept_join = {};", if self.accept_join { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 9_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 866_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "proposal_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "accept_join", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // proposal_id: u32
        w.write_all(&self.proposal_id.to_le_bytes())?;

        // accept_join: Bool
        w.write_all(u8::from(self.accept_join).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(866, "CMSG_LFG_PROPOSAL_RESULT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_PROPOSAL_RESULT {}

