use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_send_unlearn_spells.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_send_unlearn_spells.wowm#L8):
/// ```text
/// smsg SMSG_SEND_UNLEARN_SPELLS = 0x041E {
///     u32 amount_of_spells;
///     u32[amount_of_spells] spells;
/// }
/// ```
pub struct SMSG_SEND_UNLEARN_SPELLS {
    pub spells: Vec<u32>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SEND_UNLEARN_SPELLS {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SEND_UNLEARN_SPELLS {{").unwrap();
        // Members
        writeln!(s, "    amount_of_spells = {};", self.spells.len()).unwrap();
        write!(s, "    spells = [").unwrap();
        for v in self.spells.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1054_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_spells", "    ");
        if !self.spells.is_empty() {
            writeln!(s, "    /* spells: u32[amount_of_spells] start */").unwrap();
            for (i, v) in self.spells.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("spells {i}"), "    ");
            }
            writeln!(s, "    /* spells: u32[amount_of_spells] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_SEND_UNLEARN_SPELLS {}
impl crate::Message for SMSG_SEND_UNLEARN_SPELLS {
    const OPCODE: u32 = 0x041e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_SEND_UNLEARN_SPELLS::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: u32[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x041E, size: body_size });
        }

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(&mut r)?;

        // spells: u32[amount_of_spells]
        let spells = {
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for _ in 0..amount_of_spells {
                spells.push(crate::util::read_u32_le(&mut r)?);
            }
            spells
        };

        Ok(Self {
            spells,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SEND_UNLEARN_SPELLS {}

impl SMSG_SEND_UNLEARN_SPELLS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_spells: u32
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
    }
}

