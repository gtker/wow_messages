use crate::util::SIZE_LENGTH;
use std::io;

#[cfg(all(feature = "encryption", feature = "vanilla"))]
pub(crate) fn vanilla_get_encrypted_server(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut wow_srp::vanilla_header::EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(SIZE_LENGTH);
    e.write_encrypted_server_header(&mut w, size, opcode)
}

#[cfg(all(feature = "encryption", feature = "vanilla"))]
pub(crate) fn vanilla_get_encrypted_client(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut wow_srp::vanilla_header::EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(SIZE_LENGTH);
    e.write_encrypted_client_header(&mut w, size, opcode as u32)
}
