## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_SET_TRADE_ITEM = 0x011D {
    u8 trade_slot;
    u8 bag;
    u8 slot;
}
```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 1 / - | u8 | trade_slot |  |  |
| 0x07 | 1 / - | u8 | bag |  |  |
| 0x08 | 1 / - | u8 | slot |  |  |
