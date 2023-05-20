use std::io::{Read, Write};

use crate::tbc::LfgData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_set_looking_for_more.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_set_looking_for_more.wowm#L1):
/// ```text
/// cmsg CMSG_SET_LOOKING_FOR_MORE = 0x0365 {
///     LfgData data;
/// }
/// ```
pub struct CMSG_SET_LOOKING_FOR_MORE {
    pub data: LfgData,
}

impl crate::private::Sealed for CMSG_SET_LOOKING_FOR_MORE {}
impl crate::Message for CMSG_SET_LOOKING_FOR_MORE {
    const OPCODE: u32 = 0x0365;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // data: LfgData
        self.data.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0365, size: body_size });
        }

        // data: LfgData
        let data = LfgData::read(&mut r)?;

        Ok(Self {
            data,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_LOOKING_FOR_MORE {}

