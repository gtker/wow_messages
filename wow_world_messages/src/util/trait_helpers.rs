use std::io;
#[cfg(all(feature = "tbc", feature = "encryption"))]
use wow_srp::tbc_header::EncrypterHalf;
#[cfg(all(feature = "vanilla", feature = "encryption"))]
use wow_srp::vanilla_header;
#[cfg(all(feature = "wrath", feature = "encryption"))]
use wow_srp::wrath_header::{ClientEncrypterHalf, ServerEncrypterHalf};

#[cfg(feature = "wrath")]
pub(crate) fn wrath_get_unencrypted_server(
    mut w: impl io::Write,
    opcode: u16,
    size: u32,
) -> io::Result<()> {
    if size > LARGE_MESSAGE_THRESHOLD {
        let size = (size.saturating_sub(MAXIMUM_SIZE_LENGTH)).to_be_bytes();
        let opcode = opcode.to_le_bytes();

        let mut header = [0_u8; MAXIMUM_SERVER_HEADER_LENGTH as usize];
        header[0] = size[1] | 0x80;
        header[1] = size[2];
        header[2] = size[3];
        header[3] = opcode[0];
        header[4] = opcode[1];

        w.write_all(&header)
    } else {
        let size = ((size.saturating_sub(MINIMUM_SIZE_LENGTH)) as u16).to_be_bytes();
        let opcode = opcode.to_le_bytes();

        let mut header = [0_u8; MINIMUM_SERVER_HEADER_LENGTH as usize];
        header[0] = size[0];
        header[1] = size[1];
        header[2] = opcode[0];
        header[3] = opcode[1];

        w.write_all(&header)
    }
}

#[cfg(all(feature = "encryption", feature = "wrath"))]
pub(crate) fn wrath_get_encrypted_server(
    mut w: impl io::Write,
    opcode: u16,
    size: u32,
    e: &mut ServerEncrypterHalf,
) -> io::Result<()> {
    let size_length = if size > LARGE_MESSAGE_THRESHOLD {
        MAXIMUM_SIZE_LENGTH
    } else {
        MINIMUM_SIZE_LENGTH
    };

    let size = size.saturating_sub(size_length);
    e.write_encrypted_server_header(&mut w, size, opcode)
}

pub(crate) const MINIMUM_SIZE_LENGTH: u32 = 2;
pub(crate) const MAXIMUM_SIZE_LENGTH: u32 = 3;
pub(crate) const MINIMUM_SERVER_HEADER_LENGTH: u16 = 4;
pub(crate) const MAXIMUM_SERVER_HEADER_LENGTH: u16 = 5;
pub(crate) const CLIENT_HEADER_LENGTH: u16 = 6;

pub(crate) const LARGE_MESSAGE_THRESHOLD: u32 = 0x7FFF;

#[cfg(feature = "wrath")]
pub(crate) fn wrath_get_unencrypted_client(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
) -> io::Result<()> {
    let size = (size.saturating_sub(MINIMUM_SIZE_LENGTH as u16)).to_be_bytes();
    let opcode = (opcode as u32).to_le_bytes();

    let mut header = [0_u8; CLIENT_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    header[4] = opcode[2];
    header[5] = opcode[3];

    w.write_all(&header)
}

#[cfg(all(feature = "encryption", feature = "wrath"))]
pub(crate) fn wrath_get_encrypted_client(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut ClientEncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(MINIMUM_SIZE_LENGTH as u16);
    e.write_encrypted_client_header(&mut w, size, opcode as u32)
}

pub(crate) const SIZE_LENGTH: u16 = 2;
pub(crate) const SERVER_HEADER_LENGTH: u16 = 4;

#[cfg(feature = "tbc")]
pub(crate) fn tbc_get_unencrypted_server(
    w: impl io::Write,
    opcode: u16,
    size: u16,
) -> io::Result<()> {
    vanilla_get_unencrypted_server(w, opcode, size)
}

#[cfg(all(feature = "encryption", feature = "tbc"))]
pub(crate) fn tbc_get_encrypted_server(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(SIZE_LENGTH);
    e.write_encrypted_server_header(&mut w, size, opcode)
}

#[cfg(feature = "tbc")]
pub(crate) fn tbc_get_unencrypted_client(
    w: impl io::Write,
    opcode: u16,
    size: u16,
) -> io::Result<()> {
    vanilla_get_unencrypted_client(w, opcode, size)
}

#[cfg(all(feature = "encryption", feature = "tbc"))]
pub(crate) fn tbc_get_encrypted_client(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(SIZE_LENGTH);
    e.write_encrypted_client_header(&mut w, size, opcode as u32)
}

#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub(crate) fn vanilla_get_unencrypted_server(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
) -> io::Result<()> {
    let size = (size.saturating_sub(SIZE_LENGTH)).to_be_bytes();
    let opcode = opcode.to_le_bytes();

    let mut header = [0_u8; SERVER_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    w.write_all(&header)
}

#[cfg(all(feature = "encryption", feature = "vanilla"))]
pub(crate) fn vanilla_get_encrypted_server(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut vanilla_header::EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(SIZE_LENGTH);
    e.write_encrypted_server_header(&mut w, size, opcode)
}

#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub(crate) fn vanilla_get_unencrypted_client(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
) -> io::Result<()> {
    let size = (size.saturating_sub(SIZE_LENGTH)).to_be_bytes();
    let opcode = (opcode as u32).to_le_bytes();

    let mut header = [0_u8; CLIENT_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    header[4] = opcode[2];
    header[5] = opcode[3];

    w.write_all(&header)
}

#[cfg(all(feature = "encryption", feature = "vanilla"))]
pub(crate) fn vanilla_get_encrypted_client(
    mut w: impl io::Write,
    opcode: u16,
    size: u16,
    e: &mut vanilla_header::EncrypterHalf,
) -> io::Result<()> {
    let size = size.saturating_sub(SIZE_LENGTH);
    e.write_encrypted_client_header(&mut w, size, opcode as u32)
}
