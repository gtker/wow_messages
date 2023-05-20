use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_corpse_not_in_instance.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_corpse_not_in_instance.wowm#L1):
/// ```text
/// smsg SMSG_CORPSE_NOT_IN_INSTANCE = 0x0506 {
/// }
/// ```
pub struct SMSG_CORPSE_NOT_IN_INSTANCE {
}

impl crate::private::Sealed for SMSG_CORPSE_NOT_IN_INSTANCE {}
impl crate::Message for SMSG_CORPSE_NOT_IN_INSTANCE {
    const OPCODE: u32 = 0x0506;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0506, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CORPSE_NOT_IN_INSTANCE {}

