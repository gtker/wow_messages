#[allow(clippy::missing_panics_doc)]
mod impls;
mod indices;

pub use impls::*;
pub use indices::*;

use crate::helper::update_mask_common;
use crate::helper::update_mask_common::{
    skill_info, update_item, update_mask, update_mask_size, CONTAINER, CORPSE, DYNAMICOBJECT,
    GAMEOBJECT, ITEM, PLAYER, UNIT,
};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io;
use std::io::Read;
use wow_world_base::vanilla::ItemSlot;

update_item!(UpdateItem, UpdateItemBuilder, ITEM);
update_item!(UpdateContainer, UpdateContainerBuilder, ITEM | CONTAINER);
update_item!(UpdateUnit, UpdateUnitBuilder, UNIT);
update_item!(UpdatePlayer, UpdatePlayerBuilder, UNIT | PLAYER);
update_item!(UpdateGameObject, UpdateGameObjectBuilder, GAMEOBJECT);
update_item!(
    UpdateDynamicObject,
    UpdateDynamicObjectBuilder,
    DYNAMICOBJECT
);
update_item!(UpdateCorpse, UpdateCorpseBuilder, CORPSE);

update_mask!();

skill_info!(wow_world_base::vanilla::Skill, indices::SkillInfoIndex);

impl UpdatePlayer {
    pub fn set_player_field_inv_slot(&mut self, item_slot: ItemSlot, item: crate::Guid) {
        let f = match item_slot {
            ItemSlot::Head => Self::set_player_field_inv_slot_head,
            ItemSlot::Neck => Self::set_player_field_inv_slot_neck,
            ItemSlot::Shoulders => Self::set_player_field_inv_slot_shoulders,
            ItemSlot::Shirt => Self::set_player_field_inv_slot_body,
            ItemSlot::Chest => Self::set_player_field_inv_slot_chest,
            ItemSlot::Waist => Self::set_player_field_inv_slot_waist,
            ItemSlot::Legs => Self::set_player_field_inv_slot_legs,
            ItemSlot::Boots => Self::set_player_field_inv_slot_feet,
            ItemSlot::Wrist => Self::set_player_field_inv_slot_wrists,
            ItemSlot::Hands => Self::set_player_field_inv_slot_hands,
            ItemSlot::Ring1 => Self::set_player_field_inv_slot_finger1,
            ItemSlot::Ring2 => Self::set_player_field_inv_slot_finger2,
            ItemSlot::Trinket1 => Self::set_player_field_inv_slot_trinket1,
            ItemSlot::Trinket2 => Self::set_player_field_inv_slot_trinket2,
            ItemSlot::Back => Self::set_player_field_inv_slot_back,
            ItemSlot::MainHand => Self::set_player_field_inv_slot_main_hand,
            ItemSlot::OffHand => Self::set_player_field_inv_slot_off_hand,
            ItemSlot::RangedOrRelic => Self::set_player_field_inv_slot_ranged,
            ItemSlot::Tabard => Self::set_player_field_inv_slot_tabard,
            ItemSlot::Bag1 => Self::set_player_field_inv_slot_bag1,
            ItemSlot::Bag2 => Self::set_player_field_inv_slot_bag2,
            ItemSlot::Bag3 => Self::set_player_field_inv_slot_bag3,
            ItemSlot::Bag4 => Self::set_player_field_inv_slot_bag4,
            ItemSlot::Inventory0 => Self::set_player_field_pack_slot_1,
            ItemSlot::Inventory1 => Self::set_player_field_pack_slot_2,
            ItemSlot::Inventory2 => Self::set_player_field_pack_slot_3,
            ItemSlot::Inventory3 => Self::set_player_field_pack_slot_4,
            ItemSlot::Inventory4 => Self::set_player_field_pack_slot_5,
            ItemSlot::Inventory5 => Self::set_player_field_pack_slot_6,
            ItemSlot::Inventory6 => Self::set_player_field_pack_slot_7,
            ItemSlot::Inventory7 => Self::set_player_field_pack_slot_8,
            ItemSlot::Inventory8 => Self::set_player_field_pack_slot_9,
            ItemSlot::Inventory9 => Self::set_player_field_pack_slot_10,
            ItemSlot::Inventory10 => Self::set_player_field_pack_slot_11,
            ItemSlot::Inventory11 => Self::set_player_field_pack_slot_12,
            ItemSlot::Inventory12 => Self::set_player_field_pack_slot_13,
            ItemSlot::Inventory13 => Self::set_player_field_pack_slot_14,
            ItemSlot::Inventory14 => Self::set_player_field_pack_slot_15,
            ItemSlot::Inventory15 => Self::set_player_field_pack_slot_16,
            ItemSlot::Bank1 => Self::set_player_field_bank_slot_1,
            ItemSlot::Bank2 => Self::set_player_field_bank_slot_2,
            ItemSlot::Bank3 => Self::set_player_field_bank_slot_3,
            ItemSlot::Bank4 => Self::set_player_field_bank_slot_4,
            ItemSlot::Bank5 => Self::set_player_field_bank_slot_5,
            ItemSlot::Bank6 => Self::set_player_field_bank_slot_6,
            ItemSlot::Bank7 => Self::set_player_field_bank_slot_7,
            ItemSlot::Bank8 => Self::set_player_field_bank_slot_8,
            ItemSlot::Bank9 => Self::set_player_field_bank_slot_9,
            ItemSlot::Bank10 => Self::set_player_field_bank_slot_10,
            ItemSlot::Bank11 => Self::set_player_field_bank_slot_11,
            ItemSlot::Bank12 => Self::set_player_field_bank_slot_12,
            ItemSlot::Bank13 => Self::set_player_field_bank_slot_13,
            ItemSlot::Bank14 => Self::set_player_field_bank_slot_14,
            ItemSlot::Bank15 => Self::set_player_field_bank_slot_15,
            ItemSlot::Bank16 => Self::set_player_field_bank_slot_16,
            ItemSlot::Bank17 => Self::set_player_field_bank_slot_17,
            ItemSlot::Bank18 => Self::set_player_field_bank_slot_18,
            ItemSlot::Bank19 => Self::set_player_field_bank_slot_19,
            ItemSlot::Bank20 => Self::set_player_field_bank_slot_20,
            ItemSlot::Bank21 => Self::set_player_field_bank_slot_21,
            ItemSlot::Bank22 => Self::set_player_field_bank_slot_22,
            ItemSlot::Bank23 => Self::set_player_field_bank_slot_23,
            ItemSlot::Bank24 => Self::set_player_field_bank_slot_24,
            ItemSlot::BankBagSlot1 => Self::set_player_field_bankbag_slot_1,
            ItemSlot::BankBagSlot2 => Self::set_player_field_bankbag_slot_2,
            ItemSlot::BankBagSlot3 => Self::set_player_field_bankbag_slot_3,
            ItemSlot::BankBagSlot4 => Self::set_player_field_bankbag_slot_4,
            ItemSlot::BankBagSlot5 => Self::set_player_field_bankbag_slot_5,
            ItemSlot::BankBagSlot6 => Self::set_player_field_bankbag_slot_6,
            ItemSlot::VendorBuyback1 => Self::set_player_field_vendorbuyback_slot_1,
            ItemSlot::VendorBuyback2 => Self::set_player_field_vendorbuyback_slot_2,
            ItemSlot::VendorBuyback3 => Self::set_player_field_vendorbuyback_slot_3,
            ItemSlot::VendorBuyback4 => Self::set_player_field_vendorbuyback_slot_4,
            ItemSlot::VendorBuyback5 => Self::set_player_field_vendorbuyback_slot_5,
            ItemSlot::VendorBuyback6 => Self::set_player_field_vendorbuyback_slot_6,
            ItemSlot::VendorBuyback7 => Self::set_player_field_vendorbuyback_slot_7,
            ItemSlot::VendorBuyback8 => Self::set_player_field_vendorbuyback_slot_8,
            ItemSlot::VendorBuyback9 => Self::set_player_field_vendorbuyback_slot_9,
            ItemSlot::VendorBuyback10 => Self::set_player_field_vendorbuyback_slot_10,
            ItemSlot::VendorBuyback11 => Self::set_player_field_vendorbuyback_slot_11,
            ItemSlot::VendorBuyback12 => Self::set_player_field_vendorbuyback_slot_12,
            ItemSlot::Keyring1 => Self::set_player_field_keyring_slot_1,
            ItemSlot::Keyring2 => Self::set_player_field_keyring_slot_2,
            ItemSlot::Keyring3 => Self::set_player_field_keyring_slot_3,
            ItemSlot::Keyring4 => Self::set_player_field_keyring_slot_4,
            ItemSlot::Keyring5 => Self::set_player_field_keyring_slot_5,
            ItemSlot::Keyring6 => Self::set_player_field_keyring_slot_6,
            ItemSlot::Keyring7 => Self::set_player_field_keyring_slot_7,
            ItemSlot::Keyring8 => Self::set_player_field_keyring_slot_8,
            ItemSlot::Keyring9 => Self::set_player_field_keyring_slot_9,
            ItemSlot::Keyring10 => Self::set_player_field_keyring_slot_10,
            ItemSlot::Keyring11 => Self::set_player_field_keyring_slot_11,
            ItemSlot::Keyring12 => Self::set_player_field_keyring_slot_12,
            ItemSlot::Keyring13 => Self::set_player_field_keyring_slot_13,
            ItemSlot::Keyring14 => Self::set_player_field_keyring_slot_14,
            ItemSlot::Keyring15 => Self::set_player_field_keyring_slot_15,
            ItemSlot::Keyring16 => Self::set_player_field_keyring_slot_16,
            ItemSlot::Keyring17 => Self::set_player_field_keyring_slot_17,
            ItemSlot::Keyring18 => Self::set_player_field_keyring_slot_18,
            ItemSlot::Keyring19 => Self::set_player_field_keyring_slot_19,
            ItemSlot::Keyring20 => Self::set_player_field_keyring_slot_20,
            ItemSlot::Keyring21 => Self::set_player_field_keyring_slot_21,
            ItemSlot::Keyring22 => Self::set_player_field_keyring_slot_22,
            ItemSlot::Keyring23 => Self::set_player_field_keyring_slot_23,
            ItemSlot::Keyring24 => Self::set_player_field_keyring_slot_24,
            ItemSlot::Keyring25 => Self::set_player_field_keyring_slot_25,
            ItemSlot::Keyring26 => Self::set_player_field_keyring_slot_26,
            ItemSlot::Keyring27 => Self::set_player_field_keyring_slot_27,
            ItemSlot::Keyring28 => Self::set_player_field_keyring_slot_28,
            ItemSlot::Keyring29 => Self::set_player_field_keyring_slot_29,
            ItemSlot::Keyring30 => Self::set_player_field_keyring_slot_30,
            ItemSlot::Keyring31 => Self::set_player_field_keyring_slot_31,
            ItemSlot::Keyring32 => Self::set_player_field_keyring_slot_32,
        };

        f(self, item);
    }

