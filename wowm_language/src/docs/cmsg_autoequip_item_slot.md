# CMSG_AUTOEQUIP_ITEM_SLOT

## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_AUTOEQUIP_ITEM_SLOT = 0x010F {
    Guid guid;
    u8 destination_slot;
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
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x0E | 1 / - | u8 | destination_slot |  |  |

