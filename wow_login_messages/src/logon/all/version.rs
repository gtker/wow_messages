use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/challenge_client_commons.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/challenge_client_commons.wowm#L30):
/// ```text
/// struct Version {
///     u8 major;
///     u8 minor;
///     u8 patch;
///     u16 build;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u16,
}

impl Version {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // major: u8
        w.write_all(&self.major.to_le_bytes())?;

        // minor: u8
        w.write_all(&self.minor.to_le_bytes())?;

        // patch: u8
        w.write_all(&self.patch.to_le_bytes())?;

        // build: u16
        w.write_all(&self.build.to_le_bytes())?;

        Ok(())
    }
}

impl Version {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // major: u8
        let major = crate::util::read_u8_le(&mut r)?;

        // minor: u8
        let minor = crate::util::read_u8_le(&mut r)?;

        // patch: u8
        let patch = crate::util::read_u8_le(&mut r)?;

        // build: u16
        let build = crate::util::read_u16_le(&mut r)?;

        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, std::io::Error> {
        // major: u8
        let major = crate::util::tokio_read_u8_le(&mut r).await?;

        // minor: u8
        let minor = crate::util::tokio_read_u8_le(&mut r).await?;

        // patch: u8
        let patch = crate::util::tokio_read_u8_le(&mut r).await?;

        // build: u16
        let build = crate::util::tokio_read_u16_le(&mut r).await?;

        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, std::io::Error> {
        // major: u8
        let major = crate::util::astd_read_u8_le(&mut r).await?;

        // minor: u8
        let minor = crate::util::astd_read_u8_le(&mut r).await?;

        // patch: u8
        let patch = crate::util::astd_read_u8_le(&mut r).await?;

        // build: u16
        let build = crate::util::astd_read_u16_le(&mut r).await?;

        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }

}

