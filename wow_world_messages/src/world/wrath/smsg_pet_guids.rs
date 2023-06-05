use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_guids.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_guids.wowm#L1):
/// ```text
/// smsg SMSG_PET_GUIDS = 0x04AA {
///     u32 amount_of_guids;
///     Guid[amount_of_guids] guids;
/// }
/// ```
pub struct SMSG_PET_GUIDS {
    pub guids: Vec<Guid>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_PET_GUIDS {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_GUIDS {{").unwrap();
        // Members
        writeln!(s, "    amount_of_guids = {};", self.guids.len()).unwrap();
        write!(s, "    guids = [").unwrap();
        for v in self.guids.as_slice() {
            write!(s, "{v:#08X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1194_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_guids", "    ");
        if !self.guids.is_empty() {
            writeln!(s, "    /* guids: Guid[amount_of_guids] start */").unwrap();
            for (i, v) in self.guids.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("guids {i}"), "    ");
            }
            writeln!(s, "    /* guids: Guid[amount_of_guids] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_PET_GUIDS {}
impl crate::Message for SMSG_PET_GUIDS {
    const OPCODE: u32 = 0x04aa;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_PET_GUIDS::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_guids: u32
        w.write_all(&(self.guids.len() as u32).to_le_bytes())?;

        // guids: Guid[amount_of_guids]
        for i in self.guids.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04AA, size: body_size });
        }

        // amount_of_guids: u32
        let amount_of_guids = crate::util::read_u32_le(&mut r)?;

        // guids: Guid[amount_of_guids]
        let guids = {
            let mut guids = Vec::with_capacity(amount_of_guids as usize);
            for _ in 0..amount_of_guids {
                guids.push(crate::util::read_guid(&mut r)?);
            }
            guids
        };

        Ok(Self {
            guids,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_GUIDS {}

impl SMSG_PET_GUIDS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_guids: u32
        + self.guids.len() *  8 // guids: Guid[amount_of_guids]
    }
}

