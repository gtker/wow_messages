use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct TelemetryKey {
    pub unknown1: u16,
    pub unknown2: u32,
    pub unknown3: [u8; 4],
    pub unknown4: [u8; 20],
}

impl TelemetryKey {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8[4]
        for i in self.unknown3.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // unknown4: u8[20]
        for i in self.unknown4.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl TelemetryKey {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // unknown3: u8[4]
        let mut unknown3 = [0_u8; 4];
        r.read_exact(&mut unknown3)?;

        // unknown4: u8[20]
        let mut unknown4 = [0_u8; 20];
        r.read_exact(&mut unknown4)?;

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            unknown4,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // unknown1: u16
        let unknown1 = crate::util::tokio_read_u16_le(r).await?;

        // unknown2: u32
        let unknown2 = crate::util::tokio_read_u32_le(r).await?;

        // unknown3: u8[4]
        let mut unknown3 = [0_u8; 4];
        r.read_exact(&mut unknown3).await?;

        // unknown4: u8[20]
        let mut unknown4 = [0_u8; 20];
        r.read_exact(&mut unknown4).await?;

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            unknown4,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // unknown1: u16
        let unknown1 = crate::util::astd_read_u16_le(r).await?;

        // unknown2: u32
        let unknown2 = crate::util::astd_read_u32_le(r).await?;

        // unknown3: u8[4]
        let mut unknown3 = [0_u8; 4];
        r.read_exact(&mut unknown3).await?;

        // unknown4: u8[20]
        let mut unknown4 = [0_u8; 20];
        r.read_exact(&mut unknown4).await?;

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            unknown4,
        })
    }

}

