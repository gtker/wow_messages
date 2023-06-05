use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_set_action.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_set_action.wowm#L3):
/// ```text
/// cmsg CMSG_PET_SET_ACTION = 0x0174 {
///     Guid guid;
///     u32 position1;
///     u32 data1;
///     optional extra {
///         u32 position2;
///         u32 data2;
///     }
/// }
/// ```
pub struct CMSG_PET_SET_ACTION {
    pub guid: Guid,
    pub position1: u32,
    pub data1: u32,
    pub extra: Option<CMSG_PET_SET_ACTION_extra>,
}

impl crate::private::Sealed for CMSG_PET_SET_ACTION {}
impl crate::Message for CMSG_PET_SET_ACTION {
    const OPCODE: u32 = 0x0174;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(16..=24).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0174, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // position1: u32
        let position1 = crate::util::read_u32_le(&mut r)?;

        // data1: u32
        let data1 = crate::util::read_u32_le(&mut r)?;

        // optional extra
        let current_size = {
            8 // guid: Guid
            + 4 // position1: u32
            + 4 // data1: u32
        };
        let extra = if current_size < body_size as usize {
            // position2: u32
            let position2 = crate::util::read_u32_le(&mut r)?;

            // data2: u32
            let data2 = crate::util::read_u32_le(&mut r)?;

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

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_SET_ACTION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_SET_ACTION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_SET_ACTION {}

impl CMSG_PET_SET_ACTION {
    pub(crate) const fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // position1: u32
        + 4 // data1: u32
        + if let Some(extra) = &self.extra {
            4 // position2: u32
            + 4 // data2: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_PET_SET_ACTION_extra {
    pub position2: u32,
    pub data2: u32,
}

