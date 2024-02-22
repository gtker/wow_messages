use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm:140`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm#L140):
/// ```text
/// struct Aura {
///     u16 aura;
///     u8 unknown;
/// }
/// ```
pub struct Aura {
    pub aura: u16,
    pub unknown: u8,
}

