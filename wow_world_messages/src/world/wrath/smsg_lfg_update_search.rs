use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_search.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_search.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_SEARCH = 0x0369 {
///     Bool in_lfg_queue;
/// }
/// ```
pub struct SMSG_LFG_UPDATE_SEARCH {
    pub in_lfg_queue: bool,
}

impl crate::Message for SMSG_LFG_UPDATE_SEARCH {
    const OPCODE: u32 = 0x0369;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // in_lfg_queue: Bool
        w.write_all(u8::from(self.in_lfg_queue).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0369, size: body_size as u32 });
        }

        // in_lfg_queue: Bool
        let in_lfg_queue = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            in_lfg_queue,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_UPDATE_SEARCH {}

