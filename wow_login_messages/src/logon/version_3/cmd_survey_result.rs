use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_SURVEY_RESULT {
    pub survey_id: u32,
    pub error: u8,
    pub data: Vec<u8>,
}

impl ClientMessage for CMD_SURVEY_RESULT {
    const OPCODE: u8 = 0x04;
}
impl ReadableAndWritable for CMD_SURVEY_RESULT {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // survey_id: u32
        let survey_id = crate::util::read_u32_le(r)?;

        // error: u8
        let error = crate::util::read_u8_le(r)?;

        // compressed_data_length: u16
        let compressed_data_length = crate::util::read_u16_le(r)?;

        // data: u8[compressed_data_length]
        let mut data = Vec::with_capacity(compressed_data_length as usize);
        for i in 0..compressed_data_length {
            data.push(crate::util::read_u8_le(r)?);
        }

        Ok(Self {
            survey_id,
            error,
            data,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // error: u8
        w.write_all(&self.error.to_le_bytes())?;

        // compressed_data_length: u16
        w.write_all(&(self.data.len() as u16).to_le_bytes())?;

        // data: u8[compressed_data_length]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for CMD_SURVEY_RESULT {
    type Error = std::io::Error;
    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error> {
        // survey_id: u32
        let survey_id = crate::util::tokio_read_u32_le(r).await?;

        // error: u8
        let error = crate::util::tokio_read_u8_le(r).await?;

        // compressed_data_length: u16
        let compressed_data_length = crate::util::tokio_read_u16_le(r).await?;

        // data: u8[compressed_data_length]
        let mut data = Vec::with_capacity(compressed_data_length as usize);
        for i in 0..compressed_data_length {
            data.push(crate::util::tokio_read_u8_le(r).await?);
        }

        Ok(Self {
            survey_id,
            error,
            data,
        })
    }
}
impl VariableSized for CMD_SURVEY_RESULT {
    fn size(&self) -> usize {
        4 // survey_id: u32
        + 1 // error: u8
        + 2 // compressed_data_length: u16
        + self.data.len() * core::mem::size_of::<u8>() // data: u8[compressed_data_length]
    }
}

impl MaximumPossibleSized for CMD_SURVEY_RESULT {
    fn maximum_possible_size() -> usize {
        4 // survey_id: u32
        + 1 // error: u8
        + 2 // compressed_data_length: u16
        + 65535 * core::mem::size_of::<u8>() // data: u8[compressed_data_length]
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_SURVEY_RESULT;
    use crate::VariableSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    #[test]
    fn CMD_SURVEY_RESULT0() {
        let raw: Vec<u8> = vec![ 0x04, 0xDE, 0xFA, 0x00, 0x00, 0x00, 0x01, 0x00,
             0xFF, ];

        let expected = CMD_SURVEY_RESULT {
            survey_id: 0xFADE,
            error: 0x0,
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_SURVEY_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_SURVEY_RESULT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.survey_id, expected.survey_id);
        assert_eq!(t.error, expected.error);
        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
