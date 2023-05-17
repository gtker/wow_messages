use std::io::{Read, Write};

use crate::wrath::CacheMask;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Indicate when each piece of account data was last updated by a [`CMSG_UPDATE_ACCOUNT_DATA`](crate::wrath::CMSG_UPDATE_ACCOUNT_DATA). The client can check this against its own times to detect that more recent account data was written from a different client.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm:45`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm#L45):
/// ```text
/// smsg SMSG_ACCOUNT_DATA_TIMES = 0x0209 {
///     u32 unix_time;
///     u8 unknown1;
///     CacheMask mask;
///     u32[-] data;
/// }
/// ```
pub struct SMSG_ACCOUNT_DATA_TIMES {
    /// Seconds since Unix Epoch
    ///
    pub unix_time: u32,
    /// Both mangostwo and arcemu hardcode this to 1
    ///
    pub unknown1: u8,
    pub mask: CacheMask,
    /// Maximum size is 32 4-bit integers. For every bit that is 1 in the mask, write one u32 with the time
    ///
    pub data: Vec<u32>,
}

impl crate::private::Sealed for SMSG_ACCOUNT_DATA_TIMES {}
impl crate::Message for SMSG_ACCOUNT_DATA_TIMES {
    const OPCODE: u32 = 0x0209;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unix_time: u32
        w.write_all(&self.unix_time.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // mask: CacheMask
        w.write_all(&(self.mask.as_int().to_le_bytes()))?;

        // data: u32[-]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=65544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0209, size: body_size });
        }

        // unix_time: u32
        let unix_time = crate::util::read_u32_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // mask: CacheMask
        let mask: CacheMask = crate::util::read_u32_le(&mut r)?.try_into()?;

        // data: u32[-]
        let data = {
            let mut current_size = {
                4 // unix_time: u32
                + 1 // unknown1: u8
                + 4 // mask: CacheMask
            };
            let mut data = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                data.push(crate::util::read_u32_le(&mut r)?);
                current_size += 1;
            }
            data
        };

        Ok(Self {
            unix_time,
            unknown1,
            mask,
            data,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACCOUNT_DATA_TIMES {}

impl SMSG_ACCOUNT_DATA_TIMES {
    pub(crate) fn size(&self) -> usize {
        4 // unix_time: u32
        + 1 // unknown1: u8
        + 4 // mask: CacheMask
        + self.data.len() * core::mem::size_of::<u32>() // data: u32[-]
    }
}

