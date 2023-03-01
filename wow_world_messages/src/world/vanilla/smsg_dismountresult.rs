use crate:: {
};
use crate::vanilla::DismountResult;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm#L8):
/// ```text
/// smsg SMSG_DISMOUNTRESULT = 0x016F {
///     DismountResult result;
/// }
/// ```
pub struct SMSG_DISMOUNTRESULT {
    pub result: DismountResult,
}

impl crate::Message for SMSG_DISMOUNTRESULT {
    const OPCODE: u32 = 0x016f;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: DismountResult
        w.write_all(&u32::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x016F, size: body_size as u32 });
        }

        // result: DismountResult
        let result: DismountResult = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DISMOUNTRESULT {}