    pub fn player_field_inv_slot(&self, item_slot: ItemSlot) -> Option<crate::Guid> {
        let f = match item_slot {
            ItemSlot::Head => Self::player_field_inv_slot_head,
            ItemSlot::Neck => Self::player_field_inv_slot_neck,
            ItemSlot::Shoulders => Self::player_field_inv_slot_shoulders,
            ItemSlot::Shirt => Self::player_field_inv_slot_body,
            ItemSlot::Chest => Self::player_field_inv_slot_chest,
            ItemSlot::Waist => Self::player_field_inv_slot_waist,
            ItemSlot::Legs => Self::player_field_inv_slot_legs,
            ItemSlot::Boots => Self::player_field_inv_slot_feet,
            ItemSlot::Wrist => Self::player_field_inv_slot_wrists,
            ItemSlot::Hands => Self::player_field_inv_slot_hands,
            ItemSlot::Ring1 => Self::player_field_inv_slot_finger1,
            ItemSlot::Ring2 => Self::player_field_inv_slot_finger2,
            ItemSlot::Trinket1 => Self::player_field_inv_slot_trinket1,
            ItemSlot::Trinket2 => Self::player_field_inv_slot_trinket2,
            ItemSlot::Back => Self::player_field_inv_slot_back,
            ItemSlot::MainHand => Self::player_field_inv_slot_main_hand,
            ItemSlot::OffHand => Self::player_field_inv_slot_off_hand,
            ItemSlot::RangedOrRelic => Self::player_field_inv_slot_ranged,
            ItemSlot::Tabard => Self::player_field_inv_slot_tabard,
            ItemSlot::Bag1 => Self::player_field_inv_slot_bag1,
            ItemSlot::Bag2 => Self::player_field_inv_slot_bag2,
            ItemSlot::Bag3 => Self::player_field_inv_slot_bag3,
            ItemSlot::Bag4 => Self::player_field_inv_slot_bag4,
            ItemSlot::Inventory0 => Self::player_field_pack_slot_1,
            ItemSlot::Inventory1 => Self::player_field_pack_slot_2,
            ItemSlot::Inventory2 => Self::player_field_pack_slot_3,
            ItemSlot::Inventory3 => Self::player_field_pack_slot_4,
            ItemSlot::Inventory4 => Self::player_field_pack_slot_5,
            ItemSlot::Inventory5 => Self::player_field_pack_slot_6,
            ItemSlot::Inventory6 => Self::player_field_pack_slot_7,
            ItemSlot::Inventory7 => Self::player_field_pack_slot_8,
            ItemSlot::Inventory8 => Self::player_field_pack_slot_9,
            ItemSlot::Inventory9 => Self::player_field_pack_slot_10,
            ItemSlot::Inventory10 => Self::player_field_pack_slot_11,
            ItemSlot::Inventory11 => Self::player_field_pack_slot_12,
            ItemSlot::Inventory12 => Self::player_field_pack_slot_13,
            ItemSlot::Inventory13 => Self::player_field_pack_slot_14,
            ItemSlot::Inventory14 => Self::player_field_pack_slot_15,
            ItemSlot::Inventory15 => Self::player_field_pack_slot_16,
            ItemSlot::Bank1 => Self::player_field_bank_slot_1,
            ItemSlot::Bank2 => Self::player_field_bank_slot_2,
            ItemSlot::Bank3 => Self::player_field_bank_slot_3,
            ItemSlot::Bank4 => Self::player_field_bank_slot_4,
            ItemSlot::Bank5 => Self::player_field_bank_slot_5,
            ItemSlot::Bank6 => Self::player_field_bank_slot_6,
            ItemSlot::Bank7 => Self::player_field_bank_slot_7,
            ItemSlot::Bank8 => Self::player_field_bank_slot_8,
            ItemSlot::Bank9 => Self::player_field_bank_slot_9,
            ItemSlot::Bank10 => Self::player_field_bank_slot_10,
            ItemSlot::Bank11 => Self::player_field_bank_slot_11,
            ItemSlot::Bank12 => Self::player_field_bank_slot_12,
            ItemSlot::Bank13 => Self::player_field_bank_slot_13,
            ItemSlot::Bank14 => Self::player_field_bank_slot_14,
            ItemSlot::Bank15 => Self::player_field_bank_slot_15,
            ItemSlot::Bank16 => Self::player_field_bank_slot_16,
            ItemSlot::Bank17 => Self::player_field_bank_slot_17,
            ItemSlot::Bank18 => Self::player_field_bank_slot_18,
            ItemSlot::Bank19 => Self::player_field_bank_slot_19,
            ItemSlot::Bank20 => Self::player_field_bank_slot_20,
            ItemSlot::Bank21 => Self::player_field_bank_slot_21,
            ItemSlot::Bank22 => Self::player_field_bank_slot_22,
            ItemSlot::Bank23 => Self::player_field_bank_slot_23,
            ItemSlot::Bank24 => Self::player_field_bank_slot_24,
            ItemSlot::BankBagSlot1 => Self::player_field_bankbag_slot_1,
            ItemSlot::BankBagSlot2 => Self::player_field_bankbag_slot_2,
            ItemSlot::BankBagSlot3 => Self::player_field_bankbag_slot_3,
            ItemSlot::BankBagSlot4 => Self::player_field_bankbag_slot_4,
            ItemSlot::BankBagSlot5 => Self::player_field_bankbag_slot_5,
            ItemSlot::BankBagSlot6 => Self::player_field_bankbag_slot_6,
            ItemSlot::VendorBuyback1 => Self::player_field_vendorbuyback_slot_1,
            ItemSlot::VendorBuyback2 => Self::player_field_vendorbuyback_slot_2,
            ItemSlot::VendorBuyback3 => Self::player_field_vendorbuyback_slot_3,
            ItemSlot::VendorBuyback4 => Self::player_field_vendorbuyback_slot_4,
            ItemSlot::VendorBuyback5 => Self::player_field_vendorbuyback_slot_5,
            ItemSlot::VendorBuyback6 => Self::player_field_vendorbuyback_slot_6,
            ItemSlot::VendorBuyback7 => Self::player_field_vendorbuyback_slot_7,
            ItemSlot::VendorBuyback8 => Self::player_field_vendorbuyback_slot_8,
            ItemSlot::VendorBuyback9 => Self::player_field_vendorbuyback_slot_9,
            ItemSlot::VendorBuyback10 => Self::player_field_vendorbuyback_slot_10,
            ItemSlot::VendorBuyback11 => Self::player_field_vendorbuyback_slot_11,
            ItemSlot::VendorBuyback12 => Self::player_field_vendorbuyback_slot_12,
            ItemSlot::Keyring1 => Self::player_field_keyring_slot_1,
            ItemSlot::Keyring2 => Self::player_field_keyring_slot_2,
            ItemSlot::Keyring3 => Self::player_field_keyring_slot_3,
            ItemSlot::Keyring4 => Self::player_field_keyring_slot_4,
            ItemSlot::Keyring5 => Self::player_field_keyring_slot_5,
            ItemSlot::Keyring6 => Self::player_field_keyring_slot_6,
            ItemSlot::Keyring7 => Self::player_field_keyring_slot_7,
            ItemSlot::Keyring8 => Self::player_field_keyring_slot_8,
            ItemSlot::Keyring9 => Self::player_field_keyring_slot_9,
            ItemSlot::Keyring10 => Self::player_field_keyring_slot_10,
            ItemSlot::Keyring11 => Self::player_field_keyring_slot_11,
            ItemSlot::Keyring12 => Self::player_field_keyring_slot_12,
            ItemSlot::Keyring13 => Self::player_field_keyring_slot_13,
            ItemSlot::Keyring14 => Self::player_field_keyring_slot_14,
            ItemSlot::Keyring15 => Self::player_field_keyring_slot_15,
            ItemSlot::Keyring16 => Self::player_field_keyring_slot_16,
            ItemSlot::Keyring17 => Self::player_field_keyring_slot_17,
            ItemSlot::Keyring18 => Self::player_field_keyring_slot_18,
            ItemSlot::Keyring19 => Self::player_field_keyring_slot_19,
            ItemSlot::Keyring20 => Self::player_field_keyring_slot_20,
            ItemSlot::Keyring21 => Self::player_field_keyring_slot_21,
            ItemSlot::Keyring22 => Self::player_field_keyring_slot_22,
            ItemSlot::Keyring23 => Self::player_field_keyring_slot_23,
            ItemSlot::Keyring24 => Self::player_field_keyring_slot_24,
            ItemSlot::Keyring25 => Self::player_field_keyring_slot_25,
            ItemSlot::Keyring26 => Self::player_field_keyring_slot_26,
            ItemSlot::Keyring27 => Self::player_field_keyring_slot_27,
            ItemSlot::Keyring28 => Self::player_field_keyring_slot_28,
            ItemSlot::Keyring29 => Self::player_field_keyring_slot_29,
            ItemSlot::Keyring30 => Self::player_field_keyring_slot_30,
            ItemSlot::Keyring31 => Self::player_field_keyring_slot_31,
            ItemSlot::Keyring32 => Self::player_field_keyring_slot_32,
        };

        f(self)
    }
}

