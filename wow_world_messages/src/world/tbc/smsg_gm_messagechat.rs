use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    ChatType, Language, NamedGuid, PlayerChatTag,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_gm_messagechat.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_gm_messagechat.wowm#L1):
/// ```text
/// smsg SMSG_GM_MESSAGECHAT = 0x03B2 {
///     ChatType chat_type;
///     (u32)Language language;
///     if (chat_type == MONSTER_SAY
///         || chat_type == MONSTER_PARTY
///         || chat_type == MONSTER_YELL
///         || chat_type == MONSTER_WHISPER
///         || chat_type == RAID_BOSS_WHISPER
///         || chat_type == RAID_BOSS_EMOTE
///         || chat_type == MONSTER_EMOTE) {
///         SizedCString sender;
///         NamedGuid target1;
///         SizedCString message1;
///         PlayerChatTag chat_tag1;
///     }
///     else if (chat_type == BG_SYSTEM_NEUTRAL
///         || chat_type == BG_SYSTEM_ALLIANCE
///         || chat_type == BG_SYSTEM_HORDE) {
///         NamedGuid target2;
///         SizedCString message2;
///         PlayerChatTag chat_tag2;
///     }
///     else if (chat_type == CHANNEL) {
///         CString channel_name;
///         Guid target4;
///         SizedCString message3;
///         PlayerChatTag chat_tag3;
///     }
///     else {
///         Guid target5;
///         SizedCString message4;
///         PlayerChatTag chat_tag4;
///         SizedCString sender_name;
///     }
/// }
/// ```
pub struct SMSG_GM_MESSAGECHAT {
    pub chat_type: SMSG_GM_MESSAGECHAT_ChatType,
    pub language: Language,
}

