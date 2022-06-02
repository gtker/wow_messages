use crate::errors::ExpectedOpcodeError;
use crate::ClientMessage;
use tokio::io::AsyncReadExt;
use wow_srp::header_crypto::{Decrypter, CLIENT_HEADER_LENGTH};

const CLIENT_OPCODE_LENGTH: u16 = 4;

#[cfg(feature = "tokio")]
pub async fn tokio_expect_client_message_encryption<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
    D: Decrypter,
>(
    r: &mut R,
    d: &mut D,
) -> Result<M, ExpectedOpcodeError> {
    let mut buf = [0_u8; CLIENT_HEADER_LENGTH as usize];
    r.read_exact(&mut buf).await?;
    let d = d.decrypt_client_header(buf);

    let size = d.size;
    let opcode = d.opcode;

    let mut buf = vec![0; size as usize - 4];
    r.read_exact(&mut buf).await;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode as u16 == M::OPCODE {
        let m = M::read_body(&mut buf.as_slice(), (size - CLIENT_OPCODE_LENGTH) as u32);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode))
    }
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

    let mut buf = vec![0; size as usize - 4];
    r.read_exact(&mut buf).await;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode as u16 == M::OPCODE {
        let m = M::read_body(&mut buf.as_slice(), (size - CLIENT_OPCODE_LENGTH) as u32);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode))
    }
}
