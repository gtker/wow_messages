use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_abandon.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_abandon.wowm#L3):
/// ```text
/// cmsg CMSG_PET_ABANDON = 0x0176 {
///     Guid pet;
/// }
/// ```
pub struct CMSG_PET_ABANDON {
    pub pet: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_PET_ABANDON {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PET_ABANDON {{").unwrap();
        // Members
        writeln!(s, "    pet = {};", self.pet.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 374_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "pet");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_PET_ABANDON {}
impl crate::Message for CMSG_PET_ABANDON {
    const OPCODE: u32 = 0x0176;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0176, size: body_size });
        }

        // pet: Guid
        let pet = crate::util::read_guid(&mut r)?;

        Ok(Self {
            pet,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_ABANDON {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_ABANDON {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_ABANDON {}

