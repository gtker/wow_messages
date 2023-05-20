use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_deleteequipment_set.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_deleteequipment_set.wowm#L1):
/// ```text
/// cmsg CMSG_DELETEEQUIPMENT_SET = 0x013E {
///     PackedGuid set;
/// }
/// ```
pub struct CMSG_DELETEEQUIPMENT_SET {
    pub set: Guid,
}

impl crate::private::Sealed for CMSG_DELETEEQUIPMENT_SET {}
impl crate::Message for CMSG_DELETEEQUIPMENT_SET {
    const OPCODE: u32 = 0x013e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // set: PackedGuid
        self.set.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x013E, size: body_size });
        }

        // set: PackedGuid
        let set = Guid::read_packed(&mut r)?;

        Ok(Self {
            set,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_DELETEEQUIPMENT_SET {}

impl CMSG_DELETEEQUIPMENT_SET {
    pub(crate) const fn size(&self) -> usize {
        self.set.size() // set: PackedGuid
    }
}

