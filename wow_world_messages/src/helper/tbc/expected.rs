#[cfg(feature = "sync")]
pub fn expect_server_message<M: crate::tbc::ServerMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header)?;
    let d = wow_srp::tbc_header::ServerHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size - 2).into()];
    r.read_exact(&mut buf)?;

    read_server_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(feature = "sync")]
pub fn expect_client_message<M: crate::tbc::ClientMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header)?;
    let d = wow_srp::tbc_header::ClientHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size - 4).into()];
    r.read_exact(&mut buf)?;

    read_client_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(all(feature = "sync", feature = "encryption"))]
pub fn expect_server_message_encryption<M: crate::tbc::ServerMessage, R: std::io::Read>(
    r: &mut R,
    d: &mut wow_srp::tbc_header::DecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header)?;
    let d = d.decrypt_server_header(header);

    let mut buf = vec![0_u8; (d.size - 2).into()];
    r.read_exact(&mut buf)?;

    read_server_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(all(feature = "sync", feature = "encryption"))]
pub fn expect_client_message_encryption<M: crate::tbc::ClientMessage, R: std::io::Read>(
    r: &mut R,
    d: &mut wow_srp::tbc_header::DecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header)?;
    let d = d.decrypt_client_header(header);

    let mut buf = vec![0_u8; (d.size - 4).into()];
    r.read_exact(&mut buf)?;

    read_client_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_server_message<M: crate::tbc::ServerMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header).await?;
    let d = wow_srp::tbc_header::ServerHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size - 2).into()];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_client_message<M: crate::tbc::ClientMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = wow_srp::tbc_header::ClientHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size - 4).into()];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(all(feature = "tokio", feature = "encryption"))]
pub async fn tokio_expect_server_message_encryption<M: crate::tbc::ServerMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::tbc_header::DecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header).await?;
    let d = d.decrypt_server_header(header);

    let mut buf = vec![0_u8; (d.size - 2).into()];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(all(feature = "tokio", feature = "encryption"))]
pub async fn tokio_expect_client_message_encryption<M: crate::tbc::ClientMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::tbc_header::DecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = d.decrypt_client_header(header);

    let mut buf = vec![0_u8; (d.size - 4).into()];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(feature = "async-std")]
pub async fn astd_expect_server_message<M: crate::tbc::ServerMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header).await?;
    let d = wow_srp::tbc_header::ServerHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size - 2).into()];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(feature = "async-std")]
pub async fn astd_expect_client_message<M: crate::tbc::ClientMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = wow_srp::tbc_header::ClientHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size - 4).into()];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(all(feature = "async-std", feature = "encryption"))]
pub async fn astd_expect_server_message_encryption<M: crate::tbc::ServerMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::tbc_header::DecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header).await?;
    let d = d.decrypt_server_header(header);

    let mut buf = vec![0_u8; (d.size - 2).into()];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(all(feature = "async-std", feature = "encryption"))]
pub async fn astd_expect_client_message_encryption<M: crate::tbc::ClientMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::tbc_header::DecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = d.decrypt_client_header(header);

    let mut buf = vec![0_u8; (d.size - 4).into()];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size, d.opcode.into())
}

#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
fn read_server_body<M: crate::tbc::ServerMessage>(
    buf: &mut &[u8],
    size: u16,
    opcode: u32,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read_body::<crate::traits::private::Internal>(
            buf,
            size.saturating_sub(2) as u32,
        );
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(crate::errors::ExpectedOpcodeError::Opcode {
            opcode,
            name: crate::tbc::opcode_to_name(opcode),
            size: size.into(),
        })
    }
}
#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
fn read_client_body<M: crate::tbc::ClientMessage>(
    buf: &mut &[u8],
    size: u16,
    opcode: u32,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read_body::<crate::traits::private::Internal>(
            buf,
            size.saturating_sub(4) as u32,
        );
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(crate::errors::ExpectedOpcodeError::Opcode {
            opcode,
            name: crate::tbc::opcode_to_name(opcode),
            size: size.into(),
        })
    }
}
