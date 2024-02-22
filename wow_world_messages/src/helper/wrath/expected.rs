#[cfg(feature = "sync")]
pub fn expect_server_message<M: crate::wrath::ServerMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut buf = [0_u8; 4];
    r.read_exact(&mut buf)?;
    let d = if buf[0] & 0x80 != 0 {
        let mut lsb = [0_u8; 1];
        r.read_exact(&mut lsb)?;
        let buf = [buf[0], buf[1], buf[2], buf[3], lsb[0]];
        crate::util::ServerHeader::from_large_array(buf)
    } else {
        crate::util::ServerHeader::from_array(buf)
    };

    let mut buf = vec![0_u8; (d.size.saturating_sub(2)) as usize];
    r.read_exact(&mut buf)?;

    read_server_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(feature = "sync")]
pub fn expect_client_message<M: crate::wrath::ClientMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header)?;
    let d = crate::util::ClientHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size.saturating_sub(4)) as usize];
    r.read_exact(&mut buf)?;

    read_client_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(all(feature = "sync", feature = "encryption"))]
pub fn expect_server_message_encryption<M: crate::wrath::ServerMessage, R: std::io::Read>(
    r: &mut R,
    d: &mut wow_srp::wrath_header::ClientDecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut buf = [0_u8; 4];
    r.read_exact(&mut buf)?;
    let d = match d.attempt_decrypt_server_header(buf) {
        wow_srp::wrath_header::WrathServerAttempt::Header(h) => h,
        wow_srp::wrath_header::WrathServerAttempt::AdditionalByteRequired => {
            let mut lsb = [0_u8; 1];
            r.read_exact(&mut lsb)?;
            d.decrypt_large_server_header(lsb[0])
        }
    };

    let mut buf = vec![0_u8; (d.size.saturating_sub(2)) as usize];
    r.read_exact(&mut buf)?;

    read_server_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(all(feature = "sync", feature = "encryption"))]
pub fn expect_client_message_encryption<M: crate::wrath::ClientMessage, R: std::io::Read>(
    r: &mut R,
    d: &mut wow_srp::wrath_header::ServerDecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header)?;
    let d = d.decrypt_client_header(header);

    let mut buf = vec![0_u8; (d.size.saturating_sub(4)) as usize];
    r.read_exact(&mut buf)?;

    read_client_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_server_message<M: crate::wrath::ServerMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut buf = [0_u8; 4];
    r.read_exact(&mut buf).await?;
    let d = if buf[0] & 0x80 != 0 {
        let mut lsb = [0_u8; 1];
        r.read_exact(&mut lsb).await?;
        let buf = [buf[0], buf[1], buf[2], buf[3], lsb[0]];
        crate::util::ServerHeader::from_large_array(buf)
    } else {
        crate::util::ServerHeader::from_array(buf)
    };

    let mut buf = vec![0_u8; (d.size.saturating_sub(2)) as usize];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_client_message<M: crate::wrath::ClientMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = crate::util::ClientHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size.saturating_sub(4)) as usize];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(all(feature = "tokio", feature = "encryption"))]
pub async fn tokio_expect_server_message_encryption<M: crate::wrath::ServerMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::wrath_header::ClientDecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut buf = [0_u8; 4];
    r.read_exact(&mut buf).await?;
    let d = match d.attempt_decrypt_server_header(buf) {
        wow_srp::wrath_header::WrathServerAttempt::Header(h) => h,
        wow_srp::wrath_header::WrathServerAttempt::AdditionalByteRequired => {
            let mut lsb = [0_u8; 1];
            r.read_exact(&mut lsb).await?;
            d.decrypt_large_server_header(lsb[0])
        }
    };

    let mut buf = vec![0_u8; (d.size.saturating_sub(2)) as usize];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(all(feature = "tokio", feature = "encryption"))]
pub async fn tokio_expect_client_message_encryption<M: crate::wrath::ClientMessage, R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::wrath_header::ServerDecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = d.decrypt_client_header(header);

    let mut buf = vec![0_u8; (d.size.saturating_sub(4)) as usize];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(feature = "async-std")]
pub async fn astd_expect_server_message<M: crate::wrath::ServerMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut buf = [0_u8; 4];
    r.read_exact(&mut buf).await?;
    let d = if buf[0] & 0x80 != 0 {
        let mut lsb = [0_u8; 1];
        r.read_exact(&mut lsb).await?;
        let buf = [buf[0], buf[1], buf[2], buf[3], lsb[0]];
        crate::util::ServerHeader::from_large_array(buf)
    } else {
        crate::util::ServerHeader::from_array(buf)
    };

    let mut buf = vec![0_u8; (d.size.saturating_sub(2)) as usize];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(feature = "async-std")]
pub async fn astd_expect_client_message<M: crate::wrath::ClientMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = crate::util::ClientHeader::from_array(header);

    let mut buf = vec![0_u8; (d.size.saturating_sub(4)) as usize];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(all(feature = "async-std", feature = "encryption"))]
pub async fn astd_expect_server_message_encryption<M: crate::wrath::ServerMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::wrath_header::ClientDecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut buf = [0_u8; 4];
    r.read_exact(&mut buf).await?;
    let d = match d.attempt_decrypt_server_header(buf) {
        wow_srp::wrath_header::WrathServerAttempt::Header(h) => h,
        wow_srp::wrath_header::WrathServerAttempt::AdditionalByteRequired => {
            let mut lsb = [0_u8; 1];
            r.read_exact(&mut lsb).await?;
            d.decrypt_large_server_header(lsb[0])
        }
    };

    let mut buf = vec![0_u8; (d.size.saturating_sub(2)) as usize];
    r.read_exact(&mut buf).await?;

    read_server_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(all(feature = "async-std", feature = "encryption"))]
pub async fn astd_expect_client_message_encryption<M: crate::wrath::ClientMessage, R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
    d: &mut wow_srp::wrath_header::ServerDecrypterHalf,
) -> Result<M, crate::errors::ExpectedOpcodeError> {
    let mut header = [0_u8; 6];
    r.read_exact(&mut header).await?;
    let d = d.decrypt_client_header(header);

    let mut buf = vec![0_u8; (d.size.saturating_sub(4)) as usize];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), d.size.into(), d.opcode.into())
}

#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
fn read_server_body<M: crate::wrath::ServerMessage>(
    buf: &mut &[u8],
    size: u32,
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
            name: crate::wrath::opcode_to_name(opcode),
            size: size.into(),
        })
    }
}
#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
fn read_client_body<M: crate::wrath::ClientMessage>(
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
            name: crate::wrath::opcode_to_name(opcode),
            size: size.into(),
        })
    }
}
