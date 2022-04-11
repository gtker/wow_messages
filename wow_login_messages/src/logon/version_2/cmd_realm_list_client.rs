use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/client.wowm#L3):
/// ```text
/// clogin CMD_REALM_LIST_Client = 0x10 {
///     u32 padding = 0;
/// }
/// ```
pub struct CMD_REALM_LIST_Client {
}

impl ClientMessage for CMD_REALM_LIST_Client {
    const OPCODE: u8 = 0x10;
}
impl CMD_REALM_LIST_Client {
    /// The field `padding` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const PADDING_VALUE: u32 = 0x00;

}

impl ReadableAndWritable for CMD_REALM_LIST_Client {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // padding: u32
        let _padding = crate::util::read_u32_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // padding: u32
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for CMD_REALM_LIST_Client {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMD_REALM_LIST_Client {
    fn maximum_possible_size() -> usize {
        4 // padding: u32
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_REALM_LIST_Client;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ClientOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_realm/client.wowm` line 7.
    #[test]
    fn CMD_REALM_LIST_Client0() {
        let raw: Vec<u8> = vec![ 0x10, 0x00, 0x00, 0x00, 0x00, ];

        let expected = CMD_REALM_LIST_Client {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_REALM_LIST_Client::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
