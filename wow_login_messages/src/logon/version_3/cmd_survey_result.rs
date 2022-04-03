use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/survey_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/survey_result.wowm#L3):
/// ```text
/// clogin CMD_SURVEY_RESULT = 0x4 {
///     u32 survey_id;
///     u8 error;
///     u16 compressed_data_length;
///     u8[compressed_data_length] data;
/// }
/// ```
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

