## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SELL_ITEM = 0x01A1 {
    Guid guid;    
    Guid item;    
    SellItemResult result;    
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
| 0x04 | 8 / Little | Guid | guid |  |
| 0x0C | 8 / Little | Guid | item |  |
| 0x14 | ? / - | SellItemResult | result |  |
