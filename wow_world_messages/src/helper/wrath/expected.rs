use crate::errors::ExpectedOpcodeError;
use crate::wrath::{ClientMessage, ServerMessage};
use wow_srp::wrath_header::{ClientDecrypterHalf, ServerDecrypterHalf, CLIENT_HEADER_LENGTH};

const CLIENT_OPCODE_LENGTH: u16 = 4;
const SERVER_OPCODE_LENGTH: u16 = 2;

#[cfg(feature = "sync")]
pub fn expect_server_message<M: ServerMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header)?;

    let (size, opcode) = if header[0] & 0x80 != 0 {
        let size =
            u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

        let mut last_byte = [0_u8; 1];
        r.read_exact(&mut last_byte)?;
        let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
        (size, opcode)
    } else {
        let size = u16::from_be_bytes([header[0], header[1]])
            .saturating_sub(2)
            .into();
        let opcode = u16::from_le_bytes([header[2], header[3]]);
        (size, opcode)
    };

    let mut buf = vec![0; size as usize];
    r.read_exact(&mut buf)?;
    read_server_body(&mut buf.as_slice(), size, opcode as u32)
}

#[cfg(feature = "sync")]
pub fn expect_server_message_encryption<M: ServerMessage, R: std::io::Read>(
    r: &mut R,
    d: &mut ClientDecrypterHalf,
) -> Result<M, ExpectedOpcodeError> {
    let mut header = [0_u8; 4];
    r.read_exact(&mut header)?;
    d.decrypt(&mut header);

    let (body_size, opcode) = if header[0] & 0x80 != 0 {
        let size =
            u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

        let mut last_byte = [0_u8; 1];
        r.read_exact(&mut last_byte)?;
        d.decrypt(&mut last_byte);
        let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
        (size, opcode)
    } else {
        let size = u16::from_be_bytes([header[0], header[1]])
            .saturating_sub(2)
            .into();
        let opcode = u16::from_le_bytes([header[2], header[3]]);
        (size, opcode)
    };

    let mut buf = vec![0; body_size as usize];
    r.read_exact(&mut buf)?;
    read_server_body(&mut buf.as_slice(), body_size, opcode as u32)
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_client_message_encryption<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
    d: &mut ServerDecrypterHalf,
) -> Result<M, ExpectedOpcodeError> {
    let mut buf = [0_u8; CLIENT_HEADER_LENGTH as usize];
    r.read_exact(&mut buf).await?;
    let d = d.decrypt_client_header(buf);

    let size = d.size;
    let opcode = d.opcode;

    let mut buf = vec![0; (size - CLIENT_OPCODE_LENGTH).into()];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), size, opcode)
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_client_message<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let size = crate::util::tokio_read_u16_be(r).await?;
    let opcode = crate::util::tokio_read_u32_le(r).await?;

    let mut buf = vec![0; (size - CLIENT_OPCODE_LENGTH).into()];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), size, opcode)
}

#[cfg(feature = "async-std")]
pub async fn astd_expect_client_message<
    M: ClientMessage,
    R: async_std::io::ReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let size = crate::util::astd_read_u16_be(r).await?;
    let opcode = crate::util::astd_read_u32_le(r).await?;

    let mut buf = vec![0; (size - CLIENT_OPCODE_LENGTH).into()];
    r.read_exact(&mut buf).await?;

    read_client_body(&mut buf.as_slice(), size, opcode)
}

fn read_client_body<M: ClientMessage>(
    buf: &mut &[u8],
    size: u16,
    opcode: u32,
) -> Result<M, ExpectedOpcodeError> {
    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read_body(buf, (size - CLIENT_OPCODE_LENGTH) as u32);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode {
            opcode,
            size: size.into(),
        })
    }
}

fn read_server_body<M: ServerMessage>(
    buf: &mut &[u8],
    size: u32,
    opcode: u32,
) -> Result<M, ExpectedOpcodeError> {
    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read_body(buf, size - SERVER_OPCODE_LENGTH as u32);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode { opcode, size })
    }
}
