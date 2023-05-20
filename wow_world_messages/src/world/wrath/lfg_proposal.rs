use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_proposal_update.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_proposal_update.wowm#L13):
/// ```text
/// struct LfgProposal {
///     u32 role_mask;
///     u8 is_current_player;
///     u8 in_dungeon;
///     u8 in_same_group;
///     u8 has_answered;
///     u8 has_accepted;
/// }
/// ```
pub struct LfgProposal {
    pub role_mask: u32,
    pub is_current_player: u8,
    pub in_dungeon: u8,
    pub in_same_group: u8,
    pub has_answered: u8,
    pub has_accepted: u8,
}

impl LfgProposal {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // role_mask: u32
        w.write_all(&self.role_mask.to_le_bytes())?;

        // is_current_player: u8
        w.write_all(&self.is_current_player.to_le_bytes())?;

        // in_dungeon: u8
        w.write_all(&self.in_dungeon.to_le_bytes())?;

        // in_same_group: u8
        w.write_all(&self.in_same_group.to_le_bytes())?;

        // has_answered: u8
        w.write_all(&self.has_answered.to_le_bytes())?;

        // has_accepted: u8
        w.write_all(&self.has_accepted.to_le_bytes())?;

        Ok(())
    }
}

impl LfgProposal {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // role_mask: u32
        let role_mask = crate::util::read_u32_le(&mut r)?;

        // is_current_player: u8
        let is_current_player = crate::util::read_u8_le(&mut r)?;

        // in_dungeon: u8
        let in_dungeon = crate::util::read_u8_le(&mut r)?;

        // in_same_group: u8
        let in_same_group = crate::util::read_u8_le(&mut r)?;

        // has_answered: u8
        let has_answered = crate::util::read_u8_le(&mut r)?;

        // has_accepted: u8
        let has_accepted = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            role_mask,
            is_current_player,
            in_dungeon,
            in_same_group,
            has_answered,
            has_accepted,
        })
    }

}

