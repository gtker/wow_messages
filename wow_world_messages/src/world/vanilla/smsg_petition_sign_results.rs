use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::PetitionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm#L22):
/// ```text
/// smsg SMSG_PETITION_SIGN_RESULTS = 0x01C1 {
///     Guid petition;
///     Guid owner;
///     PetitionResult result;
/// }
/// ```
pub struct SMSG_PETITION_SIGN_RESULTS {
    pub petition: Guid,
    pub owner: Guid,
    pub result: PetitionResult,
}

impl crate::private::Sealed for SMSG_PETITION_SIGN_RESULTS {}
impl SMSG_PETITION_SIGN_RESULTS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C1, size: body_size });
        }

        // petition: Guid
        let petition = crate::util::read_guid(&mut r)?;

        // owner: Guid
        let owner = crate::util::read_guid(&mut r)?;

        // result: PetitionResult
        let result = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            petition,
            owner,
            result,
        })
    }

}

impl crate::Message for SMSG_PETITION_SIGN_RESULTS {
    const OPCODE: u32 = 0x01c1;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PETITION_SIGN_RESULTS {{").unwrap();
        // Members
        writeln!(s, "    petition = {};", self.petition.guid()).unwrap();
        writeln!(s, "    owner = {};", self.owner.guid()).unwrap();
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 22_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 449_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "petition", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "owner", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // owner: Guid
        w.write_all(&self.owner.guid().to_le_bytes())?;

        // result: PetitionResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PETITION_SIGN_RESULTS {}

