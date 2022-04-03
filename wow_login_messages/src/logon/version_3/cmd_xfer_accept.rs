use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L3):
/// ```text
/// clogin CMD_XFER_ACCEPT = 0x32 {
/// }
/// ```
pub struct CMD_XFER_ACCEPT {
}

impl ClientMessage for CMD_XFER_ACCEPT {
    const OPCODE: u8 = 0x32;
}
impl ReadableAndWritable for CMD_XFER_ACCEPT {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for CMD_XFER_ACCEPT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMD_XFER_ACCEPT {
    fn maximum_possible_size() -> usize {
        0
    }
}

