use crate::Message;
use crate::ClientMessage;
use std::io::{Read, Write};

use crate::logon::version_2::TelemetryKey;
use crate::logon::version_5::SecurityFlag;

/// Reply after successful [`CMD_AUTH_LOGON_CHALLENGE_Server`](crate::logon::version_5::CMD_AUTH_LOGON_CHALLENGE_Server).
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
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client {
    pub client_public_key: [u8; 32],
    pub client_proof: [u8; 20],
    pub crc_hash: [u8; 20],
    pub telemetry_keys: Vec<TelemetryKey>,
    pub security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag,
}

impl CMD_AUTH_LOGON_PROOF_Client {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_LOGON_PROOF_Client {}

impl CMD_AUTH_LOGON_PROOF_Client {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
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
            for _ in 0..number_of_telemetry_keys {
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

        let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
            inner: security_flag.as_int(),
            pin: security_flag_pin,
            matrix_card: security_flag_matrix_card,
        };

        Ok(Self {
            client_public_key,
            client_proof,
            crc_hash,
            telemetry_keys,
            security_flag,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
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
            for _ in 0..number_of_telemetry_keys {
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

        let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
            inner: security_flag.as_int(),
            pin: security_flag_pin,
            matrix_card: security_flag_matrix_card,
        };

        Ok(Self {
            client_public_key,
            client_proof,
            crc_hash,
            telemetry_keys,
            security_flag,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
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
            for _ in 0..number_of_telemetry_keys {
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

        let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
            inner: security_flag.as_int(),
            pin: security_flag_pin,
            matrix_card: security_flag_matrix_card,
        };

        Ok(Self {
            client_public_key,
            client_proof,
            crc_hash,
            telemetry_keys,
            security_flag,
        })
    }

}

impl Message for CMD_AUTH_LOGON_PROOF_Client {
    const OPCODE: u8 = 0x01;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(1, "CMD_AUTH_LOGON_PROOF_Client", kind))
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'async_trait, R, I: crate::private::Sealed>(
        r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(1, "CMD_AUTH_LOGON_PROOF_Client", kind))})
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<(), std::io::Error>>
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
        r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(1, "CMD_AUTH_LOGON_PROOF_Client", kind))})
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<(), std::io::Error>>
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

