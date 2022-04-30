use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TUTORIAL_FLAGS {
    pub tutorial_data0: u32,
    pub tutorial_data1: u32,
    pub tutorial_data2: u32,
    pub tutorial_data3: u32,
    pub tutorial_data4: u32,
    pub tutorial_data5: u32,
    pub tutorial_data6: u32,
    pub tutorial_data7: u32,
}

impl ServerMessageWrite for SMSG_TUTORIAL_FLAGS {
    const OPCODE: u16 = 0xfd;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_TUTORIAL_FLAGS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // tutorial_data0: u32
        let tutorial_data0 = crate::util::read_u32_le(r)?;

        // tutorial_data1: u32
        let tutorial_data1 = crate::util::read_u32_le(r)?;

        // tutorial_data2: u32
        let tutorial_data2 = crate::util::read_u32_le(r)?;

        // tutorial_data3: u32
        let tutorial_data3 = crate::util::read_u32_le(r)?;

        // tutorial_data4: u32
        let tutorial_data4 = crate::util::read_u32_le(r)?;

        // tutorial_data5: u32
        let tutorial_data5 = crate::util::read_u32_le(r)?;

        // tutorial_data6: u32
        let tutorial_data6 = crate::util::read_u32_le(r)?;

        // tutorial_data7: u32
        let tutorial_data7 = crate::util::read_u32_le(r)?;

        Ok(Self {
            tutorial_data0,
            tutorial_data1,
            tutorial_data2,
            tutorial_data3,
            tutorial_data4,
            tutorial_data5,
            tutorial_data6,
            tutorial_data7,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // tutorial_data0: u32
        w.write_all(&self.tutorial_data0.to_le_bytes())?;

        // tutorial_data1: u32
        w.write_all(&self.tutorial_data1.to_le_bytes())?;

        // tutorial_data2: u32
        w.write_all(&self.tutorial_data2.to_le_bytes())?;

        // tutorial_data3: u32
        w.write_all(&self.tutorial_data3.to_le_bytes())?;

        // tutorial_data4: u32
        w.write_all(&self.tutorial_data4.to_le_bytes())?;

        // tutorial_data5: u32
        w.write_all(&self.tutorial_data5.to_le_bytes())?;

        // tutorial_data6: u32
        w.write_all(&self.tutorial_data6.to_le_bytes())?;

        // tutorial_data7: u32
        w.write_all(&self.tutorial_data7.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TUTORIAL_FLAGS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_TUTORIAL_FLAGS {
    fn maximum_possible_size() -> usize {
        4 // tutorial_data0: u32
        + 4 // tutorial_data1: u32
        + 4 // tutorial_data2: u32
        + 4 // tutorial_data3: u32
        + 4 // tutorial_data4: u32
        + 4 // tutorial_data5: u32
        + 4 // tutorial_data6: u32
        + 4 // tutorial_data7: u32
    }
}

