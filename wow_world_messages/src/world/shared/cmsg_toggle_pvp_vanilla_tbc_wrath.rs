use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// vmangos: this opcode can be used in two ways: Either set explicit new status or toggle old status
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/cmsg_toggle_pvp.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/cmsg_toggle_pvp.wowm#L3):
/// ```text
/// cmsg CMSG_TOGGLE_PVP = 0x0253 {
///     optional set {
///         Bool enable_pvp;
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
        let size_assert_header_size = w.len();
        // optional set
        if let Some(v) = &self.set {
            // enable_pvp: Bool
            w.write_all(u8::from(v.enable_pvp).to_le_bytes().as_slice())?;

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size > 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0253, size: body_size as u32 });
        }

        // optional set
        let current_size = {
            0
        };
        let set = if current_size < body_size as usize {
            // enable_pvp: Bool
            let enable_pvp = crate::util::read_u8_le(r)? != 0;
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
impl crate::vanilla::ClientMessage for CMSG_TOGGLE_PVP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TOGGLE_PVP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TOGGLE_PVP {}

impl CMSG_TOGGLE_PVP {
    pub(crate) fn size(&self) -> usize {
        if let Some(set) = &self.set {
            1 // enable_pvp: Bool
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_TOGGLE_PVP_set {
    pub enable_pvp: bool,
}

impl CMSG_TOGGLE_PVP_set {
    pub(crate) fn size(&self) -> usize {
        1 // enable_pvp: Bool
    }

}

