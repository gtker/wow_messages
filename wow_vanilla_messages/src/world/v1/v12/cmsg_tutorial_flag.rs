use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_tutorial_flag.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_tutorial_flag.wowm#L3):
/// ```text
/// cmsg CMSG_TUTORIAL_FLAG = 0xFE {
///     u32 tutorial_flag;
/// }
/// ```
pub struct CMSG_TUTORIAL_FLAG {
    pub tutorial_flag: u32,
}

impl WorldClientMessageWrite for CMSG_TUTORIAL_FLAG {
    const OPCODE: u32 = 0xfe;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_TUTORIAL_FLAG {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // tutorial_flag: u32
        let tutorial_flag = crate::util::read_u32_le(r)?;

        Ok(Self {
            tutorial_flag,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // tutorial_flag: u32
        w.write_all(&self.tutorial_flag.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_TUTORIAL_FLAG {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_TUTORIAL_FLAG {
    fn maximum_possible_size() -> usize {
        4 // tutorial_flag: u32
    }
}

