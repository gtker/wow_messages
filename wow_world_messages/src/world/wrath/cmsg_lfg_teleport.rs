use std::io::{Read, Write};

use crate::wrath::LfgTeleportLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_teleport.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_teleport.wowm#L8):
/// ```text
/// cmsg CMSG_LFG_TELEPORT = 0x0370 {
///     LfgTeleportLocation location;
/// }
/// ```
pub struct CMSG_LFG_TELEPORT {
    pub location: LfgTeleportLocation,
}

#[cfg(feature = "print-testcase")]
impl CMSG_LFG_TELEPORT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LFG_TELEPORT {{").unwrap();
        // Members
        writeln!(s, "    location = {};", self.location.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 880_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "location");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_LFG_TELEPORT {}
impl crate::Message for CMSG_LFG_TELEPORT {
    const OPCODE: u32 = 0x0370;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // location: LfgTeleportLocation
        w.write_all(&(self.location.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0370, size: body_size });
        }

        // location: LfgTeleportLocation
        let location = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            location,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_TELEPORT {}

