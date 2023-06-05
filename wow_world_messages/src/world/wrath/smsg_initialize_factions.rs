use std::io::{Read, Write};

use crate::wrath::FactionInitializer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm#L27):
/// ```text
/// smsg SMSG_INITIALIZE_FACTIONS = 0x0122 {
///     u32 amount_of_factions;
///     FactionInitializer[amount_of_factions] factions;
/// }
/// ```
pub struct SMSG_INITIALIZE_FACTIONS {
    pub factions: Vec<FactionInitializer>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_INITIALIZE_FACTIONS {
    pub fn to_test_case_string(&self) -> String {
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
            writeln!(s, "    flag = {};", v.flag.as_test_case_value()).unwrap();
            writeln!(s, "    standing = {};", v.standing).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 290_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_factions");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_INITIALIZE_FACTIONS {}
impl crate::Message for SMSG_INITIALIZE_FACTIONS {
    const OPCODE: u32 = 0x0122;

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
        if !(4..=16777215).contains(&body_size) {
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INITIALIZE_FACTIONS {}

impl SMSG_INITIALIZE_FACTIONS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.len() * 5 // factions: FactionInitializer[amount_of_factions]
    }
}