impl UpdatePlayerBuilder {
    pub fn set_player_field_inv_slot(self, item_slot: ItemSlot, item: crate::Guid) -> Self {
        let f = match item_slot {
            ItemSlot::Head => Self::set_player_field_inv_slot_head,
            ItemSlot::Neck => Self::set_player_field_inv_slot_neck,
            ItemSlot::Shoulders => Self::set_player_field_inv_slot_shoulders,
            ItemSlot::Shirt => Self::set_player_field_inv_slot_body,
            ItemSlot::Chest => Self::set_player_field_inv_slot_chest,
            ItemSlot::Waist => Self::set_player_field_inv_slot_waist,
            ItemSlot::Legs => Self::set_player_field_inv_slot_legs,
            ItemSlot::Boots => Self::set_player_field_inv_slot_feet,
            ItemSlot::Wrist => Self::set_player_field_inv_slot_wrists,
            ItemSlot::Hands => Self::set_player_field_inv_slot_hands,
            ItemSlot::Ring1 => Self::set_player_field_inv_slot_finger1,
            ItemSlot::Ring2 => Self::set_player_field_inv_slot_finger2,
            ItemSlot::Trinket1 => Self::set_player_field_inv_slot_trinket1,
            ItemSlot::Trinket2 => Self::set_player_field_inv_slot_trinket2,
            ItemSlot::Back => Self::set_player_field_inv_slot_back,
            ItemSlot::MainHand => Self::set_player_field_inv_slot_main_hand,
            ItemSlot::OffHand => Self::set_player_field_inv_slot_off_hand,
            ItemSlot::RangedOrRelic => Self::set_player_field_inv_slot_ranged,
            ItemSlot::Tabard => Self::set_player_field_inv_slot_tabard,
            ItemSlot::Bag1 => Self::set_player_field_inv_slot_bag1,
            ItemSlot::Bag2 => Self::set_player_field_inv_slot_bag2,
            ItemSlot::Bag3 => Self::set_player_field_inv_slot_bag3,
            ItemSlot::Bag4 => Self::set_player_field_inv_slot_bag4,
            ItemSlot::Inventory0 => Self::set_player_field_pack_slot_1,
            ItemSlot::Inventory1 => Self::set_player_field_pack_slot_2,
            ItemSlot::Inventory2 => Self::set_player_field_pack_slot_3,
            ItemSlot::Inventory3 => Self::set_player_field_pack_slot_4,
            ItemSlot::Inventory4 => Self::set_player_field_pack_slot_5,
            ItemSlot::Inventory5 => Self::set_player_field_pack_slot_6,
            ItemSlot::Inventory6 => Self::set_player_field_pack_slot_7,
            ItemSlot::Inventory7 => Self::set_player_field_pack_slot_8,
            ItemSlot::Inventory8 => Self::set_player_field_pack_slot_9,
            ItemSlot::Inventory9 => Self::set_player_field_pack_slot_10,
            ItemSlot::Inventory10 => Self::set_player_field_pack_slot_11,
            ItemSlot::Inventory11 => Self::set_player_field_pack_slot_12,
            ItemSlot::Inventory12 => Self::set_player_field_pack_slot_13,
            ItemSlot::Inventory13 => Self::set_player_field_pack_slot_14,
            ItemSlot::Inventory14 => Self::set_player_field_pack_slot_15,
            ItemSlot::Inventory15 => Self::set_player_field_pack_slot_16,
            ItemSlot::Bank1 => Self::set_player_field_bank_slot_1,
            ItemSlot::Bank2 => Self::set_player_field_bank_slot_2,
            ItemSlot::Bank3 => Self::set_player_field_bank_slot_3,
            ItemSlot::Bank4 => Self::set_player_field_bank_slot_4,
            ItemSlot::Bank5 => Self::set_player_field_bank_slot_5,
            ItemSlot::Bank6 => Self::set_player_field_bank_slot_6,
            ItemSlot::Bank7 => Self::set_player_field_bank_slot_7,
            ItemSlot::Bank8 => Self::set_player_field_bank_slot_8,
            ItemSlot::Bank9 => Self::set_player_field_bank_slot_9,
            ItemSlot::Bank10 => Self::set_player_field_bank_slot_10,
            ItemSlot::Bank11 => Self::set_player_field_bank_slot_11,
            ItemSlot::Bank12 => Self::set_player_field_bank_slot_12,
            ItemSlot::Bank13 => Self::set_player_field_bank_slot_13,
            ItemSlot::Bank14 => Self::set_player_field_bank_slot_14,
            ItemSlot::Bank15 => Self::set_player_field_bank_slot_15,
            ItemSlot::Bank16 => Self::set_player_field_bank_slot_16,
            ItemSlot::Bank17 => Self::set_player_field_bank_slot_17,
            ItemSlot::Bank18 => Self::set_player_field_bank_slot_18,
            ItemSlot::Bank19 => Self::set_player_field_bank_slot_19,
            ItemSlot::Bank20 => Self::set_player_field_bank_slot_20,
            ItemSlot::Bank21 => Self::set_player_field_bank_slot_21,
            ItemSlot::Bank22 => Self::set_player_field_bank_slot_22,
            ItemSlot::Bank23 => Self::set_player_field_bank_slot_23,
            ItemSlot::Bank24 => Self::set_player_field_bank_slot_24,
            ItemSlot::BankBagSlot1 => Self::set_player_field_bankbag_slot_1,
            ItemSlot::BankBagSlot2 => Self::set_player_field_bankbag_slot_2,
            ItemSlot::BankBagSlot3 => Self::set_player_field_bankbag_slot_3,
            ItemSlot::BankBagSlot4 => Self::set_player_field_bankbag_slot_4,
            ItemSlot::BankBagSlot5 => Self::set_player_field_bankbag_slot_5,
            ItemSlot::BankBagSlot6 => Self::set_player_field_bankbag_slot_6,
            ItemSlot::VendorBuyback1 => Self::set_player_field_vendorbuyback_slot_1,
            ItemSlot::VendorBuyback2 => Self::set_player_field_vendorbuyback_slot_2,
            ItemSlot::VendorBuyback3 => Self::set_player_field_vendorbuyback_slot_3,
            ItemSlot::VendorBuyback4 => Self::set_player_field_vendorbuyback_slot_4,
            ItemSlot::VendorBuyback5 => Self::set_player_field_vendorbuyback_slot_5,
            ItemSlot::VendorBuyback6 => Self::set_player_field_vendorbuyback_slot_6,
            ItemSlot::VendorBuyback7 => Self::set_player_field_vendorbuyback_slot_7,
            ItemSlot::VendorBuyback8 => Self::set_player_field_vendorbuyback_slot_8,
            ItemSlot::VendorBuyback9 => Self::set_player_field_vendorbuyback_slot_9,
            ItemSlot::VendorBuyback10 => Self::set_player_field_vendorbuyback_slot_10,
            ItemSlot::VendorBuyback11 => Self::set_player_field_vendorbuyback_slot_11,
            ItemSlot::VendorBuyback12 => Self::set_player_field_vendorbuyback_slot_12,
            ItemSlot::Keyring1 => Self::set_player_field_keyring_slot_1,
            ItemSlot::Keyring2 => Self::set_player_field_keyring_slot_2,
            ItemSlot::Keyring3 => Self::set_player_field_keyring_slot_3,
            ItemSlot::Keyring4 => Self::set_player_field_keyring_slot_4,
            ItemSlot::Keyring5 => Self::set_player_field_keyring_slot_5,
            ItemSlot::Keyring6 => Self::set_player_field_keyring_slot_6,
            ItemSlot::Keyring7 => Self::set_player_field_keyring_slot_7,
            ItemSlot::Keyring8 => Self::set_player_field_keyring_slot_8,
            ItemSlot::Keyring9 => Self::set_player_field_keyring_slot_9,
            ItemSlot::Keyring10 => Self::set_player_field_keyring_slot_10,
            ItemSlot::Keyring11 => Self::set_player_field_keyring_slot_11,
            ItemSlot::Keyring12 => Self::set_player_field_keyring_slot_12,
            ItemSlot::Keyring13 => Self::set_player_field_keyring_slot_13,
            ItemSlot::Keyring14 => Self::set_player_field_keyring_slot_14,
            ItemSlot::Keyring15 => Self::set_player_field_keyring_slot_15,
            ItemSlot::Keyring16 => Self::set_player_field_keyring_slot_16,
            ItemSlot::Keyring17 => Self::set_player_field_keyring_slot_17,
            ItemSlot::Keyring18 => Self::set_player_field_keyring_slot_18,
            ItemSlot::Keyring19 => Self::set_player_field_keyring_slot_19,
            ItemSlot::Keyring20 => Self::set_player_field_keyring_slot_20,
            ItemSlot::Keyring21 => Self::set_player_field_keyring_slot_21,
            ItemSlot::Keyring22 => Self::set_player_field_keyring_slot_22,
            ItemSlot::Keyring23 => Self::set_player_field_keyring_slot_23,
            ItemSlot::Keyring24 => Self::set_player_field_keyring_slot_24,
            ItemSlot::Keyring25 => Self::set_player_field_keyring_slot_25,
            ItemSlot::Keyring26 => Self::set_player_field_keyring_slot_26,
            ItemSlot::Keyring27 => Self::set_player_field_keyring_slot_27,
            ItemSlot::Keyring28 => Self::set_player_field_keyring_slot_28,
            ItemSlot::Keyring29 => Self::set_player_field_keyring_slot_29,
            ItemSlot::Keyring30 => Self::set_player_field_keyring_slot_30,
            ItemSlot::Keyring31 => Self::set_player_field_keyring_slot_31,
            ItemSlot::Keyring32 => Self::set_player_field_keyring_slot_32,
        };

        f(self, item)
    }
}

