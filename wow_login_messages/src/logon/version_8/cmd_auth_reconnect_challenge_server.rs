use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::{LoginResult, LoginResultError};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm:34`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm#L34):
/// ```text
/// slogin CMD_AUTH_RECONNECT_CHALLENGE_Server = 0x2 {
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[16] challenge_data;
///         u8[16] checksum_salt;
///     }
/// }
/// ```
pub struct CMD_AUTH_RECONNECT_CHALLENGE_Server {
    pub result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult,
}

impl ServerMessage for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    const OPCODE: u8 = 0x02;
}
impl ReadableAndWritable for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    type Error = CMD_AUTH_RECONNECT_CHALLENGE_ServerError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // result: LoginResult
        let result = LoginResult::read(r)?;

        let result_if = match result {
            LoginResult::SUCCESS => {
                // challenge_data: u8[16]
                let mut challenge_data = [0_u8; 16];
                r.read_exact(&mut challenge_data)?;

                // checksum_salt: u8[16]
                let mut checksum_salt = [0_u8; 16];
                r.read_exact(&mut checksum_salt)?;

                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                    challenge_data,
                    checksum_salt,
                }
            }
            LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0,
            LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1,
            LoginResult::FAIL_BANNED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
            LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
            LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
            LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE,
            LoginResult::FAIL_NO_TIME => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME,
            LoginResult::FAIL_DB_BUSY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY,
            LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID,
            LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
            LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER,
            LoginResult::FAIL_SUSPENDED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED,
            LoginResult::FAIL_NO_ACCESS => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS,
            LoginResult::SUCCESS_SURVEY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY,
            LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL,
            LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED,
        };

        Ok(Self {
            result: result_if,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        self.result.write(w)?;

        match &self.result {
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                challenge_data,
                checksum_salt,
            } => {
                // challenge_data: u8[16]
                for i in challenge_data.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // checksum_salt: u8[16]
                for i in checksum_salt.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0 => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1 => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED => {}
        }

        Ok(())
    }

}

impl VariableSized for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    fn size(&self) -> usize {
        self.result.size() // result: LoginResult and subfields
    }
}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    fn maximum_possible_size() -> usize {
        LoginResult::maximum_possible_size() // result: LoginResult
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    Io(std::io::Error),
    LoginResult(LoginResultError),
}

impl std::error::Error for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {}
impl std::fmt::Display for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LoginResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LoginResultError> for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    fn from(e: LoginResultError) -> Self {
        Self::LoginResult(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    SUCCESS {
        challenge_data: [u8; 16],
        checksum_salt: [u8; 16],
    },
    FAIL_UNKNOWN0,
    FAIL_UNKNOWN1,
    FAIL_BANNED,
    FAIL_UNKNOWN_ACCOUNT,
    FAIL_INCORRECT_PASSWORD,
    FAIL_ALREADY_ONLINE,
    FAIL_NO_TIME,
    FAIL_DB_BUSY,
    FAIL_VERSION_INVALID,
    LOGIN_DOWNLOAD_FILE,
    FAIL_INVALID_SERVER,
    FAIL_SUSPENDED,
    FAIL_NO_ACCESS,
    SUCCESS_SURVEY,
    FAIL_PARENTALCONTROL,
    FAIL_LOCKED_ENFORCED,
}

impl From<&LoginResult> for CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    fn from(e: &LoginResult) -> Self {
        match &e {
            LoginResult::SUCCESS => Self::SUCCESS {
                challenge_data: Default::default(),
                checksum_salt: Default::default(),
            },
            LoginResult::FAIL_UNKNOWN0 => Self::FAIL_UNKNOWN0,
            LoginResult::FAIL_UNKNOWN1 => Self::FAIL_UNKNOWN1,
            LoginResult::FAIL_BANNED => Self::FAIL_BANNED,
            LoginResult::FAIL_UNKNOWN_ACCOUNT => Self::FAIL_UNKNOWN_ACCOUNT,
            LoginResult::FAIL_INCORRECT_PASSWORD => Self::FAIL_INCORRECT_PASSWORD,
            LoginResult::FAIL_ALREADY_ONLINE => Self::FAIL_ALREADY_ONLINE,
            LoginResult::FAIL_NO_TIME => Self::FAIL_NO_TIME,
            LoginResult::FAIL_DB_BUSY => Self::FAIL_DB_BUSY,
            LoginResult::FAIL_VERSION_INVALID => Self::FAIL_VERSION_INVALID,
            LoginResult::LOGIN_DOWNLOAD_FILE => Self::LOGIN_DOWNLOAD_FILE,
            LoginResult::FAIL_INVALID_SERVER => Self::FAIL_INVALID_SERVER,
            LoginResult::FAIL_SUSPENDED => Self::FAIL_SUSPENDED,
            LoginResult::FAIL_NO_ACCESS => Self::FAIL_NO_ACCESS,
            LoginResult::SUCCESS_SURVEY => Self::SUCCESS_SURVEY,
            LoginResult::FAIL_PARENTALCONTROL => Self::FAIL_PARENTALCONTROL,
            LoginResult::FAIL_LOCKED_ENFORCED => Self::FAIL_LOCKED_ENFORCED,
        }
    }
}

