use crate::ClientMessage;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Reply to [`CMD_AUTH_RECONNECT_CHALLENGE_Server`](crate::logon::version_2::CMD_AUTH_RECONNECT_CHALLENGE_Server).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm#L3):
/// ```text
/// clogin CMD_AUTH_RECONNECT_PROOF_Client = 0x03 {
///     u8[16] proof_data;
///     u8[20] client_proof;
///     u8[20] client_checksum;
///     u8 key_count = 0;
/// }
/// ```
pub struct CMD_AUTH_RECONNECT_PROOF_Client {
    pub proof_data: [u8; 16],
    pub client_proof: [u8; 20],
    pub client_checksum: [u8; 20],
}

impl CMD_AUTH_RECONNECT_PROOF_Client {
    /// The field `key_count` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const KEY_COUNT_VALUE: u8 = 0x00;

}

impl CMD_AUTH_RECONNECT_PROOF_Client {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // proof_data: u8[16]
        for i in self.proof_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // client_checksum: u8[20]
        for i in self.client_checksum.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // key_count: u8
        w.write_all(&Self::KEY_COUNT_VALUE.to_le_bytes())?;

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_RECONNECT_PROOF_Client {}

impl CMD_AUTH_RECONNECT_PROOF_Client {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // proof_data: u8[16]
        let proof_data = {
            let mut proof_data = [0_u8; 16];
            r.read_exact(&mut proof_data)?;
            proof_data
        };

        // client_proof: u8[20]
        let client_proof = {
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof)?;
            client_proof
        };

        // client_checksum: u8[20]
        let client_checksum = {
            let mut client_checksum = [0_u8; 20];
            r.read_exact(&mut client_checksum)?;
            client_checksum
        };

        // key_count: u8
        let _key_count = crate::util::read_u8_le(&mut r)?;
        // key_count is expected to always be 0 (0)

        Ok(Self {
            proof_data,
            client_proof,
            client_checksum,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // proof_data: u8[16]
        let proof_data = {
            let mut proof_data = [0_u8; 16];
            r.read_exact(&mut proof_data).await?;
            proof_data
        };

        // client_proof: u8[20]
        let client_proof = {
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof).await?;
            client_proof
        };

        // client_checksum: u8[20]
        let client_checksum = {
            let mut client_checksum = [0_u8; 20];
            r.read_exact(&mut client_checksum).await?;
            client_checksum
        };

        // key_count: u8
        let _key_count = crate::util::tokio_read_u8_le(&mut r).await?;
        // key_count is expected to always be 0 (0)

        Ok(Self {
            proof_data,
            client_proof,
            client_checksum,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // proof_data: u8[16]
        let proof_data = {
            let mut proof_data = [0_u8; 16];
            r.read_exact(&mut proof_data).await?;
            proof_data
        };

        // client_proof: u8[20]
        let client_proof = {
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof).await?;
            client_proof
        };

        // client_checksum: u8[20]
        let client_checksum = {
            let mut client_checksum = [0_u8; 20];
            r.read_exact(&mut client_checksum).await?;
            client_checksum
        };

        // key_count: u8
        let _key_count = crate::util::astd_read_u8_le(&mut r).await?;
        // key_count is expected to always be 0 (0)

        Ok(Self {
            proof_data,
            client_proof,
            client_checksum,
        })
    }

}

impl ClientMessage for CMD_AUTH_RECONNECT_PROOF_Client {
    const OPCODE: u8 = 0x03;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(3, "CMD_AUTH_RECONNECT_PROOF_Client", kind))
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(58);
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(3, "CMD_AUTH_RECONNECT_PROOF_Client", kind))})
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
            let mut v = Vec::with_capacity(58);
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(3, "CMD_AUTH_RECONNECT_PROOF_Client", kind))})
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
            let mut v = Vec::with_capacity(58);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

#[cfg(test)]
mod test_version_2 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_RECONNECT_PROOF_Client, expected: &CMD_AUTH_RECONNECT_PROOF_Client) {
        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);
    }

    const RAW0: [u8; 58] = [ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2,
         0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A, 0x0D,
         0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71,
         0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82,
         0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5,
         0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Client {
        CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9, 0x32,
                 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_5 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_5::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_RECONNECT_PROOF_Client, expected: &CMD_AUTH_RECONNECT_PROOF_Client) {
        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);
    }

    const RAW0: [u8; 58] = [ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2,
         0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A, 0x0D,
         0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71,
         0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82,
         0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5,
         0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Client {
        CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9, 0x32,
                 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_6 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_6::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_RECONNECT_PROOF_Client, expected: &CMD_AUTH_RECONNECT_PROOF_Client) {
        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);
    }

    const RAW0: [u8; 58] = [ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2,
         0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A, 0x0D,
         0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71,
         0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82,
         0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5,
         0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Client {
        CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9, 0x32,
                 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_7 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_7::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_RECONNECT_PROOF_Client, expected: &CMD_AUTH_RECONNECT_PROOF_Client) {
        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);
    }

    const RAW0: [u8; 58] = [ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2,
         0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A, 0x0D,
         0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71,
         0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82,
         0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5,
         0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Client {
        CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9, 0x32,
                 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_8 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_RECONNECT_PROOF_Client, expected: &CMD_AUTH_RECONNECT_PROOF_Client) {
        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);
    }

    const RAW0: [u8; 58] = [ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2,
         0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A, 0x0D,
         0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71,
         0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82,
         0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5,
         0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Client {
        CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9, 0x32,
                 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(57 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

