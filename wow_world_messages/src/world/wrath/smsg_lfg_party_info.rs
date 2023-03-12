use std::io::{Read, Write};

use crate::wrath::LfgPartyInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_party_info.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_party_info.wowm#L9):
/// ```text
/// smsg SMSG_LFG_PARTY_INFO = 0x0372 {
///     u8 amount_of_infos;
///     LfgPartyInfo[amount_of_infos] infos;
/// }
/// ```
pub struct SMSG_LFG_PARTY_INFO {
    pub infos: Vec<LfgPartyInfo>,
}

impl crate::Message for SMSG_LFG_PARTY_INFO {
    const OPCODE: u32 = 0x0372;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_infos: u8
        w.write_all(&(self.infos.len() as u8).to_le_bytes())?;

        // infos: LfgPartyInfo[amount_of_infos]
        for i in self.infos.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0372, size: body_size as u32 });
        }

        // amount_of_infos: u8
        let amount_of_infos = crate::util::read_u8_le(&mut r)?;

        // infos: LfgPartyInfo[amount_of_infos]
        let infos = {
            let mut infos = Vec::with_capacity(amount_of_infos as usize);
            for i in 0..amount_of_infos {
                infos.push(LfgPartyInfo::read(&mut r)?);
            }
            infos
        };

        Ok(Self {
            infos,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_PARTY_INFO {}

impl SMSG_LFG_PARTY_INFO {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_infos: u8
        + self.infos.iter().fold(0, |acc, x| acc + x.size()) // infos: LfgPartyInfo[amount_of_infos]
    }
}

