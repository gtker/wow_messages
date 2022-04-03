use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L12):
/// ```text
/// slogin CMD_XFER_INITIATE = 0x30 {
/// }
/// ```
pub struct CMD_XFER_INITIATE {
}

impl ServerMessage for CMD_XFER_INITIATE {
    const OPCODE: u8 = 0x30;
}
impl ReadableAndWritable for CMD_XFER_INITIATE {
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

impl ConstantSized for CMD_XFER_INITIATE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMD_XFER_INITIATE {
    fn maximum_possible_size() -> usize {
        0
    }
}

