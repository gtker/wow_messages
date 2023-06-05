use std::io::{Read, Write};

use crate::tbc::LfgData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_set_looking_for_group.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_set_looking_for_group.wowm#L1):
/// ```text
/// cmsg CMSG_SET_LOOKING_FOR_GROUP = 0x0200 {
///     u32 slot;
///     LfgData data;
/// }
/// ```
pub struct CMSG_SET_LOOKING_FOR_GROUP {
    pub slot: u32,
    pub data: LfgData,
}

#[cfg(feature = "print-testcase")]
impl CMSG_SET_LOOKING_FOR_GROUP {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_LOOKING_FOR_GROUP {{").unwrap();
        // Members
        writeln!(s, "    slot = {};", self.slot).unwrap();
        // data: LfgData
        writeln!(s, "    data = {{").unwrap();
        // Members
        writeln!(s, "    entry = {};", self.data.entry).unwrap();
        writeln!(s, "    lfg_type = {};", self.data.lfg_type.as_test_case_value()).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 512_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "slot");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_SET_LOOKING_FOR_GROUP {}
impl crate::Message for CMSG_SET_LOOKING_FOR_GROUP {
    const OPCODE: u32 = 0x0200;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // data: LfgData
        self.data.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0200, size: body_size });
        }

        // slot: u32
        let slot = crate::util::read_u32_le(&mut r)?;

        // data: LfgData
        let data = LfgData::read(&mut r)?;

        Ok(Self {
            slot,
            data,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_LOOKING_FOR_GROUP {}

