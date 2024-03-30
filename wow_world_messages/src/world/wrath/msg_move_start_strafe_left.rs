use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MovementFlags, MovementInfo, TransportInfo, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_strafe_left.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_strafe_left.wowm#L39):
/// ```text
/// msg MSG_MOVE_START_STRAFE_LEFT = 0x00B8 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MSG_MOVE_START_STRAFE_LEFT {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_START_STRAFE_LEFT {}
impl MSG_MOVE_START_STRAFE_LEFT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(31..=97).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for MSG_MOVE_START_STRAFE_LEFT {
    const OPCODE: u32 = 0x00b8;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_START_STRAFE_LEFT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        panic!("MSG types not supported");
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(184, "MSG_MOVE_START_STRAFE_LEFT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_START_STRAFE_LEFT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_START_STRAFE_LEFT {}

impl MSG_MOVE_START_STRAFE_LEFT {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

