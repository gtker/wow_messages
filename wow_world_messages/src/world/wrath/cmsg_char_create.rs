use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/unimplemented/wrath/cmsg_char_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/wrath/cmsg_char_create.wowm#L3):
/// ```text
/// cmsg CMSG_CHAR_CREATE = 0x0036 {
///     u8[-] unimplemented;
/// }
/// ```
pub struct CMSG_CHAR_CREATE {
    pub unimplemented: Vec<u8>,
}

impl crate::Message for CMSG_CHAR_CREATE {
    const OPCODE: u32 = 0x0036;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unimplemented: u8[-]
        for i in self.unimplemented.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unimplemented: u8[-]
        let mut current_size = {
            0
        };
        let mut unimplemented = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            unimplemented.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            unimplemented,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CHAR_CREATE {}

impl CMSG_CHAR_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.unimplemented.len() * core::mem::size_of::<u8>() // unimplemented: u8[-]
    }
}

