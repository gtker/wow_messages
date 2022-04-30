## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_TRADE_STATUS = 0x0120 {
    TradeStatus status;
    if (status == BEGIN_TRADE) {
        Guid unknown1;
    }
    else if (status == CLOSE_WINDOW) {
        InventoryResult inventory_result;
        u8 target_error;
        u32 item_limit_category_id;
    }
    else if (status == ONLY_CONJURED
        || status == NOT_ON_TAPLIST) {
        u8 slot;
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
| 0x04 | ? / - | [TradeStatus](tradestatus.md) | status |  |

If status is equal to `BEGIN_TRADE`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | unknown1 |  |

Else If status is equal to `CLOSE_WINDOW`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | [InventoryResult](inventoryresult.md) | inventory_result |  |
| - | 1 / - | u8 | target_error |  |
| - | 4 / Little | u32 | item_limit_category_id |  |

Else If status is equal to `ONLY_CONJURED` **or** 
is equal to `NOT_ON_TAPLIST`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | slot |  |