impl crate::private::Sealed for SMSG_GM_MESSAGECHAT {}
impl crate::Message for SMSG_GM_MESSAGECHAT {
    const OPCODE: u32 = 0x03b2;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GM_MESSAGECHAT {{").unwrap();
        // Members
        writeln!(s, "    chat_type = {};", crate::tbc::ChatType::try_from(self.chat_type.as_int()).unwrap().as_test_case_value()).unwrap();
        writeln!(s, "    language = {};", self.language.as_test_case_value()).unwrap();
        match &self.chat_type {
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::System {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Say {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Party {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Raid {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Guild {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Officer {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Yell {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Whisper {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::WhisperInform {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Reply {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Emote {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::TextEmote {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterSay {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                writeln!(s, "    sender = \"{}\";", sender).unwrap();
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target1'");
                writeln!(s, "    message1 = \"{}\";", message1).unwrap();
                writeln!(s, "    chat_tag1 = {};", chat_tag1.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterParty {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                writeln!(s, "    sender = \"{}\";", sender).unwrap();
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target1'");
                writeln!(s, "    message1 = \"{}\";", message1).unwrap();
                writeln!(s, "    chat_tag1 = {};", chat_tag1.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterYell {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                writeln!(s, "    sender = \"{}\";", sender).unwrap();
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target1'");
                writeln!(s, "    message1 = \"{}\";", message1).unwrap();
                writeln!(s, "    chat_tag1 = {};", chat_tag1.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterWhisper {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                writeln!(s, "    sender = \"{}\";", sender).unwrap();
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target1'");
                writeln!(s, "    message1 = \"{}\";", message1).unwrap();
                writeln!(s, "    chat_tag1 = {};", chat_tag1.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterEmote {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                writeln!(s, "    sender = \"{}\";", sender).unwrap();
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target1'");
                writeln!(s, "    message1 = \"{}\";", message1).unwrap();
                writeln!(s, "    chat_tag1 = {};", chat_tag1.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Channel {
                channel_name,
                chat_tag3,
                message3,
                target4,
            } => {
                writeln!(s, "    channel_name = \"{}\";", channel_name).unwrap();
                writeln!(s, "    target4 = {};", target4.guid()).unwrap();
                writeln!(s, "    message3 = \"{}\";", message3).unwrap();
                writeln!(s, "    chat_tag3 = {};", chat_tag3.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelJoin {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelLeave {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelList {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelNotice {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelNoticeUser {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Afk {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Dnd {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Ignored {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Skill {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Loot {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Money {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Opening {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Tradeskills {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::PetInfo {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatMiscInfo {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatXpGain {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatHonorGain {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatFactionChange {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BgSystemNeutral {
                chat_tag2,
                message2,
                target2,
            } => {
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target2'");
                writeln!(s, "    message2 = \"{}\";", message2).unwrap();
                writeln!(s, "    chat_tag2 = {};", chat_tag2.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BgSystemAlliance {
                chat_tag2,
                message2,
                target2,
            } => {
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target2'");
                writeln!(s, "    message2 = \"{}\";", message2).unwrap();
                writeln!(s, "    chat_tag2 = {};", chat_tag2.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BgSystemHorde {
                chat_tag2,
                message2,
                target2,
            } => {
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target2'");
                writeln!(s, "    message2 = \"{}\";", message2).unwrap();
                writeln!(s, "    chat_tag2 = {};", chat_tag2.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidLeader {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidWarning {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidBossWhisper {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                writeln!(s, "    sender = \"{}\";", sender).unwrap();
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target1'");
                writeln!(s, "    message1 = \"{}\";", message1).unwrap();
                writeln!(s, "    chat_tag1 = {};", chat_tag1.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidBossEmote {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                writeln!(s, "    sender = \"{}\";", sender).unwrap();
                panic!("unsupported type for test case printing: 'NamedGuid' for variable 'target1'");
                writeln!(s, "    message1 = \"{}\";", message1).unwrap();
                writeln!(s, "    chat_tag1 = {};", chat_tag1.as_test_case_value()).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Filtered {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Battleground {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BattlegroundLeader {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Restricted {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                writeln!(s, "    target5 = {};", target5.guid()).unwrap();
                writeln!(s, "    message4 = \"{}\";", message4).unwrap();
                writeln!(s, "    chat_tag4 = {};", chat_tag4.as_test_case_value()).unwrap();
                writeln!(s, "    sender_name = \"{}\";", sender_name).unwrap();
            }
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 946_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "language", "    ");
        match &self.chat_type {
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::System {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Say {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Party {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Raid {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Guild {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Officer {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Yell {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Whisper {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::WhisperInform {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Reply {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Emote {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::TextEmote {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterSay {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender.len() + 5, "sender", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
                crate::util::write_bytes(&mut s, &mut bytes, message1.len() + 5, "message1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag1", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterParty {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender.len() + 5, "sender", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
                crate::util::write_bytes(&mut s, &mut bytes, message1.len() + 5, "message1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag1", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterYell {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender.len() + 5, "sender", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
                crate::util::write_bytes(&mut s, &mut bytes, message1.len() + 5, "message1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag1", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterWhisper {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender.len() + 5, "sender", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
                crate::util::write_bytes(&mut s, &mut bytes, message1.len() + 5, "message1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag1", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::MonsterEmote {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender.len() + 5, "sender", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
                crate::util::write_bytes(&mut s, &mut bytes, message1.len() + 5, "message1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag1", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Channel {
                channel_name,
                chat_tag3,
                message3,
                target4,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, channel_name.len() + 1, "channel_name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message3.len() + 5, "message3", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag3", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelJoin {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelLeave {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelList {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelNotice {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::ChannelNoticeUser {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Afk {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Dnd {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Ignored {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Skill {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Loot {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Money {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Opening {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Tradeskills {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::PetInfo {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatMiscInfo {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatXpGain {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatHonorGain {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::CombatFactionChange {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BgSystemNeutral {
                chat_tag2,
                message2,
                target2,
            } => {
                panic!("unsupported type NamedGuid for variable 'target2'");
                crate::util::write_bytes(&mut s, &mut bytes, message2.len() + 5, "message2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag2", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BgSystemAlliance {
                chat_tag2,
                message2,
                target2,
            } => {
                panic!("unsupported type NamedGuid for variable 'target2'");
                crate::util::write_bytes(&mut s, &mut bytes, message2.len() + 5, "message2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag2", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BgSystemHorde {
                chat_tag2,
                message2,
                target2,
            } => {
                panic!("unsupported type NamedGuid for variable 'target2'");
                crate::util::write_bytes(&mut s, &mut bytes, message2.len() + 5, "message2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag2", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidLeader {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidWarning {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidBossWhisper {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender.len() + 5, "sender", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
                crate::util::write_bytes(&mut s, &mut bytes, message1.len() + 5, "message1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag1", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::RaidBossEmote {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, sender.len() + 5, "sender", "    ");
                panic!("unsupported type NamedGuid for variable 'target1'");
                crate::util::write_bytes(&mut s, &mut bytes, message1.len() + 5, "message1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag1", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Filtered {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Battleground {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::BattlegroundLeader {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
            crate::tbc::SMSG_GM_MESSAGECHAT_ChatType::Restricted {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target5", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, message4.len() + 5, "message4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "chat_tag4", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, sender_name.len() + 5, "sender_name", "    ");
            }
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // chat_type: ChatType
        w.write_all(&(self.chat_type.as_int().to_le_bytes()))?;

        // language: Language
        w.write_all(&u32::from(self.language.as_int()).to_le_bytes())?;

        match &self.chat_type {
            SMSG_GM_MESSAGECHAT_ChatType::System {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Say {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Party {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Raid {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Guild {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Officer {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Yell {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Whisper {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::WhisperInform {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Reply {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Emote {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::TextEmote {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterSay {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

                // message1: SizedCString
                w.write_all(&((message1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag1: PlayerChatTag
                w.write_all(&(chat_tag1.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterParty {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

                // message1: SizedCString
                w.write_all(&((message1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag1: PlayerChatTag
                w.write_all(&(chat_tag1.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterYell {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

                // message1: SizedCString
                w.write_all(&((message1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag1: PlayerChatTag
                w.write_all(&(chat_tag1.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterWhisper {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

                // message1: SizedCString
                w.write_all(&((message1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag1: PlayerChatTag
                w.write_all(&(chat_tag1.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::MonsterEmote {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

                // message1: SizedCString
                w.write_all(&((message1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag1: PlayerChatTag
                w.write_all(&(chat_tag1.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Channel {
                channel_name,
                chat_tag3,
                message3,
                target4,
            } => {
                // channel_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
                w.write_all(channel_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

                // message3: SizedCString
                w.write_all(&((message3.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message3.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag3: PlayerChatTag
                w.write_all(&(chat_tag3.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelJoin {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelLeave {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelList {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelNotice {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::ChannelNoticeUser {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Afk {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Dnd {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Ignored {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Skill {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Loot {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Money {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Opening {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Tradeskills {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::PetInfo {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatMiscInfo {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatXpGain {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatHonorGain {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::CombatFactionChange {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemNeutral {
                chat_tag2,
                message2,
                target2,
            } => {
                // target2: NamedGuid
                target2.write_into_vec(&mut w)?;

                // message2: SizedCString
                w.write_all(&((message2.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag2: PlayerChatTag
                w.write_all(&(chat_tag2.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemAlliance {
                chat_tag2,
                message2,
                target2,
            } => {
                // target2: NamedGuid
                target2.write_into_vec(&mut w)?;

                // message2: SizedCString
                w.write_all(&((message2.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag2: PlayerChatTag
                w.write_all(&(chat_tag2.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BgSystemHorde {
                chat_tag2,
                message2,
                target2,
            } => {
                // target2: NamedGuid
                target2.write_into_vec(&mut w)?;

                // message2: SizedCString
                w.write_all(&((message2.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag2: PlayerChatTag
                w.write_all(&(chat_tag2.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidLeader {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidWarning {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidBossWhisper {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

                // message1: SizedCString
                w.write_all(&((message1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag1: PlayerChatTag
                w.write_all(&(chat_tag1.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::RaidBossEmote {
                chat_tag1,
                message1,
                sender,
                target1,
            } => {
                // sender: SizedCString
                w.write_all(&((sender.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // target1: NamedGuid
                target1.write_into_vec(&mut w)?;

                // message1: SizedCString
                w.write_all(&((message1.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag1: PlayerChatTag
                w.write_all(&(chat_tag1.as_int().to_le_bytes()))?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Filtered {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Battleground {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::BattlegroundLeader {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_GM_MESSAGECHAT_ChatType::Restricted {
                chat_tag4,
                message4,
                sender_name,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // message4: SizedCString
                w.write_all(&((message4.len() + 1) as u32).to_le_bytes())?;
                w.write_all(message4.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // chat_tag4: PlayerChatTag
                w.write_all(&(chat_tag4.as_int().to_le_bytes()))?;

                // sender_name: SizedCString
                w.write_all(&((sender_name.len() + 1) as u32).to_le_bytes())?;
                w.write_all(sender_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(19..=24022).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03B2, size: body_size });
        }

        // chat_type: ChatType
        let chat_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // language: Language
        let language = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        let chat_type_if = match chat_type {
            ChatType::System => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::System {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Say => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Say {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Party => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Party {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Raid => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Raid {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Guild => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Guild {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Officer => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Officer {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Yell => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Yell {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Whisper => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Whisper {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::WhisperInform => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::WhisperInform {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Reply => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Reply {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Emote => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Emote {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::TextEmote => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::TextEmote {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::MonsterSay => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                // message1: SizedCString
                let message1 = {
                    let message1 = crate::util::read_u32_le(&mut r)?;
                    let message1 = crate::util::read_sized_c_string_to_vec(&mut r, message1)?;
                    String::from_utf8(message1)?
                };

                // chat_tag1: PlayerChatTag
                let chat_tag1 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::MonsterSay {
                    chat_tag1,
                    message1,
                    sender,
                    target1,
                }
            }
            ChatType::MonsterParty => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                // message1: SizedCString
                let message1 = {
                    let message1 = crate::util::read_u32_le(&mut r)?;
                    let message1 = crate::util::read_sized_c_string_to_vec(&mut r, message1)?;
                    String::from_utf8(message1)?
                };

                // chat_tag1: PlayerChatTag
                let chat_tag1 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::MonsterParty {
                    chat_tag1,
                    message1,
                    sender,
                    target1,
                }
            }
            ChatType::MonsterYell => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                // message1: SizedCString
                let message1 = {
                    let message1 = crate::util::read_u32_le(&mut r)?;
                    let message1 = crate::util::read_sized_c_string_to_vec(&mut r, message1)?;
                    String::from_utf8(message1)?
                };

                // chat_tag1: PlayerChatTag
                let chat_tag1 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::MonsterYell {
                    chat_tag1,
                    message1,
                    sender,
                    target1,
                }
            }
            ChatType::MonsterWhisper => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                // message1: SizedCString
                let message1 = {
                    let message1 = crate::util::read_u32_le(&mut r)?;
                    let message1 = crate::util::read_sized_c_string_to_vec(&mut r, message1)?;
                    String::from_utf8(message1)?
                };

                // chat_tag1: PlayerChatTag
                let chat_tag1 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::MonsterWhisper {
                    chat_tag1,
                    message1,
                    sender,
                    target1,
                }
            }
            ChatType::MonsterEmote => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                // message1: SizedCString
                let message1 = {
                    let message1 = crate::util::read_u32_le(&mut r)?;
                    let message1 = crate::util::read_sized_c_string_to_vec(&mut r, message1)?;
                    String::from_utf8(message1)?
                };

                // chat_tag1: PlayerChatTag
                let chat_tag1 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::MonsterEmote {
                    chat_tag1,
                    message1,
                    sender,
                    target1,
                }
            }
            ChatType::Channel => {
                // channel_name: CString
                let channel_name = {
                    let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(channel_name)?
                };

                // target4: Guid
                let target4 = crate::util::read_guid(&mut r)?;

                // message3: SizedCString
                let message3 = {
                    let message3 = crate::util::read_u32_le(&mut r)?;
                    let message3 = crate::util::read_sized_c_string_to_vec(&mut r, message3)?;
                    String::from_utf8(message3)?
                };

                // chat_tag3: PlayerChatTag
                let chat_tag3 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::Channel {
                    channel_name,
                    chat_tag3,
                    message3,
                    target4,
                }
            }
            ChatType::ChannelJoin => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::ChannelJoin {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::ChannelLeave => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::ChannelLeave {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::ChannelList => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::ChannelList {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::ChannelNotice => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::ChannelNotice {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::ChannelNoticeUser => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::ChannelNoticeUser {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Afk => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Afk {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Dnd => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Dnd {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Ignored => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Ignored {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Skill => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Skill {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Loot => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Loot {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Money => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Money {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Opening => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Opening {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Tradeskills => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Tradeskills {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::PetInfo => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::PetInfo {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::CombatMiscInfo => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::CombatMiscInfo {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::CombatXpGain => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::CombatXpGain {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::CombatHonorGain => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::CombatHonorGain {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::CombatFactionChange => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::CombatFactionChange {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::BgSystemNeutral => {
                // target2: NamedGuid
                let target2 = NamedGuid::read(&mut r)?;

                // message2: SizedCString
                let message2 = {
                    let message2 = crate::util::read_u32_le(&mut r)?;
                    let message2 = crate::util::read_sized_c_string_to_vec(&mut r, message2)?;
                    String::from_utf8(message2)?
                };

                // chat_tag2: PlayerChatTag
                let chat_tag2 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::BgSystemNeutral {
                    chat_tag2,
                    message2,
                    target2,
                }
            }
            ChatType::BgSystemAlliance => {
                // target2: NamedGuid
                let target2 = NamedGuid::read(&mut r)?;

                // message2: SizedCString
                let message2 = {
                    let message2 = crate::util::read_u32_le(&mut r)?;
                    let message2 = crate::util::read_sized_c_string_to_vec(&mut r, message2)?;
                    String::from_utf8(message2)?
                };

                // chat_tag2: PlayerChatTag
                let chat_tag2 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::BgSystemAlliance {
                    chat_tag2,
                    message2,
                    target2,
                }
            }
            ChatType::BgSystemHorde => {
                // target2: NamedGuid
                let target2 = NamedGuid::read(&mut r)?;

                // message2: SizedCString
                let message2 = {
                    let message2 = crate::util::read_u32_le(&mut r)?;
                    let message2 = crate::util::read_sized_c_string_to_vec(&mut r, message2)?;
                    String::from_utf8(message2)?
                };

                // chat_tag2: PlayerChatTag
                let chat_tag2 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::BgSystemHorde {
                    chat_tag2,
                    message2,
                    target2,
                }
            }
            ChatType::RaidLeader => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::RaidLeader {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::RaidWarning => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::RaidWarning {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::RaidBossWhisper => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                // message1: SizedCString
                let message1 = {
                    let message1 = crate::util::read_u32_le(&mut r)?;
                    let message1 = crate::util::read_sized_c_string_to_vec(&mut r, message1)?;
                    String::from_utf8(message1)?
                };

                // chat_tag1: PlayerChatTag
                let chat_tag1 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::RaidBossWhisper {
                    chat_tag1,
                    message1,
                    sender,
                    target1,
                }
            }
            ChatType::RaidBossEmote => {
                // sender: SizedCString
                let sender = {
                    let sender = crate::util::read_u32_le(&mut r)?;
                    let sender = crate::util::read_sized_c_string_to_vec(&mut r, sender)?;
                    String::from_utf8(sender)?
                };

                // target1: NamedGuid
                let target1 = NamedGuid::read(&mut r)?;

                // message1: SizedCString
                let message1 = {
                    let message1 = crate::util::read_u32_le(&mut r)?;
                    let message1 = crate::util::read_sized_c_string_to_vec(&mut r, message1)?;
                    String::from_utf8(message1)?
                };

                // chat_tag1: PlayerChatTag
                let chat_tag1 = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_GM_MESSAGECHAT_ChatType::RaidBossEmote {
                    chat_tag1,
                    message1,
                    sender,
                    target1,
                }
            }
            ChatType::Filtered => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Filtered {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Battleground => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Battleground {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::BattlegroundLeader => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::BattlegroundLeader {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
            ChatType::Restricted => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // message4: SizedCString
                let message4 = {
                    let message4 = crate::util::read_u32_le(&mut r)?;
                    let message4 = crate::util::read_sized_c_string_to_vec(&mut r, message4)?;
                    String::from_utf8(message4)?
                };

                // chat_tag4: PlayerChatTag
                let chat_tag4 = crate::util::read_u8_le(&mut r)?.try_into()?;

                // sender_name: SizedCString
                let sender_name = {
                    let sender_name = crate::util::read_u32_le(&mut r)?;
                    let sender_name = crate::util::read_sized_c_string_to_vec(&mut r, sender_name)?;
                    String::from_utf8(sender_name)?
                };

                SMSG_GM_MESSAGECHAT_ChatType::Restricted {
                    chat_tag4,
                    message4,
                    sender_name,
                    target5,
                }
            }
        };

        Ok(Self {
            chat_type: chat_type_if,
            language,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GM_MESSAGECHAT {}

impl SMSG_GM_MESSAGECHAT {
    pub(crate) fn size(&self) -> usize {
        self.chat_type.size() // chat_type: SMSG_GM_MESSAGECHAT_ChatType
        + 4 // language: Language
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_GM_MESSAGECHAT_ChatType {
    System {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Say {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Party {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Raid {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Guild {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Officer {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Yell {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Whisper {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    WhisperInform {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Reply {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Emote {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    TextEmote {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    MonsterSay {
        chat_tag1: PlayerChatTag,
        message1: String,
        sender: String,
        target1: NamedGuid,
    },
    MonsterParty {
        chat_tag1: PlayerChatTag,
        message1: String,
        sender: String,
        target1: NamedGuid,
    },
    MonsterYell {
        chat_tag1: PlayerChatTag,
        message1: String,
        sender: String,
        target1: NamedGuid,
    },
    MonsterWhisper {
        chat_tag1: PlayerChatTag,
        message1: String,
        sender: String,
        target1: NamedGuid,
    },
    MonsterEmote {
        chat_tag1: PlayerChatTag,
        message1: String,
        sender: String,
        target1: NamedGuid,
    },
    Channel {
        channel_name: String,
        chat_tag3: PlayerChatTag,
        message3: String,
        target4: Guid,
    },
    ChannelJoin {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    ChannelLeave {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    ChannelList {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    ChannelNotice {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    ChannelNoticeUser {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Afk {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Dnd {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Ignored {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Skill {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Loot {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Money {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Opening {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Tradeskills {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    PetInfo {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    CombatMiscInfo {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    CombatXpGain {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    CombatHonorGain {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    CombatFactionChange {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    BgSystemNeutral {
        chat_tag2: PlayerChatTag,
        message2: String,
        target2: NamedGuid,
    },
    BgSystemAlliance {
        chat_tag2: PlayerChatTag,
        message2: String,
        target2: NamedGuid,
    },
    BgSystemHorde {
        chat_tag2: PlayerChatTag,
        message2: String,
        target2: NamedGuid,
    },
    RaidLeader {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    RaidWarning {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    RaidBossWhisper {
        chat_tag1: PlayerChatTag,
        message1: String,
        sender: String,
        target1: NamedGuid,
    },
    RaidBossEmote {
        chat_tag1: PlayerChatTag,
        message1: String,
        sender: String,
        target1: NamedGuid,
    },
    Filtered {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Battleground {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    BattlegroundLeader {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
    Restricted {
        chat_tag4: PlayerChatTag,
        message4: String,
        sender_name: String,
        target5: Guid,
    },
}

impl Default for SMSG_GM_MESSAGECHAT_ChatType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::System {
            chat_tag4: Default::default(),
            message4: Default::default(),
            sender_name: Default::default(),
            target5: Default::default(),
        }
    }
}

impl SMSG_GM_MESSAGECHAT_ChatType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::System { .. } => 0,
            Self::Say { .. } => 1,
            Self::Party { .. } => 2,
            Self::Raid { .. } => 3,
            Self::Guild { .. } => 4,
            Self::Officer { .. } => 5,
            Self::Yell { .. } => 6,
            Self::Whisper { .. } => 7,
            Self::WhisperInform { .. } => 8,
            Self::Reply { .. } => 9,
            Self::Emote { .. } => 10,
            Self::TextEmote { .. } => 11,
            Self::MonsterSay { .. } => 12,
            Self::MonsterParty { .. } => 13,
            Self::MonsterYell { .. } => 14,
            Self::MonsterWhisper { .. } => 15,
            Self::MonsterEmote { .. } => 16,
            Self::Channel { .. } => 17,
            Self::ChannelJoin { .. } => 18,
            Self::ChannelLeave { .. } => 19,
            Self::ChannelList { .. } => 20,
            Self::ChannelNotice { .. } => 21,
            Self::ChannelNoticeUser { .. } => 22,
            Self::Afk { .. } => 23,
            Self::Dnd { .. } => 24,
            Self::Ignored { .. } => 25,
            Self::Skill { .. } => 26,
            Self::Loot { .. } => 27,
            Self::Money { .. } => 28,
            Self::Opening { .. } => 29,
            Self::Tradeskills { .. } => 30,
            Self::PetInfo { .. } => 31,
            Self::CombatMiscInfo { .. } => 32,
            Self::CombatXpGain { .. } => 33,
            Self::CombatHonorGain { .. } => 34,
            Self::CombatFactionChange { .. } => 35,
            Self::BgSystemNeutral { .. } => 36,
            Self::BgSystemAlliance { .. } => 37,
            Self::BgSystemHorde { .. } => 38,
            Self::RaidLeader { .. } => 39,
            Self::RaidWarning { .. } => 40,
            Self::RaidBossWhisper { .. } => 41,
            Self::RaidBossEmote { .. } => 42,
            Self::Filtered { .. } => 43,
            Self::Battleground { .. } => 44,
            Self::BattlegroundLeader { .. } => 45,
            Self::Restricted { .. } => 46,
        }
    }

}

impl std::fmt::Display for SMSG_GM_MESSAGECHAT_ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System{ .. } => f.write_str("System"),
            Self::Say{ .. } => f.write_str("Say"),
            Self::Party{ .. } => f.write_str("Party"),
            Self::Raid{ .. } => f.write_str("Raid"),
            Self::Guild{ .. } => f.write_str("Guild"),
            Self::Officer{ .. } => f.write_str("Officer"),
            Self::Yell{ .. } => f.write_str("Yell"),
            Self::Whisper{ .. } => f.write_str("Whisper"),
            Self::WhisperInform{ .. } => f.write_str("WhisperInform"),
            Self::Reply{ .. } => f.write_str("Reply"),
            Self::Emote{ .. } => f.write_str("Emote"),
            Self::TextEmote{ .. } => f.write_str("TextEmote"),
            Self::MonsterSay{ .. } => f.write_str("MonsterSay"),
            Self::MonsterParty{ .. } => f.write_str("MonsterParty"),
            Self::MonsterYell{ .. } => f.write_str("MonsterYell"),
            Self::MonsterWhisper{ .. } => f.write_str("MonsterWhisper"),
            Self::MonsterEmote{ .. } => f.write_str("MonsterEmote"),
            Self::Channel{ .. } => f.write_str("Channel"),
            Self::ChannelJoin{ .. } => f.write_str("ChannelJoin"),
            Self::ChannelLeave{ .. } => f.write_str("ChannelLeave"),
            Self::ChannelList{ .. } => f.write_str("ChannelList"),
            Self::ChannelNotice{ .. } => f.write_str("ChannelNotice"),
            Self::ChannelNoticeUser{ .. } => f.write_str("ChannelNoticeUser"),
            Self::Afk{ .. } => f.write_str("Afk"),
            Self::Dnd{ .. } => f.write_str("Dnd"),
            Self::Ignored{ .. } => f.write_str("Ignored"),
            Self::Skill{ .. } => f.write_str("Skill"),
            Self::Loot{ .. } => f.write_str("Loot"),
            Self::Money{ .. } => f.write_str("Money"),
            Self::Opening{ .. } => f.write_str("Opening"),
            Self::Tradeskills{ .. } => f.write_str("Tradeskills"),
            Self::PetInfo{ .. } => f.write_str("PetInfo"),
            Self::CombatMiscInfo{ .. } => f.write_str("CombatMiscInfo"),
            Self::CombatXpGain{ .. } => f.write_str("CombatXpGain"),
            Self::CombatHonorGain{ .. } => f.write_str("CombatHonorGain"),
            Self::CombatFactionChange{ .. } => f.write_str("CombatFactionChange"),
            Self::BgSystemNeutral{ .. } => f.write_str("BgSystemNeutral"),
            Self::BgSystemAlliance{ .. } => f.write_str("BgSystemAlliance"),
            Self::BgSystemHorde{ .. } => f.write_str("BgSystemHorde"),
            Self::RaidLeader{ .. } => f.write_str("RaidLeader"),
            Self::RaidWarning{ .. } => f.write_str("RaidWarning"),
            Self::RaidBossWhisper{ .. } => f.write_str("RaidBossWhisper"),
            Self::RaidBossEmote{ .. } => f.write_str("RaidBossEmote"),
            Self::Filtered{ .. } => f.write_str("Filtered"),
            Self::Battleground{ .. } => f.write_str("Battleground"),
            Self::BattlegroundLeader{ .. } => f.write_str("BattlegroundLeader"),
            Self::Restricted{ .. } => f.write_str("Restricted"),
        }
    }
}

impl SMSG_GM_MESSAGECHAT_ChatType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::System {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Say {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Party {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Raid {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Guild {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Officer {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Yell {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Whisper {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::WhisperInform {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Reply {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Emote {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::TextEmote {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::MonsterSay {
                message1,
                sender,
                target1,
                ..
            } => {
                1
                + 1 // chat_tag1: PlayerChatTag
                + message1.len() + 5 // message1: SizedCString
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterParty {
                message1,
                sender,
                target1,
                ..
            } => {
                1
                + 1 // chat_tag1: PlayerChatTag
                + message1.len() + 5 // message1: SizedCString
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterYell {
                message1,
                sender,
                target1,
                ..
            } => {
                1
                + 1 // chat_tag1: PlayerChatTag
                + message1.len() + 5 // message1: SizedCString
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterWhisper {
                message1,
                sender,
                target1,
                ..
            } => {
                1
                + 1 // chat_tag1: PlayerChatTag
                + message1.len() + 5 // message1: SizedCString
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::MonsterEmote {
                message1,
                sender,
                target1,
                ..
            } => {
                1
                + 1 // chat_tag1: PlayerChatTag
                + message1.len() + 5 // message1: SizedCString
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::Channel {
                channel_name,
                message3,
                ..
            } => {
                1
                + channel_name.len() + 1 // channel_name: CString
                + 1 // chat_tag3: PlayerChatTag
                + message3.len() + 5 // message3: SizedCString
                + 8 // target4: Guid
            }
            Self::ChannelJoin {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::ChannelLeave {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::ChannelList {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::ChannelNotice {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::ChannelNoticeUser {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Afk {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Dnd {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Ignored {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Skill {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Loot {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Money {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Opening {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Tradeskills {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::PetInfo {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::CombatMiscInfo {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::CombatXpGain {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::CombatHonorGain {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::CombatFactionChange {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::BgSystemNeutral {
                message2,
                target2,
                ..
            } => {
                1
                + 1 // chat_tag2: PlayerChatTag
                + message2.len() + 5 // message2: SizedCString
                + target2.size() // target2: NamedGuid
            }
            Self::BgSystemAlliance {
                message2,
                target2,
                ..
            } => {
                1
                + 1 // chat_tag2: PlayerChatTag
                + message2.len() + 5 // message2: SizedCString
                + target2.size() // target2: NamedGuid
            }
            Self::BgSystemHorde {
                message2,
                target2,
                ..
            } => {
                1
                + 1 // chat_tag2: PlayerChatTag
                + message2.len() + 5 // message2: SizedCString
                + target2.size() // target2: NamedGuid
            }
            Self::RaidLeader {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::RaidWarning {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::RaidBossWhisper {
                message1,
                sender,
                target1,
                ..
            } => {
                1
                + 1 // chat_tag1: PlayerChatTag
                + message1.len() + 5 // message1: SizedCString
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::RaidBossEmote {
                message1,
                sender,
                target1,
                ..
            } => {
                1
                + 1 // chat_tag1: PlayerChatTag
                + message1.len() + 5 // message1: SizedCString
                + sender.len() + 5 // sender: SizedCString
                + target1.size() // target1: NamedGuid
            }
            Self::Filtered {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Battleground {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::BattlegroundLeader {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
            Self::Restricted {
                message4,
                sender_name,
                ..
            } => {
                1
                + 1 // chat_tag4: PlayerChatTag
                + message4.len() + 5 // message4: SizedCString
                + sender_name.len() + 5 // sender_name: SizedCString
                + 8 // target5: Guid
            }
        }
    }
}

