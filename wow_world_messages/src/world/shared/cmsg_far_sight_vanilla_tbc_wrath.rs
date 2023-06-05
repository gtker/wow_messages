use std::io::{Read, Write};

use wow_world_base::shared::far_sight_operation_vanilla_tbc_wrath::FarSightOperation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm#L8):
/// ```text
/// cmsg CMSG_FAR_SIGHT = 0x027A {
///     FarSightOperation operation;
/// }
/// ```
pub struct CMSG_FAR_SIGHT {
    pub operation: FarSightOperation,
}

impl crate::private::Sealed for CMSG_FAR_SIGHT {}
impl crate::Message for CMSG_FAR_SIGHT {
    const OPCODE: u32 = 0x027a;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // operation: FarSightOperation
        w.write_all(&(self.operation.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x027A, size: body_size });
        }

        // operation: FarSightOperation
        let operation = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            operation,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_FAR_SIGHT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_FAR_SIGHT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_FAR_SIGHT {}

