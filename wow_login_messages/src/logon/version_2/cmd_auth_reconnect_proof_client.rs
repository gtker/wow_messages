use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMD_AUTH_RECONNECT_PROOF_Client {
    pub proof_data: [u8; 16],
    pub client_proof: [u8; 20],
    pub client_checksum: [u8; 20],
}

impl ClientMessage for CMD_AUTH_RECONNECT_PROOF_Client {
    const OPCODE: u8 = 0x03;
}
impl CMD_AUTH_RECONNECT_PROOF_Client {
    pub const KEY_COUNT_VALUE: u8 = 0x00;

}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for CMD_AUTH_RECONNECT_PROOF_Client {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // proof_data: u8[16]
        let mut proof_data = [0_u8; 16];
        r.read_exact(&mut proof_data)?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof)?;

        // client_checksum: u8[20]
        let mut client_checksum = [0_u8; 20];
        r.read_exact(&mut client_checksum)?;

        // key_count: u8
        let _key_count = crate::util::read_u8_le(r)?;
        // key_count is expected to always be 0 (0)

        Ok(Self {
            proof_data,
            client_proof,
            client_checksum,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // proof_data: u8[16]
        let mut proof_data = [0_u8; 16];
        r.read_exact(&mut proof_data).await?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof).await?;

        // client_checksum: u8[20]
        let mut client_checksum = [0_u8; 20];
        r.read_exact(&mut client_checksum).await?;

        // key_count: u8
        let _key_count = crate::util::tokio_read_u8_le(r).await?;
        // key_count is expected to always be 0 (0)

        Ok(Self {
            proof_data,
            client_proof,
            client_checksum,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // proof_data: u8[16]
        for i in self.proof_data.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        // client_checksum: u8[20]
        for i in self.client_checksum.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        // key_count: u8
        w.write_all(&Self::KEY_COUNT_VALUE.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // proof_data: u8[16]
        let mut proof_data = [0_u8; 16];
        r.read_exact(&mut proof_data).await?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof).await?;

        // client_checksum: u8[20]
        let mut client_checksum = [0_u8; 20];
        r.read_exact(&mut client_checksum).await?;

        // key_count: u8
        let _key_count = crate::util::astd_read_u8_le(r).await?;
        // key_count is expected to always be 0 (0)

        Ok(Self {
            proof_data,
            client_proof,
            client_checksum,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // proof_data: u8[16]
        for i in self.proof_data.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        // client_checksum: u8[20]
        for i in self.client_checksum.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        // key_count: u8
        w.write_all(&Self::KEY_COUNT_VALUE.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for CMD_AUTH_RECONNECT_PROOF_Client {}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_PROOF_Client {
    fn maximum_possible_size() -> usize {
        0
        + 16 // proof_data: u8[16]
        + 20 // client_proof: u8[20]
        + 20 // client_checksum: u8[20]
        + 1 // key_count: u8
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::CMD_AUTH_RECONNECT_PROOF_Client;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ClientOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_PROOF_Client0() {
        let raw: Vec<u8> = vec![ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B,
             0xF2, 0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A,
             0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05,
             0x71, 0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6,
             0x82, 0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0,
             0xF5, 0x00, ];

        let expected = CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9,
                 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);

        assert_eq!(CMD_AUTH_RECONNECT_PROOF_Client::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", async_std::test)]
    async fn tokio_CMD_AUTH_RECONNECT_PROOF_Client0() {
        let raw: Vec<u8> = vec![ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B,
             0xF2, 0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A,
             0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05,
             0x71, 0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6,
             0x82, 0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0,
             0xF5, 0x00, ];

        let expected = CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9,
                 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);

        assert_eq!(CMD_AUTH_RECONNECT_PROOF_Client::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", tokio::test)]
    async fn astd_CMD_AUTH_RECONNECT_PROOF_Client0() {
        let raw: Vec<u8> = vec![ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B,
             0xF2, 0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A,
             0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05,
             0x71, 0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6,
             0x82, 0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0,
             0xF5, 0x00, ];

        let expected = CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9,
                 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);

        assert_eq!(CMD_AUTH_RECONNECT_PROOF_Client::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
