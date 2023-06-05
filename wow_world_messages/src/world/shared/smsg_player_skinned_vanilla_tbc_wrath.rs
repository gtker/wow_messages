use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_player_skinned.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_player_skinned.wowm#L3):
/// ```text
/// smsg SMSG_PLAYER_SKINNED = 0x02BC {
///     Bool spirit_released;
/// }
/// ```
pub struct SMSG_PLAYER_SKINNED {
    pub spirit_released: bool,
}

#[cfg(feature = "print-testcase")]
impl SMSG_PLAYER_SKINNED {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PLAYER_SKINNED {{").unwrap();
        // Members
        writeln!(s, "    spirit_released = {};", if self.spirit_released { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 3_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 700_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "spirit_released", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_PLAYER_SKINNED {}
impl crate::Message for SMSG_PLAYER_SKINNED {
    const OPCODE: u32 = 0x02bc;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_PLAYER_SKINNED::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spirit_released: Bool
        w.write_all(u8::from(self.spirit_released).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02BC, size: body_size });
        }

        // spirit_released: Bool
        let spirit_released = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            spirit_released,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAYER_SKINNED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAYER_SKINNED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAYER_SKINNED {}

