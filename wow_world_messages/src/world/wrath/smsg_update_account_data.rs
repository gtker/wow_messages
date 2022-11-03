use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Sent as response to [`CMSG_REQUEST_ACCOUNT_DATA`](crate::world::vanilla::CMSG_REQUEST_ACCOUNT_DATA)
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_update_account_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_update_account_data.wowm#L1):
/// ```text
/// smsg SMSG_UPDATE_ACCOUNT_DATA = 0x020C {
///     u32 data_type;
///     u32 decompressed_size;
///     u8[-] compressed_data;
/// }
/// ```
pub struct SMSG_UPDATE_ACCOUNT_DATA {
    pub data_type: u32,
    pub decompressed_size: u32,
    pub compressed_data: Vec<u8>,
}

impl crate::Message for SMSG_UPDATE_ACCOUNT_DATA {
    const OPCODE: u32 = 0x020c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // data_type: u32
        w.write_all(&self.data_type.to_le_bytes())?;

        // decompressed_size: u32
        w.write_all(&self.decompressed_size.to_le_bytes())?;

        // compressed_data: u8[-]
        for i in self.compressed_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 8 || body_size > 65543 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x020C, size: body_size as u32 });
        }

        // data_type: u32
        let data_type = crate::util::read_u32_le(r)?;

        // decompressed_size: u32
        let decompressed_size = crate::util::read_u32_le(r)?;

        // compressed_data: u8[-]
        let mut current_size = {
            4 // data_type: u32
            + 4 // decompressed_size: u32
        };
        let mut compressed_data = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            compressed_data.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            data_type,
            decompressed_size,
            compressed_data,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_UPDATE_ACCOUNT_DATA {}

impl SMSG_UPDATE_ACCOUNT_DATA {
    pub(crate) fn size(&self) -> usize {
        4 // data_type: u32
        + 4 // decompressed_size: u32
        + self.compressed_data.len() * core::mem::size_of::<u8>() // compressed_data: u8[-]
    }
}

