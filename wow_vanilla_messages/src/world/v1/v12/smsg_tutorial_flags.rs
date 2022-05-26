use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

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

impl SMSG_TUTORIAL_FLAGS {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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

impl ServerMessage for SMSG_TUTORIAL_FLAGS {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
    const OPCODE: u16 = 0x00fd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        32
    }

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

}

