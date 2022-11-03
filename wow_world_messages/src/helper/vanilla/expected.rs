use crate::errors::ExpectedOpcodeError;
use crate::vanilla::{opcode_to_name, ClientMessage, ServerMessage};
#[cfg(feature = "encryption")]
use wow_srp::vanilla_header::{DecrypterHalf, CLIENT_HEADER_LENGTH, SERVER_HEADER_LENGTH};

const CLIENT_OPCODE_LENGTH: u16 = 4;
const SERVER_OPCODE_LENGTH: u16 = 2;

#[cfg(feature = "sync")]
pub fn expect_server_message<M: ServerMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let size = crate::util::read_u16_be(r)?;
    let opcode = crate::util::read_u16_le(r)?;

    let mut buf = vec![0; (size - SERVER_OPCODE_LENGTH).into()];
    r.read_exact(&mut buf)?;

    read_server_body(&mut buf.as_slice(), size, opcode as u32)
}

#[cfg(all(feature = "sync", feature = "encryption"))]
pub fn expect_server_message_encryption<M: ServerMessage, R: std::io::Read>(
    r: &mut R,
    d: &mut DecrypterHalf,
) -> Result<M, ExpectedOpcodeError> {
    let mut buf = [0_u8; SERVER_HEADER_LENGTH as usize];
    r.read_exact(&mut buf)?;
    let d = d.decrypt_server_header(buf);

    let size = d.size;
    let opcode = d.opcode;

    let mut buf = vec![0; (size - SERVER_OPCODE_LENGTH).into()];
    r.read_exact(&mut buf)?;

    read_server_body(&mut buf.as_slice(), size, opcode as u32)
}

#[cfg(all(feature = "tokio", feature = "encryption"))]
pub async fn tokio_expect_client_message_encryption<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
    d: &mut DecrypterHalf,
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
            name: opcode_to_name(opcode),
            size: size.into(),
        })
    }
}
fn read_server_body<M: ServerMessage>(
    buf: &mut &[u8],
    size: u16,
    opcode: u32,
) -> Result<M, ExpectedOpcodeError> {
    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read_body(buf, (size - SERVER_OPCODE_LENGTH) as u32);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode {
            opcode,
            name: opcode_to_name(opcode),
            size: size.into(),
        })
    }
}
