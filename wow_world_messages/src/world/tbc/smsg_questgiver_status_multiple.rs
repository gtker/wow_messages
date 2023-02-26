use crate::tbc::QuestGiverStatusReport;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_QUESTGIVER_STATUS_MULTIPLE {
    const OPCODE: u32 = 0x0417;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_statuses: u32
        w.write_all(&(self.statuses.len() as u32).to_le_bytes())?;

        // statuses: QuestGiverStatusReport[amount_of_statuses]
        for i in self.statuses.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0417, size: body_size as u32 });
        }

        // amount_of_statuses: u32
        let amount_of_statuses = crate::util::read_u32_le(r)?;

        // statuses: QuestGiverStatusReport[amount_of_statuses]
        let statuses = {
            let mut statuses = Vec::with_capacity(amount_of_statuses as usize);
            for i in 0..amount_of_statuses {
                statuses.push(QuestGiverStatusReport::read(r)?);
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

