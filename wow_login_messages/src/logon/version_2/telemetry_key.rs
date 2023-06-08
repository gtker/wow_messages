use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm#L2):
/// ```text
/// struct TelemetryKey {
///     u16 unknown1;
///     u32 unknown2;
///     u8[4] unknown3;
///     u8[20] cd_key_proof;
/// }
/// ```
pub struct TelemetryKey {
    pub unknown1: u16,
    pub unknown2: u32,
    pub unknown3: [u8; 4],
    /// SHA1 hash of the session key, server public key, and an unknown 20 byte value.
    pub cd_key_proof: [u8; 20],
}

impl TelemetryKey {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8[4]
        for i in self.unknown3.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // cd_key_proof: u8[20]
        for i in self.cd_key_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl TelemetryKey {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, std::io::Error> {
        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // unknown3: u8[4]
        let unknown3 = {
            let mut unknown3 = [0_u8; 4];
            r.read_exact(&mut unknown3)?;
            unknown3
        };

        // cd_key_proof: u8[20]
        let cd_key_proof = {
            let mut cd_key_proof = [0_u8; 20];
            r.read_exact(&mut cd_key_proof)?;
            cd_key_proof
        };

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            cd_key_proof,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, std::io::Error> {
        // unknown1: u16
        let unknown1 = crate::util::tokio_read_u16_le(&mut r).await?;

        // unknown2: u32
        let unknown2 = crate::util::tokio_read_u32_le(&mut r).await?;

        // unknown3: u8[4]
        let unknown3 = {
            let mut unknown3 = [0_u8; 4];
            r.read_exact(&mut unknown3).await?;
            unknown3
        };

        // cd_key_proof: u8[20]
        let cd_key_proof = {
            let mut cd_key_proof = [0_u8; 20];
            r.read_exact(&mut cd_key_proof).await?;
            cd_key_proof
        };

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            cd_key_proof,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, std::io::Error> {
        // unknown1: u16
        let unknown1 = crate::util::astd_read_u16_le(&mut r).await?;

        // unknown2: u32
        let unknown2 = crate::util::astd_read_u32_le(&mut r).await?;

        // unknown3: u8[4]
        let unknown3 = {
            let mut unknown3 = [0_u8; 4];
            r.read_exact(&mut unknown3).await?;
            unknown3
        };

        // cd_key_proof: u8[20]
        let cd_key_proof = {
            let mut cd_key_proof = [0_u8; 20];
            r.read_exact(&mut cd_key_proof).await?;
            cd_key_proof
        };

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            cd_key_proof,
        })
    }

}

