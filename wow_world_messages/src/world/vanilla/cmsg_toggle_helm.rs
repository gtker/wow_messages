use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_toggle_helm.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_toggle_helm.wowm#L3):
/// ```text
/// cmsg CMSG_TOGGLE_HELM = 0x02B9 {
/// }
/// ```
pub struct CMSG_TOGGLE_HELM {
}

impl crate::Message for CMSG_TOGGLE_HELM {
    const OPCODE: u32 = 0x02b9;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_TOGGLE_HELM {}

