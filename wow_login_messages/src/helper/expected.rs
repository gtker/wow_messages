#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
use crate::errors::ExpectedOpcodeError;
#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
use crate::{ClientMessage, ServerMessage};

/// Read a complete message _from_ the **client** or return an error otherwise.
///
/// ```
/// use wow_login_messages::helper::expect_client_message;
///
/// use wow_login_messages::version_2::CMD_AUTH_LOGON_PROOF_Client;
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// let client = expect_client_message::<CMD_AUTH_LOGON_PROOF_Client, _>(&mut reader)?;
/// // We can now use the message
/// let client_proof = client.client_proof;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "sync")]
pub fn expect_client_message<M: ClientMessage, R: std::io::Read>(
    mut r: R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::read_u8_le(&mut r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read::<&mut R, crate::private::Internal>(&mut r);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

#[cfg(feature = "tokio")]
/// See docs for the sync version called [`expect_client_message`].
pub async fn tokio_expect_client_message<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    mut r: R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(&mut r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read::<&mut R, crate::private::Internal>(&mut r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

#[cfg(feature = "async-std")]
/// See docs for the sync version called [`expect_client_message`].
pub async fn astd_expect_client_message<
    M: ClientMessage,
    R: async_std::io::ReadExt + Unpin + Send,
>(
    mut r: R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::astd_read_u8_le(&mut r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::astd_read::<&mut R, crate::private::Internal>(&mut r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

/// Read a complete message _from_ the **server** or return an error otherwise.
///
/// ```
/// use wow_login_messages::helper::expect_server_message;
///
/// use wow_login_messages::version_2::CMD_AUTH_LOGON_PROOF_Server;
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// let server = expect_server_message::<CMD_AUTH_LOGON_PROOF_Server, _>(&mut reader)?;
/// // We can now use the message
/// let login_result = server.result;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "sync")]
pub fn expect_server_message<M: ServerMessage, R: std::io::Read>(
    mut r: R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::read_u8_le(&mut r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read::<&mut R, crate::private::Internal>(&mut r);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

/// See docs for the sync version called [`expect_server_message`].
#[cfg(feature = "tokio")]
pub async fn tokio_expect_server_message<
    M: ServerMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    mut r: R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(&mut r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read::<&mut R, crate::private::Internal>(&mut r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

/// See docs for the sync version called [`expect_server_message`].
#[cfg(feature = "async-std")]
pub async fn astd_expect_server_message<
    M: ServerMessage,
    R: async_std::io::ReadExt + Unpin + Send,
>(
    mut r: R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::astd_read_u8_le(&mut r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::astd_read::<&mut R, crate::private::Internal>(&mut r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}
