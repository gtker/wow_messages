use std::io::{Read, Write};

use crate::shared::faction_initializer_vanilla_tbc::FactionInitializer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm#L9):
/// ```text
/// smsg SMSG_INITIALIZE_FACTIONS = 0x0122 {
///     u32 amount_of_factions;
///     FactionInitializer[amount_of_factions] factions;
/// }
/// ```
pub struct SMSG_INITIALIZE_FACTIONS {
    pub factions: Vec<FactionInitializer>,
}

impl crate::private::Sealed for SMSG_INITIALIZE_FACTIONS {}
impl crate::Message for SMSG_INITIALIZE_FACTIONS {
    const OPCODE: u32 = 0x0122;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INITIALIZE_FACTIONS {{").unwrap();
        // Members
        writeln!(s, "    amount_of_factions = {};", self.factions.len()).unwrap();
        write!(s, "    factions = [").unwrap();
        for v in self.factions.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        flag = {};", v.flag.as_test_case_value()).unwrap();
            writeln!(s, "        standing = {};", v.standing).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 290_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_factions", "    ");
        if !self.factions.is_empty() {
            writeln!(s, "    /* factions: FactionInitializer[amount_of_factions] start */").unwrap();
            for (i, v) in self.factions.iter().enumerate() {
                writeln!(s, "    /* factions: FactionInitializer[amount_of_factions] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "flag", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "standing", "        ");
                writeln!(s, "    /* factions: FactionInitializer[amount_of_factions] {i} end */").unwrap();
            }
            writeln!(s, "    /* factions: FactionInitializer[amount_of_factions] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0122, size: body_size });
        }

        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(&mut r)?;

        // factions: FactionInitializer[amount_of_factions]
        let factions = {
            let mut factions = Vec::with_capacity(amount_of_factions as usize);
            for _ in 0..amount_of_factions {
                factions.push(FactionInitializer::read(&mut r)?);
            }
            factions
        };

        Ok(Self {
            factions,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_INITIALIZE_FACTIONS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INITIALIZE_FACTIONS {}

impl SMSG_INITIALIZE_FACTIONS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.len() * 5 // factions: FactionInitializer[amount_of_factions]
    }
}

