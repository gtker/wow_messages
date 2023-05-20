/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm#L13):
/// ```text
/// enum MailMessageType : u32 {
///     NORMAL = 0;
///     AUCTION = 2;
///     CREATURE = 3;
///     GAMEOBJECT = 4;
///     ITEM = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MailMessageType {
    Normal,
    Auction,
    /// client send CMSG_CREATURE_QUERY on this mailmessagetype
    ///
    Creature,
    /// client send CMSG_GAMEOBJECT_QUERY on this mailmessagetype
    ///
    Gameobject,
    /// client send CMSG_ITEM_QUERY on this mailmessagetype
    ///
    Item,
}

impl MailMessageType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Normal => 0x0,
            Self::Auction => 0x2,
            Self::Creature => 0x3,
            Self::Gameobject => 0x4,
            Self::Item => 0x5,
        }
    }

}

impl Default for MailMessageType {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for MailMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Auction => f.write_str("Auction"),
            Self::Creature => f.write_str("Creature"),
            Self::Gameobject => f.write_str("Gameobject"),
            Self::Item => f.write_str("Item"),
        }
    }
}

impl TryFrom<u32> for MailMessageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            2 => Ok(Self::Auction),
            3 => Ok(Self::Creature),
            4 => Ok(Self::Gameobject),
            5 => Ok(Self::Item),
            v => Err(crate::errors::EnumError::new("MailMessageType", v as u64),)
        }
    }
}

