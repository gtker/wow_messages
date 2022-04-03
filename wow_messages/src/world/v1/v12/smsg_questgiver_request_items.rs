use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{QuestCompletable, QuestCompletableError};
use crate::world::v1::v12::QuestItemRequirement;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new5.wowm:606`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new5.wowm#L606):
/// ```text
/// smsg SMSG_QUESTGIVER_REQUEST_ITEMS = 0x18B {
///     Guid npc;
///     u32 quest_id;
///     CString title;
///     CString request_items_text;
///     u32 emote_delay;
///     u32 emote;
///     u32 auto_finish;
///     u32 required_money;
///     u32 amount_of_required_items;
///     QuestItemRequirement[amount_of_required_items] required_items;
///     u32 unknown1;
///     QuestCompletable completable;
///     u32 flags2;
///     u32 flags3;
/// }
/// ```
pub struct SMSG_QUESTGIVER_REQUEST_ITEMS {
    pub npc: Guid,
    pub quest_id: u32,
    pub title: String,
    pub request_items_text: String,
    pub emote_delay: u32,
    pub emote: u32,
    pub auto_finish: u32,
    pub required_money: u32,
    pub amount_of_required_items: u32,
    pub required_items: Vec<QuestItemRequirement>,
    pub unknown1: u32,
    pub completable: QuestCompletable,
    pub flags2: u32,
    pub flags3: u32,
}

impl WorldServerMessageWrite for SMSG_QUESTGIVER_REQUEST_ITEMS {
    const OPCODE: u16 = 0x18b;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_QUESTGIVER_REQUEST_ITEMS {
    type Error = SMSG_QUESTGIVER_REQUEST_ITEMSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // request_items_text: CString
        let request_items_text = crate::util::read_c_string_to_vec(r)?;
        let request_items_text = String::from_utf8(request_items_text)?;

        // emote_delay: u32
        let emote_delay = crate::util::read_u32_le(r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // auto_finish: u32
        let auto_finish = crate::util::read_u32_le(r)?;

        // required_money: u32
        let required_money = crate::util::read_u32_le(r)?;

        // amount_of_required_items: u32
        let amount_of_required_items = crate::util::read_u32_le(r)?;

        // required_items: QuestItemRequirement[amount_of_required_items]
        let mut required_items = Vec::with_capacity(amount_of_required_items as usize);
        for i in 0..amount_of_required_items {
            required_items.push(QuestItemRequirement::read(r)?);
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // completable: QuestCompletable
        let completable = QuestCompletable::read(r)?;

        // flags2: u32
        let flags2 = crate::util::read_u32_le(r)?;

        // flags3: u32
        let flags3 = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            quest_id,
            title,
            request_items_text,
            emote_delay,
            emote,
            auto_finish,
            required_money,
            amount_of_required_items,
            required_items,
            unknown1,
            completable,
            flags2,
            flags3,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.write(w)?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // request_items_text: CString
        w.write_all(self.request_items_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // emote_delay: u32
        w.write_all(&self.emote_delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // auto_finish: u32
        w.write_all(&self.auto_finish.to_le_bytes())?;

        // required_money: u32
        w.write_all(&self.required_money.to_le_bytes())?;

        // amount_of_required_items: u32
        w.write_all(&(self.required_items.len() as u32).to_le_bytes())?;

        // required_items: QuestItemRequirement[amount_of_required_items]
        for i in self.required_items.iter() {
            i.write(w)?;
        }

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // completable: QuestCompletable
        self.completable.write(w)?;

        // flags2: u32
        w.write_all(&self.flags2.to_le_bytes())?;

        // flags3: u32
        w.write_all(&self.flags3.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_QUESTGIVER_REQUEST_ITEMS {
    fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString and Null Terminator
        + self.request_items_text.len() + 1 // request_items_text: CString and Null Terminator
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 4 // auto_finish: u32
        + 4 // required_money: u32
        + 4 // amount_of_required_items: u32
        + self.required_items.iter().fold(0, |acc, x| acc + QuestItemRequirement::size()) // required_items: QuestItemRequirement[amount_of_required_items]
        + 4 // unknown1: u32
        + QuestCompletable::size() // completable: QuestCompletable
        + 4 // flags2: u32
        + 4 // flags3: u32
    }
}

impl MaximumPossibleSized for SMSG_QUESTGIVER_REQUEST_ITEMS {
    fn maximum_possible_size() -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + 256 // title: CString
        + 256 // request_items_text: CString
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 4 // auto_finish: u32
        + 4 // required_money: u32
        + 4 // amount_of_required_items: u32
        + 4294967295 * QuestItemRequirement::maximum_possible_size() // required_items: QuestItemRequirement[amount_of_required_items]
        + 4 // unknown1: u32
        + QuestCompletable::maximum_possible_size() // completable: QuestCompletable
        + 4 // flags2: u32
        + 4 // flags3: u32
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_REQUEST_ITEMSError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    QuestCompletable(QuestCompletableError),
}

impl std::error::Error for SMSG_QUESTGIVER_REQUEST_ITEMSError {}
impl std::fmt::Display for SMSG_QUESTGIVER_REQUEST_ITEMSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::QuestCompletable(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_REQUEST_ITEMSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUESTGIVER_REQUEST_ITEMSError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<QuestCompletableError> for SMSG_QUESTGIVER_REQUEST_ITEMSError {
    fn from(e: QuestCompletableError) -> Self {
        Self::QuestCompletable(e)
    }
}

