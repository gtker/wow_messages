use crate::util::CLIENT_HEADER_LENGTH;
use std::io;

pub(crate) const SIZE_LENGTH: u16 = 2;
pub(crate) const SERVER_HEADER_LENGTH: u16 = 4;

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
