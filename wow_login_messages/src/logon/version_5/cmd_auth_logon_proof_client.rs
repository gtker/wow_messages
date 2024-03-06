use crate::Message;
use crate::ClientMessage;
use std::io::{Read, Write};

use crate::logon::version_2::TelemetryKey;
use crate::logon::version_5::SecurityFlag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;

        let mut s = String::new();

        writeln!(s, "test CMD_AUTH_LOGON_PROOF_Client {{").unwrap();
        // Members
        writeln!(s, "    client_public_key = [").unwrap();
        for v in self.client_public_key.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    client_proof = [").unwrap();
        for v in self.client_proof.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    crc_hash = [").unwrap();
        for v in self.crc_hash.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    number_of_telemetry_keys = {};", self.telemetry_keys.len()).unwrap();
        writeln!(s, "    telemetry_keys = [").unwrap();
        for v in self.telemetry_keys.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            unknown1 = {};", v.unknown1).unwrap();
            writeln!(s, "            unknown2 = {};", v.unknown2).unwrap();
            writeln!(s, "            unknown3 = [").unwrap();
            for v in v.unknown3.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "            ];").unwrap();
            writeln!(s, "            cd_key_proof = [").unwrap();
            for v in v.cd_key_proof.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "            ];").unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    security_flag = {};", SecurityFlag::new(self.security_flag.as_int()).as_test_case_value()).unwrap();
        if let Some(if_statement) = &self.security_flag.get_pin() {
            writeln!(s, "    pin_salt = [").unwrap();
            for v in if_statement.pin_salt.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "    ];").unwrap();
            writeln!(s, "    pin_hash = [").unwrap();
            for v in if_statement.pin_hash.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "    ];").unwrap();
        }

        if let Some(if_statement) = &self.security_flag.get_matrix_card() {
            writeln!(s, "    matrix_card_proof = [").unwrap();
            for v in if_statement.matrix_card_proof.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "    ];").unwrap();
        }


        writeln!(s, "}} [").unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    {:#04X}, /* opcode */ ", bytes.next().unwrap()).unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, self.client_public_key.len(), "client_public_key", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.client_proof.len(), "client_proof", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.crc_hash.len(), "crc_hash", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "number_of_telemetry_keys", "    ");
        if !self.telemetry_keys.is_empty() {
            writeln!(s, "    /* telemetry_keys: TelemetryKey[number_of_telemetry_keys] start */").unwrap();
            for (i, v) in self.telemetry_keys.iter().enumerate() {
                writeln!(s, "    /* telemetry_keys: TelemetryKey[number_of_telemetry_keys] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown1", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.unknown3.len(), "unknown3", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.cd_key_proof.len(), "cd_key_proof", "        ");
                writeln!(s, "    /* telemetry_keys: TelemetryKey[number_of_telemetry_keys] {i} end */").unwrap();
            }
            writeln!(s, "    /* telemetry_keys: TelemetryKey[number_of_telemetry_keys] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 1, "security_flag", "    ");
        if let Some(if_statement) = &self.security_flag.get_pin() {
            crate::util::write_bytes(&mut s, &mut bytes, if_statement.pin_salt.len(), "pin_salt", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, if_statement.pin_hash.len(), "pin_hash", "    ");
        }

        if let Some(if_statement) = &self.security_flag.get_matrix_card() {
            crate::util::write_bytes(&mut s, &mut bytes, if_statement.matrix_card_proof.len(), "matrix_card_proof", "    ");
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    login_versions = \"{}\";", std::env::var("WOWM_TEST_CASE_LOGIN_VERSION").unwrap_or("5 6 7".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

