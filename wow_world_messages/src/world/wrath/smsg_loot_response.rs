use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::wrath::{
    LootItem, LootMethod, LootMethodError,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:131`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L131):
/// ```text
/// smsg SMSG_LOOT_RESPONSE = 0x0160 {
///     Guid guid;
///     LootMethod loot_method;
///     if (loot_method == ERROR) {
///         LootMethodError loot_error;
///     }
///     Gold gold;
///     u8 amount_of_items;
///     LootItem[amount_of_items] items;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_LOOT_RESPONSE {
    pub guid: Guid,
    pub loot_method: SMSG_LOOT_RESPONSE_LootMethod,
    pub gold: Gold,
    pub items: Vec<LootItem>,
}

impl crate::private::Sealed for SMSG_LOOT_RESPONSE {}
impl SMSG_LOOT_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=5647).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // loot_method: LootMethod
        let loot_method = crate::util::read_u8_le(&mut r)?.try_into()?;

        let loot_method_if = match loot_method {
            LootMethod::ErrorX => {
                // loot_error: LootMethodError
                let loot_error = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_LOOT_RESPONSE_LootMethod::ErrorX {
                    loot_error,
                }
            }
            LootMethod::Corpse => SMSG_LOOT_RESPONSE_LootMethod::Corpse,
            LootMethod::Pickpocketing => SMSG_LOOT_RESPONSE_LootMethod::Pickpocketing,
            LootMethod::Fishing => SMSG_LOOT_RESPONSE_LootMethod::Fishing,
            LootMethod::Disenchanting => SMSG_LOOT_RESPONSE_LootMethod::Disenchanting,
            LootMethod::Skinning => SMSG_LOOT_RESPONSE_LootMethod::Skinning,
            LootMethod::Fishinghole => SMSG_LOOT_RESPONSE_LootMethod::Fishinghole,
            LootMethod::FishingFail => SMSG_LOOT_RESPONSE_LootMethod::FishingFail,
            LootMethod::Insignia => SMSG_LOOT_RESPONSE_LootMethod::Insignia,
        };

        // gold: Gold
        let gold = Gold::new(crate::util::read_u32_le(&mut r)?);

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(&mut r)?;

        // items: LootItem[amount_of_items]
        let items = {
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for _ in 0..amount_of_items {
                items.push(LootItem::read(&mut r)?);
            }
            items
        };

        Ok(Self {
            guid,
            loot_method: loot_method_if,
            gold,
            items,
        })
    }

}

impl crate::Message for SMSG_LOOT_RESPONSE {
    const OPCODE: u32 = 0x0160;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LOOT_RESPONSE"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // loot_method: LootMethod
        w.write_all(&(self.loot_method.as_int().to_le_bytes()))?;

        match &self.loot_method {
            SMSG_LOOT_RESPONSE_LootMethod::ErrorX {
                loot_error,
            } => {
                // loot_error: LootMethodError
                w.write_all(&(loot_error.as_int().to_le_bytes()))?;

            }
            _ => {}
        }

        // gold: Gold
        w.write_all((self.gold.as_int()).to_le_bytes().as_slice())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: LootItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(352, "SMSG_LOOT_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_RESPONSE {}

impl SMSG_LOOT_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.loot_method.size() // loot_method: SMSG_LOOT_RESPONSE_LootMethod
        + 4 // gold: Gold
        + 1 // amount_of_items: u8
        + self.items.len() * 22 // items: LootItem[amount_of_items]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_LOOT_RESPONSE_LootMethod {
    ErrorX {
        loot_error: LootMethodError,
    },
    Corpse,
    Pickpocketing,
    Fishing,
    Disenchanting,
    Skinning,
    Fishinghole,
    FishingFail,
    Insignia,
}

impl Default for SMSG_LOOT_RESPONSE_LootMethod {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Corpse
    }
}

impl SMSG_LOOT_RESPONSE_LootMethod {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::ErrorX { .. } => 0,
            Self::Corpse => 1,
            Self::Pickpocketing => 2,
            Self::Fishing => 3,
            Self::Disenchanting => 4,
            Self::Skinning => 6,
            Self::Fishinghole => 20,
            Self::FishingFail => 21,
            Self::Insignia => 22,
        }
    }

}

impl std::fmt::Display for SMSG_LOOT_RESPONSE_LootMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrorX{ .. } => f.write_str("ErrorX"),
            Self::Corpse => f.write_str("Corpse"),
            Self::Pickpocketing => f.write_str("Pickpocketing"),
            Self::Fishing => f.write_str("Fishing"),
            Self::Disenchanting => f.write_str("Disenchanting"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Fishinghole => f.write_str("Fishinghole"),
            Self::FishingFail => f.write_str("FishingFail"),
            Self::Insignia => f.write_str("Insignia"),
        }
    }
}

impl SMSG_LOOT_RESPONSE_LootMethod {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::ErrorX {
                ..
            } => {
                1
                + 1 // loot_error: LootMethodError
            }
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_LOOT_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 40] = [ 0x00, 0x26, 0x60, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x75, 0x00,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_LOOT_RESPONSE {
        SMSG_LOOT_RESPONSE {
            guid: Guid::new(0x1),
            loot_method: SMSG_LOOT_RESPONSE_LootMethod::Corpse,
            gold: Gold::try_from(0x0).unwrap(),
            items: vec![
                LootItem {
                    index: 0x0,
                    item: 0x75,
                    count: 0x1,
                    display_id: 0x0,
                    random_suffix: 0x0,
                    random_property: 0x0,
                    ty: LootSlotType::TypeAllowLoot,
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/loot/smsg_loot_response.wowm` line 85.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_loot_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOOT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOOT_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/loot/smsg_loot_response.wowm` line 85.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_loot_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOOT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOOT_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/loot/smsg_loot_response.wowm` line 85.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_loot_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOOT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOOT_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

