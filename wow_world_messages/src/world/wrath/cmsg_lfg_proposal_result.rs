use std::io::{Write, Read};

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

impl crate::Message for CMSG_LFG_PROPOSAL_RESULT {
    const OPCODE: u32 = 0x0362;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // proposal_id: u32
        w.write_all(&self.proposal_id.to_le_bytes())?;

        // accept_join: Bool
        w.write_all(u8::from(self.accept_join).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0362, size: body_size as u32 });
        }

        // proposal_id: u32
        let proposal_id = crate::util::read_u32_le(r)?;

        // accept_join: Bool
        let accept_join = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            proposal_id,
            accept_join,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_PROPOSAL_RESULT {}

