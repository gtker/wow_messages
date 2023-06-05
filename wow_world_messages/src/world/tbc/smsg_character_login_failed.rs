use std::io::{Read, Write};

use crate::tbc::WorldResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response if [`CMSG_PLAYER_LOGIN`](crate::vanilla::CMSG_PLAYER_LOGIN) fails. If successful it should instead be [`SMSG_LOGIN_VERIFY_WORLD`](crate::tbc::SMSG_LOGIN_VERIFY_WORLD).
///
/// Client seems to always send a [`CMSG_CANCEL_TRADE`](crate::vanilla::CMSG_CANCEL_TRADE) after receiving this message, for unknown reasons.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm#L1):
/// ```text
/// smsg SMSG_CHARACTER_LOGIN_FAILED = 0x0041 {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHARACTER_LOGIN_FAILED {
    pub result: WorldResult,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CHARACTER_LOGIN_FAILED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHARACTER_LOGIN_FAILED {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 65_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_CHARACTER_LOGIN_FAILED {}
impl crate::Message for SMSG_CHARACTER_LOGIN_FAILED {
    const OPCODE: u32 = 0x0041;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0041, size: body_size });
        }

        // result: WorldResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHARACTER_LOGIN_FAILED {}

