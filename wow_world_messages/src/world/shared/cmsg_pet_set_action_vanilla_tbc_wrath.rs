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
impl CMSG_PET_SET_ACTION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=24).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for CMSG_PET_SET_ACTION {
    const OPCODE: u32 = 0x0174;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PET_SET_ACTION {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    position1 = {};", self.position1).unwrap();
        writeln!(s, "    data1 = {};", self.data1).unwrap();
        if let Some(extra) = &self.extra {
            writeln!(s, "    position2 = {};", extra.position2).unwrap();
            writeln!(s, "    data2 = {};", extra.data2).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 372_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "position1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "data1", "    ");
        if let Some(extra) = &self.extra {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "position2", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "data2", "    ");
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(372, "CMSG_PET_SET_ACTION", body_size, a))
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

