use crate::Guid;
use crate::wrath::FriendResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_status.wowm:62`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_status.wowm#L62):
/// ```text
/// smsg SMSG_FRIEND_STATUS = 0x0068 {
///     FriendResult result;
///     Guid guid;
/// }
/// ```
pub struct SMSG_FRIEND_STATUS {
    pub result: FriendResult,
    pub guid: Guid,
}

impl crate::Message for SMSG_FRIEND_STATUS {
    const OPCODE: u32 = 0x0068;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: FriendResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0068, size: body_size as u32 });
        }

        // result: FriendResult
        let result: FriendResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            result,
            guid,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FRIEND_STATUS {}

