use crate::shared::mount_result_vanilla_tbc_wrath::MountResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// This is not used in any TBC emulator, but trinitycore has it implemented so it is assumed to be valid for TBC as well.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_mountresult.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_mountresult.wowm#L39):
/// ```text
/// smsg SMSG_MOUNTRESULT = 0x016E {
///     MountResult result;
/// }
/// ```
pub struct SMSG_MOUNTRESULT {
    pub result: MountResult,
}

impl crate::Message for SMSG_MOUNTRESULT {
    const OPCODE: u32 = 0x016e;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: MountResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x016E, size: body_size as u32 });
        }

        // result: MountResult
        let result: MountResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MOUNTRESULT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOUNTRESULT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOUNTRESULT {}

