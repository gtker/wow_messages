use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_set_action.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_set_action.wowm#L3):
/// ```text
/// cmsg CMSG_PET_SET_ACTION = 0x174 {
///     Guid guid;
///     u32 position1;
///     u32 data1;
///     OPTIONAL-STATEMENT-DOCC: unimplemented
/// }
/// ```
pub struct CMSG_PET_SET_ACTION {
    pub guid: Guid,
    pub position1: u32,
    pub data1: u32,
    pub extra: Option<CMSG_PET_SET_ACTION_extra>,
}

impl WorldClientMessageWrite for CMSG_PET_SET_ACTION {
    const OPCODE: u32 = 0x174;

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
impl WorldMessageBody for CMSG_PET_SET_ACTION {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // position1: u32
        let position1 = crate::util::read_u32_le(r)?;

        // data1: u32
        let data1 = crate::util::read_u32_le(r)?;

        // optional extra
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
            + 8 // guid: Guid
            + 4 // position1: u32
            + 4 // data1: u32
        };
        let extra = if current_size < body_size as usize {
            // position2: u32
            let position2 = crate::util::read_u32_le(r)?;

            // data2: u32
            let data2 = crate::util::read_u32_le(r)?;

            Some(CMSG_PET_SET_ACTION_extra {
                position2,
                data2,
            })
        } else {
            None
        };

        Ok(Self {
            guid,
            position1,
            data1,
            extra,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // position1: u32
        w.write_all(&self.position1.to_le_bytes())?;

        // data1: u32
        w.write_all(&self.data1.to_le_bytes())?;

        // optional extra
        if let Some(v) = &self.extra {
            // position2: u32
            w.write_all(&v.position2.to_le_bytes())?;

            // data2: u32
            w.write_all(&v.data2.to_le_bytes())?;

        }

        Ok(())
    }
}

impl VariableSized for CMSG_PET_SET_ACTION {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // position1: u32
        + 4 // data1: u32
        + {
            if let Some(v) = &self.extra {
                v.size()
            } else {
                0
            }
        } // optional extra
    }
}

impl MaximumPossibleSized for CMSG_PET_SET_ACTION {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // position1: u32
        + 4 // data1: u32
        + 65536 // optional extra
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_PET_SET_ACTION_extra {
    pub position2: u32,
    pub data2: u32,
}

impl CMSG_PET_SET_ACTION_extra {
    pub fn size(&self) -> usize {
        4 // position2: u32
        + 4 // data2: u32
    }
}

