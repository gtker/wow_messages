## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_INVENTORY_CHANGE_FAILURE = 0x0112 {
    InventoryResult result;
    if (result == CANT_EQUIP_SKILL
        || result == ITEM_DOESNT_GO_TO_SLOT
        || result == BAG_FULL
        || result == NONEMPTY_BAG_OVER_OTHER_BAG
        || result == CANT_TRADE_EQUIP_BAGS
        || result == ONLY_AMMO_CAN_GO_HERE
        || result == NO_REQUIRED_PROFICIENCY
        || result == NO_EQUIPMENT_SLOT_AVAILABLE
        || result == YOU_CAN_NEVER_USE_THAT_ITEM
        || result == YOU_CAN_NEVER_USE_THAT_ITEM2
        || result == NO_EQUIPMENT_SLOT_AVAILABLE2
        || result == CANT_EQUIP_WITH_TWOHANDED
        || result == CANT_DUAL_WIELD
        || result == ITEM_DOESNT_GO_INTO_BAG
        || result == ITEM_DOESNT_GO_INTO_BAG2
        || result == CANT_CARRY_MORE_OF_THIS
        || result == NO_EQUIPMENT_SLOT_AVAILABLE3
        || result == ITEM_CANT_STACK
        || result == ITEM_CANT_BE_EQUIPPED
        || result == ITEMS_CANT_BE_SWAPPED
        || result == SLOT_IS_EMPTY
        || result == ITEM_NOT_FOUND
        || result == CANT_DROP_SOULBOUND
        || result == OUT_OF_RANGE
        || result == TRIED_TO_SPLIT_MORE_THAN_COUNT
        || result == COULDNT_SPLIT_ITEMS
        || result == MISSING_REAGENT
        || result == NOT_ENOUGH_MONEY
        || result == NOT_A_BAG
        || result == CAN_ONLY_DO_WITH_EMPTY_BAGS
        || result == DONT_OWN_THAT_ITEM
        || result == CAN_EQUIP_ONLY1_QUIVER
        || result == MUST_PURCHASE_THAT_BAG_SLOT
        || result == TOO_FAR_AWAY_FROM_BANK
        || result == ITEM_LOCKED
        || result == YOU_ARE_STUNNED
        || result == YOU_ARE_DEAD
        || result == CANT_DO_RIGHT_NOW
        || result == INT_BAG_ERROR
        || result == CAN_EQUIP_ONLY1_BOLT
        || result == CAN_EQUIP_ONLY1_AMMOPOUCH
        || result == STACKABLE_CANT_BE_WRAPPED
        || result == EQUIPPED_CANT_BE_WRAPPED
        || result == WRAPPED_CANT_BE_WRAPPED
        || result == BOUND_CANT_BE_WRAPPED
        || result == UNIQUE_CANT_BE_WRAPPED
        || result == BAGS_CANT_BE_WRAPPED
        || result == ALREADY_LOOTED
        || result == INVENTORY_FULL
        || result == BANK_FULL
        || result == ITEM_IS_CURRENTLY_SOLD_OUT
        || result == BAG_FULL3
        || result == ITEM_NOT_FOUND2
        || result == ITEM_CANT_STACK2
        || result == BAG_FULL4
        || result == ITEM_SOLD_OUT
        || result == OBJECT_IS_BUSY
        || result == NONE
        || result == NOT_IN_COMBAT
        || result == NOT_WHILE_DISARMED
        || result == BAG_FULL6
        || result == CANT_EQUIP_RANK
        || result == CANT_EQUIP_REPUTATION
        || result == TOO_MANY_SPECIAL_BAGS
        || result == LOOT_CANT_LOOT_THAT_NOW) {
        u64 item1_guid;
        u64 item2_guid;
        u8 bag_type_subclass;
    }
    else if (result == CANT_EQUIP_LEVEL_I) {
        u32 required_level;
        u64 item1_guid2;
        u64 item2_guid2;
        u8 bag_type_subclass2;
    }
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | ? / - | [InventoryResult](inventoryresult.md) | result |  |

