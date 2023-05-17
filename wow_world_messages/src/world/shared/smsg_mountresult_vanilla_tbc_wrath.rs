use std::io::{Read, Write};

use wow_world_base::shared::mount_result_vanilla_tbc_wrath::MountResult;

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

impl crate::private::Sealed for SMSG_MOUNTRESULT {}
impl crate::Message for SMSG_MOUNTRESULT {
    const OPCODE: u32 = 0x016e;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: MountResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x016E, size: body_size });
        }

        // result: MountResult
        let result: MountResult = crate::util::read_u32_le(&mut r)?.try_into()?;

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

