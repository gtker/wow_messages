use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_send_unlearn_spells.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_send_unlearn_spells.wowm#L8):
/// ```text
/// smsg SMSG_SEND_UNLEARN_SPELLS = 0x041E {
///     u32 amount_of_spells;
///     Spell[amount_of_spells] spells;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SEND_UNLEARN_SPELLS {
    pub spells: Vec<u32>,
}

impl crate::private::Sealed for SMSG_SEND_UNLEARN_SPELLS {}
impl SMSG_SEND_UNLEARN_SPELLS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(&mut r)?;

        // spells: Spell[amount_of_spells]
        let spells = {
            let mut spells = Vec::with_capacity(amount_of_spells as usize);

            let allocation_size = u64::from(amount_of_spells) * 4;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

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

impl crate::Message for SMSG_SEND_UNLEARN_SPELLS {
    const OPCODE: u32 = 0x041e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SEND_UNLEARN_SPELLS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SEND_UNLEARN_SPELLS {{").unwrap();
        // Members
        writeln!(s, "    amount_of_spells = {};", self.spells.len()).unwrap();
        writeln!(s, "    spells = [").unwrap();
        for v in self.spells.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

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
            writeln!(s, "    /* spells: Spell[amount_of_spells] start */").unwrap();
            for (i, v) in self.spells.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("spells {i}"), "    ");
            }
            writeln!(s, "    /* spells: Spell[amount_of_spells] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: Spell[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1054, "SMSG_SEND_UNLEARN_SPELLS", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SEND_UNLEARN_SPELLS {}

impl SMSG_SEND_UNLEARN_SPELLS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_spells: u32
        + self.spells.len() * 4 // spells: Spell[amount_of_spells]
    }
}