If result is equal to `CANT_EQUIP_SKILL` **or** 
is equal to `ITEM_DOESNT_GO_TO_SLOT` **or** 
is equal to `BAG_FULL` **or** 
is equal to `NONEMPTY_BAG_OVER_OTHER_BAG` **or** 
is equal to `CANT_TRADE_EQUIP_BAGS` **or** 
is equal to `ONLY_AMMO_CAN_GO_HERE` **or** 
is equal to `NO_REQUIRED_PROFICIENCY` **or** 
is equal to `NO_EQUIPMENT_SLOT_AVAILABLE` **or** 
is equal to `YOU_CAN_NEVER_USE_THAT_ITEM` **or** 
is equal to `YOU_CAN_NEVER_USE_THAT_ITEM2` **or** 
is equal to `NO_EQUIPMENT_SLOT_AVAILABLE2` **or** 
is equal to `CANT_EQUIP_WITH_TWOHANDED` **or** 
is equal to `CANT_DUAL_WIELD` **or** 
is equal to `ITEM_DOESNT_GO_INTO_BAG` **or** 
is equal to `ITEM_DOESNT_GO_INTO_BAG2` **or** 
is equal to `CANT_CARRY_MORE_OF_THIS` **or** 
is equal to `NO_EQUIPMENT_SLOT_AVAILABLE3` **or** 
is equal to `ITEM_CANT_STACK` **or** 
is equal to `ITEM_CANT_BE_EQUIPPED` **or** 
is equal to `ITEMS_CANT_BE_SWAPPED` **or** 
is equal to `SLOT_IS_EMPTY` **or** 
is equal to `ITEM_NOT_FOUND` **or** 
is equal to `CANT_DROP_SOULBOUND` **or** 
is equal to `OUT_OF_RANGE` **or** 
is equal to `TRIED_TO_SPLIT_MORE_THAN_COUNT` **or** 
is equal to `COULDNT_SPLIT_ITEMS` **or** 
is equal to `MISSING_REAGENT` **or** 
is equal to `NOT_ENOUGH_MONEY` **or** 
is equal to `NOT_A_BAG` **or** 
is equal to `CAN_ONLY_DO_WITH_EMPTY_BAGS` **or** 
is equal to `DONT_OWN_THAT_ITEM` **or** 
is equal to `CAN_EQUIP_ONLY1_QUIVER` **or** 
is equal to `MUST_PURCHASE_THAT_BAG_SLOT` **or** 
is equal to `TOO_FAR_AWAY_FROM_BANK` **or** 
is equal to `ITEM_LOCKED` **or** 
is equal to `YOU_ARE_STUNNED` **or** 
is equal to `YOU_ARE_DEAD` **or** 
is equal to `CANT_DO_RIGHT_NOW` **or** 
is equal to `INT_BAG_ERROR` **or** 
is equal to `CAN_EQUIP_ONLY1_BOLT` **or** 
is equal to `CAN_EQUIP_ONLY1_AMMOPOUCH` **or** 
is equal to `STACKABLE_CANT_BE_WRAPPED` **or** 
is equal to `EQUIPPED_CANT_BE_WRAPPED` **or** 
is equal to `WRAPPED_CANT_BE_WRAPPED` **or** 
is equal to `BOUND_CANT_BE_WRAPPED` **or** 
is equal to `UNIQUE_CANT_BE_WRAPPED` **or** 
is equal to `BAGS_CANT_BE_WRAPPED` **or** 
is equal to `ALREADY_LOOTED` **or** 
is equal to `INVENTORY_FULL` **or** 
is equal to `BANK_FULL` **or** 
is equal to `ITEM_IS_CURRENTLY_SOLD_OUT` **or** 
is equal to `BAG_FULL3` **or** 
is equal to `ITEM_NOT_FOUND2` **or** 
is equal to `ITEM_CANT_STACK2` **or** 
is equal to `BAG_FULL4` **or** 
is equal to `ITEM_SOLD_OUT` **or** 
is equal to `OBJECT_IS_BUSY` **or** 
is equal to `NONE` **or** 
is equal to `NOT_IN_COMBAT` **or** 
is equal to `NOT_WHILE_DISARMED` **or** 
is equal to `BAG_FULL6` **or** 
is equal to `CANT_EQUIP_RANK` **or** 
is equal to `CANT_EQUIP_REPUTATION` **or** 
is equal to `TOO_MANY_SPECIAL_BAGS` **or** 
is equal to `LOOT_CANT_LOOT_THAT_NOW`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 8 / Little | u64 | item1_guid |  |
| - | 8 / Little | u64 | item2_guid |  |
| - | 1 / - | u8 | bag_type_subclass |  |

Else If result is equal to `CANT_EQUIP_LEVEL_I`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | required_level |  |
| - | 8 / Little | u64 | item1_guid2 |  |
| - | 8 / Little | u64 | item2_guid2 |  |
| - | 1 / - | u8 | bag_type_subclass2 |  |
