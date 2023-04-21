use std::io::{Read, Write};

use crate::wrath::WorldResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`CMSG_CHAR_CREATE`](crate::wrath::CMSG_CHAR_CREATE).
///
/// Every `WorldResult` except `CHAR_CREATE_SUCCESS` will lead to a popup showing. `CHAR_CREATE_SUCCESS` will cause the client to send a [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm#L1):
/// ```text
/// smsg SMSG_CHAR_CREATE = 0x003A {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHAR_CREATE {
    pub result: WorldResult,
}

impl crate::private::Sealed for SMSG_CHAR_CREATE {}
impl crate::Message for SMSG_CHAR_CREATE {
    const OPCODE: u32 = 0x003a;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003A, size: body_size as u32 });
        }

        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHAR_CREATE {}