#[cfg(test)]
mod test {
    use crate::vanilla::update_mask::indices::SkillInfoIndex;
    use crate::vanilla::update_mask::UpdatePlayer;
    use crate::vanilla::{Class, Gender, Power, Race};
    use crate::vanilla::{SkillInfo, UpdateMask};
    use crate::Guid;
    use wow_world_base::vanilla::Skill;

    #[test]
    fn most_minimal_example() {
        let b = [
            2_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            7, 0, 64, 0, 16, 0, 0, 0, // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
        ];

        let update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_unit_health(100)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn skill_info_in_out() {
        let b = [
            23_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            7, 0, 64, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 192, 1, 0, 4, 0, 0, 0, 0, 0, 0,
            0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            98, 0, 5, 0, // PLAYER_SKILL_INFO_1_1
            44, 1, 46, 1, // PLAYER_SKILL_INFO_1_2
            10, 0, 7, 0, // PLAYER_SKILL_INFO_1_3
        ];

        let skill_info = SkillInfo::new(Skill::LanguageCommon, 5, 300, 302, 7, 10);

        let update_mask = UpdateMask::Player(
            UpdatePlayer::builder()
                .set_object_guid(Guid::new(4))
                .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
                .set_unit_health(100)
                .set_player_skill_info(skill_info, SkillInfoIndex::Index0)
                .finalize(),
        );

        assert_eq!(b.len(), update_mask.size());

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
        let update_mask = UpdateMask::read(&mut b.as_slice()).unwrap();
        match update_mask {
            UpdateMask::Player(p) => {
                assert_eq!(
                    p.player_skill_info(SkillInfoIndex::Index0).unwrap(),
                    skill_info
                );
                assert_eq!(p.player_skill_info(SkillInfoIndex::Index1), None)
            }
            _ => panic!(),
        }
    }

    #[test]
    fn reset_header_add_one() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();
        update_mask.dirty_reset();

        update_mask.set_object_guid(4.into());

        let update_mask = UpdateMask::Player(update_mask);
        assert_eq!(b.len(), update_mask.size());

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_header() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
            // End of mask blocks
            // No value blocks
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();

        update_mask.dirty_reset();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn small_example() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            23, 0, 64, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            0, 0, 128, 63, // Scale (1.0)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            100, 0, 0, 0, // UNIT_FIELD_MAXHEALTH (100)
            1, 0, 0, 0, // UNIT_FIELD_LEVEL (1)
            1, 0, 0, 0, // UNIT_FIELD_FACTIONTEMPLATE (1)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            50, 0, 0, 0, // UNIT_FIELD_DISPLAYD (50, Human Female)
            50, 0, 0, 0, // UNIT_FIELD_NATIVEDISPLAYID (50, Human Female)
        ];

        let update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_dirty_and_mark_full_dirty() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            23, 0, 64, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            0, 0, 128, 63, // Scale (1.0)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            100, 0, 0, 0, // UNIT_FIELD_MAXHEALTH (100)
            1, 0, 0, 0, // UNIT_FIELD_LEVEL (1)
            1, 0, 0, 0, // UNIT_FIELD_FACTIONTEMPLATE (1)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            50, 0, 0, 0, // UNIT_FIELD_DISPLAYD (50, Human Female)
            50, 0, 0, 0, // UNIT_FIELD_NATIVEDISPLAYID (50, Human Female)
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();
        //Fully clear the dirty mask
        update_mask.dirty_reset();
        assert!(!update_mask.has_any_dirty_fields());

        //Mark completely dirty
        update_mask.mark_fully_dirty();
        assert!(update_mask.has_any_dirty_fields());

        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();

        //Output should match all previously written fields
        assert_eq!(b.as_slice(), v.as_slice());
    }
}
