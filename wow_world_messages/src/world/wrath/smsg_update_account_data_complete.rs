use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_update_account_data_complete.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_update_account_data_complete.wowm#L1):
/// ```text
/// smsg SMSG_UPDATE_ACCOUNT_DATA_COMPLETE = 0x0463 {
///     u32 data_type;
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {
    pub data_type: u32,
    /// mangostwo hardcodes this to 0
    ///
    pub unknown1: u32,
}

impl crate::private::Sealed for SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {}
impl crate::Message for SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {
    const OPCODE: u32 = 0x0463;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // data_type: u32
        w.write_all(&self.data_type.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0463, size: body_size });
        }

        // data_type: u32
        let data_type = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            data_type,
            unknown1,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_ACCOUNT_DATA_COMPLETE {}

