use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L31):
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

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_XFER_ACCEPT;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 33.
    #[test]
    fn CMD_XFER_ACCEPT0() {
        let raw: Vec<u8> = vec![ 0x32, ];

        let expected = CMD_XFER_ACCEPT {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_ACCEPT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_ACCEPT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_XFER_ACCEPT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
