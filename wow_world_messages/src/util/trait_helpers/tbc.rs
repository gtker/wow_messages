use crate::util::{vanilla_get_unencrypted_client, vanilla_get_unencrypted_server};
use std::io;

pub(crate) fn tbc_get_unencrypted_server(
    w: impl io::Write,
    opcode: u16,
    size: u16,
) -> io::Result<()> {
    vanilla_get_unencrypted_server(w, opcode, size)
}

#[cfg(feature = "encryption")]
pub(crate) fn tbc_get_encrypted_server(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut wow_srp::tbc_header::EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(crate::util::SIZE_LENGTH);
    e.write_encrypted_server_header(&mut w, size, opcode)
}

pub(crate) fn tbc_get_unencrypted_client(
    w: impl io::Write,
    opcode: u16,
    size: u16,
) -> io::Result<()> {
    vanilla_get_unencrypted_client(w, opcode, size)
}

#[cfg(feature = "encryption")]
pub(crate) fn tbc_get_encrypted_client(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut wow_srp::tbc_header::EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(crate::util::SIZE_LENGTH);
    e.write_encrypted_client_header(&mut w, size, opcode as u32)
}
