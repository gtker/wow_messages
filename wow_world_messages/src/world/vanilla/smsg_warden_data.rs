use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/unimplemented/vanilla/smsg_warden_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/vanilla/smsg_warden_data.wowm#L1):
/// ```text
/// smsg SMSG_WARDEN_DATA = 0x02E6 {
///     unimplemented
/// }
/// ```
pub struct SMSG_WARDEN_DATA {
    pub unimplemented: Vec<u8>,
}

impl crate::Message for SMSG_WARDEN_DATA {
    const OPCODE: u32 = 0x02e6;

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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_WARDEN_DATA {}

impl SMSG_WARDEN_DATA {
    pub(crate) fn size(&self) -> usize {
        self.unimplemented.len() * core::mem::size_of::<u8>() // unimplemented: u8[-]
    }
}

