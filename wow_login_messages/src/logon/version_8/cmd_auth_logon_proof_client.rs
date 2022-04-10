use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::{SecurityFlag};
use crate::logon::version_2::TelemetryKey;
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm:315`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm#L315):
/// ```text
/// clogin CMD_AUTH_LOGON_PROOF_Client = 0x1 {
///     u8[32] client_public_key;
///     u8[20] client_proof;
///     u8[20] crc_hash;
///     u8 number_of_telemetry_keys;
///     TelemetryKey[number_of_telemetry_keys] telemetry_keys;
///     SecurityFlag security_flag;
///     if (security_flag & PIN) {
///         u8[16] pin_salt;
///         u8[20] pin_hash;
///     }
///     if (security_flag & UNKNOWN0) {
///         u8 unknown0;
///         u8 unknown1;
///         u8 unknown2;
///         u8 unknown3;
///         u64 unknown4;
///     }
///     if (security_flag & AUTHENTICATOR) {
///         u8 unknown5 = 1;
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_PROOF_Client {
    pub client_public_key: [u8; 32],
    pub client_proof: [u8; 20],
    pub crc_hash: [u8; 20],
    pub telemetry_keys: Vec<TelemetryKey>,
    pub security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag,
}

impl ClientMessage for CMD_AUTH_LOGON_PROOF_Client {
    const OPCODE: u8 = 0x01;
}
impl CMD_AUTH_LOGON_PROOF_Client {
    /// The field `unknown5` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `1` |
    /// | Hex | `0x01` |
    /// | Original | `1` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const UNKNOWN5_VALUE: u8 = 0x01;

}

impl ReadableAndWritable for CMD_AUTH_LOGON_PROOF_Client {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // client_public_key: u8[32]
        let mut client_public_key = [0_u8; 32];
        r.read_exact(&mut client_public_key)?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof)?;

        // crc_hash: u8[20]
        let mut crc_hash = [0_u8; 20];
        r.read_exact(&mut crc_hash)?;

        // number_of_telemetry_keys: u8
        let number_of_telemetry_keys = crate::util::read_u8_le(r)?;

        // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        let mut telemetry_keys = Vec::with_capacity(number_of_telemetry_keys as usize);
        for i in 0..number_of_telemetry_keys {
            telemetry_keys.push(TelemetryKey::read(r)?);
        }

        // security_flag: SecurityFlag
        let security_flag = SecurityFlag::read(r)?;

        let security_flag_PIN = if security_flag.is_PIN() {
            // pin_salt: u8[16]
            let mut pin_salt = [0_u8; 16];
            r.read_exact(&mut pin_salt)?;

            // pin_hash: u8[20]
            let mut pin_hash = [0_u8; 20];
            r.read_exact(&mut pin_hash)?;

            Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
                pin_salt,
                pin_hash,
            })
        } else {
            None
        };

        let security_flag_UNKNOWN0 = if security_flag.is_UNKNOWN0() {
            // unknown0: u8
            let unknown0 = crate::util::read_u8_le(r)?;

            // unknown1: u8
            let unknown1 = crate::util::read_u8_le(r)?;

            // unknown2: u8
            let unknown2 = crate::util::read_u8_le(r)?;

            // unknown3: u8
            let unknown3 = crate::util::read_u8_le(r)?;

            // unknown4: u64
            let unknown4 = crate::util::read_u64_le(r)?;

            Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0 {
                unknown0,
                unknown1,
                unknown2,
                unknown3,
                unknown4,
            })
        } else {
            None
        };

        let security_flag_AUTHENTICATOR = if security_flag.is_AUTHENTICATOR() {
            // unknown5: u8
            let _unknown5 = crate::util::read_u8_le(r)?;
            // unknown5 is expected to always be 1 (1)

            Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR {
                unknown5: _unknown5,
            })
        } else {
            None
        };

        let security_flag = CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
            inner: security_flag.as_u8(),
            pin: security_flag_PIN,
            unknown0: security_flag_UNKNOWN0,
            authenticator: security_flag_AUTHENTICATOR,
        };

        Ok(Self {
            client_public_key,
            client_proof,
            crc_hash,
            telemetry_keys,
            security_flag,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // client_public_key: u8[32]
        for i in self.client_public_key.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // crc_hash: u8[20]
        for i in self.crc_hash.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // number_of_telemetry_keys: u8
        w.write_all(&(self.telemetry_keys.len() as u8).to_le_bytes())?;

        // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        for i in self.telemetry_keys.iter() {
            i.write(w)?;
        }

        // security_flag: SecurityFlag
        self.security_flag.write(w)?;

        if let Some(s) = &self.security_flag.pin {
            s.write(w)?;
        }

        if let Some(s) = &self.security_flag.unknown0 {
            s.write(w)?;
        }

        if let Some(s) = &self.security_flag.authenticator {
            s.write(w)?;
        }

        Ok(())
    }

}

impl VariableSized for CMD_AUTH_LOGON_PROOF_Client {
    fn size(&self) -> usize {
        32 * core::mem::size_of::<u8>() // client_public_key: u8[32]
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 20 * core::mem::size_of::<u8>() // crc_hash: u8[20]
        + 1 // number_of_telemetry_keys: u8
        + self.telemetry_keys.iter().fold(0, |acc, x| acc + TelemetryKey::size()) // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        + self.security_flag.size() // security_flag: SecurityFlag and subfields
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_Client {
    fn maximum_possible_size() -> usize {
        32 * core::mem::size_of::<u8>() // client_public_key: u8[32]
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 20 * core::mem::size_of::<u8>() // crc_hash: u8[20]
        + 1 // number_of_telemetry_keys: u8
        + 255 * TelemetryKey::maximum_possible_size() // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        + SecurityFlag::maximum_possible_size() // security_flag: SecurityFlag
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN>,
    unknown0: Option<CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0>,
    authenticator: Option<CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR>,
}

impl From<&CMD_AUTH_LOGON_PROOF_ClientSecurityFlag> for SecurityFlag {
    fn from(e: &CMD_AUTH_LOGON_PROOF_ClientSecurityFlag) -> Self {
        Self::new(e.inner)
    }
}

impl CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: SecurityFlag::NONE,
            pin: None,
            unknown0: None,
            authenticator: None,
        }
    }

