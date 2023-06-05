use std::io::{Read, Write};

use crate::Guid;
use crate::shared::loot_item_vanilla_tbc_wrath::LootItem;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;
use wow_world_base::shared::loot_method_error_vanilla_tbc_wrath::LootMethodError;
use wow_world_base::shared::loot_method_vanilla_tbc_wrath::LootMethod;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:95`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L95):
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
pub struct SMSG_LOOT_RESPONSE {
    pub guid: Guid,
    pub loot_method: SMSG_LOOT_RESPONSE_LootMethod,
    pub gold: Gold,
    pub items: Vec<LootItem>,
}

impl crate::private::Sealed for SMSG_LOOT_RESPONSE {}
impl crate::Message for SMSG_LOOT_RESPONSE {
    const OPCODE: u32 = 0x0160;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(14..=1551).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0160, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // loot_method: LootMethod
        let loot_method: LootMethod = crate::util::read_u8_le(&mut r)?.try_into()?;

        let loot_method_if = match loot_method {
            LootMethod::ErrorX => {
                // loot_error: LootMethodError
                let loot_error: LootMethodError = crate::util::read_u8_le(&mut r)?.try_into()?;

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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_RESPONSE {}

impl SMSG_LOOT_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.loot_method.size() // loot_method: SMSG_LOOT_RESPONSE_LootMethod
        + 4 // gold: Gold
        + 1 // amount_of_items: u8
        + self.items.len() * 6 // items: LootItem[amount_of_items]
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

