use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm#L1):
/// ```text
/// struct QuestObjective {
///     u32 creature_id;
///     u32 kill_count;
///     u32 required_item_id;
///     u32 required_item_count;
/// }
/// ```
pub struct QuestObjective {
    /// cmangos: client expected gameobject template id in form (id|0x80000000)
    ///
    pub creature_id: u32,
    pub kill_count: u32,
    pub required_item_id: u32,
    pub required_item_count: u32,
}

impl QuestObjective {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // creature_id: u32
        w.write_all(&self.creature_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_item_id: u32
        w.write_all(&self.required_item_id.to_le_bytes())?;

        // required_item_count: u32
        w.write_all(&self.required_item_count.to_le_bytes())?;

        Ok(())
    }
}

impl QuestObjective {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // creature_id: u32
        let creature_id = crate::util::read_u32_le(r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(r)?;

        // required_item_id: u32
        let required_item_id = crate::util::read_u32_le(r)?;

        // required_item_count: u32
        let required_item_count = crate::util::read_u32_le(r)?;

        Ok(Self {
            creature_id,
            kill_count,
            required_item_id,
            required_item_count,
        })
    }

}

