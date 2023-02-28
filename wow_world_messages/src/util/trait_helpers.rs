#[cfg(feature = "tbc")]
use wow_srp::tbc_header::EncrypterHalf;
#[cfg(feature = "vanilla")]
use wow_srp::vanilla_header;
#[cfg(feature = "wrath")]
use wow_srp::wrath_header::{ClientEncrypterHalf, ServerEncrypterHalf};

#[cfg(feature = "wrath")]
pub(crate) fn wrath_get_unencrypted_server(opcode: u16, size: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    if size > LARGE_MESSAGE_THRESHOLD {
        let size = (size.saturating_sub(MAXIMUM_SIZE_LENGTH)).to_be_bytes();
        let opcode = opcode.to_le_bytes();

        let mut header = [0_u8; MAXIMUM_SERVER_HEADER_LENGTH as usize];
        header[0] = size[1] | 0x80;
        header[1] = size[2];
        header[2] = size[3];
        header[3] = opcode[0];
        header[4] = opcode[1];

        v.extend_from_slice(&header);
    } else {
        let size = ((size.saturating_sub(MINIMUM_SIZE_LENGTH)) as u16).to_be_bytes();
        let opcode = opcode.to_le_bytes();

        let mut header = [0_u8; MINIMUM_SERVER_HEADER_LENGTH as usize];
        header[0] = size[0];
        header[1] = size[1];
        header[2] = opcode[0];
        header[3] = opcode[1];

        v.extend_from_slice(&header);
    }

    v
}

#[cfg(all(feature = "encryption", feature = "wrath"))]
pub(crate) fn wrath_get_encrypted_server(
    opcode: u16,
    size: u32,
    e: &mut ServerEncrypterHalf,
) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size_length = if size > LARGE_MESSAGE_THRESHOLD {
        MAXIMUM_SIZE_LENGTH
    } else {
        MINIMUM_SIZE_LENGTH
    };

    v.extend_from_slice(e.encrypt_server_header(size.saturating_sub(size_length), opcode));

    v
}

pub(crate) const MINIMUM_SIZE_LENGTH: u32 = 2;
pub(crate) const MAXIMUM_SIZE_LENGTH: u32 = 3;
pub(crate) const MINIMUM_SERVER_HEADER_LENGTH: u16 = 4;
pub(crate) const MAXIMUM_SERVER_HEADER_LENGTH: u16 = 5;
pub(crate) const CLIENT_HEADER_LENGTH: u16 = 6;

pub(crate) const LARGE_MESSAGE_THRESHOLD: u32 = 0x7FFF;

#[cfg(feature = "wrath")]
pub(crate) fn wrath_get_unencrypted_client(opcode: u16, size: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size = (size.saturating_sub(MINIMUM_SIZE_LENGTH as u16)).to_be_bytes();
    let opcode = (opcode as u32).to_le_bytes();

    let mut header = [0_u8; CLIENT_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    header[4] = opcode[2];
    header[5] = opcode[3];

    v.extend_from_slice(&header);

    v
}

#[cfg(all(feature = "encryption", feature = "wrath"))]
pub(crate) fn wrath_get_encrypted_client(
    opcode: u16,
    size: u16,
    e: &mut ClientEncrypterHalf,
) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_client_header(
        size.saturating_sub(MINIMUM_SIZE_LENGTH as u16),
        opcode as u32,
    ));

    v
}

pub(crate) const SIZE_LENGTH: u16 = 2;
pub(crate) const SERVER_HEADER_LENGTH: u16 = 4;

#[cfg(feature = "tbc")]
pub(crate) fn tbc_get_unencrypted_server(opcode: u16, size: u16) -> Vec<u8> {
    vanilla_get_unencrypted_server(opcode, size)
}

#[cfg(all(feature = "encryption", feature = "tbc"))]
pub(crate) fn tbc_get_encrypted_server(opcode: u16, size: u16, e: &mut EncrypterHalf) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_server_header(size.saturating_sub(SIZE_LENGTH), opcode));

    v
}

#[cfg(feature = "tbc")]
pub(crate) fn tbc_get_unencrypted_client(opcode: u16, size: u16) -> Vec<u8> {
    vanilla_get_unencrypted_client(opcode, size)
}

#[cfg(all(feature = "encryption", feature = "tbc"))]
pub(crate) fn tbc_get_encrypted_client(opcode: u16, size: u16, e: &mut EncrypterHalf) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_client_header(size.saturating_sub(SIZE_LENGTH), opcode as u32));

    v
}

#[cfg(feature = "vanilla")]
pub(crate) fn vanilla_get_unencrypted_server(opcode: u16, size: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size = (size.saturating_sub(SIZE_LENGTH)).to_be_bytes();
    let opcode = opcode.to_le_bytes();

    let mut header = [0_u8; SERVER_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    v.extend_from_slice(&header);

    v
}

#[cfg(all(feature = "encryption", feature = "vanilla"))]
pub(crate) fn vanilla_get_encrypted_server(
    opcode: u16,
    size: u16,
    e: &mut vanilla_header::EncrypterHalf,
) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_server_header(size.saturating_sub(SIZE_LENGTH), opcode));

    v
}

#[cfg(feature = "vanilla")]
pub(crate) fn vanilla_get_unencrypted_client(opcode: u16, size: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size = (size.saturating_sub(SIZE_LENGTH)).to_be_bytes();
    let opcode = (opcode as u32).to_le_bytes();

    let mut header = [0_u8; CLIENT_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    header[4] = opcode[2];
    header[5] = opcode[3];

    v.extend_from_slice(&header);

    v
}

#[cfg(all(feature = "encryption", feature = "vanilla"))]
pub(crate) fn vanilla_get_encrypted_client(
    opcode: u16,
    size: u16,
    e: &mut vanilla_header::EncrypterHalf,
) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_client_header(size.saturating_sub(SIZE_LENGTH), opcode as u32));

    v
}
