use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:37`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L37):
/// ```text
/// clogin CMD_XFER_RESUME = 0x33 {
///     u64 offset;
/// }
/// ```
pub struct CMD_XFER_RESUME {
    pub offset: u64,
}

impl ClientMessage for CMD_XFER_RESUME {
    const OPCODE: u8 = 0x33;
}
impl ReadableAndWritable for CMD_XFER_RESUME {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // offset: u64
        let offset = crate::util::read_u64_le(r)?;

        Ok(Self {
            offset,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // offset: u64
        w.write_all(&self.offset.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for CMD_XFER_RESUME {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMD_XFER_RESUME {
    fn maximum_possible_size() -> usize {
        8 // offset: u64
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_XFER_RESUME;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 41.
    #[test]
    fn CMD_XFER_RESUME0() {
        let raw: Vec<u8> = vec![ 0x33, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(CMD_XFER_RESUME::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
