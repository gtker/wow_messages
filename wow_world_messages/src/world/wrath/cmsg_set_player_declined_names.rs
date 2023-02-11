use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm#L9):
/// ```text
/// cmsg CMSG_SET_PLAYER_DECLINED_NAMES = 0x0419 {
///     Guid player;
///     CString name;
///     CString[5] declined_names;
/// }
/// ```
pub struct CMSG_SET_PLAYER_DECLINED_NAMES {
    pub player: Guid,
    pub name: String,
    pub declined_names: [String; 5],
}

impl crate::Message for CMSG_SET_PLAYER_DECLINED_NAMES {
    const OPCODE: u32 = 0x0419;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // declined_names: CString[5]
        for i in self.declined_names.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=1544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0419, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // declined_names: CString[5]
        let mut declined_names = Vec::with_capacity(5);
        for i in 0..5 {
            let s = crate::util::read_c_string_to_vec(r)?;
            declined_names.push(String::from_utf8(s)?);
        }
        let declined_names = declined_names.try_into().unwrap();

        Ok(Self {
            player,
            name,
            declined_names,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_PLAYER_DECLINED_NAMES {}

impl CMSG_SET_PLAYER_DECLINED_NAMES {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.name.len() + 1 // name: CString
        + self.declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
    }
}

