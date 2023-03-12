use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm#L1):
/// ```text
/// cmsg CMSG_WARDEN_DATA = 0x02E7 {
///     u8[-] encrypted_data;
/// }
/// ```
pub struct CMSG_WARDEN_DATA {
    pub encrypted_data: Vec<u8>,
}

impl crate::Message for CMSG_WARDEN_DATA {
    const OPCODE: u32 = 0x02e7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // encrypted_data: u8[-]
        for i in self.encrypted_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size > 65535 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E7, size: body_size as u32 });
        }

        // encrypted_data: u8[-]
        let encrypted_data = {
            let mut current_size = {
                0
            };
            let mut encrypted_data = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                encrypted_data.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            encrypted_data
        };

        Ok(Self {
            encrypted_data,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_WARDEN_DATA {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_WARDEN_DATA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_WARDEN_DATA {}

impl CMSG_WARDEN_DATA {
    pub(crate) fn size(&self) -> usize {
        self.encrypted_data.len() * core::mem::size_of::<u8>() // encrypted_data: u8[-]
    }
}

