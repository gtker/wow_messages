use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::{LoginResult, LoginResultError};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm:25`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm#L25):
/// ```text
/// slogin CMD_AUTH_RECONNECT_PROOF_Server = 0x3 {
///     LoginResult result;
///     u16 padding = 0;
/// }
/// ```
pub struct CMD_AUTH_RECONNECT_PROOF_Server {
    pub result: LoginResult,
}

impl ServerMessage for CMD_AUTH_RECONNECT_PROOF_Server {
    const OPCODE: u8 = 0x03;
}
impl CMD_AUTH_RECONNECT_PROOF_Server {
    /// The field `padding` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const PADDING_VALUE: u16 = 0x00;

}

impl ReadableAndWritable for CMD_AUTH_RECONNECT_PROOF_Server {
    type Error = CMD_AUTH_RECONNECT_PROOF_ServerError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // result: LoginResult
        let result = LoginResult::read(r)?;

        // padding: u16
        let _padding = crate::util::read_u16_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            result,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        self.result.write(w)?;

        // padding: u16
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for CMD_AUTH_RECONNECT_PROOF_Server {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_PROOF_Server {
    fn maximum_possible_size() -> usize {
        LoginResult::size() // result: LoginResult
        + 2 // padding: u16
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_RECONNECT_PROOF_ServerError {
    Io(std::io::Error),
    LoginResult(LoginResultError),
}

impl std::error::Error for CMD_AUTH_RECONNECT_PROOF_ServerError {}
impl std::fmt::Display for CMD_AUTH_RECONNECT_PROOF_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LoginResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_RECONNECT_PROOF_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LoginResultError> for CMD_AUTH_RECONNECT_PROOF_ServerError {
    fn from(e: LoginResultError) -> Self {
        Self::LoginResult(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_RECONNECT_PROOF_Server;
    use crate::ConstantSized;
    use crate::logon::version_8::LoginResult;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 32.
    #[test]
    fn CMD_AUTH_RECONNECT_PROOF_Server0() {
        let raw: Vec<u8> = vec![ 0x03, 0x00, 0x00, 0x00, ];

        let expected = CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::SUCCESS,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(CMD_AUTH_RECONNECT_PROOF_Server::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
