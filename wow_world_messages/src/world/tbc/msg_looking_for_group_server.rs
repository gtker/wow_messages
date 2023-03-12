use std::io::{Read, Write};
use crate::tbc::LfgPlayer;
use crate::tbc::LfgType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L50):
/// ```text
/// smsg MSG_LOOKING_FOR_GROUP_Server = 0x01FF {
///     (u32)LfgType lfg_type;
///     u32 entry;
///     u32 amount_of_players_displayed;
///     u32 amount_of_players_found;
///     LfgPlayer[amount_of_players_displayed] players_displayed;
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Server {
    pub lfg_type: LfgType,
    /// entry from LfgDunggeons.dbc
    ///
    pub entry: u32,
    pub amount_of_players_found: u32,
    pub players_displayed: Vec<LfgPlayer>,
}

impl crate::Message for MSG_LOOKING_FOR_GROUP_Server {
    const OPCODE: u32 = 0x01ff;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // lfg_type: LfgType
        w.write_all(&u32::from(self.lfg_type.as_int()).to_le_bytes())?;

        // entry: u32
        w.write_all(&self.entry.to_le_bytes())?;

        // amount_of_players_displayed: u32
        w.write_all(&(self.players_displayed.len() as u32).to_le_bytes())?;

        // amount_of_players_found: u32
        w.write_all(&self.amount_of_players_found.to_le_bytes())?;

        // players_displayed: LfgPlayer[amount_of_players_displayed]
        for i in self.players_displayed.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(16..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FF, size: body_size as u32 });
        }

        // lfg_type: LfgType
        let lfg_type: LfgType = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // entry: u32
        let entry = crate::util::read_u32_le(&mut r)?;

        // amount_of_players_displayed: u32
        let amount_of_players_displayed = crate::util::read_u32_le(&mut r)?;

        // amount_of_players_found: u32
        let amount_of_players_found = crate::util::read_u32_le(&mut r)?;

        // players_displayed: LfgPlayer[amount_of_players_displayed]
        let players_displayed = {
            let mut players_displayed = Vec::with_capacity(amount_of_players_displayed as usize);
            for i in 0..amount_of_players_displayed {
                players_displayed.push(LfgPlayer::read(&mut r)?);
            }
            players_displayed
        };

        Ok(Self {
            lfg_type,
            entry,
            amount_of_players_found,
            players_displayed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_LOOKING_FOR_GROUP_Server {}

impl MSG_LOOKING_FOR_GROUP_Server {
    pub(crate) fn size(&self) -> usize {
        4 // lfg_type: LfgType
        + 4 // entry: u32
        + 4 // amount_of_players_displayed: u32
        + 4 // amount_of_players_found: u32
        + self.players_displayed.iter().fold(0, |acc, x| acc + x.size()) // players_displayed: LfgPlayer[amount_of_players_displayed]
    }
}

