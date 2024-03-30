use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::wrath::{
    QuestCompletable, QuestItemRequirement,
};

/// mangoszero/vmangos: Quests that don't require items use the `RequestItemsText` field to store the text that is shown when you talk to the quest giver while the quest is incomplete. Therefore the text should not be shown for them when the quest is complete. For quests that do require items, it is self explanatory.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm:64`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm#L64):
/// ```text
/// smsg SMSG_QUESTGIVER_REQUEST_ITEMS = 0x018B {
///     Guid npc;
///     u32 quest_id;
///     CString title;
///     CString request_items_text;
///     u32 emote_delay;
///     u32 emote;
///     Bool32 auto_finish;
///     u32 flags1;
///     u32 suggested_players;
///     Gold required_money;
///     u32 amount_of_required_items;
///     QuestItemRequirement[amount_of_required_items] required_items;
///     QuestCompletable completable;
///     u32 flags2;
///     u32 flags3;
///     u32 flags4;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_QUESTGIVER_REQUEST_ITEMS {
    pub npc: Guid,
    pub quest_id: u32,
    pub title: String,
    pub request_items_text: String,
    pub emote_delay: u32,
    pub emote: u32,
    pub auto_finish: bool,
    /// mangostwo: 3.3.3 questFlags
    pub flags1: u32,
    pub suggested_players: u32,
    pub required_money: Gold,
    pub required_items: Vec<QuestItemRequirement>,
    pub completable: QuestCompletable,
    /// mangostwo: set to 0x04
    pub flags2: u32,
    /// mangostwo: set to 0x08
    pub flags3: u32,
    /// mangostwo: set to 0x10
    pub flags4: u32,
}

impl crate::private::Sealed for SMSG_QUESTGIVER_REQUEST_ITEMS {}
impl SMSG_QUESTGIVER_REQUEST_ITEMS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(58..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // request_items_text: CString
        let request_items_text = {
            let request_items_text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(request_items_text)?
        };

        // emote_delay: u32
        let emote_delay = crate::util::read_u32_le(&mut r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(&mut r)?;

        // auto_finish: Bool32
        let auto_finish = crate::util::read_bool_u32(&mut r)?;

        // flags1: u32
        let flags1 = crate::util::read_u32_le(&mut r)?;

        // suggested_players: u32
        let suggested_players = crate::util::read_u32_le(&mut r)?;

        // required_money: Gold
        let required_money = Gold::new(crate::util::read_u32_le(&mut r)?);

        // amount_of_required_items: u32
        let amount_of_required_items = crate::util::read_u32_le(&mut r)?;

        // required_items: QuestItemRequirement[amount_of_required_items]
        let required_items = {
            let mut required_items = Vec::with_capacity(amount_of_required_items as usize);

            let allocation_size = u64::from(amount_of_required_items) * 12;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_required_items {
                required_items.push(QuestItemRequirement::read(&mut r)?);
            }
            required_items
        };

        // completable: QuestCompletable
        let completable = crate::util::read_u32_le(&mut r)?.try_into()?;

        // flags2: u32
        let flags2 = crate::util::read_u32_le(&mut r)?;

        // flags3: u32
        let flags3 = crate::util::read_u32_le(&mut r)?;

        // flags4: u32
        let flags4 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            npc,
            quest_id,
            title,
            request_items_text,
            emote_delay,
            emote,
            auto_finish,
            flags1,
            suggested_players,
            required_money,
            required_items,
            completable,
            flags2,
            flags3,
            flags4,
        })
    }

}

impl crate::Message for SMSG_QUESTGIVER_REQUEST_ITEMS {
    const OPCODE: u32 = 0x018b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUESTGIVER_REQUEST_ITEMS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_REQUEST_ITEMS {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    request_items_text = \"{}\";", self.request_items_text).unwrap();
        writeln!(s, "    emote_delay = {};", self.emote_delay).unwrap();
        writeln!(s, "    emote = {};", self.emote).unwrap();
        writeln!(s, "    auto_finish = {};", if self.auto_finish { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    flags1 = {};", self.flags1).unwrap();
        writeln!(s, "    suggested_players = {};", self.suggested_players).unwrap();
        writeln!(s, "    required_money = {};", self.required_money.as_int()).unwrap();
        writeln!(s, "    amount_of_required_items = {};", self.required_items.len()).unwrap();
        writeln!(s, "    required_items = [").unwrap();
        for v in self.required_items.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            item = {};", v.item).unwrap();
            writeln!(s, "            item_count = {};", v.item_count).unwrap();
            writeln!(s, "            item_display_id = {};", v.item_display_id).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    completable = {};", self.completable.as_test_case_value()).unwrap();
        writeln!(s, "    flags2 = {};", self.flags2).unwrap();
        writeln!(s, "    flags3 = {};", self.flags3).unwrap();
        writeln!(s, "    flags4 = {};", self.flags4).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 395_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.request_items_text.len() + 1, "request_items_text", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emote_delay", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auto_finish", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "suggested_players", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "required_money", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_required_items", "    ");
        if !self.required_items.is_empty() {
            writeln!(s, "    /* required_items: QuestItemRequirement[amount_of_required_items] start */").unwrap();
            for (i, v) in self.required_items.iter().enumerate() {
                writeln!(s, "    /* required_items: QuestItemRequirement[amount_of_required_items] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_display_id", "        ");
                writeln!(s, "    /* required_items: QuestItemRequirement[amount_of_required_items] {i} end */").unwrap();
            }
            writeln!(s, "    /* required_items: QuestItemRequirement[amount_of_required_items] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "completable", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags3", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags4", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().next_back(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // request_items_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.request_items_text.as_bytes().iter().next_back(), Some(&0_u8), "String `request_items_text` must not be null-terminated.");
        w.write_all(self.request_items_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // emote_delay: u32
        w.write_all(&self.emote_delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // auto_finish: Bool32
        w.write_all(u32::from(self.auto_finish).to_le_bytes().as_slice())?;

        // flags1: u32
        w.write_all(&self.flags1.to_le_bytes())?;

        // suggested_players: u32
        w.write_all(&self.suggested_players.to_le_bytes())?;

        // required_money: Gold
        w.write_all((self.required_money.as_int()).to_le_bytes().as_slice())?;

        // amount_of_required_items: u32
        w.write_all(&(self.required_items.len() as u32).to_le_bytes())?;

        // required_items: QuestItemRequirement[amount_of_required_items]
        for i in self.required_items.iter() {
            i.write_into_vec(&mut w)?;
        }

        // completable: QuestCompletable
        w.write_all(&(self.completable.as_int().to_le_bytes()))?;

        // flags2: u32
        w.write_all(&self.flags2.to_le_bytes())?;

        // flags3: u32
        w.write_all(&self.flags3.to_le_bytes())?;

        // flags4: u32
        w.write_all(&self.flags4.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(395, "SMSG_QUESTGIVER_REQUEST_ITEMS", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTGIVER_REQUEST_ITEMS {}

impl SMSG_QUESTGIVER_REQUEST_ITEMS {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.request_items_text.len() + 1 // request_items_text: CString
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 4 // auto_finish: Bool32
        + 4 // flags1: u32
        + 4 // suggested_players: u32
        + 4 // required_money: Gold
        + 4 // amount_of_required_items: u32
        + self.required_items.len() * 12 // required_items: QuestItemRequirement[amount_of_required_items]
        + 4 // completable: QuestCompletable
        + 4 // flags2: u32
        + 4 // flags3: u32
        + 4 // flags4: u32
    }
}

