use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_socket_gems_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_socket_gems_result.wowm#L1):
/// ```text
/// smsg SMSG_SOCKET_GEMS_RESULT = 0x050B {
///     Guid item;
///     u32[3] sockets;
/// }
/// ```
pub struct SMSG_SOCKET_GEMS_RESULT {
    pub item: Guid,
    pub sockets: [u32; 3],
}

impl crate::Message for SMSG_SOCKET_GEMS_RESULT {
    const OPCODE: u32 = 0x050b;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // sockets: u32[3]
        for i in self.sockets.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x050B, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        // sockets: u32[3]
        let sockets = {
            let mut sockets = [u32::default(); 3];
            for i in sockets.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            sockets
        };

        Ok(Self {
            item,
            sockets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SOCKET_GEMS_RESULT {}

