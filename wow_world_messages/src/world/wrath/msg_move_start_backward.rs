use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm#L13):
/// ```text
/// msg MSG_MOVE_START_BACKWARD = 0x00B6 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_START_BACKWARD {
    pub guid: Guid,
    pub info: MovementInfo,
}

#[cfg(feature = "print-testcase")]
impl MSG_MOVE_START_BACKWARD {
    pub fn to_test_case_string(&self) -> String {
        panic!("MSG types not supported");
    }

}

impl crate::private::Sealed for MSG_MOVE_START_BACKWARD {}
impl crate::Message for MSG_MOVE_START_BACKWARD {
    const OPCODE: u32 = 0x00b6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(32..=97).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B6, size: body_size });
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_START_BACKWARD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_START_BACKWARD {}

impl MSG_MOVE_START_BACKWARD {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

