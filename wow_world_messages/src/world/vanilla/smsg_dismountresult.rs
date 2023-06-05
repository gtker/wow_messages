use std::io::{Read, Write};

use crate::vanilla::DismountResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm#L8):
/// ```text
/// smsg SMSG_DISMOUNTRESULT = 0x016F {
///     DismountResult result;
/// }
/// ```
pub struct SMSG_DISMOUNTRESULT {
    pub result: DismountResult,
}

#[cfg(feature = "print-testcase")]
impl SMSG_DISMOUNTRESULT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DISMOUNTRESULT {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 367_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_DISMOUNTRESULT {}
impl crate::Message for SMSG_DISMOUNTRESULT {
    const OPCODE: u32 = 0x016f;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_DISMOUNTRESULT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: DismountResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x016F, size: body_size });
        }

        // result: DismountResult
        let result = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DISMOUNTRESULT {}

