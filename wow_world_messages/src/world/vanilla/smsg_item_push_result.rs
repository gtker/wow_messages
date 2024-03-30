use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    NewItemChatAlert, NewItemCreationType, NewItemSource,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L22):
/// ```text
/// smsg SMSG_ITEM_PUSH_RESULT = 0x0166 {
///     Guid guid;
///     NewItemSource source;
///     NewItemCreationType creation_type;
///     NewItemChatAlert alert_chat;
///     u8 bag_slot;
///     u32 item_slot;
///     Item item;
///     u32 item_suffix_factor;
///     u32 item_random_property_id;
///     u32 item_count;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ITEM_PUSH_RESULT {
    pub guid: Guid,
    pub source: NewItemSource,
    pub creation_type: NewItemCreationType,
    pub alert_chat: NewItemChatAlert,
    pub bag_slot: u8,
    /// mangoszero: item slot, but when added to stack: 0xFFFFFFFF
    pub item_slot: u32,
    pub item: u32,
    /// mangoszero: SuffixFactor
    pub item_suffix_factor: u32,
    /// mangoszero: random item property id
    pub item_random_property_id: u32,
    pub item_count: u32,
}

impl crate::private::Sealed for SMSG_ITEM_PUSH_RESULT {}
impl SMSG_ITEM_PUSH_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 41 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // source: NewItemSource
        let source = crate::util::read_u32_le(&mut r)?.try_into()?;

        // creation_type: NewItemCreationType
        let creation_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // alert_chat: NewItemChatAlert
        let alert_chat = crate::util::read_u32_le(&mut r)?.try_into()?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(&mut r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            source,
            creation_type,
            alert_chat,
            bag_slot,
            item_slot,
            item,
            item_suffix_factor,
            item_random_property_id,
            item_count,
        })
    }

}

impl crate::Message for SMSG_ITEM_PUSH_RESULT {
    const OPCODE: u32 = 0x0166;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ITEM_PUSH_RESULT"
    }

    fn size_without_header(&self) -> u32 {
        41
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // source: NewItemSource
        w.write_all(&(self.source.as_int().to_le_bytes()))?;

        // creation_type: NewItemCreationType
        w.write_all(&(self.creation_type.as_int().to_le_bytes()))?;

        // alert_chat: NewItemChatAlert
        w.write_all(&(self.alert_chat.as_int().to_le_bytes()))?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(358, "SMSG_ITEM_PUSH_RESULT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ITEM_PUSH_RESULT {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ITEM_PUSH_RESULT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 45] = [ 0x00, 0x2B, 0x66, 0x01, 0x04, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
         0x00, 0x00, 0x00, 0xFF, 0x18, 0x00, 0x00, 0x00, 0x60, 0x31, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ITEM_PUSH_RESULT {
        SMSG_ITEM_PUSH_RESULT {
            guid: Guid::new(0x4),
            source: NewItemSource::Looted,
            creation_type: NewItemCreationType::Created,
            alert_chat: NewItemChatAlert::Show,
            bag_slot: 0xFF,
            item_slot: 0x18,
            item: 0x3160,
            item_suffix_factor: 0x0,
            item_random_property_id: 0x0,
            item_count: 0x1,
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/smsg_item_push_result.wowm` line 41.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_item_push_result0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ITEM_PUSH_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ITEM_PUSH_RESULT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(41 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/smsg_item_push_result.wowm` line 41.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_item_push_result0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ITEM_PUSH_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ITEM_PUSH_RESULT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(41 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/smsg_item_push_result.wowm` line 41.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_item_push_result0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ITEM_PUSH_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ITEM_PUSH_RESULT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(41 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

