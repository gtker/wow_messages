use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_unlearn_confirm.wowm#L3):
/// ```text
/// smsg SMSG_PET_UNLEARN_CONFIRM = 0x02F1 {
///     Guid pet;
///     u32 talent_reset_cost;
/// }
/// ```
pub struct SMSG_PET_UNLEARN_CONFIRM {
    pub pet: Guid,
    pub talent_reset_cost: u32,
}

impl crate::private::Sealed for SMSG_PET_UNLEARN_CONFIRM {}
impl SMSG_PET_UNLEARN_CONFIRM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // pet: Guid
        let pet = crate::util::read_guid(&mut r)?;

        // talent_reset_cost: u32
        let talent_reset_cost = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            pet,
            talent_reset_cost,
        })
    }

}

impl crate::Message for SMSG_PET_UNLEARN_CONFIRM {
    const OPCODE: u32 = 0x02f1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PET_UNLEARN_CONFIRM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_UNLEARN_CONFIRM {{").unwrap();
        // Members
        writeln!(s, "    pet = {};", self.pet.guid()).unwrap();
        writeln!(s, "    talent_reset_cost = {};", self.talent_reset_cost).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 753_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "pet", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "talent_reset_cost", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // talent_reset_cost: u32
        w.write_all(&self.talent_reset_cost.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(753, "SMSG_PET_UNLEARN_CONFIRM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_UNLEARN_CONFIRM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_UNLEARN_CONFIRM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_UNLEARN_CONFIRM {}

