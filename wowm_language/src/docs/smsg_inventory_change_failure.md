# SMSG_INVENTORY_CHANGE_FAILURE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_INVENTORY_CHANGE_FAILURE = 0x0112 {
    InventoryResult result;
    if (result == CANT_EQUIP_LEVEL_I) {
        u32 required_level;
    }
    if (result != OK) {
        u64 item1_guid;
        u64 item2_guid;
        u8 bag_type_subclass;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | ? / - | [InventoryResult](inventoryresult.md) | result |  |  |

If result is equal to `CANT_EQUIP_LEVEL_I`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | required_level |  |  |

If result is not equal to `OK`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 8 / Little | u64 | item1_guid |  |  |
| - | 8 / Little | u64 | item2_guid |  |  |
| - | 1 / - | u8 | bag_type_subclass |  | cmangos: bag type subclass, used with EQUIP_ERR_EVENT_AUTOEQUIP_BIND_CONFIRM and EQUIP_ERR_ITEM_DOESNT_GO_INTO_BAG2<br/>vmangos sets to 0 |

