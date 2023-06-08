use std::io::{Read, Write};

use crate::tbc::LfgData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_lfg.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_lfg.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_LFG = 0x036E {
///     LfgData[3] data;
/// }
/// ```
pub struct SMSG_LFG_UPDATE_LFG {
    pub data: [LfgData; 3],
}

impl crate::private::Sealed for SMSG_LFG_UPDATE_LFG {}
impl crate::Message for SMSG_LFG_UPDATE_LFG {
    const OPCODE: u32 = 0x036e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_UPDATE_LFG {{").unwrap();
        // Members
        write!(s, "    data = [").unwrap();
        for v in self.data.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        entry = {};", v.entry).unwrap();
            writeln!(s, "        lfg_type = {};", v.lfg_type.as_test_case_value()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 878_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    /* data: LfgData[3] start */").unwrap();
        for (i, v) in self.data.iter().enumerate() {
            writeln!(s, "    /* data: LfgData[3] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 2, "entry", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 2, "lfg_type", "        ");
            writeln!(s, "    /* data: LfgData[3] {i} end */").unwrap();
        }
        writeln!(s, "    /* data: LfgData[3] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // data: LfgData[3]
        for i in self.data.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036E, size: body_size });
        }

        // data: LfgData[3]
        let data = {
            let mut data = [LfgData::default(); 3];
            for i in data.iter_mut() {
                *i = LfgData::read(&mut r)?;
            }
            data
        };

        Ok(Self {
            data,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LFG_UPDATE_LFG {}

