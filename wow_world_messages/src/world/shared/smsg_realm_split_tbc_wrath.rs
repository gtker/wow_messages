use std::convert::{TryFrom, TryInto};
use crate::world::shared::realm_split_state_tbc_wrath::RealmSplitState;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_realm_split.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_realm_split.wowm#L9):
/// ```text
/// smsg SMSG_REALM_SPLIT = 0x038B {
///     u32 realm_id;
///     RealmSplitState state;
///     CString split_date;
/// }
/// ```
pub struct SMSG_REALM_SPLIT {
    /// ArcEmu/TrinityCore/mangosthree send realm_id from [`CMSG_REALM_SPLIT`](crate::world::tbc::CMSG_REALM_SPLIT) back.
    ///
    pub realm_id: u32,
    pub state: RealmSplitState,
    /// Seems to be slash separated string, like '01/01/01'.
    ///
    pub split_date: String,
}

impl crate::Message for SMSG_REALM_SPLIT {
    const OPCODE: u32 = 0x038b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // realm_id: u32
        w.write_all(&self.realm_id.to_le_bytes())?;

        // state: RealmSplitState
        w.write_all(&(self.state.as_int() as u32).to_le_bytes())?;

        // split_date: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.split_date.as_bytes().iter().rev().next(), Some(&0_u8), "String `split_date` must not be null-terminated.");
        w.write_all(self.split_date.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 9 || body_size > 264 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x038B, size: body_size as u32 });
        }

        // realm_id: u32
        let realm_id = crate::util::read_u32_le(r)?;

        // state: RealmSplitState
        let state: RealmSplitState = crate::util::read_u32_le(r)?.try_into()?;

        // split_date: CString
        let split_date = crate::util::read_c_string_to_vec(r)?;
        let split_date = String::from_utf8(split_date)?;

        Ok(Self {
            realm_id,
            state,
            split_date,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_REALM_SPLIT {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_REALM_SPLIT {}

impl SMSG_REALM_SPLIT {
    pub(crate) fn size(&self) -> usize {
        4 // realm_id: u32
        + 4 // state: RealmSplitState
        + self.split_date.len() + 1 // split_date: CString
    }
}

