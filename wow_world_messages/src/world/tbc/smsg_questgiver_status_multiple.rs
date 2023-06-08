use std::io::{Read, Write};

use crate::tbc::QuestGiverStatusReport;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_status_multiple.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_status_multiple.wowm#L8):
/// ```text
/// smsg SMSG_QUESTGIVER_STATUS_MULTIPLE = 0x0417 {
///     u32 amount_of_statuses;
///     QuestGiverStatusReport[amount_of_statuses] statuses;
/// }
/// ```
pub struct SMSG_QUESTGIVER_STATUS_MULTIPLE {
    pub statuses: Vec<QuestGiverStatusReport>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_QUESTGIVER_STATUS_MULTIPLE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_STATUS_MULTIPLE {{").unwrap();
        // Members
        writeln!(s, "    amount_of_statuses = {};", self.statuses.len()).unwrap();
        write!(s, "    statuses = [").unwrap();
        for v in self.statuses.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        npc = {};", v.npc.guid()).unwrap();
            writeln!(s, "        dialog_status = {};", v.dialog_status.as_test_case_value()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1047_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_statuses", "    ");
        if !self.statuses.is_empty() {
            writeln!(s, "    /* statuses: QuestGiverStatusReport[amount_of_statuses] start */").unwrap();
            for (i, v) in self.statuses.iter().enumerate() {
                writeln!(s, "    /* statuses: QuestGiverStatusReport[amount_of_statuses] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "dialog_status", "        ");
                writeln!(s, "    /* statuses: QuestGiverStatusReport[amount_of_statuses] {i} end */").unwrap();
            }
            writeln!(s, "    /* statuses: QuestGiverStatusReport[amount_of_statuses] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_QUESTGIVER_STATUS_MULTIPLE {}
impl crate::Message for SMSG_QUESTGIVER_STATUS_MULTIPLE {
    const OPCODE: u32 = 0x0417;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_QUESTGIVER_STATUS_MULTIPLE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_statuses: u32
        w.write_all(&(self.statuses.len() as u32).to_le_bytes())?;

        // statuses: QuestGiverStatusReport[amount_of_statuses]
        for i in self.statuses.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0417, size: body_size });
        }

        // amount_of_statuses: u32
        let amount_of_statuses = crate::util::read_u32_le(&mut r)?;

        // statuses: QuestGiverStatusReport[amount_of_statuses]
        let statuses = {
            let mut statuses = Vec::with_capacity(amount_of_statuses as usize);
            for _ in 0..amount_of_statuses {
                statuses.push(QuestGiverStatusReport::read(&mut r)?);
            }
            statuses
        };

        Ok(Self {
            statuses,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTGIVER_STATUS_MULTIPLE {}

impl SMSG_QUESTGIVER_STATUS_MULTIPLE {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_statuses: u32
        + self.statuses.len() * 9 // statuses: QuestGiverStatusReport[amount_of_statuses]
    }
}

