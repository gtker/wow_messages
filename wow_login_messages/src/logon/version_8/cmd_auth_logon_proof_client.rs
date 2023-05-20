use crate::ClientMessage;
use std::io::{Read, Write};

use crate::logon::version_2::TelemetryKey;
use crate::logon::version_8::SecurityFlag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Reply after successful [`CMD_AUTH_LOGON_CHALLENGE_Server`](crate::logon::version_8::CMD_AUTH_LOGON_CHALLENGE_Server).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm:319`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm#L319):
/// ```text
/// clogin CMD_AUTH_LOGON_PROOF_Client = 0x01 {
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
///     if (security_flag & MATRIX_CARD) {
///         u8[20] matrix_card_proof;
///     }
///     if (security_flag & AUTHENTICATOR) {
///         u8 amount_of_tokens;
///         u8[amount_of_tokens] tokens;
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_PROOF_Client {
    pub client_public_key: [u8; 32],
    pub client_proof: [u8; 20],
    pub crc_hash: [u8; 20],
    pub telemetry_keys: Vec<TelemetryKey>,
    pub security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag,
}

impl CMD_AUTH_LOGON_PROOF_Client {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
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
            i.write_into_vec(&mut w)?;
        }

        // security_flag: SecurityFlag
        w.write_all(&(self.security_flag.as_int().to_le_bytes()))?;

        if let Some(if_statement) = &self.security_flag.pin {
            // pin_salt: u8[16]
            for i in if_statement.pin_salt.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

            // pin_hash: u8[20]
            for i in if_statement.pin_hash.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        if let Some(if_statement) = &self.security_flag.matrix_card {
            // matrix_card_proof: u8[20]
            for i in if_statement.matrix_card_proof.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        if let Some(if_statement) = &self.security_flag.authenticator {
            // amount_of_tokens: u8
            w.write_all(&(if_statement.tokens.len() as u8).to_le_bytes())?;

            // tokens: u8[amount_of_tokens]
            for i in if_statement.tokens.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_LOGON_PROOF_Client {}

impl ClientMessage for CMD_AUTH_LOGON_PROOF_Client {
    const OPCODE: u8 = 0x01;

    fn read<R: std::io::Read, I: crate::private::Sealed>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // client_public_key: u8[32]
        let client_public_key = {
            let mut client_public_key = [0_u8; 32];
            r.read_exact(&mut client_public_key)?;
            client_public_key
        };

        // client_proof: u8[20]
        let client_proof = {
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof)?;
            client_proof
        };

        // crc_hash: u8[20]
        let crc_hash = {
            let mut crc_hash = [0_u8; 20];
            r.read_exact(&mut crc_hash)?;
            crc_hash
        };

        // number_of_telemetry_keys: u8
        let number_of_telemetry_keys = crate::util::read_u8_le(&mut r)?;

        // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        let telemetry_keys = {
            let mut telemetry_keys = Vec::with_capacity(number_of_telemetry_keys as usize);
            for i in 0..number_of_telemetry_keys {
                telemetry_keys.push(TelemetryKey::read(&mut r)?);
            }
            telemetry_keys
        };

        // security_flag: SecurityFlag
        let security_flag = SecurityFlag::new(crate::util::read_u8_le(&mut r)?);

        let security_flag_pin = if security_flag.is_pin() {
            // pin_salt: u8[16]
            let pin_salt = {
                let mut pin_salt = [0_u8; 16];
                r.read_exact(&mut pin_salt)?;
                pin_salt
            };

            // pin_hash: u8[20]
            let pin_hash = {
                let mut pin_hash = [0_u8; 20];
                r.read_exact(&mut pin_hash)?;
                pin_hash
            };

            Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                pin_hash,
                pin_salt,
            })
        }
        else {
            None
        };

        let security_flag_matrix_card = if security_flag.is_matrix_card() {
            // matrix_card_proof: u8[20]
            let matrix_card_proof = {
                let mut matrix_card_proof = [0_u8; 20];
                r.read_exact(&mut matrix_card_proof)?;
                matrix_card_proof
            };

            Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
                matrix_card_proof,
            })
        }
        else {
            None
        };

        let security_flag_authenticator = if security_flag.is_authenticator() {
            // amount_of_tokens: u8
            let amount_of_tokens = crate::util::read_u8_le(&mut r)?;

            // tokens: u8[amount_of_tokens]
            let tokens = {
                let mut tokens = Vec::with_capacity(amount_of_tokens as usize);
                for i in 0..amount_of_tokens {
                    tokens.push(crate::util::read_u8_le(&mut r)?);
                }
                tokens
            };

            Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                tokens,
            })
        }
        else {
            None
        };

        let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
            inner: security_flag.as_int(),
            pin: security_flag_pin,
            matrix_card: security_flag_matrix_card,
            authenticator: security_flag_authenticator,
        };

        Ok(Self {
            client_public_key,
            client_proof,
            crc_hash,
            telemetry_keys,
            security_flag,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'async_trait, R, I: crate::private::Sealed>(
        mut r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // client_public_key: u8[32]
            let client_public_key = {
                let mut client_public_key = [0_u8; 32];
                r.read_exact(&mut client_public_key).await?;
                client_public_key
            };

            // client_proof: u8[20]
            let client_proof = {
                let mut client_proof = [0_u8; 20];
                r.read_exact(&mut client_proof).await?;
                client_proof
            };

            // crc_hash: u8[20]
            let crc_hash = {
                let mut crc_hash = [0_u8; 20];
                r.read_exact(&mut crc_hash).await?;
                crc_hash
            };

            // number_of_telemetry_keys: u8
            let number_of_telemetry_keys = crate::util::tokio_read_u8_le(&mut r).await?;

            // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
            let telemetry_keys = {
                let mut telemetry_keys = Vec::with_capacity(number_of_telemetry_keys as usize);
                for i in 0..number_of_telemetry_keys {
                    telemetry_keys.push(TelemetryKey::tokio_read(&mut r).await?);
                }
                telemetry_keys
            };

            // security_flag: SecurityFlag
            let security_flag = SecurityFlag::new(crate::util::tokio_read_u8_le(&mut r).await?);

            let security_flag_pin = if security_flag.is_pin() {
                // pin_salt: u8[16]
                let pin_salt = {
                    let mut pin_salt = [0_u8; 16];
                    r.read_exact(&mut pin_salt).await?;
                    pin_salt
                };

                // pin_hash: u8[20]
                let pin_hash = {
                    let mut pin_hash = [0_u8; 20];
                    r.read_exact(&mut pin_hash).await?;
                    pin_hash
                };

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash,
                    pin_salt,
                })
            }
            else {
                None
            };

            let security_flag_matrix_card = if security_flag.is_matrix_card() {
                // matrix_card_proof: u8[20]
                let matrix_card_proof = {
                    let mut matrix_card_proof = [0_u8; 20];
                    r.read_exact(&mut matrix_card_proof).await?;
                    matrix_card_proof
                };

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
                    matrix_card_proof,
                })
            }
            else {
                None
            };

            let security_flag_authenticator = if security_flag.is_authenticator() {
                // amount_of_tokens: u8
                let amount_of_tokens = crate::util::tokio_read_u8_le(&mut r).await?;

                // tokens: u8[amount_of_tokens]
                let tokens = {
                    let mut tokens = Vec::with_capacity(amount_of_tokens as usize);
                    for i in 0..amount_of_tokens {
                        tokens.push(crate::util::tokio_read_u8_le(&mut r).await?);
                    }
                    tokens
                };

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                    tokens,
                })
            }
            else {
                None
            };

            let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
                inner: security_flag.as_int(),
                pin: security_flag_pin,
                matrix_card: security_flag_matrix_card,
                authenticator: security_flag_authenticator,
            };

            Ok(Self {
                client_public_key,
                client_proof,
                crc_hash,
                telemetry_keys,
                security_flag,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'async_trait, R, I: crate::private::Sealed>(
        mut r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // client_public_key: u8[32]
            let client_public_key = {
                let mut client_public_key = [0_u8; 32];
                r.read_exact(&mut client_public_key).await?;
                client_public_key
            };

            // client_proof: u8[20]
            let client_proof = {
                let mut client_proof = [0_u8; 20];
                r.read_exact(&mut client_proof).await?;
                client_proof
            };

            // crc_hash: u8[20]
            let crc_hash = {
                let mut crc_hash = [0_u8; 20];
                r.read_exact(&mut crc_hash).await?;
                crc_hash
            };

            // number_of_telemetry_keys: u8
            let number_of_telemetry_keys = crate::util::astd_read_u8_le(&mut r).await?;

            // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
            let telemetry_keys = {
                let mut telemetry_keys = Vec::with_capacity(number_of_telemetry_keys as usize);
                for i in 0..number_of_telemetry_keys {
                    telemetry_keys.push(TelemetryKey::astd_read(&mut r).await?);
                }
                telemetry_keys
            };

            // security_flag: SecurityFlag
            let security_flag = SecurityFlag::new(crate::util::astd_read_u8_le(&mut r).await?);

            let security_flag_pin = if security_flag.is_pin() {
                // pin_salt: u8[16]
                let pin_salt = {
                    let mut pin_salt = [0_u8; 16];
                    r.read_exact(&mut pin_salt).await?;
                    pin_salt
                };

                // pin_hash: u8[20]
                let pin_hash = {
                    let mut pin_hash = [0_u8; 20];
                    r.read_exact(&mut pin_hash).await?;
                    pin_hash
                };

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash,
                    pin_salt,
                })
            }
            else {
                None
            };

            let security_flag_matrix_card = if security_flag.is_matrix_card() {
                // matrix_card_proof: u8[20]
                let matrix_card_proof = {
                    let mut matrix_card_proof = [0_u8; 20];
                    r.read_exact(&mut matrix_card_proof).await?;
                    matrix_card_proof
                };

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
                    matrix_card_proof,
                })
            }
            else {
                None
            };

            let security_flag_authenticator = if security_flag.is_authenticator() {
                // amount_of_tokens: u8
                let amount_of_tokens = crate::util::astd_read_u8_le(&mut r).await?;

                // tokens: u8[amount_of_tokens]
                let tokens = {
                    let mut tokens = Vec::with_capacity(amount_of_tokens as usize);
                    for i in 0..amount_of_tokens {
                        tokens.push(crate::util::astd_read_u8_le(&mut r).await?);
                    }
                    tokens
                };

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                    tokens,
                })
            }
            else {
                None
            };

            let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
                inner: security_flag.as_int(),
                pin: security_flag_pin,
                matrix_card: security_flag_matrix_card,
                authenticator: security_flag_authenticator,
            };

            Ok(Self {
                client_public_key,
                client_proof,
                crc_hash,
                telemetry_keys,
                security_flag,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_AUTH_LOGON_PROOF_Client {
    pub(crate) fn size(&self) -> usize {
        32 // client_public_key: u8[32]
        + 20 // client_proof: u8[20]
        + 20 // crc_hash: u8[20]
        + 1 // number_of_telemetry_keys: u8
        + self.telemetry_keys.len() * 30 // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        + self.security_flag.size() // security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin>,
    matrix_card: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard>,
    authenticator: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator>,
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
    pub const fn new(inner: u8, pin: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin>,matrix_card: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard>,authenticator: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator>,) -> Self {
        Self {
            inner,
            pin, 
            matrix_card, 
            authenticator, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            pin: None,
            matrix_card: None,
            authenticator: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.pin.is_none()
        && self.matrix_card.is_none()
        && self.authenticator.is_none()
    }

    pub const fn new_pin(pin: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            matrix_card: None,
            authenticator: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pin(mut self, pin: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin) -> Self {
        self.inner |= SecurityFlag::PIN;
        self.pin = Some(pin);
        self
    }

    pub const fn get_pin(&self) -> Option<&CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin> {
        self.pin.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pin(mut self) -> Self {
        self.inner &= SecurityFlag::PIN.reverse_bits();
        self.pin = None;
        self
    }

    pub const fn new_matrix_card(matrix_card: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard) -> Self {
        Self {
            inner: SecurityFlag::MATRIX_CARD,
            pin: None,
            matrix_card: Some(matrix_card),
            authenticator: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_matrix_card(mut self, matrix_card: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard) -> Self {
        self.inner |= SecurityFlag::MATRIX_CARD;
        self.matrix_card = Some(matrix_card);
        self
    }

    pub const fn get_matrix_card(&self) -> Option<&CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard> {
        self.matrix_card.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_matrix_card(mut self) -> Self {
        self.inner &= SecurityFlag::MATRIX_CARD.reverse_bits();
        self.matrix_card = None;
        self
    }

    pub const fn new_authenticator(authenticator: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator) -> Self {
        Self {
            inner: SecurityFlag::AUTHENTICATOR,
            pin: None,
            matrix_card: None,
            authenticator: Some(authenticator),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_authenticator(mut self, authenticator: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator) -> Self {
        self.inner |= SecurityFlag::AUTHENTICATOR;
        self.authenticator = Some(authenticator);
        self
    }

    pub const fn get_authenticator(&self) -> Option<&CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator> {
        self.authenticator.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_authenticator(mut self) -> Self {
        self.inner &= SecurityFlag::AUTHENTICATOR.reverse_bits();
        self.authenticator = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
    pub(crate) fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.pin {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.matrix_card {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
    pub pin_hash: [u8; 20],
    pub pin_salt: [u8; 16],
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
    pub(crate) const fn size(&self) -> usize {
        20 // pin_hash: u8[20]
        + 16 // pin_salt: u8[16]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
    pub matrix_card_proof: [u8; 20],
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
    pub(crate) const fn size(&self) -> usize {
        20 // matrix_card_proof: u8[20]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
    pub tokens: Vec<u8>,
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_tokens: u8
        + self.tokens.len() * core::mem::size_of::<u8>() // tokens: u8[amount_of_tokens]
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_LOGON_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_LOGON_PROOF_Client, expected: &CMD_AUTH_LOGON_PROOF_Client) {
        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);
    }

    const RAW0: [u8; 135] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x02, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x01,
         0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xFE,
         0x00, 0xEE, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x01, 0x02, 0x03,
         0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
         0x10, 0x11, 0x12, 0x13, 0x14, 0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_PROOF_Client {
        CMD_AUTH_LOGON_PROOF_Client {
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
                    cd_key_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
                TelemetryKey {
                    unknown1: 0xFE,
                    unknown2: 0xDEADBEEE,
                    unknown3: [ 0x00, 0x01, 0x02, 0x03, ],
                    cd_key_proof: [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
                         0x13, 0x14, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 348.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 348.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 348.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 105] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x01, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x01,
         0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x00, ];

    pub(crate) fn expected1() -> CMD_AUTH_LOGON_PROOF_Client {
        CMD_AUTH_LOGON_PROOF_Client {
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
                    cd_key_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 405.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 405.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 405.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 75] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x00, 0x00, ];

    pub(crate) fn expected2() -> CMD_AUTH_LOGON_PROOF_Client {
        CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 447.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client2() {
        let expected = expected2();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 447.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client2() {
        let expected = expected2();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 447.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client2() {
        let expected = expected2();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 111] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x00, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
         0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01,
         0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
         0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, ];

    pub(crate) fn expected3() -> CMD_AUTH_LOGON_PROOF_Client {
        CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_pin(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
                         0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                ,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 473.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client3() {
        let expected = expected3();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 473.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client3() {
        let expected = expected3();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 473.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client3() {
        let expected = expected3();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

}

