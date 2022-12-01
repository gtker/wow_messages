use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_battlefield_mgr_exit_request.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_battlefield_mgr_exit_request.wowm#L1):
/// ```text
/// cmsg CMSG_BATTLEFIELD_MGR_EXIT_REQUEST = 0x04E7 {
///     u32 battle_id;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_MGR_EXIT_REQUEST {
    pub battle_id: u32,
}

impl crate::Message for CMSG_BATTLEFIELD_MGR_EXIT_REQUEST {
    const OPCODE: u32 = 0x04e7;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E7, size: body_size as u32 });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            battle_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_BATTLEFIELD_MGR_EXIT_REQUEST {}

