use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/9needs_optional/needs_optional.wowm:205`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/9needs_optional/needs_optional.wowm#L205):
/// ```text
/// cmsg CMSG_TOGGLE_PVP = 0x253 {
///     OPTIONAL-STATEMENT-DOCC: unimplemented
/// }
/// ```
pub struct CMSG_TOGGLE_PVP {
    pub set: Option<CMSG_TOGGLE_PVP_set>,
}

impl WorldClientMessageWrite for CMSG_TOGGLE_PVP {
    const OPCODE: u32 = 0x253;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_TOGGLE_PVP {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional set
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
        };
        let set = if current_size < body_size as usize {
            // enable_pvp: u8
            let enable_pvp = crate::util::read_u8_le(r)?;

            Some(CMSG_TOGGLE_PVP_set {
                enable_pvp,
            })
        } else {
            None
        };

        Ok(Self {
            set,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // optional set
        if let Some(v) = &self.set {
            // enable_pvp: u8
            w.write_all(&v.enable_pvp.to_le_bytes())?;

        }

        Ok(())
    }
}

impl VariableSized for CMSG_TOGGLE_PVP {
    fn size(&self) -> usize {
        {
            if let Some(v) = &self.set {
                v.size()
            } else {
                0
            }
        } // optional set
    }
}

impl MaximumPossibleSized for CMSG_TOGGLE_PVP {
    fn maximum_possible_size() -> usize {
        65536 // optional set
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_TOGGLE_PVP_set {
    pub enable_pvp: u8,
}

impl CMSG_TOGGLE_PVP_set {
    pub fn size(&self) -> usize {
        1 // enable_pvp: u8
    }
}

