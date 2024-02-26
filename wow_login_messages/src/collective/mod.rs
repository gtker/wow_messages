mod cmd_auth_logon_challenge_client;
mod cmd_auth_logon_challenge_server;
mod cmd_auth_logon_proof_client;
mod cmd_auth_logon_proof_server;
mod cmd_auth_reconnect_challenge_client;
mod cmd_auth_reconnect_challenge_server;
mod cmd_auth_reconnect_proof_client;
mod cmd_auth_reconnect_proof_server;
mod cmd_realm_list_client;
mod cmd_realm_list_server;
mod cmd_xfer_accept;
mod cmd_xfer_cancel;
mod cmd_xfer_data;
mod cmd_xfer_initiate;
mod cmd_xfer_resume;

use crate::all::ProtocolVersion;
use crate::errors::CollectiveError;
use crate::{errors, private, Message};

pub trait CollectiveMessage: Message + Send + Sync {
    type Version2: Message + Send + Sync;
    type Version3: Message + Send + Sync;
    type Version5: Message + Send + Sync;
    type Version6: Message + Send + Sync;
    type Version7: Message + Send + Sync;
    type Version8: Message + Send + Sync;

    fn from_version_2(v: Self::Version2) -> Self;
    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError>;

    fn from_version_3(v: Self::Version3) -> Self;
    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError>;

    fn from_version_5(v: Self::Version5) -> Self;
    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError>;

    fn from_version_6(v: Self::Version6) -> Self;
    fn to_version_6(&self) -> Result<Self::Version6, CollectiveError>;

    fn from_version_7(v: Self::Version7) -> Self;
    fn to_version_7(&self) -> Result<Self::Version7, CollectiveError>;

    #[doc(hidden)]
    #[cfg(feature = "sync")]
    fn read_protocol<R: std::io::Read, I: private::Sealed>(
        r: R,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, errors::ParseError> {
        Ok(match protocol_version {
            ProtocolVersion::Two => Self::from_version_2(Self::Version2::read::<R, I>(r)?),
            ProtocolVersion::Three => Self::from_version_3(Self::Version3::read::<R, I>(r)?),
            ProtocolVersion::Five => Self::from_version_5(Self::Version5::read::<R, I>(r)?),
            ProtocolVersion::Six => Self::from_version_6(Self::Version6::read::<R, I>(r)?),
            ProtocolVersion::Seven => Self::from_version_7(Self::Version7::read::<R, I>(r)?),
            ProtocolVersion::Eight => Self::read::<R, I>(r)?,
        })
    }

    #[cfg(feature = "sync")]
    fn write_protocol<W: std::io::Write>(
        &self,
        w: W,
        protocol_version: ProtocolVersion,
    ) -> Result<(), CollectiveError> {
        match protocol_version {
            ProtocolVersion::Two => self.to_version_2()?.write(w)?,
            ProtocolVersion::Three => self.to_version_3()?.write(w)?,
            ProtocolVersion::Five => self.to_version_5()?.write(w)?,
            ProtocolVersion::Six => self.to_version_6()?.write(w)?,
            ProtocolVersion::Seven => self.to_version_7()?.write(w)?,
            ProtocolVersion::Eight => self.write(w)?,
        }

        Ok(())
    }

    #[doc(hidden)]
    #[cfg(feature = "async-std")]
    fn astd_read_protocol<'async_trait, R, I: private::Sealed>(
        r: R,
        protocol_version: ProtocolVersion,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Self, errors::ParseError>>
                + Send
                + 'async_trait,
        >,
    >
    where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
    {
        Box::pin(async move {
            Ok(match protocol_version {
                ProtocolVersion::Two => {
                    Self::from_version_2(Self::Version2::astd_read::<R, I>(r).await?)
                }
                ProtocolVersion::Three => {
                    Self::from_version_3(Self::Version3::astd_read::<R, I>(r).await?)
                }
                ProtocolVersion::Five => {
                    Self::from_version_5(Self::Version5::astd_read::<R, I>(r).await?)
                }
                ProtocolVersion::Six => {
                    Self::from_version_6(Self::Version6::astd_read::<R, I>(r).await?)
                }
                ProtocolVersion::Seven => {
                    Self::from_version_7(Self::Version7::astd_read::<R, I>(r).await?)
                }
                ProtocolVersion::Eight => Self::astd_read::<R, I>(r).await?,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_protocol<'life0, 'async_trait, W>(
        &'life0 self,
        w: W,
        protocol_version: ProtocolVersion,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CollectiveError>> + Send + 'async_trait>,
    >
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            match protocol_version {
                ProtocolVersion::Two => self.to_version_2()?.astd_write(w).await?,
                ProtocolVersion::Three => self.to_version_3()?.astd_write(w).await?,
                ProtocolVersion::Five => self.to_version_5()?.astd_write(w).await?,
                ProtocolVersion::Six => self.to_version_6()?.astd_write(w).await?,
                ProtocolVersion::Seven => self.to_version_7()?.astd_write(w).await?,
                ProtocolVersion::Eight => self.astd_write(w).await?,
            }

            Ok(())
        })
    }

    #[doc(hidden)]
    #[cfg(feature = "tokio")]
    fn tokio_read_protocol<'async_trait, R, I: private::Sealed>(
        r: R,
        protocol_version: ProtocolVersion,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Self, errors::ParseError>>
                + Send
                + 'async_trait,
        >,
    >
    where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
    {
        Box::pin(async move {
            Ok(match protocol_version {
                ProtocolVersion::Two => {
                    Self::from_version_2(Self::Version2::tokio_read::<R, I>(r).await?)
                }
                ProtocolVersion::Three => {
                    Self::from_version_3(Self::Version3::tokio_read::<R, I>(r).await?)
                }
                ProtocolVersion::Five => {
                    Self::from_version_5(Self::Version5::tokio_read::<R, I>(r).await?)
                }
                ProtocolVersion::Six => {
                    Self::from_version_6(Self::Version6::tokio_read::<R, I>(r).await?)
                }
                ProtocolVersion::Seven => {
                    Self::from_version_7(Self::Version7::tokio_read::<R, I>(r).await?)
                }
                ProtocolVersion::Eight => Self::tokio_read::<R, I>(r).await?,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_protocol<'life0, 'async_trait, W>(
        &'life0 self,
        w: W,
        protocol_version: ProtocolVersion,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CollectiveError>> + Send + 'async_trait>,
    >
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            match protocol_version {
                ProtocolVersion::Two => self.to_version_2()?.tokio_write(w).await?,
                ProtocolVersion::Three => self.to_version_3()?.tokio_write(w).await?,
                ProtocolVersion::Five => self.to_version_5()?.tokio_write(w).await?,
                ProtocolVersion::Six => self.to_version_6()?.tokio_write(w).await?,
                ProtocolVersion::Seven => self.to_version_7()?.tokio_write(w).await?,
                ProtocolVersion::Eight => self.tokio_write(w).await?,
            }

            Ok(())
        })
    }
}
