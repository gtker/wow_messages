use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_proposal_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_proposal_result.wowm#L1):
/// ```text
/// cmsg CMSG_LFG_PROPOSAL_RESULT = 0x0362 {
///     u32 proposal_id;
///     Bool accept_join;
/// }
/// ```
pub struct CMSG_LFG_PROPOSAL_RESULT {
    pub proposal_id: u32,
    pub accept_join: bool,
}

#[cfg(feature = "print-testcase")]
impl CMSG_LFG_PROPOSAL_RESULT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LFG_PROPOSAL_RESULT {{").unwrap();
        // Members
        writeln!(s, "    proposal_id = {};", self.proposal_id).unwrap();
        writeln!(s, "    accept_join = {};", if self.accept_join { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 866_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "proposal_id");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_LFG_PROPOSAL_RESULT {}
impl crate::Message for CMSG_LFG_PROPOSAL_RESULT {
    const OPCODE: u32 = 0x0362;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0362, size: body_size });
        }

        // proposal_id: u32
        let proposal_id = crate::util::read_u32_le(&mut r)?;

        // accept_join: Bool
        let accept_join = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            proposal_id,
            accept_join,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_PROPOSAL_RESULT {}

