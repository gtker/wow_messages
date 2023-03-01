use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_socket_gems.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_socket_gems.wowm#L1):
/// ```text
/// cmsg CMSG_SOCKET_GEMS = 0x0347 {
///     Guid item;
///     Guid[3] gems;
/// }
/// ```
pub struct CMSG_SOCKET_GEMS {
    pub item: Guid,
    pub gems: [Guid; 3],
}

impl crate::Message for CMSG_SOCKET_GEMS {
    const OPCODE: u32 = 0x0347;

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // gems: Guid[3]
        for i in self.gems.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 32 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0347, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        // gems: Guid[3]
        let gems = {
            let mut gems = [Guid::default(); 3];
            for i in gems.iter_mut() {
                *i = Guid::read(&mut r)?;
            }
            gems
        };

        Ok(Self {
            item,
            gems,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SOCKET_GEMS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SOCKET_GEMS {}