impl From<&CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult> for LoginResult {
    fn from(v: &CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult) -> Self {
        match &v {
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS { .. } => Self::SUCCESS,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0 => Self::FAIL_UNKNOWN0,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1 => Self::FAIL_UNKNOWN1,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED => Self::FAIL_BANNED,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => Self::FAIL_UNKNOWN_ACCOUNT,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD => Self::FAIL_INCORRECT_PASSWORD,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE => Self::FAIL_ALREADY_ONLINE,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME => Self::FAIL_NO_TIME,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY => Self::FAIL_DB_BUSY,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID => Self::FAIL_VERSION_INVALID,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE => Self::LOGIN_DOWNLOAD_FILE,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER => Self::FAIL_INVALID_SERVER,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED => Self::FAIL_SUSPENDED,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS => Self::FAIL_NO_ACCESS,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY => Self::SUCCESS_SURVEY,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL => Self::FAIL_PARENTALCONTROL,
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED => Self::FAIL_LOCKED_ENFORCED,
        }
    }
}

impl Default for CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            challenge_data: Default::default(),
            checksum_salt: Default::default(),
        }
    }
}

impl CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    fn size(&self) -> usize {
        match self {
            Self::SUCCESS  {
                challenge_data,
                checksum_salt,
            } => {
                1
                + 16 * core::mem::size_of::<u8>() // challenge_data: u8[16]
                + 16 * core::mem::size_of::<u8>() // checksum_salt: u8[16]
            }
            Self::FAIL_UNKNOWN0 =>  {
                1
            }
            Self::FAIL_UNKNOWN1 =>  {
                1
            }
            Self::FAIL_BANNED =>  {
                1
            }
            Self::FAIL_UNKNOWN_ACCOUNT =>  {
                1
            }
            Self::FAIL_INCORRECT_PASSWORD =>  {
                1
            }
            Self::FAIL_ALREADY_ONLINE =>  {
                1
            }
            Self::FAIL_NO_TIME =>  {
                1
            }
            Self::FAIL_DB_BUSY =>  {
                1
            }
            Self::FAIL_VERSION_INVALID =>  {
                1
            }
            Self::LOGIN_DOWNLOAD_FILE =>  {
                1
            }
            Self::FAIL_INVALID_SERVER =>  {
                1
            }
            Self::FAIL_SUSPENDED =>  {
                1
            }
            Self::FAIL_NO_ACCESS =>  {
                1
            }
            Self::SUCCESS_SURVEY =>  {
                1
            }
            Self::FAIL_PARENTALCONTROL =>  {
                1
            }
            Self::FAIL_LOCKED_ENFORCED =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_RECONNECT_CHALLENGE_Server;
    use crate::VariableSized;
    use crate::logon::version_8::LoginResult;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 44.
    #[test]
    fn CMD_AUTH_RECONNECT_CHALLENGE_Server0() {
        let raw: Vec<u8> = vec![ 0x02, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
             0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0xFF, 0xFE,
             0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2,
             0xF1, 0xF0, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                challenge_data: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                checksum_salt: [ 0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8,
                     0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1, 0xF0, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 57.
    #[test]
    fn CMD_AUTH_RECONNECT_CHALLENGE_Server1() {
        let raw: Vec<u8> = vec![ 0x02, 0x03, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
