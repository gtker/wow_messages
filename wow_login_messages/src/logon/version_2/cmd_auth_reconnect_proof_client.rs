use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm#L3):
/// ```text
/// clogin CMD_AUTH_RECONNECT_PROOF_Client = 0x3 {
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

impl ClientMessage for CMD_AUTH_RECONNECT_PROOF_Client {
    const OPCODE: u8 = 0x03;
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
    /// **This field is not in the struct, but is written as this constant value.**
    pub const KEY_COUNT_VALUE: u8 = 0x00;

}

impl ReadableAndWritable for CMD_AUTH_RECONNECT_PROOF_Client {
    type Error = std::io::Error;

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

}

impl ConstantSized for CMD_AUTH_RECONNECT_PROOF_Client {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_PROOF_Client {
    fn maximum_possible_size() -> usize {
        16 * core::mem::size_of::<u8>() // proof_data: u8[16]
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 20 * core::mem::size_of::<u8>() // client_checksum: u8[20]
        + 1 // key_count: u8
    }
}

