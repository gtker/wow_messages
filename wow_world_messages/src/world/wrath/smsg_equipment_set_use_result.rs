use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_equipment_set_use_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_equipment_set_use_result.wowm#L1):
/// ```text
/// smsg SMSG_EQUIPMENT_SET_USE_RESULT = 0x04D6 {
///     u8 result;
/// }
/// ```
pub struct SMSG_EQUIPMENT_SET_USE_RESULT {
    pub result: u8,
}

#[cfg(feature = "print-testcase")]
impl SMSG_EQUIPMENT_SET_USE_RESULT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_EQUIPMENT_SET_USE_RESULT {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1238_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result");
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

impl crate::private::Sealed for SMSG_EQUIPMENT_SET_USE_RESULT {}
impl crate::Message for SMSG_EQUIPMENT_SET_USE_RESULT {
    const OPCODE: u32 = 0x04d6;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: u8
        w.write_all(&self.result.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D6, size: body_size });
        }

        // result: u8
        let result = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_EQUIPMENT_SET_USE_RESULT {}

