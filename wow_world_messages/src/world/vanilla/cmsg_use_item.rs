use std::io::{Read, Write};

use crate::vanilla::SpellCastTargets;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_use_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_use_item.wowm#L1):
/// ```text
/// cmsg CMSG_USE_ITEM = 0x00AB {
///     u8 bag_index;
///     u8 bag_slot;
///     u8 spell_index;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for CMSG_USE_ITEM {}
impl CMSG_USE_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=321).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x00AB, size: body_size });
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(&mut r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(&mut r)?;

        // spell_index: u8
        let spell_index = crate::util::read_u8_le(&mut r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            targets,
        })
    }

}

impl crate::Message for CMSG_USE_ITEM {
    const OPCODE: u32 = 0x00ab;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_USE_ITEM {}

impl CMSG_USE_ITEM {
    pub(crate) fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_USE_ITEM;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_USE_ITEM, expected: &CMSG_USE_ITEM) {
        assert_eq!(t.bag_index, expected.bag_index);
        assert_eq!(t.bag_slot, expected.bag_slot);
        assert_eq!(t.spell_index, expected.spell_index);
        assert_eq!(t.targets, expected.targets);
    }

    const RAW0: [u8; 11] = [ 0x00, 0x09, 0xAB, 0x00, 0x00, 0x00, 0xFF, 0x18, 0x00,
         0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_USE_ITEM {
        CMSG_USE_ITEM {
            bag_index: 0xFF,
            bag_slot: 0x18,
            spell_index: 0x0,
            targets: SpellCastTargets {
                target_flags: SpellCastTargets_SpellCastTargetFlags::empty()
                    ,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_use_item.wowm` line 63.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_use_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_USE_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_USE_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_use_item.wowm` line 63.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_use_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_USE_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_USE_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_use_item.wowm` line 63.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_use_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_USE_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_USE_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