impl ClientMessage for CMD_AUTH_LOGON_PROOF_Client {}
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin>,
    matrix_card: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard>,
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
    pub const fn new(inner: u8, pin: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin>,matrix_card: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard>,) -> Self {
        Self {
            inner,
            pin, 
            matrix_card, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            pin: None,
            matrix_card: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.pin.is_none()
        && self.matrix_card.is_none()
    }

    pub const fn new_pin(pin: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            matrix_card: None,
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

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
    pub(crate) const fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.pin {
                36
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.matrix_card {
                20
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
    pub matrix_card_proof: [u8; 20],
}

#[cfg(test)]
mod test_version_5 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_LOGON_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_5::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 131] = [ 0x01, 0x04, 0x49, 0x57, 0xDD, 0x20, 0x51, 0x62, 0xF5,
         0xFA, 0xFE, 0xB3, 0x67, 0x07, 0x72, 0x09, 0x51, 0x56, 0x20, 0x08, 0x08,
         0x20, 0xC1, 0x26, 0xCA, 0xC8, 0xF7, 0x3B, 0x46, 0xFB, 0x88, 0x32, 0x06,
         0x82, 0xC9, 0x97, 0x60, 0x42, 0xE4, 0x75, 0xF9, 0x7C, 0x60, 0x62, 0xE4,
         0x54, 0x66, 0xA6, 0xFE, 0xDC, 0xE9, 0xAA, 0x7C, 0xFE, 0x74, 0xDA, 0x70,
         0x88, 0xCC, 0x76, 0x24, 0xC4, 0x28, 0x88, 0xB5, 0xEF, 0xC4, 0x1D, 0xB4,
         0x6B, 0xC5, 0x2C, 0xFB, 0x00, 0x03, 0xDD, 0x69, 0xF0, 0xF7, 0x58, 0x4C,
         0x58, 0xF0, 0x86, 0x36, 0x3A, 0x1A, 0xBE, 0x6E, 0x1E, 0x4D, 0x5A, 0x4E,
         0xC0, 0x56, 0x58, 0x88, 0xE6, 0x29, 0x01, 0x6C, 0xBF, 0x3D, 0xF7, 0x8E,
         0x82, 0x93, 0x6F, 0x1D, 0xBE, 0xE5, 0x69, 0x34, 0xCD, 0x08, 0x82, 0x94,
         0xEF, 0x5D, 0x0F, 0x96, 0x9F, 0xFC, 0x17, 0x0B, 0xE4, 0x42, 0x08, 0x2E,
         0xD1, 0x10, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_PROOF_Client {
        CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0x04, 0x49, 0x57, 0xDD, 0x20, 0x51, 0x62, 0xF5,
                 0xFA, 0xFE, 0xB3, 0x67, 0x07, 0x72, 0x09, 0x51, 0x56, 0x20, 0x08,
                 0x08, 0x20, 0xC1, 0x26, 0xCA, 0xC8, 0xF7, 0x3B, 0x46, 0xFB, 0x88,
                 0x32, 0x06, ],
            client_proof: [ 0x82, 0xC9, 0x97, 0x60, 0x42, 0xE4, 0x75, 0xF9, 0x7C,
                 0x60, 0x62, 0xE4, 0x54, 0x66, 0xA6, 0xFE, 0xDC, 0xE9, 0xAA, 0x7C, ],
            crc_hash: [ 0xFE, 0x74, 0xDA, 0x70, 0x88, 0xCC, 0x76, 0x24, 0xC4, 0x28,
                 0x88, 0xB5, 0xEF, 0xC4, 0x1D, 0xB4, 0x6B, 0xC5, 0x2C, 0xFB, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_pin(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x5A, 0x4E, 0xC0, 0x56, 0x58, 0x88, 0xE6, 0x29, 0x01,
                         0x6C, 0xBF, 0x3D, 0xF7, 0x8E, 0x82, 0x93, 0x6F, 0x1D, 0xBE,
                         0xE5, ],
                    pin_salt: [ 0xDD, 0x69, 0xF0, 0xF7, 0x58, 0x4C, 0x58, 0xF0, 0x86,
                         0x36, 0x3A, 0x1A, 0xBE, 0x6E, 0x1E, 0x4D, ],
                })
                .set_matrix_card(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
                    matrix_card_proof: [ 0x69, 0x34, 0xCD, 0x08, 0x82, 0x94, 0xEF,
                         0x5D, 0x0F, 0x96, 0x9F, 0xFC, 0x17, 0x0B, 0xE4, 0x42, 0x08,
                         0x2E, 0xD1, 0x10, ],
                })
                ,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_6 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_LOGON_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_6::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 131] = [ 0x01, 0x04, 0x49, 0x57, 0xDD, 0x20, 0x51, 0x62, 0xF5,
         0xFA, 0xFE, 0xB3, 0x67, 0x07, 0x72, 0x09, 0x51, 0x56, 0x20, 0x08, 0x08,
         0x20, 0xC1, 0x26, 0xCA, 0xC8, 0xF7, 0x3B, 0x46, 0xFB, 0x88, 0x32, 0x06,
         0x82, 0xC9, 0x97, 0x60, 0x42, 0xE4, 0x75, 0xF9, 0x7C, 0x60, 0x62, 0xE4,
         0x54, 0x66, 0xA6, 0xFE, 0xDC, 0xE9, 0xAA, 0x7C, 0xFE, 0x74, 0xDA, 0x70,
         0x88, 0xCC, 0x76, 0x24, 0xC4, 0x28, 0x88, 0xB5, 0xEF, 0xC4, 0x1D, 0xB4,
         0x6B, 0xC5, 0x2C, 0xFB, 0x00, 0x03, 0xDD, 0x69, 0xF0, 0xF7, 0x58, 0x4C,
         0x58, 0xF0, 0x86, 0x36, 0x3A, 0x1A, 0xBE, 0x6E, 0x1E, 0x4D, 0x5A, 0x4E,
         0xC0, 0x56, 0x58, 0x88, 0xE6, 0x29, 0x01, 0x6C, 0xBF, 0x3D, 0xF7, 0x8E,
         0x82, 0x93, 0x6F, 0x1D, 0xBE, 0xE5, 0x69, 0x34, 0xCD, 0x08, 0x82, 0x94,
         0xEF, 0x5D, 0x0F, 0x96, 0x9F, 0xFC, 0x17, 0x0B, 0xE4, 0x42, 0x08, 0x2E,
         0xD1, 0x10, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_PROOF_Client {
        CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0x04, 0x49, 0x57, 0xDD, 0x20, 0x51, 0x62, 0xF5,
                 0xFA, 0xFE, 0xB3, 0x67, 0x07, 0x72, 0x09, 0x51, 0x56, 0x20, 0x08,
                 0x08, 0x20, 0xC1, 0x26, 0xCA, 0xC8, 0xF7, 0x3B, 0x46, 0xFB, 0x88,
                 0x32, 0x06, ],
            client_proof: [ 0x82, 0xC9, 0x97, 0x60, 0x42, 0xE4, 0x75, 0xF9, 0x7C,
                 0x60, 0x62, 0xE4, 0x54, 0x66, 0xA6, 0xFE, 0xDC, 0xE9, 0xAA, 0x7C, ],
            crc_hash: [ 0xFE, 0x74, 0xDA, 0x70, 0x88, 0xCC, 0x76, 0x24, 0xC4, 0x28,
                 0x88, 0xB5, 0xEF, 0xC4, 0x1D, 0xB4, 0x6B, 0xC5, 0x2C, 0xFB, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_pin(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x5A, 0x4E, 0xC0, 0x56, 0x58, 0x88, 0xE6, 0x29, 0x01,
                         0x6C, 0xBF, 0x3D, 0xF7, 0x8E, 0x82, 0x93, 0x6F, 0x1D, 0xBE,
                         0xE5, ],
                    pin_salt: [ 0xDD, 0x69, 0xF0, 0xF7, 0x58, 0x4C, 0x58, 0xF0, 0x86,
                         0x36, 0x3A, 0x1A, 0xBE, 0x6E, 0x1E, 0x4D, ],
                })
                .set_matrix_card(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
                    matrix_card_proof: [ 0x69, 0x34, 0xCD, 0x08, 0x82, 0x94, 0xEF,
                         0x5D, 0x0F, 0x96, 0x9F, 0xFC, 0x17, 0x0B, 0xE4, 0x42, 0x08,
                         0x2E, 0xD1, 0x10, ],
                })
                ,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_7 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_LOGON_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_7::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 131] = [ 0x01, 0x04, 0x49, 0x57, 0xDD, 0x20, 0x51, 0x62, 0xF5,
         0xFA, 0xFE, 0xB3, 0x67, 0x07, 0x72, 0x09, 0x51, 0x56, 0x20, 0x08, 0x08,
         0x20, 0xC1, 0x26, 0xCA, 0xC8, 0xF7, 0x3B, 0x46, 0xFB, 0x88, 0x32, 0x06,
         0x82, 0xC9, 0x97, 0x60, 0x42, 0xE4, 0x75, 0xF9, 0x7C, 0x60, 0x62, 0xE4,
         0x54, 0x66, 0xA6, 0xFE, 0xDC, 0xE9, 0xAA, 0x7C, 0xFE, 0x74, 0xDA, 0x70,
         0x88, 0xCC, 0x76, 0x24, 0xC4, 0x28, 0x88, 0xB5, 0xEF, 0xC4, 0x1D, 0xB4,
         0x6B, 0xC5, 0x2C, 0xFB, 0x00, 0x03, 0xDD, 0x69, 0xF0, 0xF7, 0x58, 0x4C,
         0x58, 0xF0, 0x86, 0x36, 0x3A, 0x1A, 0xBE, 0x6E, 0x1E, 0x4D, 0x5A, 0x4E,
         0xC0, 0x56, 0x58, 0x88, 0xE6, 0x29, 0x01, 0x6C, 0xBF, 0x3D, 0xF7, 0x8E,
         0x82, 0x93, 0x6F, 0x1D, 0xBE, 0xE5, 0x69, 0x34, 0xCD, 0x08, 0x82, 0x94,
         0xEF, 0x5D, 0x0F, 0x96, 0x9F, 0xFC, 0x17, 0x0B, 0xE4, 0x42, 0x08, 0x2E,
         0xD1, 0x10, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_PROOF_Client {
        CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0x04, 0x49, 0x57, 0xDD, 0x20, 0x51, 0x62, 0xF5,
                 0xFA, 0xFE, 0xB3, 0x67, 0x07, 0x72, 0x09, 0x51, 0x56, 0x20, 0x08,
                 0x08, 0x20, 0xC1, 0x26, 0xCA, 0xC8, 0xF7, 0x3B, 0x46, 0xFB, 0x88,
                 0x32, 0x06, ],
            client_proof: [ 0x82, 0xC9, 0x97, 0x60, 0x42, 0xE4, 0x75, 0xF9, 0x7C,
                 0x60, 0x62, 0xE4, 0x54, 0x66, 0xA6, 0xFE, 0xDC, 0xE9, 0xAA, 0x7C, ],
            crc_hash: [ 0xFE, 0x74, 0xDA, 0x70, 0x88, 0xCC, 0x76, 0x24, 0xC4, 0x28,
                 0x88, 0xB5, 0xEF, 0xC4, 0x1D, 0xB4, 0x6B, 0xC5, 0x2C, 0xFB, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_pin(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x5A, 0x4E, 0xC0, 0x56, 0x58, 0x88, 0xE6, 0x29, 0x01,
                         0x6C, 0xBF, 0x3D, 0xF7, 0x8E, 0x82, 0x93, 0x6F, 0x1D, 0xBE,
                         0xE5, ],
                    pin_salt: [ 0xDD, 0x69, 0xF0, 0xF7, 0x58, 0x4C, 0x58, 0xF0, 0x86,
                         0x36, 0x3A, 0x1A, 0xBE, 0x6E, 0x1E, 0x4D, ],
                })
                .set_matrix_card(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_MatrixCard {
                    matrix_card_proof: [ 0x69, 0x34, 0xCD, 0x08, 0x82, 0x94, 0xEF,
                         0x5D, 0x0F, 0x96, 0x9F, 0xFC, 0x17, 0x0B, 0xE4, 0x42, 0x08,
                         0x2E, 0xD1, 0x10, ],
                })
                ,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 340.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

