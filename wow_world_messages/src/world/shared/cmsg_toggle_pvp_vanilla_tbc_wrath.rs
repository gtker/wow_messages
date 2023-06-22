use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// vmangos: this opcode can be used in two ways: Either set explicit new status or toggle old status
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

impl crate::private::Sealed for CMSG_TOGGLE_PVP {}
impl crate::Message for CMSG_TOGGLE_PVP {
    const OPCODE: u32 = 0x0253;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_TOGGLE_PVP {{").unwrap();
        // Members
        if let Some(set) = &self.set {
            writeln!(s, "    enable_pvp = {};", if set.enable_pvp { "TRUE" } else { "FALSE" }).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 595_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        if let Some(set) = &self.set {
            crate::util::write_bytes(&mut s, &mut bytes, 1, "enable_pvp", "    ");
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
        // optional set
        if let Some(v) = &self.set {
            // enable_pvp: Bool
            w.write_all(u8::from(v.enable_pvp).to_le_bytes().as_slice())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size > 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0253, size: body_size });
        }

        // optional set
        let current_size = {
            0
        };
        let set = if current_size < body_size as usize {
            // enable_pvp: Bool
            let enable_pvp = crate::util::read_u8_le(&mut r)? != 0;

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
    pub(crate) const fn size(&self) -> usize {
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

