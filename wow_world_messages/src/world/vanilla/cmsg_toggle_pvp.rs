use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// vmangos: this opcode can be used in two ways: Either set explicit new status or toggle old status
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/cmsg_toggle_pvp.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/cmsg_toggle_pvp.wowm#L5):
/// ```text
/// cmsg CMSG_TOGGLE_PVP = 0x0253 {
///     optional set {
///         u8 enable_pvp;
///     }
/// }
/// ```
pub struct CMSG_TOGGLE_PVP {
    pub set: Option<CMSG_TOGGLE_PVP_set>,
}

impl crate::Message for CMSG_TOGGLE_PVP {
    const OPCODE: u32 = 0x0253;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // optional set
        if let Some(v) = &self.set {
            // enable_pvp: u8
            w.write_all(&v.enable_pvp.to_le_bytes())?;

        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // optional set
        let current_size = {
            0
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

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_TOGGLE_PVP {}

impl CMSG_TOGGLE_PVP {
    pub(crate) fn size(&self) -> usize {
        if let Some(set) = &self.set {
            1 // enable_pvp: u8
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_TOGGLE_PVP_set {
    pub enable_pvp: u8,
}

impl CMSG_TOGGLE_PVP_set {
    pub(crate) fn size(&self) -> usize {
        1 // enable_pvp: u8
    }

}