    pub const fn new_PIN(pin: CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            unknown0: None,
            authenticator: None,
        }
    }

    pub const fn new_UNKNOWN0(unknown0: CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0) -> Self {
        Self {
            inner: SecurityFlag::UNKNOWN0,
            pin: None,
            unknown0: Some(unknown0),
            authenticator: None,
        }
    }

    pub const fn new_AUTHENTICATOR(authenticator: CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR) -> Self {
        Self {
            inner: SecurityFlag::AUTHENTICATOR,
            pin: None,
            unknown0: None,
            authenticator: Some(authenticator),
        }
    }

}
impl VariableSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    fn size(&self) -> usize {
        1 // inner: SecurityFlag (u8)
        + {
            if let Some(s) = &self.pin {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.unknown0 {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.authenticator {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    fn maximum_possible_size() -> usize {
        1 // inner: SecurityFlag (u8)
        + CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN::maximum_possible_size() // PIN enumerator
        + CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0::maximum_possible_size() // UNKNOWN0 enumerator
        + CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR::maximum_possible_size() // AUTHENTICATOR enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
    pub pin_salt: [u8; 16],
    pub pin_hash: [u8; 20],
}

impl VariableSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
    fn size(&self) -> usize {
        16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
        + 20 * core::mem::size_of::<u8>() // pin_hash: u8[20]
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
    fn maximum_possible_size() -> usize {
        16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
        + 20 * core::mem::size_of::<u8>() // pin_hash: u8[20]
    }
}

impl CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        for i in self.pin_salt.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        for i in self.pin_hash.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0 {
    pub unknown0: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
    pub unknown4: u64,
}

impl VariableSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0 {
    fn size(&self) -> usize {
        1 // unknown0: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
        + 8 // unknown4: u64
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0 {
    fn maximum_possible_size() -> usize {
        1 // unknown0: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
        + 8 // unknown4: u64
    }
}

impl CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0 {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.unknown0.to_le_bytes())?;

        w.write_all(&self.unknown1.to_le_bytes())?;

        w.write_all(&self.unknown2.to_le_bytes())?;

        w.write_all(&self.unknown3.to_le_bytes())?;

        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR {
    pub unknown5: u8,
}

impl VariableSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR {
    fn size(&self) -> usize {
        1 // unknown5: u8
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR {
    fn maximum_possible_size() -> usize {
        1 // unknown5: u8
    }
}

impl CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR {
    /// The field `unknown5` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `1` |
    /// | Hex | `0x01` |
    /// | Original | `1` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const UNKNOWN5_VALUE: u8 = 0x01;

    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&Self::UNKNOWN5_VALUE.to_le_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_LOGON_PROOF_Client;
    use crate::VariableSized;
    use crate::logon::version_8::SecurityFlag;
    use crate::logon::version_2::TelemetryKey;
    use super::CMD_AUTH_LOGON_PROOF_ClientSecurityFlag;
    use super::CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN;
    use super::CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0;
    use super::CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR;
    use crate::logon::version_8::opcodes::ClientOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 341.
    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client0() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x02, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x01, 0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
             0xFE, 0x00, 0xEE, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x01, 0x02,
             0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
             0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
                TelemetryKey {
                    unknown1: 0xFE,
                    unknown2: 0xDEADBEEE,
                    unknown3: [ 0x00, 0x01, 0x02, 0x03, ],
                    unknown4: [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
                         0x13, 0x14, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
                inner: 0 | SecurityFlag::NONE,
                pin: None,
                unknown0: None,
                authenticator: None,
            },
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 398.
    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client1() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x01, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x01, 0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
             0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
                inner: 0 | SecurityFlag::NONE,
                pin: None,
                unknown0: None,
                authenticator: None,
            },
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 440.
    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client2() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
                inner: 0 | SecurityFlag::NONE,
                pin: None,
                unknown0: None,
                authenticator: None,
            },
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 466.
    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client3() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x00, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00,
             0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
             0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, ];

        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
                inner: 0 | SecurityFlag::PIN,
                pin: Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                }),
                unknown0: None,
                authenticator: None,
            },
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 500.
    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client4() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x00, 0x03, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00,
             0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
             0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x10, 0x20, 0x30, 0x40, 0xEF,
             0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
                inner: 0 | SecurityFlag::PIN | SecurityFlag::UNKNOWN0,
                pin: Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                }),
                unknown0: Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                }),
                authenticator: None,
            },
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 544.
    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client5() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x00, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00,
             0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
             0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x10, 0x20, 0x30, 0x40, 0xEF,
             0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x01, ];

        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
                inner: 0 | SecurityFlag::PIN | SecurityFlag::UNKNOWN0 | SecurityFlag::AUTHENTICATOR,
                pin: Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagPIN {
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                }),
                unknown0: Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagUNKNOWN0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                }),
                authenticator: Some(CMD_AUTH_LOGON_PROOF_ClientSecurityFlagAUTHENTICATOR {
                    unknown5: 0x1,
                }),
            },
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
